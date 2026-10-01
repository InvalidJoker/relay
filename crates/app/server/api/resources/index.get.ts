import { desc, eq } from 'drizzle-orm'

export default defineEventHandler(async (event) => {
  const user = await requireUser(event)
  const { persistentPort, customDomain, subdomain } = schema

  const [ports, domains, subdomains] = await Promise.all([
    db.select().from(persistentPort).where(eq(persistentPort.userId, user.id)).orderBy(desc(persistentPort.createdAt)),
    db.select().from(customDomain).where(eq(customDomain.userId, user.id)).orderBy(desc(customDomain.createdAt)),
    db.select().from(subdomain).where(eq(subdomain.userId, user.id)).orderBy(desc(subdomain.createdAt))
  ])

  return { ports, domains, subdomains }
})
