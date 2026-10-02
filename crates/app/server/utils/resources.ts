import { count, eq, inArray } from 'drizzle-orm'
import type { PgTable } from 'drizzle-orm/pg-core'

export type ResourceKind = 'ports' | 'domains' | 'subdomains'

export const resourceTables = {
  ports: schema.persistentPort,
  domains: schema.customDomain,
  subdomains: schema.subdomain
} as const

export const SUBDOMAIN_RE = /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/
export const DOMAIN_RE = /^(?=.{1,253}$)(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,63}$/

export function isUniqueViolation(error: unknown) {
  const cause = (error as { cause?: { code?: string } })?.cause ?? error
  return (cause as { code?: string })?.code === '23505'
}

export async function countOwned(kind: ResourceKind, userId: string) {
  const table = resourceTables[kind]
  const [row] = await db.select({ value: count() }).from(table as PgTable).where(eq(table.userId, userId))
  return row?.value ?? 0
}

// Sum of ports, domains and subdomains per user.
export async function resourceTotals(userIds?: string[]) {
  const totals = new Map<string, number>()
  if (userIds?.length === 0) return totals

  const rows = await Promise.all(Object.values(resourceTables).map(table =>
    db.select({ userId: table.userId, value: count() })
      .from(table as PgTable)
      .where(userIds ? inArray(table.userId, userIds) : undefined)
      .groupBy(table.userId)
  ))

  for (const row of rows.flat()) {
    totals.set(row.userId, (totals.get(row.userId) ?? 0) + row.value)
  }
  return totals
}
