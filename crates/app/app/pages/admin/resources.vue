<script setup lang="ts">
import type { TabsItem } from '@nuxt/ui'

definePageMeta({ layout: 'dashboard', middleware: 'admin' })
useSeoMeta({ title: 'All resources - Relay' })

type Kind = 'subdomains' | 'domains' | 'ports'

const { data: config } = useRelayConfig()
const { data, refresh, status } = useFetch('/api/admin/resources', {
  default: () => ({ domains: [], subdomains: [], ports: [] })
})
const toast = useToast()
const confirm = useConfirm()

const tab = ref<Kind>('subdomains')
const tabs = computed<TabsItem[]>(() => [
  { label: 'Subdomains', value: 'subdomains', badge: data.value.subdomains.length },
  { label: 'Custom domains', value: 'domains', badge: data.value.domains.length },
  { label: 'Ports', value: 'ports', badge: data.value.ports.length }
])

const rows = computed(() => {
  const { relayDomain, tcpDomain } = config.value
  switch (tab.value) {
    case 'subdomains': return data.value.subdomains.map(r => ({ ...r, value: `${r.subdomain}.${relayDomain}`, meta: null }))
    case 'domains': return data.value.domains.map(r => ({ ...r, value: r.domain, meta: null }))
    case 'ports': return data.value.ports.map(r => ({ ...r, value: `${tcpDomain}:${r.port}`, meta: r.description }))
  }
  return []
})

async function remove(id: string, label: string) {
  const ok = await confirm({ title: `Remove ${label}?`, description: 'The owner will lose this reservation.', confirmLabel: 'Remove' })
  if (!ok) return
  try {
    await $fetch(`/api/admin/resources/${tab.value}/${id}`, { method: 'DELETE' })
    await refresh()
    toast.add({ title: `Removed ${label}`, color: 'success', icon: 'i-lucide-check' })
  } catch (error) {
    toast.add({ title: 'Could not remove', description: errorMessage(error), color: 'error' })
  }
}
</script>

<template>
  <UDashboardPanel id="admin-resources">
    <template #header>
      <UDashboardNavbar title="All resources">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="mx-auto flex w-full max-w-5xl flex-col gap-6">
        <div>
          <span class="eyebrow">[ Admin ]</span>
          <h1 class="mt-2 text-2xl font-semibold tracking-tight text-highlighted">
            Domains &amp; ports
          </h1>
          <p class="mt-1 text-sm text-muted">
            All reserved resources across every user.
          </p>
        </div>

        <div class="relative border border-default">
          <Crosses />
          <UTabs
            v-model="tab"
            :items="tabs"
            :content="false"
            variant="link"
            color="neutral"
            :ui="{ root: 'border-b border-default', list: 'px-3 border-b-0', trigger: 'py-3 font-medium' }"
          />

          <div v-if="status === 'pending'" class="space-y-px p-5">
            <USkeleton v-for="i in 4" :key="i" class="h-10 w-full" />
          </div>
          <ul v-else-if="rows.length" class="divide-y divide-default">
            <li v-for="row in rows" :key="row.id" class="flex items-center gap-4 px-5 py-3 transition-colors hover:bg-muted/60">
              <div class="min-w-0 flex-1">
                <p class="truncate font-mono text-sm text-highlighted">
                  {{ row.value }}
                </p>
                <p v-if="row.meta" class="truncate text-xs text-muted">
                  {{ row.meta }}
                </p>
              </div>
              <div class="hidden min-w-0 text-right md:block">
                <p class="truncate text-sm text-toned">
                  {{ row.owner.name }}
                </p>
                <p class="truncate text-xs text-muted">
                  {{ row.owner.email }}
                </p>
              </div>
              <span class="hidden w-24 text-right text-xs text-dimmed sm:block">{{ formatDate(row.createdAt) }}</span>
              <UButton icon="i-lucide-trash-2" color="neutral" variant="ghost" size="sm" aria-label="Remove" class="hover:text-error" @click="remove(row.id, row.value)" />
            </li>
          </ul>
          <div v-else class="flex flex-col items-center gap-1 px-5 py-12 text-center">
            <span class="eyebrow">Empty</span>
            <p class="text-sm text-muted">
              Nothing reserved yet.
            </p>
          </div>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
