import { desc, eq } from 'drizzle-orm'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { user, customDomain, subdomain, persistentPort } = schema
  const owner = { id: user.id, name: user.name, email: user.email }

  const [domains, subdomains, ports] = await Promise.all([
    db.select({ id: customDomain.id, domain: customDomain.domain, createdAt: customDomain.createdAt, owner })
      .from(customDomain).innerJoin(user, eq(customDomain.userId, user.id)).orderBy(desc(customDomain.createdAt)),
    db.select({ id: subdomain.id, subdomain: subdomain.subdomain, createdAt: subdomain.createdAt, owner })
      .from(subdomain).innerJoin(user, eq(subdomain.userId, user.id)).orderBy(desc(subdomain.createdAt)),
    db.select({ id: persistentPort.id, port: persistentPort.port, description: persistentPort.description, createdAt: persistentPort.createdAt, owner })
      .from(persistentPort).innerJoin(user, eq(persistentPort.userId, user.id)).orderBy(desc(persistentPort.createdAt))
  ])

  return { domains, subdomains, ports }
})
