import { randomBytes } from 'node:crypto'
import { z } from 'zod'

const host = z.string().trim().toLowerCase().regex(DOMAIN_RE, 'Not a valid hostname')

const Body = z.object({
  name: z.string().trim().min(1).max(64),
  region: z.string().trim().max(64).optional(),
  host,
  httpDomain: host,
  tcpDomain: host
})

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const body = await readValidatedBody(event, Body.parse)

  try {
    const [created] = await db.insert(schema.relay).values({
      id: crypto.randomUUID(),
      ...body,
      region: body.region || null,
      secret: randomBytes(32).toString('hex')
    }).returning()
    if (!created) throw createError({ statusCode: 500, statusMessage: 'Failed to create relay' })
    return created
  } catch (error) {
    if (isUniqueViolation(error)) throw createError({ statusCode: 409, statusMessage: 'A relay with this host or domain already exists' })
    throw error
  }
})
