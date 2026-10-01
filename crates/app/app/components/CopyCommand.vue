<script setup lang="ts">
import { useClipboard } from '@vueuse/core'

const props = withDefaults(defineProps<{ code: string, prompt?: string | false }>(), { prompt: '$' })

const { copy, copied } = useClipboard({ copiedDuring: 1600 })
</script>

<template>
  <div class="group flex items-center gap-3 border border-default bg-muted pl-4 pr-1.5 py-1.5 font-mono text-[13px]">
    <span v-if="props.prompt" class="select-none text-dimmed">{{ props.prompt }}</span>
    <code class="min-w-0 flex-1 overflow-x-auto whitespace-nowrap py-1 text-highlighted [scrollbar-width:none]">{{ props.code }}</code>
    <UButton
      :icon="copied ? 'i-lucide-check' : 'i-lucide-copy'"
      color="neutral"
      variant="ghost"
      size="sm"
      :aria-label="copied ? 'Copied' : 'Copy command'"
      @click="copy(props.code)"
    />
  </div>
</template>
