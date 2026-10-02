<script setup lang="ts">
definePageMeta({ layout: 'auth', middleware: 'auth' })
useSeoMeta({ title: 'Authorize CLI - Relay' })

type Status = 'loading' | 'enter' | 'pending' | 'approved' | 'denied' | 'error'

const { client, user } = useAuth()
const route = useRoute()

const code = computed(() => String(route.query.user_code ?? '').toUpperCase().replace(/[^A-Z0-9]/g, ''))
const input = ref<string[]>([])
const status = ref<Status>('loading')
const errorText = ref('')
const busy = ref<'approve' | 'deny' | null>(null)

// Opening the verification page claims the code for the signed-in user.
async function verify() {
  if (!code.value) {
    status.value = 'enter'
    return
  }
  status.value = 'loading'
  const { data, error } = await client.device({ query: { user_code: code.value } })
  if (error || !data) {
    status.value = 'error'
    errorText.value = error?.error_description || 'This code is invalid or has expired.'
    return
  }
  status.value = data.status as Status
}

async function decide(action: 'approve' | 'deny') {
  busy.value = action
  const { error } = action === 'approve'
    ? await client.device.approve({ userCode: code.value })
    : await client.device.deny({ userCode: code.value })
  busy.value = null
  if (error) {
    status.value = 'error'
    errorText.value = error.error_description || 'Something went wrong.'
    return
  }
  status.value = action === 'approve' ? 'approved' : 'denied'
}

function submitCode() {
  navigateTo({ query: { user_code: input.value.join('') } })
}

watch(code, verify)
onMounted(verify)
</script>

<template>
  <div class="p-6 sm:p-8">
    <div class="mb-6 flex size-10 items-center justify-center border border-default bg-muted">
      <UIcon name="i-lucide-terminal" class="size-5 text-highlighted" />
    </div>

    <template v-if="status === 'enter'">
      <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
        Connect the CLI
      </h1>
      <p class="mt-1 text-sm text-muted">
        Enter the code shown in your terminal after running <code class="font-mono text-toned">relay login</code>.
      </p>
      <form class="mt-6 space-y-4" @submit.prevent="submitCode">
        <UPinInput v-model="input" :length="8" size="lg" class="w-full" :ui="{ root: 'w-full justify-between gap-1', base: 'font-mono uppercase w-full' }" />
        <UButton type="submit" label="Continue" block trailing-icon="i-lucide-arrow-right" :disabled="input.join('').length < 8" />
      </form>
    </template>

    <template v-else-if="status === 'loading'">
      <USkeleton class="h-7 w-2/3" />
      <USkeleton class="mt-3 h-4 w-full" />
      <USkeleton class="mt-6 h-16 w-full" />
    </template>

    <template v-else-if="status === 'pending'">
      <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
        Authorize the Relay CLI
      </h1>
      <p class="mt-1 text-sm text-muted">
        Make sure this code matches the one in your terminal.
      </p>

      <div class="relative my-6 border border-default bg-muted py-5 text-center font-mono text-2xl tracking-[0.35em] text-highlighted">
        {{ code.slice(0, 4) }}-{{ code.slice(4) }}
      </div>

      <p class="mb-4 text-sm text-muted">
        Signing in as <span class="text-highlighted">{{ user?.email }}</span>
      </p>

      <div class="grid grid-cols-2 gap-2">
        <UButton label="Deny" color="neutral" variant="outline" block :loading="busy === 'deny'" :disabled="!!busy" @click="decide('deny')" />
        <UButton label="Authorize" block :loading="busy === 'approve'" :disabled="!!busy" @click="decide('approve')" />
      </div>
    </template>

    <template v-else-if="status === 'approved'">
      <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
        CLI connected
      </h1>
      <p class="mt-1 text-sm text-muted">
        You can now close this window and return to the CLI.
      </p>
      <UButton to="/dashboard" label="Go to dashboard" color="neutral" variant="outline" block class="mt-6" />
    </template>

    <template v-else-if="status === 'denied'">
      <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
        Request denied
      </h1>
      <p class="mt-1 text-sm text-muted">
        The CLI was not given access to your account.
      </p>
    </template>

    <template v-else>
      <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
        Invalid code
      </h1>
      <p class="mt-1 text-sm text-muted">
        {{ errorText }}
      </p>
      <UButton label="Enter a different code" color="neutral" variant="outline" block class="mt-6" @click="navigateTo({ query: {} })" />
    </template>
  </div>
</template>
