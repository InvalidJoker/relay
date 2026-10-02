<script setup lang="ts">
useHead({
  titleTemplate: title => title || 'Relay'
})
</script>

<template>
  <UApp :toaster="{ position: 'bottom-right' }">
    <NuxtLoadingIndicator color="var(--ui-text-highlighted)" :height="2" />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
