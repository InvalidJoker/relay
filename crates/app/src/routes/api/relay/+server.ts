import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import {auth} from "$lib/server/auth";
import { z } from "zod";

const RelayTypeSchema = z.enum(["tcp", "http"]);

const HeadersSchema = z.object({
    relayType: RelayTypeSchema,
    provided: z.string().optional(),
});

export const GET: RequestHandler = async ({ request }) => {
    const session = await auth.api.getSession({
        headers: request.headers
    });

    if (!session) {
        throw error(401, 'Unauthorized');
    }

    const parsed = HeadersSchema.safeParse({
        relayType: request.headers.get("X-Relay-Type"),
        provided: request.headers.get("X-Provided"),
    });

    if (!parsed.success) {
        throw error(400, JSON.stringify(z.prettifyError(parsed.error)));
    }

    const { relayType, provided } = parsed.data;

    console.log(`Received request for relay type: ${relayType} with provided: ${provided}`);

    return new Response();
};