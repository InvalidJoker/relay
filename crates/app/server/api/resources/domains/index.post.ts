import { z } from 'zod'

export default defineEventHandler(async (event) => {
  const user = await requireUser(event)

  const { domain } = await readValidatedBody(event, z.object({
    domain: z.string().trim().toLowerCase().regex(DOMAIN_RE, 'Enter a valid domain like example.com.')
  }).parse)

  if (user.role !== 'admin' && await countOwned('domains', user.id) >= LIMITS.domains) {
    throw createError({ statusCode: 400, statusMessage: `Maximum of ${LIMITS.domains} custom domain allowed.` })
  }

  try {
    const [row] = await db.insert(schema.customDomain).values({ id: crypto.randomUUID(), domain, userId: user.id }).returning()
    return row
  } catch (error) {
    if (isUniqueViolation(error)) throw createError({ statusCode: 409, statusMessage: 'This domain is already in use.' })
    throw error
  }
})
