<script setup lang="ts">
import type { DropdownMenuItem, TableColumn } from '@nuxt/ui'
import { refDebounced } from '@vueuse/core'

definePageMeta({ layout: 'dashboard', middleware: 'admin' })
useSeoMeta({ title: 'Users - Relay' })

const { client, user: me } = useAuth()
const toast = useToast()
const confirm = useConfirm()

const search = ref('')
const q = refDebounced(search, 250)
const { data, refresh, status } = useFetch('/api/admin/users', { query: { q } })

type Row = NonNullable<typeof data.value>['users'][number]

const columns: TableColumn<Row>[] = [
  { accessorKey: 'name', header: 'User' },
  { accessorKey: 'role', header: 'Role' },
  { accessorKey: 'banned', header: 'Status' },
  { accessorKey: 'resources', header: 'Resources' },
  { accessorKey: 'createdAt', header: 'Joined' },
  { id: 'actions', header: '', meta: { class: { td: 'text-right w-12' } } }
]

async function run(action: () => Promise<{ error: { message?: string } | null }>, success: string) {
  const { error } = await action()
  if (error) {
    toast.add({ title: 'Action failed', description: error.message, color: 'error', icon: 'i-lucide-circle-alert' })
    return
  }
  toast.add({ title: success, color: 'success', icon: 'i-lucide-check' })
  await refresh()
}

// Ban dialog
const banTarget = ref<Row | null>(null)
const banReason = ref('')
const banOpen = computed({
  get: () => !!banTarget.value,
  set: (v) => { if (!v) banTarget.value = null }
})

async function submitBan() {
  const target = banTarget.value
  if (!target) return
  banTarget.value = null
  await run(() => client.admin.banUser({ userId: target.id, banReason: banReason.value || undefined }), `${target.name} banned`)
  banReason.value = ''
}

function actions(row: Row): DropdownMenuItem[][] {
  const isAdmin = row.role === 'admin'
  return [
    [{
      label: isAdmin ? 'Remove admin' : 'Make admin',
      icon: isAdmin ? 'i-lucide-shield-off' : 'i-lucide-shield-check',
      onSelect: () => run(() => client.admin.setRole({ userId: row.id, role: isAdmin ? 'user' : 'admin' }), 'Role updated')
    }, row.banned
      ? { label: 'Unban', icon: 'i-lucide-undo-2', onSelect: () => run(() => client.admin.unbanUser({ userId: row.id }), `${row.name} unbanned`) }
      : { label: 'Ban', icon: 'i-lucide-ban', onSelect: () => { banTarget.value = row } }],
    [{
      label: 'Delete user',
      icon: 'i-lucide-trash-2',
      color: 'error',
      onSelect: async () => {
        const ok = await confirm({
          title: `Delete ${row.name}?`,
          description: 'This permanently deletes the account and all of its reserved resources.',
          confirmLabel: 'Delete user'
        })
        if (ok) await run(() => client.admin.removeUser({ userId: row.id }), 'User deleted')
      }
    }]
  ]
}
</script>

<template>
  <UDashboardPanel id="admin-users">
    <template #header>
      <UDashboardNavbar title="Users">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="mx-auto flex w-full max-w-6xl flex-col gap-6">
        <div class="flex flex-wrap items-end justify-between gap-4">
          <div>
            <span class="eyebrow">[ Admin ]</span>
            <h1 class="mt-2 text-2xl font-semibold tracking-tight text-highlighted">
              Users
            </h1>
            <p class="mt-1 text-sm text-muted">
              {{ data?.total ?? 0 }} {{ data?.total === 1 ? 'account' : 'accounts' }}
            </p>
          </div>
          <UInput v-model="search" icon="i-lucide-search" placeholder="Search name or email" size="md" class="sm:w-72" />
        </div>

        <div class="relative border border-default">
          <Crosses />
          <UTable :data="data?.users ?? []" :columns :loading="status === 'pending'" empty="No users found.">
            <template #name-cell="{ row }">
              <div class="flex items-center gap-3">
                <UAvatar :alt="row.original.name" size="sm" :ui="{ root: 'rounded-none bg-accented', fallback: 'font-mono text-xs' }" />
                <div class="min-w-0">
                  <p class="truncate text-highlighted">
                    {{ row.original.name }}
                    <span v-if="row.original.id === me?.id" class="text-xs text-dimmed">(you)</span>
                  </p>
                  <p class="truncate text-xs text-muted">
                    {{ row.original.email }}
                  </p>
                </div>
              </div>
            </template>
            <template #role-cell="{ row }">
              <UBadge :label="row.original.role || 'user'" :variant="row.original.role === 'admin' ? 'solid' : 'outline'" />
            </template>
            <template #banned-cell="{ row }">
              <UTooltip v-if="row.original.banned" :text="row.original.banReason || 'No reason given'">
                <UBadge color="error" label="Banned" />
              </UTooltip>
              <UBadge v-else-if="row.original.emailVerified" color="success" label="Verified" />
              <span v-else class="text-xs text-dimmed">Unverified</span>
            </template>
            <template #resources-cell="{ row }">
              <span class="font-mono tabular-nums">{{ row.original.resources }}</span>
            </template>
            <template #createdAt-cell="{ row }">
              {{ formatDate(row.original.createdAt) }}
            </template>
            <template #actions-cell="{ row }">
              <UDropdownMenu v-if="row.original.id !== me?.id" :items="actions(row.original)" :content="{ align: 'end' }">
                <UButton icon="i-lucide-ellipsis" color="neutral" variant="ghost" size="sm" aria-label="Actions" />
              </UDropdownMenu>
            </template>
          </UTable>
        </div>
      </div>

      <UModal v-model:open="banOpen" :title="`Ban ${banTarget?.name ?? ''}`" description="All of their active sessions will be revoked." :ui="{ footer: 'justify-end bg-muted' }">
        <template #body>
          <UFormField label="Reason" hint="Optional">
            <UInput v-model="banReason" placeholder="e.g. Abuse of service" autofocus @keydown.enter="submitBan" />
          </UFormField>
        </template>
        <template #footer>
          <UButton label="Cancel" color="neutral" variant="outline" @click="banOpen = false" />
          <UButton label="Ban user" color="error" @click="submitBan" />
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>
