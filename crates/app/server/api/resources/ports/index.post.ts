import { z } from 'zod'

export default defineEventHandler(async (event) => {
  const user = await requireUser(event)
  const { portRange } = relayConfig()

  const body = await readValidatedBody(event, z.object({
    port: z.coerce.number().int(),
    description: z.string().trim().max(120).optional()
  }).parse)

  if (body.port < portRange.start || body.port > portRange.end) {
    throw createError({ statusCode: 400, statusMessage: `Remote port must be between ${portRange.start} and ${portRange.end}.` })
  }
  if (user.role !== 'admin' && await countOwned('ports', user.id) >= LIMITS.ports) {
    throw createError({ statusCode: 400, statusMessage: `Maximum of ${LIMITS.ports} ports allowed.` })
  }

  try {
    const [row] = await db.insert(schema.persistentPort).values({
      id: crypto.randomUUID(),
      port: body.port,
      description: body.description || null,
      userId: user.id
    }).returning()
    return row
  } catch (error) {
    if (isUniqueViolation(error)) throw createError({ statusCode: 409, statusMessage: 'This port is already reserved.' })
    throw error
  }
})
