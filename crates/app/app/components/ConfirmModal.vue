<script setup lang="ts">
const props = withDefaults(defineProps<{
  title: string
  description?: string
  confirmLabel?: string
  danger?: boolean
}>(), { confirmLabel: 'Confirm', danger: true })

const emit = defineEmits<{ close: [confirmed: boolean] }>()
</script>

<template>
  <UModal
    :title="props.title"
    :description="props.description"
    :close="false"
    :ui="{ footer: 'justify-end bg-muted' }"
  >
    <template #footer>
      <UButton label="Cancel" color="neutral" variant="outline" @click="emit('close', false)" />
      <UButton :label="props.confirmLabel" :color="props.danger ? 'error' : 'primary'" @click="emit('close', true)" />
    </template>
  </UModal>
</template>
