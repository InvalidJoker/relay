<script setup lang="ts">
withDefaults(defineProps<{ wordmark?: boolean }>(), { wordmark: true })
</script>

<template>
  <span class="inline-flex items-center gap-2 text-highlighted">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="size-5 shrink-0" aria-hidden="true">
      <path d="M0 3L2 0" transform="translate(10.5 13.5)" />
      <path d="M3.306 8.3a2.4 2.4 0 0 0 3.4 0L9.006 6l-6-6L.706 2.3a2.4 2.4 0 0 0 0 3.4z" transform="translate(2.994 12)" />
      <path d="M0 3L3 0" transform="translate(2 19)" />
      <path d="M0 2.5L2.5 0" transform="translate(7.5 11)" />
      <path d="M4 0L0 4L6 4L2 8" transform="translate(14 3)" />
    </svg>
    <span v-if="wordmark" class="font-mono text-sm font-medium uppercase tracking-[0.18em]">Relay</span>
  </span>
</template>
