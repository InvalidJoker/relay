import { count, desc, eq, gte, inArray, type SQL } from 'drizzle-orm'
import type { PgTable } from 'drizzle-orm/pg-core'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { user } = schema
  const since = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000)
  const total = (table: PgTable, where?: SQL) =>
    db.select({ value: count() }).from(table).where(where).then(([row]) => row?.value ?? 0)

  const [users, banned, admins, newUsers, ports, domains, subdomains, recentUsers, totals] = await Promise.all([
    total(user),
    total(user, eq(user.banned, true)),
    total(user, eq(user.role, 'admin')),
    total(user, gte(user.createdAt, since)),
    total(schema.persistentPort),
    total(schema.customDomain),
    total(schema.subdomain),
    db.select({ id: user.id, name: user.name, email: user.email, role: user.role, banned: user.banned, createdAt: user.createdAt })
      .from(user).orderBy(desc(user.createdAt)).limit(5),
    resourceTotals()
  ])

  const top = [...totals.entries()].sort((a, b) => b[1] - a[1]).slice(0, 5)
  const topRows = top.length
    ? await db.select({ id: user.id, name: user.name, email: user.email }).from(user).where(inArray(user.id, top.map(([id]) => id)))
    : []
  const topUsers = top.flatMap(([id, resources]) => {
    const row = topRows.find(u => u.id === id)
    return row ? [{ ...row, resources }] : []
  })

  return {
    stats: { users, banned, admins, newUsers, ports, domains, subdomains },
    recentUsers,
    topUsers
  }
})
