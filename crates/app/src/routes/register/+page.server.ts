import { fail, redirect } from '@sveltejs/kit';
import { auth } from '$lib/server/auth';
import { APIError } from 'better-auth/api';
import type {PageServerLoad} from "./$types";

export const load: PageServerLoad = (event) => {
    if (event.locals.user) {
        return redirect(302, '/dashboard');
    }
};

export const actions = {
    signUpEmail: async (event) => {
        const formData = await event.request.formData();

        try {
            await auth.api.signUpEmail({
                body: {
                    name: String(formData.get('name')),
                    email: String(formData.get('email')),
                    password: String(formData.get('password'))
                },
                headers: event.request.headers
            });
        } catch (error) {
            if (error instanceof APIError) {
                return fail(400, {
                    message: error.message
                });
            }

            return fail(500, {
                message: 'Unexpected error'
            });
        }

        throw redirect(302, '/dashboard');
    }
};