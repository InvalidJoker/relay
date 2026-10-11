<script setup lang="ts">
definePageMeta({ layout: 'dashboard', middleware: 'admin' })
useSeoMeta({ title: 'Relays - Relay' })

const { data: relays, refresh, status } = useFetch('/api/admin/relays', { default: () => [] })
const toast = useToast()
const confirm = useConfirm()

const empty = () => ({ name: '', region: '', host: '', httpDomain: '', tcpDomain: '' })
const form = reactive(empty())
const pending = ref(false)
const revealed = ref<string | null>(null)

async function add() {
  pending.value = true
  try {
    const created = await $fetch('/api/admin/relays', { method: 'POST', body: { ...form } })
    Object.assign(form, empty())
    revealed.value = created.id
    await refresh()
    toast.add({ title: `Added ${created.name}`, description: 'Copy the relay key into its RELAY_KEY variable.', color: 'success', icon: 'i-lucide-check' })
  } catch (error) {
    toast.add({ title: 'Could not add relay', description: errorMessage(error), color: 'error' })
  } finally {
    pending.value = false
  }
}

async function toggle(id: string, enabled: boolean) {
  try {
    await $fetch(`/api/admin/relays/${id}`, { method: 'PATCH', body: { enabled } })
    await refresh()
  } catch (error) {
    toast.add({ title: 'Could not update relay', description: errorMessage(error), color: 'error' })
  }
}

async function remove(id: string, name: string) {
  const ok = await confirm({ title: `Remove ${name}?`, description: 'Clients can no longer pick this relay and it can no longer authenticate tunnels.', confirmLabel: 'Remove' })
  if (!ok) return
  try {
    await $fetch(`/api/admin/relays/${id}`, { method: 'DELETE' })
    await refresh()
    toast.add({ title: `Removed ${name}`, color: 'success', icon: 'i-lucide-check' })
  } catch (error) {
    toast.add({ title: 'Could not remove', description: errorMessage(error), color: 'error' })
  }
}

async function copy(value: string) {
  await navigator.clipboard.writeText(value)
  toast.add({ title: 'Copied', color: 'success', icon: 'i-lucide-check' })
}
</script>

<template>
  <UDashboardPanel id="admin-relays">
    <template #header>
      <UDashboardNavbar title="Relays">
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
            Relays
          </h1>
          <p class="mt-1 text-sm text-muted">
            The CLI measures latency to every enabled relay and connects to the fastest. Run each relay with its key as <code class="font-mono">RELAY_KEY</code>.
          </p>
        </div>

        <div class="relative border border-default">
          <Crosses />
          <form class="grid gap-2 border-b border-default p-5 sm:grid-cols-2" @submit.prevent="add">
            <UInput v-model="form.name" placeholder="Name (Frankfurt)" aria-label="Name" />
            <UInput v-model="form.region" placeholder="Region (eu-central, optional)" aria-label="Region" />
            <UInput v-model="form.host" placeholder="Control host (eu.relay.example.com)" class="font-mono" aria-label="Control host" />
            <UInput v-model="form.httpDomain" placeholder="Subdomain base (eu.relay.example.com)" class="font-mono" aria-label="Subdomain base domain" />
            <UInput v-model="form.tcpDomain" placeholder="TCP host (tcp.eu.relay.example.com)" class="font-mono" aria-label="TCP host" />
            <UButton type="submit" label="Add relay" icon="i-lucide-plus" :loading="pending" :disabled="!form.name || !form.host || !form.httpDomain || !form.tcpDomain" class="justify-center" />
          </form>

          <div v-if="status === 'pending'" class="space-y-px p-5">
            <USkeleton v-for="i in 3" :key="i" class="h-12 w-full" />
          </div>
          <ul v-else-if="relays.length" class="divide-y divide-default">
            <li v-for="relay in relays" :key="relay.id" class="flex flex-col gap-2 px-5 py-3">
              <div class="flex items-center gap-4">
                <div class="min-w-0 flex-1">
                  <p class="truncate text-sm text-highlighted">
                    {{ relay.name }}
                    <span v-if="relay.region" class="text-muted">- {{ relay.region }}</span>
                  </p>
                  <p class="truncate font-mono text-xs text-muted">
                    {{ relay.host }} | *.{{ relay.httpDomain }} | {{ relay.tcpDomain }}
                  </p>
                </div>
                <USwitch :model-value="relay.enabled" aria-label="Enabled" @update:model-value="toggle(relay.id, $event)" />
                <UButton :icon="revealed === relay.id ? 'i-lucide-eye-off' : 'i-lucide-key-round'" color="neutral" variant="ghost" size="sm" aria-label="Show relay key" @click="revealed = revealed === relay.id ? null : relay.id" />
                <UButton icon="i-lucide-trash-2" color="neutral" variant="ghost" size="sm" aria-label="Remove" class="hover:text-error" @click="remove(relay.id, relay.name)" />
              </div>
              <div v-if="revealed === relay.id" class="flex items-center gap-2">
                <code class="min-w-0 flex-1 truncate border border-default bg-muted px-2 py-1 font-mono text-xs">RELAY_KEY={{ relay.secret }}</code>
                <UButton icon="i-lucide-copy" color="neutral" variant="ghost" size="sm" aria-label="Copy relay key" @click="copy(relay.secret)" />
              </div>
            </li>
          </ul>
          <div v-else class="flex flex-col items-center gap-1 px-5 py-12 text-center">
            <span class="eyebrow">Default relay</span>
            <p class="text-sm text-muted">
              No relays added. The relay from the environment configuration is used.
            </p>
          </div>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
