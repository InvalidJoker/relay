import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { db } from '$lib/server/db';
import { user, session, persistentPort, customDomain, subdomain } from '$lib/server/db/schema';
import { count, desc, eq, ilike, inArray, or } from 'drizzle-orm';

export const load: PageServerLoad = async ({ url }) => {
	const q = url.searchParams.get('q')?.trim() ?? '';

	const where = q
		? or(ilike(user.name, `%${q}%`), ilike(user.email, `%${q}%`))
		: undefined;

	const rows = await db
		.select({
			id: user.id,
			name: user.name,
			email: user.email,
			emailVerified: user.emailVerified,
			role: user.role,
			banned: user.banned,
			banReason: user.banReason,
			createdAt: user.createdAt
		})
		.from(user)
		.where(where)
		.orderBy(desc(user.createdAt))
		.limit(100);

	const [{ value: total }] = await db.select({ value: count() }).from(user).where(where);

	// Resource counts for the listed users, aggregated in JS.
	const ids = rows.map((r) => r.id);
	const resourceTotals = new Map<string, number>();

	if (ids.length > 0) {
		const [portCounts, domainCounts, subCounts] = await Promise.all([
			db
				.select({ userId: persistentPort.userId, value: count() })
				.from(persistentPort)
				.where(inArray(persistentPort.userId, ids))
				.groupBy(persistentPort.userId),
			db
				.select({ userId: customDomain.userId, value: count() })
				.from(customDomain)
				.where(inArray(customDomain.userId, ids))
				.groupBy(customDomain.userId),
			db
				.select({ userId: subdomain.userId, value: count() })
				.from(subdomain)
				.where(inArray(subdomain.userId, ids))
				.groupBy(subdomain.userId)
		]);

		for (const row of [...portCounts, ...domainCounts, ...subCounts]) {
			resourceTotals.set(row.userId, (resourceTotals.get(row.userId) ?? 0) + row.value);
		}
	}

	const users = rows.map((r) => ({ ...r, resources: resourceTotals.get(r.id) ?? 0 }));

	return { users, total, q };
};

export const actions: Actions = {
	setRole: async (event) => {
		const me = event.locals.user;
		if (!me) return fail(401);

		const form = await event.request.formData();
		const id = form.get('id')?.toString();
		const role = form.get('role')?.toString();

		if (!id || (role !== 'admin' && role !== 'user')) return fail(400, { message: 'Invalid request.' });
		if (id === me.id) return fail(400, { message: 'You cannot change your own role.' });

		await db.update(user).set({ role }).where(eq(user.id, id));
		return { success: true };
	},

	ban: async (event) => {
		const me = event.locals.user;
		if (!me) return fail(401);

		const form = await event.request.formData();
		const id = form.get('id')?.toString();
		const reason = form.get('reason')?.toString()?.trim() || null;

		if (!id) return fail(400, { message: 'Missing user id.' });
		if (id === me.id) return fail(400, { message: 'You cannot ban yourself.' });

		await db.update(user).set({ banned: true, banReason: reason }).where(eq(user.id, id));
		// Revoke all active sessions for the banned user.
		await db.delete(session).where(eq(session.userId, id));
		return { success: true };
	},

	unban: async (event) => {
		if (!event.locals.user) return fail(401);

		const form = await event.request.formData();
		const id = form.get('id')?.toString();
		if (!id) return fail(400, { message: 'Missing user id.' });

		await db
			.update(user)
			.set({ banned: false, banReason: null, banExpires: null })
			.where(eq(user.id, id));
		return { success: true };
	},

	remove: async (event) => {
		const me = event.locals.user;
		if (!me) return fail(401);

		const form = await event.request.formData();
		const id = form.get('id')?.toString();

		if (!id) return fail(400, { message: 'Missing user id.' });
		if (id === me.id) return fail(400, { message: 'You cannot delete your own account.' });

		// Resources, sessions and accounts cascade via the foreign keys.
		await db.delete(user).where(eq(user.id, id));
		return { success: true };
	}
};
