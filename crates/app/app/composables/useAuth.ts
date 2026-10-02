import { createAuthClient } from 'better-auth/vue'
import { adminClient, deviceAuthorizationClient } from 'better-auth/client/plugins'

type Client = ReturnType<typeof makeClient>
type SessionData = NonNullable<Awaited<ReturnType<Client['getSession']>>['data']>

function makeClient(baseURL: string, headers?: Record<string, string>) {
  return createAuthClient({
    baseURL,
    fetchOptions: { headers },
    plugins: [adminClient(), deviceAuthorizationClient()]
  })
}

let browserClient: Client | undefined

export function useAuth() {
  const url = useRequestURL()
  const client = import.meta.server
    ? makeClient(url.origin, useRequestHeaders(['cookie']))
    : (browserClient ??= makeClient(url.origin))

  const session = useState<SessionData['session'] | null>('auth:session', () => null)
  const user = useState<SessionData['user'] | null>('auth:user', () => null)
  const ready = useState('auth:ready', () => false)

  async function fetchSession() {
    const { data } = await client.getSession()
    session.value = data?.session ?? null
    user.value = data?.user ?? null
    ready.value = true
    return data
  }

  async function signOut(redirectTo = '/') {
    await client.signOut()
    session.value = null
    user.value = null
    await navigateTo(redirectTo)
  }

  return {
    client,
    session,
    user,
    ready,
    loggedIn: computed(() => !!user.value),
    isAdmin: computed(() => user.value?.role === 'admin'),
    fetchSession,
    signOut
  }
}
