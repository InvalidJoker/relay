import { eq } from 'drizzle-orm'
import { z } from 'zod'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { kind, id } = await getValidatedRouterParams(event, z.object({
    kind: z.enum(['ports', 'domains', 'subdomains']),
    id: z.string().min(1)
  }).parse)

  const table = resourceTables[kind]
  await db.delete(table).where(eq(table.id, id))
  return { success: true }
})
