<script setup lang="ts">
const { data: config } = useRelayConfig()
</script>

<template>
  <div class="relative border border-default bg-default font-mono text-[13px] leading-6 shadow-2xl shadow-black/20">
    <Crosses />
    <div class="flex items-center gap-2 border-b border-default px-4 py-2.5">
      <span class="size-2.5 rounded-full bg-accented" />
      <span class="size-2.5 rounded-full bg-accented" />
      <span class="size-2.5 rounded-full bg-accented" />
      <span class="ml-2 text-xs text-dimmed">~/projects/my-app</span>
    </div>
    <div class="space-y-1 p-5">
      <p class="text-highlighted">
        <span class="text-dimmed select-none">$ </span>relay http 8080
      </p>
      <p class="animate-[line-in_.4s_.3s_both] text-muted">
        <span class="text-info">INFO</span> Reaching out to relay...
      </p>
      <p class="animate-[line-in_.4s_.9s_both] text-muted">
        <span class="text-info">INFO</span> Tunnel established at
      </p>
      <p class="animate-[line-in_.4s_1.3s_both] pl-4 text-highlighted">
        https://swiftfox.{{ config.relayDomain }}
      </p>
      <p class="animate-[line-in_.4s_1.7s_both]">
        <span class="text-dimmed select-none">$ </span><span class="inline-block h-4 w-2 translate-y-0.5 bg-(--ui-text-highlighted) animate-[caret_1s_steps(1)_infinite]" />
      </p>
    </div>
  </div>
</template>
