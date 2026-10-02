<script setup lang="ts">
defineProps<{
  title: string
  description: string
  icon: string
  used: number
  limit?: number
  empty: string
  count: number
}>()
</script>

<template>
  <section class="relative border border-default bg-default">
    <Crosses />
    <header class="flex flex-wrap items-start gap-4 border-b border-default p-5">
      <div class="flex size-9 shrink-0 items-center justify-center border border-default bg-muted">
        <UIcon :name="icon" class="size-4 text-highlighted" />
      </div>
      <div class="min-w-0 flex-1">
        <h2 class="font-medium text-highlighted">
          {{ title }}
        </h2>
        <p class="mt-0.5 text-sm text-muted">
          {{ description }}
        </p>
      </div>
      <span class="eyebrow tabular-nums">
        {{ used }}<template v-if="limit !== undefined"> / {{ limit }}</template> used
      </span>
    </header>

    <div class="border-b border-default bg-muted p-5">
      <slot name="form" />
    </div>

    <ul v-if="count" class="divide-y divide-default">
      <slot />
    </ul>
    <div v-else class="flex flex-col items-center gap-1 px-5 py-10 text-center">
      <span class="eyebrow">Empty</span>
      <p class="text-sm text-muted">
        {{ empty }}
      </p>
    </div>
  </section>
</template>
