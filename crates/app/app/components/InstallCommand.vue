<script setup lang="ts">
type OS = keyof typeof INSTALL_COMMANDS

const os = ref<OS>('unix')
onMounted(() => {
  if (/win/i.test(navigator.userAgent)) os.value = 'windows'
})
</script>

<template>
  <div>
    <div class="flex border border-b-0 border-default w-fit">
      <button
        v-for="(cmd, key) in INSTALL_COMMANDS"
        :key="key"
        type="button"
        class="px-3 py-1.5 font-mono text-xs uppercase tracking-wider transition-colors not-last:border-r border-default"
        :class="os === key ? 'bg-muted text-highlighted' : 'text-dimmed hover:text-toned'"
        @click="os = key"
      >
        {{ cmd.label }}
      </button>
    </div>
    <CopyCommand :code="INSTALL_COMMANDS[os].code" :prompt="os === 'windows' ? '>' : '$'" />
  </div>
</template>
