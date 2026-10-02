export default defineEventHandler(() => {
  const { relayUrl, relayDomain, authRequired } = relayConfig()
  return {
    relay_url: relayUrl,
    public_domain: relayDomain,
    auth_required: authRequired
  }
})
