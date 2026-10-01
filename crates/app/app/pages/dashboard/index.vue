<script setup lang="ts">
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useSeoMeta({ title: 'Resources - Relay' })

type Kind = 'ports' | 'domains' | 'subdomains'

const { isAdmin } = useAuth()
const { data: config } = useRelayConfig()
const { data, refresh, status } = useFetch('/api/resources', {
  default: () => ({ ports: [], domains: [], subdomains: [] })
})

const toast = useToast()
const confirm = useConfirm()
const pending = ref<Kind | null>(null)

const forms = reactive({
  subdomain: '',
  domain: '',
  port: undefined as number | undefined,
  description: ''
})

const limitFor = (kind: Kind) => isAdmin.value ? undefined : config.value.limits[kind]
const atLimit = (kind: Kind) => {
  const limit = limitFor(kind)
  return limit !== undefined && data.value[kind].length >= limit
}

async function add(kind: Kind, body: Record<string, unknown>, reset: () => void) {
  pending.value = kind
  try {
    await $fetch(`/api/resources/${kind}`, { method: 'POST', body })
    reset()
    await refresh()
    toast.add({ title: 'Added', color: 'success', icon: 'i-lucide-check' })
  } catch (error) {
    toast.add({ title: 'Could not add', description: errorMessage(error), color: 'error', icon: 'i-lucide-circle-alert' })
  } finally {
    pending.value = null
  }
}

async function remove(kind: Kind, id: string, label: string) {
  const ok = await confirm({
    title: `Remove ${label}?`,
    description: 'Tunnels using it will stop resolving. Someone else may claim it afterwards.',
    confirmLabel: 'Remove'
  })
  if (!ok) return
  try {
    await $fetch(`/api/resources/${kind}/${id}`, { method: 'DELETE' })
    await refresh()
    toast.add({ title: `Removed ${label}`, color: 'success', icon: 'i-lucide-check' })
  } catch (error) {
    toast.add({ title: 'Could not remove', description: errorMessage(error), color: 'error' })
  }
}
</script>

<template>
  <UDashboardPanel id="resources">
    <template #header>
      <UDashboardNavbar title="Resources">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
        <template #right>
          <UButton to="/dashboard/cli" label="Connect CLI" icon="i-lucide-terminal" color="neutral" variant="outline" size="sm" />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="mx-auto flex w-full max-w-4xl flex-col gap-8">
        <div>
          <span class="eyebrow">[ Workspace ]</span>
          <h1 class="mt-2 text-2xl font-semibold tracking-tight text-highlighted">
            Your resources
          </h1>
          <p class="mt-1 text-sm text-muted">
            Reserve names and ports so your tunnels keep the same address every time.
          </p>
        </div>

        <div v-if="status === 'pending' && !data.subdomains.length && !data.domains.length && !data.ports.length" class="space-y-6">
          <USkeleton v-for="i in 3" :key="i" class="h-48 w-full" />
        </div>

        <template v-else>
          <ResourceSection
            title="Subdomains"
            :description="`Claim a subdomain on ${config.relayDomain} and share a permanent URL.`"
            icon="i-lucide-at-sign"
            :used="data.subdomains.length"
            :limit="limitFor('subdomains')"
            :count="data.subdomains.length"
            empty="No subdomains claimed yet."
          >
            <template #form>
              <form class="flex flex-col gap-2 sm:flex-row" @submit.prevent="add('subdomains', { subdomain: forms.subdomain }, () => forms.subdomain = '')">
                <UFieldGroup class="flex-1">
                  <UInput v-model="forms.subdomain" placeholder="my-app" :disabled="atLimit('subdomains')" class="font-mono" aria-label="Subdomain" />
                  <UBadge color="neutral" variant="outline" size="lg" class="normal-case tracking-normal text-muted">
                    .{{ config.relayDomain }}
                  </UBadge>
                </UFieldGroup>
                <UButton type="submit" label="Claim" icon="i-lucide-plus" :loading="pending === 'subdomains'" :disabled="!forms.subdomain || atLimit('subdomains')" />
              </form>
            </template>
            <ResourceRow
              v-for="item in data.subdomains"
              :key="item.id"
              :value="`${item.subdomain}.${config.relayDomain}`"
              :href="`https://${item.subdomain}.${config.relayDomain}`"
              :created-at="item.createdAt"
              @remove="remove('subdomains', item.id, item.subdomain)"
            />
          </ResourceSection>

          <ResourceSection
            title="Custom domains"
            description="Map your own domain to a relay tunnel."
            icon="i-lucide-globe"
            :used="data.domains.length"
            :limit="limitFor('domains')"
            :count="data.domains.length"
            empty="Add your own domain to get a branded tunnel URL."
          >
            <template #form>
              <form class="flex flex-col gap-2 sm:flex-row" @submit.prevent="add('domains', { domain: forms.domain }, () => forms.domain = '')">
                <UInput v-model="forms.domain" placeholder="example.com" :disabled="atLimit('domains')" class="flex-1 font-mono" aria-label="Domain" />
                <UButton type="submit" label="Add domain" icon="i-lucide-plus" :loading="pending === 'domains'" :disabled="!forms.domain || atLimit('domains')" />
              </form>
            </template>
            <ResourceRow
              v-for="item in data.domains"
              :key="item.id"
              :value="item.domain"
              :href="`https://${item.domain}`"
              :created-at="item.createdAt"
              @remove="remove('domains', item.id, item.domain)"
            />
          </ResourceSection>

          <ResourceSection
            title="Persistent ports"
            :description="`Reserve a remote port on ${config.tcpDomain} between ${config.portRange.start} and ${config.portRange.end}.`"
            icon="i-lucide-plug"
            :used="data.ports.length"
            :limit="limitFor('ports')"
            :count="data.ports.length"
            empty="Reserve a port to get a permanent tunnel address."
          >
            <template #form>
              <form
                class="flex flex-col gap-2 sm:flex-row"
                @submit.prevent="add('ports', { port: forms.port, description: forms.description }, () => { forms.port = undefined; forms.description = '' })"
              >
                <UInput
                  v-model.number="forms.port"
                  type="number"
                  :min="config.portRange.start"
                  :max="config.portRange.end"
                  :placeholder="String(config.portRange.start)"
                  :disabled="atLimit('ports')"
                  class="font-mono sm:w-36"
                  aria-label="Remote port"
                />
                <UInput v-model="forms.description" placeholder="Description (optional)" :disabled="atLimit('ports')" class="flex-1" aria-label="Description" />
                <UButton type="submit" label="Reserve" icon="i-lucide-plus" :loading="pending === 'ports'" :disabled="!forms.port || atLimit('ports')" />
              </form>
            </template>
            <ResourceRow
              v-for="item in data.ports"
              :key="item.id"
              :value="`${config.tcpDomain}:${item.port}`"
              :meta="item.description"
              :created-at="item.createdAt"
              @remove="remove('ports', item.id, `port ${item.port}`)"
            />
          </ResourceSection>
        </template>
      </div>
    </template>
  </UDashboardPanel>
</template>
