import { eq } from 'drizzle-orm'
import { timingSafeEqual } from 'node:crypto'

export interface RelayTarget {
  id: string
  name: string
  region: string | null
  host: string
  httpDomain: string
  tcpDomain: string
}

export function defaultRelay(): RelayTarget {
  const { relayUrl, relayDomain, tcpDomain } = relayConfig()
  return { id: 'default', name: 'default', region: null, host: relayUrl, httpDomain: relayDomain, tcpDomain }
}

export async function listRelays(): Promise<RelayTarget[]> {
  const rows = await db.select().from(schema.relay).where(eq(schema.relay.enabled, true))
  return rows.length ? rows : [defaultRelay()]
}

function safeEqual(a: string, b: string) {
  const left = Buffer.from(a)
  const right = Buffer.from(b)
  return left.length === right.length && timingSafeEqual(left, right)
}

// Resolves the relay calling the panel from its X-Relay-Key. Without a key the default relay is assumed.
export async function resolveRelay(key: string | undefined): Promise<RelayTarget | null> {
  if (!key) return defaultRelay()
  const row = await db.query.relay.findFirst({ where: eq(schema.relay.secret, key) })
  if (!row || !row.enabled || !safeEqual(row.secret, key)) return null
  return row
}
