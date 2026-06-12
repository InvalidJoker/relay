import type { Handle } from '@sveltejs/kit';
import { building } from '$app/environment';
import { auth } from '$lib/server/auth';
import { svelteKitHandler } from 'better-auth/svelte-kit';
import { sequence } from '@sveltejs/kit/hooks';

const handleBetterAuth: Handle = async ({ event, resolve }) => {
	const session = await auth.api.getSession({ headers: event.request.headers });

	if (session) {
		event.locals.session = session.session;
		event.locals.user = session.user;
	}

	return svelteKitHandler({ event, resolve, auth, building });
};

const handleAdmin: Handle = async ({ event, resolve }) => {
	if (event.url.pathname.startsWith('/admin')) {
		if (!event.locals.user || event.locals.user?.role != "admin") {
			return new Response('Unauthorized', { status: 401 });
		}
	}

	return resolve(event);
}

export const handle: Handle = sequence(handleBetterAuth, handleAdmin);
