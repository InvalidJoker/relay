<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps<{ error: NuxtError }>()
useSeoMeta({ title: `${props.error.statusCode} - Relay` })
</script>

<template>
  <UApp>
    <div class="relative flex min-h-svh flex-col items-center justify-center gap-6 px-4 text-center">
      <div class="bg-grid pointer-events-none absolute inset-0 [mask-image:radial-gradient(ellipse_at_center,black_20%,transparent_70%)]" aria-hidden="true" />
      <span class="eyebrow relative">[ Error {{ error.statusCode }} ]</span>
      <h1 class="relative text-4xl font-semibold tracking-tight text-highlighted">
        {{ error.statusCode === 404 ? 'Page not found' : 'Something went wrong' }}
      </h1>
      <p class="relative max-w-md text-muted">
        {{ error.statusCode === 404 ? 'The page you are looking for does not exist.' : error.statusMessage || error.message }}
      </p>
      <UButton class="relative" label="Back home" icon="i-lucide-arrow-left" @click="clearError({ redirect: '/' })" />
    </div>
  </UApp>
</template>
