export const REPO_URL = 'https://github.com/InvalidJoker/relay'

export const INSTALL_COMMANDS = {
  unix: { label: 'macOS / Linux', code: 'curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh' },
  windows: { label: 'Windows', code: 'irm https://raw.githubusercontent.com/InvalidJoker/relay/main/install.ps1 | iex' }
} as const

export function useRelayConfig() {
  return useFetch('/api/config', {
    key: 'relay-config',
    default: () => ({
      relayDomain: 'relay.invalidjoker.dev',
      tcpDomain: 'tcp.invalidjoker.dev',
      portRange: { start: 10000, end: 20000 },
      limits: { ports: 2, domains: 1, subdomains: 3 }
    })
  })
}

export function formatDate(value: string | Date) {
  return new Date(value).toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' })
}

export function errorMessage(error: unknown, fallback = 'Something went wrong') {
  const e = error as { data?: { statusMessage?: string, message?: string }, statusMessage?: string, message?: string }
  return e?.data?.statusMessage || e?.data?.message || e?.statusMessage || e?.message || fallback
}
