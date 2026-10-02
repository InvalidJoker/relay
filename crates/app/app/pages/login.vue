<script setup lang="ts">
import * as z from 'zod'
import type { AuthFormField, FormSubmitEvent } from '@nuxt/ui'

definePageMeta({ layout: 'auth', middleware: 'guest' })
useSeoMeta({ title: 'Sign in - Relay' })

const { client, fetchSession } = useAuth()
const route = useRoute()
const toast = useToast()
const loading = ref(false)

const fields: AuthFormField[] = [
  { name: 'email', type: 'email', label: 'Email', placeholder: 'you@example.com', required: true, autocomplete: 'email' },
  { name: 'password', type: 'password', label: 'Password', placeholder: '••••••••', required: true, autocomplete: 'current-password' }
]

const schema = z.object({
  email: z.email('Enter a valid email'),
  password: z.string().min(1, 'Password is required')
})

const redirectTo = computed(() => {
  const target = route.query.redirect
  return typeof target === 'string' && target.startsWith('/') && !target.startsWith('//') ? target : '/dashboard'
})

async function onSubmit({ data }: FormSubmitEvent<z.output<typeof schema>>) {
  loading.value = true
  const { error } = await client.signIn.email(data)
  if (error) {
    loading.value = false
    toast.add({ title: 'Could not sign in', description: error.message, color: 'error', icon: 'i-lucide-circle-alert' })
    return
  }
  await fetchSession()
  await navigateTo(redirectTo.value)
}
</script>

<template>
  <div class="p-6 sm:p-8">
    <UAuthForm
      :schema
      :fields
      :loading
      title="Sign in"
      description="Welcome back. Sign in to manage your tunnels."
      :submit="{ label: 'Continue', block: true, trailingIcon: 'i-lucide-arrow-right' }"
      :ui="{ header: 'text-left', title: 'text-2xl tracking-tight', description: 'text-sm' }"
      @submit="onSubmit"
    />
  </div>
  <div class="border-t border-default bg-muted px-6 py-4 text-center text-sm text-muted sm:px-8">
    Don't have an account?
    <NuxtLink :to="{ path: '/register', query: route.query }" class="font-medium text-highlighted underline-offset-4 hover:underline">
      Sign up
    </NuxtLink>
  </div>
</template>
