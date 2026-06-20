import { redirect, fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import type { PageServerLoad } from './$types';
import { auth } from '$lib/server/auth';
import { db } from '$lib/server/db';
import { persistentPort, customDomain, subdomain } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { env } from '$env/dynamic/public';

const PORT_RANGE_START = Number(env.PUBLIC_PORT_RANGE_START) || 10000;
const PORT_RANGE_END = Number(env.PUBLIC_PORT_RANGE_END) || 20000;

export const load: PageServerLoad = async (event) => {

	const userId = event.locals.user.id;
	
	const ports = await db.query.persistentPort.findMany({
		where: eq(persistentPort.userId, userId)
	});
	
	const domains = await db.query.customDomain.findMany({
		where: eq(customDomain.userId, userId)
	});
	
	const subdomains = await db.query.subdomain.findMany({
		where: eq(subdomain.userId, userId)
	});

	return {
		user: event.locals.user,
		ports,
		domains,
		subdomains,
		portRange: { start: PORT_RANGE_START, end: PORT_RANGE_END }
	};
};

export const actions: Actions = {
	signOut: async (event) => {
		await auth.api.signOut({
			headers: event.request.headers
		});
		return redirect(302, '/login');
	},
	addPort: async (event) => {
		if (!event.locals.user) return fail(401);
		const userId = event.locals.user.id;
		
		const ports = await db.query.persistentPort.findMany({
			where: eq(persistentPort.userId, userId)
		});
		
		if (ports.length >= 2) {
			return fail(400, { message: "Maximum of 2 ports allowed." });
		}
		
		const formData = await event.request.formData();
		const portStr = formData.get('port')?.toString();
		const description = formData.get('description')?.toString() || "";
		
		if (!portStr) return fail(400, { message: "Port is required." });
		const port = parseInt(portStr);
		if (isNaN(port) || port < PORT_RANGE_START || port > PORT_RANGE_END) {
			return fail(400, {
				message: `Remote port must be between ${PORT_RANGE_START} and ${PORT_RANGE_END}.`
			});
		}
		
		try {
			await db.insert(persistentPort).values({
				id: crypto.randomUUID(),
				port,
				description,
				userId
			});
		} catch (e) {
			console.error(e);
			return fail(400, { message: "Port already in use or error." });
		}
		return { success: true };
	},
	removePort: async (event) => {
		if (!event.locals.user) return fail(401);
		const formData = await event.request.formData();
		const id = formData.get('id')?.toString();
		if (!id) return fail(400);
		
		await db.delete(persistentPort).where(eq(persistentPort.id, id));
		return { success: true };
	},
	addDomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const userId = event.locals.user.id;
		
		const domains = await db.query.customDomain.findMany({
			where: eq(customDomain.userId, userId)
		});
		
		if (domains.length >= 1) {
			return fail(400, { message: "Maximum of 1 custom domain allowed." });
		}
		
		const formData = await event.request.formData();
		const domain = formData.get('domain')?.toString();

		if (!domain) return fail(400, { message: "Domain and target port required." });

		try {
			await db.insert(customDomain).values({
				id: crypto.randomUUID(),
				domain,
				userId
			});
		} catch (e) {
			console.error(e);
			return fail(400, { message: "Domain already in use or error." });
		}
		return { success: true };
	},
	removeDomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const formData = await event.request.formData();
		const id = formData.get('id')?.toString();
		if (!id) return fail(400);
		
		await db.delete(customDomain).where(eq(customDomain.id, id));
		return { success: true };
	},
	addSubdomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const userId = event.locals.user.id;
		
		const subdomains = await db.query.subdomain.findMany({
			where: eq(subdomain.userId, userId)
		});
		
		if (subdomains.length >= 3) {
			return fail(400, { message: "Maximum of 3 subdomains allowed." });
		}
		
		const formData = await event.request.formData();
		const sub = formData.get('subdomain')?.toString();

		if (!sub) return fail(400, { message: "Subdomain and target port required." });

		// todo: validate if already exists

		try {
			await db.insert(subdomain).values({
				id: crypto.randomUUID(),
				subdomain: sub,
				userId
			});
		} catch (e) {
			console.error(e);
			return fail(400, { message: "Subdomain already in use or error." });
		}
		return { success: true };
	},
	removeSubdomain: async (event) => {
		if (!event.locals.user) return fail(401);
		const formData = await event.request.formData();
		const id = formData.get('id')?.toString();
		if (!id) return fail(400);
		
		await db.delete(subdomain).where(eq(subdomain.id, id));
		return { success: true };
	}
};
