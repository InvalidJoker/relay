<script setup lang="ts">
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useSeoMeta({ title: 'Connect CLI - Relay' })

const origin = useRequestURL().origin

const steps = [
  { title: 'Install the CLI', description: 'Downloads the right binary for your OS and architecture.' },
  { title: 'Log in', description: 'Opens this site so you can approve the CLI. Credentials are saved to ~/.config/relay.toml.', command: `relay login ${origin}` },
  { title: 'Expose an HTTP service', description: 'Forward a local HTTP server to a public URL. Use one of your subdomains or custom domains.', command: 'relay http 3000 --subdomain myapp' },
  { title: 'Expose a TCP service', description: 'Forward a local TCP port. Pass one of your reserved ports as the second argument.', command: 'relay tcp 25565 10000' }
]
</script>

<template>
  <UDashboardPanel id="cli">
    <template #header>
      <UDashboardNavbar title="Connect CLI">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="mx-auto flex w-full max-w-3xl flex-col gap-8">
        <div>
          <span class="eyebrow">[ Quickstart ]</span>
          <h1 class="mt-2 text-2xl font-semibold tracking-tight text-highlighted">
            Connect the Relay CLI
          </h1>
          <p class="mt-1 text-sm text-muted">
            Install the CLI and link it to your account.
          </p>
        </div>

        <ol class="relative border border-default">
          <Crosses />
          <li
            v-for="(step, i) in steps"
            :key="step.title"
            class="grid gap-4 p-5 not-last:border-b border-default sm:grid-cols-[2rem_1fr]"
          >
            <span class="flex size-7 items-center justify-center border border-default bg-muted font-mono text-xs text-highlighted">
              {{ i + 1 }}
            </span>
            <div class="min-w-0 space-y-3">
              <div>
                <h2 class="font-medium text-highlighted">
                  {{ step.title }}
                </h2>
                <p class="mt-0.5 text-sm text-muted">
                  {{ step.description }}
                </p>
              </div>
              <InstallCommand v-if="i === 0" />
              <CopyCommand v-else :code="step.command!" />
            </div>
          </li>
        </ol>
      </div>
    </template>
  </UDashboardPanel>
</template>
