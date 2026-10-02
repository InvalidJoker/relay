<template>
  <footer class="border-t border-default">
    <div class="mx-auto flex max-w-6xl flex-col gap-4 border-x border-default px-4 py-8 sm:flex-row sm:items-center sm:px-6">
      <AppLogo />
      <p class="text-sm text-muted sm:ml-4">
        Open source tunneling for developers.
      </p>
      <div class="flex items-center gap-4 sm:ml-auto">
        <span class="eyebrow">MIT License</span>
        <UButton
          :to="REPO_URL"
          target="_blank"
          icon="i-simple-icons-github"
          color="neutral"
          variant="ghost"
          size="sm"
          aria-label="GitHub"
        />
      </div>
    </div>
  </footer>
</template>
