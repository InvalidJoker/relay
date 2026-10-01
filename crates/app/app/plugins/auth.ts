export default defineNuxtPlugin(async () => {
  const { ready, fetchSession } = useAuth()
  if (!ready.value) await fetchSession()
})
