<script setup lang="ts">
definePageMeta({ layout: 'dashboard', middleware: 'admin' })
useSeoMeta({ title: 'Admin - Relay' })

const { data, status } = useFetch('/api/admin/stats')

const stats = computed(() => {
  const s = data.value?.stats
  return [
    { label: 'Total users', value: s?.users, icon: 'i-lucide-users' },
    { label: 'New (7d)', value: s?.newUsers, icon: 'i-lucide-user-plus' },
    { label: 'Admins', value: s?.admins, icon: 'i-lucide-shield-check' },
    { label: 'Banned', value: s?.banned, icon: 'i-lucide-ban' },
    { label: 'Subdomains', value: s?.subdomains, icon: 'i-lucide-at-sign' },
    { label: 'Custom domains', value: s?.domains, icon: 'i-lucide-globe' },
    { label: 'Ports', value: s?.ports, icon: 'i-lucide-plug' }
  ]
})
</script>

<template>
  <UDashboardPanel id="admin-overview">
    <template #header>
      <UDashboardNavbar title="Overview">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="mx-auto flex w-full max-w-5xl flex-col gap-8">
        <div>
          <span class="eyebrow">[ Admin ]</span>
          <h1 class="mt-2 text-2xl font-semibold tracking-tight text-highlighted">
            Overview
          </h1>
          <p class="mt-1 text-sm text-muted">
            Platform-wide usage and statistics.
          </p>
        </div>

        <div class="relative grid grid-cols-2 border-t border-l border-default sm:grid-cols-4">
          <Crosses />
          <div
            v-for="stat in stats"
            :key="stat.label"
            class="flex flex-col gap-3 border-r border-b border-default p-5"
          >
            <div class="flex items-center justify-between">
              <span class="eyebrow">{{ stat.label }}</span>
              <UIcon :name="stat.icon" class="size-4 text-dimmed" />
            </div>
            <USkeleton v-if="status === 'pending'" class="h-8 w-12" />
            <span v-else class="text-3xl font-semibold tabular-nums tracking-tight text-highlighted">{{ stat.value ?? 0 }}</span>
          </div>
          <NuxtLink to="/admin/users" class="group hidden items-center justify-between border-r border-b border-default p-5 text-sm text-muted transition-colors hover:bg-muted hover:text-highlighted sm:flex">
            Manage users
            <UIcon name="i-lucide-arrow-right" class="size-4 transition-transform group-hover:translate-x-0.5" />
          </NuxtLink>
        </div>

        <div class="grid gap-6 lg:grid-cols-2">
          <section class="border border-default">
            <header class="flex items-center justify-between border-b border-default px-5 py-3">
              <h2 class="text-sm font-medium text-highlighted">
                Recent signups
              </h2>
              <UButton to="/admin/users" label="View all" color="neutral" variant="link" size="xs" />
            </header>
            <ul v-if="data?.recentUsers.length" class="divide-y divide-default">
              <li v-for="u in data.recentUsers" :key="u.id" class="flex items-center gap-3 px-5 py-3">
                <UAvatar :alt="u.name" size="sm" :ui="{ root: 'rounded-none bg-accented', fallback: 'font-mono text-xs' }" />
                <div class="min-w-0 flex-1">
                  <p class="truncate text-sm text-highlighted">
                    {{ u.name }}
                  </p>
                  <p class="truncate text-xs text-muted">
                    {{ u.email }}
                  </p>
                </div>
                <UBadge v-if="u.banned" color="error" label="Banned" />
                <UBadge v-else-if="u.role === 'admin'" label="Admin" />
                <span class="text-xs text-dimmed">{{ formatDate(u.createdAt) }}</span>
              </li>
            </ul>
            <p v-else class="px-5 py-10 text-center text-sm text-muted">
              No users yet.
            </p>
          </section>

          <section class="border border-default">
            <header class="flex items-center justify-between border-b border-default px-5 py-3">
              <h2 class="text-sm font-medium text-highlighted">
                Top users by resources
              </h2>
              <UButton to="/admin/resources" label="View all" color="neutral" variant="link" size="xs" />
            </header>
            <ul v-if="data?.topUsers.length" class="divide-y divide-default">
              <li v-for="(u, i) in data.topUsers" :key="u.id" class="flex items-center gap-3 px-5 py-3">
                <span class="w-5 font-mono text-xs text-dimmed">{{ i + 1 }}</span>
                <div class="min-w-0 flex-1">
                  <p class="truncate text-sm text-highlighted">
                    {{ u.name }}
                  </p>
                  <p class="truncate text-xs text-muted">
                    {{ u.email }}
                  </p>
                </div>
                <span class="font-mono text-sm tabular-nums text-highlighted">{{ u.resources }}</span>
              </li>
            </ul>
            <p v-else class="px-5 py-10 text-center text-sm text-muted">
              No reserved resources yet.
            </p>
          </section>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
