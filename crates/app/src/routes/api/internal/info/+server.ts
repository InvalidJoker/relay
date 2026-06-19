import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';
import { env as envPublic } from "$env/dynamic/public";

const relayPubDomain = envPublic.PUBLIC_RELAY_DOMAIN || "relay.invalidjoker.dev";
const authRequired = env.REQUIRE_AUTH == null ? true : env.REQUIRE_AUTH.toLowerCase() === "true";

export const GET: RequestHandler = async ({ request }) => {
    return json({
        relay_url: env.RELAY_URL,
        public_domain: relayPubDomain,
        auth_required: authRequired
    });
};