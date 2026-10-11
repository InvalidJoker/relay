import { asc } from 'drizzle-orm'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  return db.select().from(schema.relay).orderBy(asc(schema.relay.createdAt))
})
