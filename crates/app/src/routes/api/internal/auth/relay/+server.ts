import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import {auth} from "$lib/server/auth";
import { z } from "zod";
import {db} from "$lib/server/db";
import {and, eq} from "drizzle-orm";
import {persistentPort, customDomain, subdomain} from "$lib/server/db/relay.schema";
import { env as envPublic } from "$env/dynamic/public";

const RelayTypeSchema = z.enum(["tcp", "http"]);

const HeadersSchema = z.object({
    relayType: RelayTypeSchema,
    provided: z.string().optional(),
});

const relayPubDomain = envPublic.PUBLIC_RELAY_DOMAIN || "relay.invalidjoker.dev";
const relayTcpDomain = envPublic.PUBLIC_TCP_DOMAIN || "tcp.invalidjoker.dev";

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

    const {relayType, provided} = parsed.data;

    console.log(`Received request for relay type: ${relayType} with provided: ${provided}`);

    let result: string | null = null;

    if (relayType === "tcp") {
        if (provided == null || provided == "") return new Response(); // always allowed

        let port: number;
        try {
            port = parseInt(provided, 10);
            if (isNaN(port) || port < 1 || port > 65535) {
                throw new Error('Invalid port number');
            }
        } catch (e) {
            throw error(400, 'Provided value is not a valid port number');
        }

        const ports = await db.query.persistentPort.findFirst({
            where: and(eq(persistentPort.port, port), eq(persistentPort.userId, session.user.id))
        });

        if (!ports) {
            throw error(403, 'Forbidden: You do not have access to this port');
        }

        result = relayTcpDomain + ":" + port;
    } else if (relayType === "http" && (provided != null && provided != "")) {
        let hostname: string;
        try {
            hostname = provided;
            if (hostname.length === 0 || hostname.length > 255) {
                throw new Error('Invalid hostname length');
            }
        } catch (e) {
            throw error(400, 'Provided value is not a valid hostname');
        }

        const subdomainRecord = await db.query.subdomain.findFirst({
            where: and(eq(subdomain.subdomain, hostname), eq(subdomain.userId, session.user.id))
        });

        // if subdomain return subdomain.<envconfigured>
        // if domain return <domain>

        if (subdomainRecord) {
            result = `${subdomainRecord.subdomain}.${relayPubDomain}`;
        } else {
            const domainRecord = await db.query.customDomain.findFirst({
                where: and(eq(customDomain.domain, hostname), eq(customDomain.userId, session.user.id))
            });

            if (!domainRecord) {
                throw error(403, 'Forbidden: You do not have access to this domain');
            }

            result = domainRecord.domain;
        }
    } else if (relayType === "http") {
        const adjectives = [
            "swift", "bright", "silent", "rapid", "lucky",
            "crimson", "golden", "frosty", "stellar", "shadow"
        ];

        const nouns = [
            "fox", "wolf", "hawk", "river", "cloud",
            "forest", "comet", "flame", "storm", "peak"
        ];

        let randomSubdomain: string | null = null;

        for (let i = 0; i < 5; i++) {
            const candidate =
                `${adjectives[Math.floor(Math.random() * adjectives.length)]}` +
                `${nouns[Math.floor(Math.random() * nouns.length)]}`;

            const existing = await db.query.subdomain.findFirst({
                where: eq(subdomain.subdomain, candidate)
            });

            if (!existing) {
                randomSubdomain = candidate;
                break;
            }
        }

        if (!randomSubdomain) {
            throw error(500, "Failed to generate unique subdomain after 5 attempts");
        }

        result = `${randomSubdomain}.${relayPubDomain}`;
    } else {
        throw error(400, 'Invalid relay type');
    }

    return json({ success: true, result: result });
};