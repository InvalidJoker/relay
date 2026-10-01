<script setup lang="ts">
defineProps<{ value: string, meta?: string | null, createdAt: string | Date, href?: string }>()
defineEmits<{ remove: [] }>()
</script>

<template>
  <li class="group flex items-center gap-4 px-5 py-3 transition-colors hover:bg-muted/60">
    <div class="min-w-0 flex-1">
      <p class="truncate font-mono text-sm text-highlighted">
        <a v-if="href" :href="href" target="_blank" rel="noopener" class="underline-offset-4 hover:underline">{{ value }}</a>
        <template v-else>{{ value }}</template>
      </p>
      <p v-if="meta" class="truncate text-xs text-muted">
        {{ meta }}
      </p>
    </div>
    <span class="hidden text-xs text-dimmed sm:block">{{ formatDate(createdAt) }}</span>
    <UTooltip text="Remove">
      <UButton icon="i-lucide-trash-2" color="neutral" variant="ghost" size="sm" aria-label="Remove" class="hover:text-error" @click="$emit('remove')" />
    </UTooltip>
  </li>
</template>
