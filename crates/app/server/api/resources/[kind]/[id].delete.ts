import { and, eq } from 'drizzle-orm'
import { z } from 'zod'

export default defineEventHandler(async (event) => {
  const user = await requireUser(event)
  const { kind, id } = await getValidatedRouterParams(event, z.object({
    kind: z.enum(['ports', 'domains', 'subdomains']),
    id: z.string().min(1)
  }).parse)

  const table = resourceTables[kind]
  const deleted = await db.delete(table)
    .where(and(eq(table.id, id), eq(table.userId, user.id)))
    .returning({ id: table.id })

  if (!deleted.length) throw createError({ statusCode: 404, statusMessage: 'Not found' })
  return { success: true }
})
