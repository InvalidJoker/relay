export default defineEventHandler(() => {
  const { relayDomain, tcpDomain, portRange } = relayConfig()
  return { relayDomain, tcpDomain, portRange, limits: LIMITS }
})
