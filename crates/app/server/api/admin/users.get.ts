import { count, desc, ilike, or } from 'drizzle-orm'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { user } = schema
  const q = String(getQuery(event).q ?? '').trim()
  const where = q ? or(ilike(user.name, `%${q}%`), ilike(user.email, `%${q}%`)) : undefined

  const [rows, [totalRow]] = await Promise.all([
    db.select({
      id: user.id,
      name: user.name,
      email: user.email,
      emailVerified: user.emailVerified,
      role: user.role,
      banned: user.banned,
      banReason: user.banReason,
      createdAt: user.createdAt
    }).from(user).where(where).orderBy(desc(user.createdAt)).limit(100),
    db.select({ value: count() }).from(user).where(where)
  ])

  const totals = await resourceTotals(rows.map(r => r.id))
  return {
    users: rows.map(r => ({ ...r, resources: totals.get(r.id) ?? 0 })),
    total: totalRow?.value ?? 0
  }
})
