import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { db } from '$lib/server/db';
import { user, persistentPort, customDomain, subdomain } from '$lib/server/db/schema';
import { desc, eq } from 'drizzle-orm';

export const load: PageServerLoad = async () => {
	const owner = { id: user.id, name: user.name, email: user.email };

	const [domains, subdomains, ports] = await Promise.all([
		db
			.select({
				id: customDomain.id,
				domain: customDomain.domain,
				createdAt: customDomain.createdAt,
				owner
			})
			.from(customDomain)
			.innerJoin(user, eq(customDomain.userId, user.id))
			.orderBy(desc(customDomain.createdAt)),
		db
			.select({
				id: subdomain.id,
				subdomain: subdomain.subdomain,
				createdAt: subdomain.createdAt,
				owner
			})
			.from(subdomain)
			.innerJoin(user, eq(subdomain.userId, user.id))
			.orderBy(desc(subdomain.createdAt)),
		db
			.select({
				id: persistentPort.id,
				port: persistentPort.port,
				description: persistentPort.description,
				createdAt: persistentPort.createdAt,
				owner
			})
			.from(persistentPort)
			.innerJoin(user, eq(persistentPort.userId, user.id))
			.orderBy(desc(persistentPort.createdAt))
	]);

	return { domains, subdomains, ports };
};

export const actions: Actions = {
	removeDomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const id = (await event.request.formData()).get('id')?.toString();
		if (!id) return fail(400, { message: 'Missing id.' });
		await db.delete(customDomain).where(eq(customDomain.id, id));
		return { success: true };
	},
	removeSubdomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const id = (await event.request.formData()).get('id')?.toString();
		if (!id) return fail(400, { message: 'Missing id.' });
		await db.delete(subdomain).where(eq(subdomain.id, id));
		return { success: true };
	},
	removePort: async (event) => {
		if (!event.locals.user) return fail(401);
		const id = (await event.request.formData()).get('id')?.toString();
		if (!id) return fail(400, { message: 'Missing id.' });
		await db.delete(persistentPort).where(eq(persistentPort.id, id));
		return { success: true };
	}
};
