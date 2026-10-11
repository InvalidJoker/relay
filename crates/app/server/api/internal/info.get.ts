export default defineEventHandler(async () => {
  const { relayDomain, authRequired } = relayConfig()
  const relays = (await listRelays()).map(({ id, name, region, host }) => ({ id, name, region, host }))
  return {
    // Kept for CLIs without multi relay support
    relay_url: relays[0]?.host ?? '',
    relays,
    public_domain: relayDomain,
    auth_required: authRequired
  }
})
