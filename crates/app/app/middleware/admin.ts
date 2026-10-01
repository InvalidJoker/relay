export default defineNuxtRouteMiddleware((to) => {
  const { loggedIn, isAdmin } = useAuth()
  if (!loggedIn.value) return navigateTo({ path: '/login', query: { redirect: to.fullPath } })
  if (!isAdmin.value) return navigateTo('/dashboard')
})
