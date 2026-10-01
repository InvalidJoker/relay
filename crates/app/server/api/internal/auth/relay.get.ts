import { and, eq } from 'drizzle-orm'
import { z } from 'zod'

const HeadersSchema = z.object({
  relayType: z.enum(['tcp', 'http']),
  provided: z.string().nullish()
})

const ADJECTIVES = ['swift', 'bright', 'silent', 'rapid', 'lucky', 'crimson', 'golden', 'frosty', 'stellar', 'shadow']
const NOUNS = ['fox', 'wolf', 'hawk', 'river', 'cloud', 'forest', 'comet', 'flame', 'storm', 'peak']

const pick = (list: string[]) => list[Math.floor(Math.random() * list.length)]

// Called by the relay server to resolve which public address a tunnel may use.
export default defineEventHandler(async (event) => {
  const session = await getAuthSession(event)
  if (!session) throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })

  const parsed = HeadersSchema.safeParse({
    relayType: getHeader(event, 'x-relay-type'),
    provided: getHeader(event, 'x-provided')
  })
  if (!parsed.success) {
    throw createError({ statusCode: 400, statusMessage: z.prettifyError(parsed.error) })
  }

  const { relayType, provided } = parsed.data
  const { relayDomain, tcpDomain } = relayConfig()
  const userId = session.user.id

  if (relayType === 'tcp') {
    if (!provided) return { success: true, result: tcpDomain }

    const port = Number.parseInt(provided, 10)
    if (Number.isNaN(port) || port < 1 || port > 65535) {
      throw createError({ statusCode: 400, statusMessage: 'Provided value is not a valid port number' })
    }

    const owned = await db.query.persistentPort.findFirst({
      where: and(eq(schema.persistentPort.port, port), eq(schema.persistentPort.userId, userId))
    })
    if (!owned) throw createError({ statusCode: 403, statusMessage: 'Forbidden: You do not have access to this port' })

    return { success: true, result: `${tcpDomain}:${port}` }
  }

  if (provided) {
    if (provided.length > 255) {
      throw createError({ statusCode: 400, statusMessage: 'Provided value is not a valid hostname' })
    }

    const sub = await db.query.subdomain.findFirst({
      where: and(eq(schema.subdomain.subdomain, provided), eq(schema.subdomain.userId, userId))
    })
    if (sub) return { success: true, result: `${sub.subdomain}.${relayDomain}` }

    const domain = await db.query.customDomain.findFirst({
      where: and(eq(schema.customDomain.domain, provided), eq(schema.customDomain.userId, userId))
    })
    if (!domain) throw createError({ statusCode: 403, statusMessage: 'Forbidden: You do not have access to this domain' })

    return { success: true, result: domain.domain }
  }

  for (let i = 0; i < 5; i++) {
    const candidate = `${pick(ADJECTIVES)}${pick(NOUNS)}`
    const taken = await db.query.subdomain.findFirst({ where: eq(schema.subdomain.subdomain, candidate) })
    if (!taken) return { success: true, result: `${candidate}.${relayDomain}` }
  }

  throw createError({ statusCode: 500, statusMessage: 'Failed to generate unique subdomain after 5 attempts' })
})
