import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import {auth} from "$lib/server/auth";

export const GET: RequestHandler = async ({ url, locals, request }) => {
    const userCode = url.searchParams.get('user_code');
    if (!userCode) {
        throw error(400, 'Missing user_code');
    }

    if (!locals?.user?.id) {
        throw error(401, 'Unauthorized');
    }

    await auth.api.deviceApprove({
        body: { userCode },
        headers: request.headers
    })

    return new Response('You can now close this window and return to the CLI');
};