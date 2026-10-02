import type { H3Event } from 'h3'
import { betterAuth } from 'better-auth'
import { drizzleAdapter } from 'better-auth/adapters/drizzle'
import { admin, bearer, deviceAuthorization } from 'better-auth/plugins'

export const auth = betterAuth({
  baseURL: process.env.ORIGIN || process.env.BETTER_AUTH_URL,
  secret: process.env.BETTER_AUTH_SECRET,
  database: drizzleAdapter(db, { provider: 'pg', schema }),
  emailAndPassword: { enabled: true },
  plugins: [
    deviceAuthorization({
      verificationUri: '/device',
      validateClient: clientId => clientId === 'cli'
    }),
    bearer(),
    admin()
  ]
})

export type AuthSession = NonNullable<Awaited<ReturnType<typeof auth.api.getSession>>>

export async function getAuthSession(event: H3Event) {
  if (event.context.auth === undefined) {
    event.context.auth = await auth.api.getSession({ headers: event.headers })
  }
  return event.context.auth as AuthSession | null
}

export async function requireUser(event: H3Event) {
  const session = await getAuthSession(event)
  if (!session) throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })
  return session.user
}

export async function requireAdmin(event: H3Event) {
  const user = await requireUser(event)
  if (user.role !== 'admin') throw createError({ statusCode: 403, statusMessage: 'Forbidden' })
  return user
}
