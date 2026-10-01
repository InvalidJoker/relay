<script setup lang="ts">
const { loggedIn } = useAuth()

const links = [
  { label: 'Features', to: '/#features' },
  { label: 'How it works', to: '/#how-it-works' },
  { label: 'GitHub', to: REPO_URL, target: '_blank' }
]
</script>

<template>
  <header class="sticky top-0 z-50 border-b border-default bg-default/80 backdrop-blur">
    <div class="mx-auto flex h-(--ui-header-height) max-w-6xl items-center gap-6 border-x border-default px-4 sm:px-6">
      <NuxtLink to="/" aria-label="Relay home">
        <AppLogo />
      </NuxtLink>

      <nav class="hidden items-center gap-1 md:flex">
        <UButton
          v-for="link in links"
          :key="link.label"
          :to="link.to"
          :target="link.target"
          :label="link.label"
          color="neutral"
          variant="link"
          size="sm"
          class="text-muted hover:text-highlighted"
        />
      </nav>

      <div class="ml-auto flex items-center gap-1.5">
        <UColorModeButton color="neutral" variant="ghost" size="sm" />
        <template v-if="loggedIn">
          <UButton to="/dashboard" label="Dashboard" size="sm" trailing-icon="i-lucide-arrow-up-right" />
        </template>
        <template v-else>
          <UButton to="/login" label="Sign in" color="neutral" variant="ghost" size="sm" class="hidden sm:inline-flex" />
          <UButton to="/register" label="Get started" size="sm" />
        </template>
      </div>
    </div>
  </header>
</template>
