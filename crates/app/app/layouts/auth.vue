<template>
  <div class="relative flex min-h-svh flex-col">
    <div class="bg-grid pointer-events-none absolute inset-0 [mask-image:radial-gradient(ellipse_at_center,black_20%,transparent_70%)]" aria-hidden="true" />

    <header class="relative flex h-(--ui-header-height) items-center justify-between px-4 sm:px-6">
      <NuxtLink to="/" aria-label="Relay home">
        <AppLogo />
      </NuxtLink>
      <UColorModeButton color="neutral" variant="ghost" size="sm" />
    </header>

    <main class="relative flex flex-1 items-center justify-center px-4 py-10">
      <div class="relative w-full max-w-sm border border-default bg-default">
        <Crosses />
        <slot />
      </div>
    </main>

    <p class="relative pb-6 text-center eyebrow">
      Open source tunneling · MIT
    </p>
  </div>
</template>
