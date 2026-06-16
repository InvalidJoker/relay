import type { PageServerLoad } from './$types';
import type {Actions} from "../../.svelte-kit/types/src/routes/dashboard/$types";
import {auth} from "$lib/server/auth.ts";
import {redirect} from "@sveltejs/kit";

export const load: PageServerLoad = async (event) => {
	return {
		user: event.locals.user ?? null
	};
};

export const actions: Actions = {
	signOut: async (event) => {
		await auth.api.signOut({
			headers: event.request.headers
		});
		return redirect(302, '/login');
	}
};