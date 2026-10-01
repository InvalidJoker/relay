import { z } from 'zod'

export default defineEventHandler(async (event) => {
  const user = await requireUser(event)

  const { subdomain } = await readValidatedBody(event, z.object({
    subdomain: z.string().trim().toLowerCase().regex(SUBDOMAIN_RE, 'Use lowercase letters, numbers and hyphens.')
  }).parse)

  if (user.role !== 'admin' && await countOwned('subdomains', user.id) >= LIMITS.subdomains) {
    throw createError({ statusCode: 400, statusMessage: `Maximum of ${LIMITS.subdomains} subdomains allowed.` })
  }

  try {
    const [row] = await db.insert(schema.subdomain).values({ id: crypto.randomUUID(), subdomain, userId: user.id }).returning()
    return row
  } catch (error) {
    if (isUniqueViolation(error)) throw createError({ statusCode: 409, statusMessage: 'This subdomain is already taken.' })
    throw error
  }
})
