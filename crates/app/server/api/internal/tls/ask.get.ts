import { eq } from 'drizzle-orm'

// Caddy's on-demand TLS calls this before issuing a certificate: 200 allows it, anything else denies.
export default defineEventHandler(async (event) => {
  const domain = String(getQuery(event).domain ?? '').toLowerCase()
  if (!domain) throw createError({ statusCode: 400, statusMessage: 'Missing domain' })

  const known = await db.query.customDomain.findFirst({ where: eq(schema.customDomain.domain, domain) })
  if (!known) throw createError({ statusCode: 404, statusMessage: 'Unknown domain' })

  return { ok: true }
})
