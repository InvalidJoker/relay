import { eq } from 'drizzle-orm'
import { z } from 'zod'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { id } = await getValidatedRouterParams(event, z.object({ id: z.string().min(1) }).parse)
  await db.delete(schema.relay).where(eq(schema.relay.id, id))
  return { success: true }
})
