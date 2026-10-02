// Read at runtime so the Docker image can be configured with the same env vars as before.
export function relayConfig() {
  const env = process.env
  return {
    relayDomain: env.PUBLIC_RELAY_DOMAIN || 'relay.invalidjoker.dev',
    tcpDomain: env.PUBLIC_TCP_DOMAIN || 'tcp.invalidjoker.dev',
    relayUrl: env.RELAY_URL ?? '',
    authRequired: env.REQUIRE_AUTH == null ? true : env.REQUIRE_AUTH.toLowerCase() === 'true',
    portRange: {
      start: Number(env.PUBLIC_PORT_RANGE_START) || 10000,
      end: Number(env.PUBLIC_PORT_RANGE_END) || 20000
    }
  }
}

export const LIMITS = { ports: 2, domains: 1, subdomains: 3 } as const
