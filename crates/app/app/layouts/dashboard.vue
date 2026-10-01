<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'

const { isAdmin } = useAuth()
const open = ref(false)
const close = () => { open.value = false }

const links = computed<NavigationMenuItem[][]>(() => {
  const main: NavigationMenuItem[] = [
    { label: 'Workspace', type: 'label' },
    { label: 'Resources', icon: 'i-lucide-layers', to: '/dashboard', exact: true, onSelect: close },
    { label: 'Connect CLI', icon: 'i-lucide-terminal', to: '/dashboard/cli', onSelect: close }
  ]
  if (isAdmin.value) {
    main.push(
      { label: 'Admin', type: 'label' },
      { label: 'Overview', icon: 'i-lucide-chart-no-axes-column', to: '/admin', exact: true, onSelect: close },
      { label: 'Users', icon: 'i-lucide-users', to: '/admin/users', onSelect: close },
      { label: 'All resources', icon: 'i-lucide-globe', to: '/admin/resources', onSelect: close }
    )
  }
  return [main, [{ label: 'GitHub', icon: 'i-simple-icons-github', to: REPO_URL, target: '_blank' }]]
})
</script>

<template>
  <UDashboardGroup unit="rem">
    <UDashboardSidebar
      id="default"
      v-model:open="open"
      collapsible
      resizable
      :default-size="15"
      :min-size="12"
      :max-size="20"
    >
      <template #header="{ collapsed }">
        <NuxtLink to="/" aria-label="Relay home" :class="collapsed && 'mx-auto'">
          <AppLogo :wordmark="!collapsed" />
        </NuxtLink>
      </template>

      <template #default="{ collapsed }">
        <UNavigationMenu
          :collapsed
          :items="links[0]"
          orientation="vertical"
          tooltip
          :ui="{ label: 'eyebrow font-normal pt-4 first:pt-1.5' }"
        />
        <UNavigationMenu
          :collapsed
          :items="links[1]"
          orientation="vertical"
          tooltip
          class="mt-auto"
        />
      </template>

      <template #footer="{ collapsed }">
        <UserMenu :collapsed />
      </template>
    </UDashboardSidebar>

    <slot />
  </UDashboardGroup>
</template>
