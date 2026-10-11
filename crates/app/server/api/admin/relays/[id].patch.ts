import { eq } from 'drizzle-orm'
import { z } from 'zod'

export default defineEventHandler(async (event) => {
  await requireAdmin(event)
  const { id } = await getValidatedRouterParams(event, z.object({ id: z.string().min(1) }).parse)
  const { enabled } = await readValidatedBody(event, z.object({ enabled: z.boolean() }).parse)
  await db.update(schema.relay).set({ enabled }).where(eq(schema.relay.id, id))
  return { success: true }
})
