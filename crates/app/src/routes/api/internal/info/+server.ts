import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const relayPubDomain = process.env.RELAY_PUBLIC_DOMAIN || "relay.invalidjoker.dev";

export const GET: RequestHandler = async ({ request }) => {
    return json({
        relay_url: env.RELAY_URL,
        public_domain: relayPubDomain
    });
};