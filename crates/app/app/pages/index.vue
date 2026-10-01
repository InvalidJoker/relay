<script setup lang="ts">
useSeoMeta({
  title: 'Relay - Open source TCP tunneling',
  description: 'Relay is a free, open-source TCP tunneling service with persistent domains, custom domains, subdomains, and self-hosting support.'
})

const { loggedIn } = useAuth()
const { data: config } = useRelayConfig()

const features = [
  { icon: 'i-lucide-globe', title: 'Persistent domains', description: 'Your friends never need to update the URL. Get a permanent domain that sticks across sessions.' },
  { icon: 'i-lucide-link', title: 'Custom domains', description: 'Bring your own domain and map it directly to your relay tunnel. Full ownership.' },
  { icon: 'i-lucide-zap', title: 'Subdomains', description: 'Claim up to 3 subdomains on our domain. Share clean, memorable URLs instantly.' },
  { icon: 'i-lucide-plug', title: 'Remote ports', description: 'Reserve a fixed remote port so your tunnels are always reachable on the same address.' },
  { icon: 'i-lucide-lock', title: 'Open source', description: 'No black boxes. Relay\'s source is fully open. Audit it, contribute, or self-host.' },
  { icon: 'i-lucide-server', title: 'Self-hostable', description: 'Run your own Relay server. Total control over your infrastructure and data.' }
]

const steps = computed(() => [
  { title: 'Install', description: 'Grab the relay CLI with a single command.' },
  { title: 'Connect', description: 'Forward any local port to the internet with one command.', command: 'relay http 8080' },
  { title: 'Share', description: 'Instantly share the URL with anyone.', command: `myapp.${config.value.relayDomain}` }
])
</script>

<template>
  <div class="flex min-h-svh flex-col">
    <AppHeader />

    <main class="flex-1">
      <!-- Hero -->
      <section class="border-b border-default">
        <div class="relative mx-auto grid max-w-6xl border-x border-default lg:grid-cols-[1.1fr_1fr]">
          <div class="relative flex flex-col justify-center gap-8 px-4 py-16 sm:px-10 sm:py-24">
            <span class="eyebrow">[ Open source &amp; free to use ]</span>
            <h1 class="text-4xl font-semibold tracking-tighter text-balance text-highlighted sm:text-5xl lg:text-6xl">
              Share your local server with the world.
            </h1>
            <p class="max-w-lg text-lg text-pretty text-muted">
              Relay tunnels your localhost to the internet instantly - with persistent domains, custom URLs, and zero configuration.
            </p>
            <div class="flex flex-wrap gap-3">
              <UButton
                v-if="loggedIn"
                to="/dashboard"
                label="Go to dashboard"
                size="lg"
                trailing-icon="i-lucide-arrow-right"
              />
              <UButton
                v-else
                to="/register"
                label="Start tunneling free"
                size="lg"
                trailing-icon="i-lucide-arrow-right"
              />
              <UButton
                :to="REPO_URL"
                target="_blank"
                label="View on GitHub"
                icon="i-simple-icons-github"
                color="neutral"
                variant="outline"
                size="lg"
              />
            </div>
          </div>

          <div class="relative flex items-center border-t border-default px-4 py-12 sm:px-10 lg:border-t-0 lg:border-l">
            <div class="bg-stripes absolute inset-0 opacity-70" aria-hidden="true" />
            <TerminalDemo class="w-full" />
          </div>

          <span class="cross absolute -bottom-[5px] -left-[5px] z-10" aria-hidden="true" />
          <span class="cross absolute -bottom-[5px] -right-[5px] z-10" aria-hidden="true" />
        </div>
      </section>

      <!-- Features -->
      <section id="features" class="scroll-mt-(--ui-header-height) border-b border-default">
        <div class="mx-auto max-w-6xl border-x border-default">
          <div class="flex flex-col gap-3 border-b border-default px-4 py-12 sm:px-10">
            <span class="eyebrow">[ Features ]</span>
            <h2 class="text-3xl font-semibold tracking-tight text-highlighted">
              Everything you need to tunnel
            </h2>
            <p class="text-muted">
              Built for developers who want reliability, flexibility, and freedom.
            </p>
          </div>

          <div class="grid sm:grid-cols-2 lg:grid-cols-3">
            <div
              v-for="(feature, i) in features"
              :key="feature.title"
              class="group relative flex flex-col gap-3 border-default p-6 transition-colors hover:bg-muted sm:p-8 max-sm:not-last:border-b sm:max-lg:odd:border-r sm:max-lg:[&:nth-child(-n+4)]:border-b lg:not-[:nth-child(3n)]:border-r lg:[&:nth-child(-n+3)]:border-b"
            >
              <div class="flex items-center justify-between">
                <UIcon :name="feature.icon" class="size-5 text-highlighted" />
                <span class="font-mono text-xs text-dimmed">0{{ i + 1 }}</span>
              </div>
              <h3 class="font-medium text-highlighted">
                {{ feature.title }}
              </h3>
              <p class="text-sm leading-relaxed text-muted">
                {{ feature.description }}
              </p>
            </div>
          </div>
        </div>
      </section>

      <!-- How it works -->
      <section id="how-it-works" class="scroll-mt-(--ui-header-height) border-b border-default">
        <div class="mx-auto max-w-6xl border-x border-default">
          <div class="flex flex-col gap-3 border-b border-default px-4 py-12 sm:px-10">
            <span class="eyebrow">[ How it works ]</span>
            <h2 class="text-3xl font-semibold tracking-tight text-highlighted">
              Up and running in 60 seconds
            </h2>
            <p class="text-muted">
              Three commands. That's it.
            </p>
          </div>

          <ol class="grid lg:grid-cols-3">
            <li
              v-for="(step, i) in steps"
              :key="step.title"
              class="flex min-w-0 flex-col gap-4 border-default p-6 sm:p-8 max-lg:not-last:border-b lg:not-last:border-r"
            >
              <div class="flex items-center gap-3">
                <span class="flex size-7 items-center justify-center border border-default font-mono text-xs text-highlighted">
                  {{ i + 1 }}
                </span>
                <h3 class="font-medium text-highlighted">
                  {{ step.title }}
                </h3>
              </div>
              <p class="text-sm text-muted">
                {{ step.description }}
              </p>
              <div class="mt-auto">
                <InstallCommand v-if="i === 0" />
                <CopyCommand v-else :code="step.command!" :prompt="i === 1 ? '$' : '→'" />
              </div>
            </li>
          </ol>
        </div>
      </section>

      <!-- Open source -->
      <section class="border-b border-default">
        <div class="mx-auto grid max-w-6xl border-x border-default lg:grid-cols-2">
          <div class="flex flex-col gap-4 px-4 py-12 sm:px-10">
            <span class="eyebrow">[ Built in the open ]</span>
            <h2 class="text-3xl font-semibold tracking-tight text-highlighted">
              MIT licensed, no black boxes
            </h2>
            <p class="max-w-md text-muted">
              Relay is MIT-licensed and fully open source. Star us on GitHub, file issues, contribute code, or fork it to build your own tunneling service.
            </p>
            <div>
              <UButton
                :to="REPO_URL"
                target="_blank"
                label="View on GitHub"
                icon="i-simple-icons-github"
                color="neutral"
                variant="outline"
                trailing-icon="i-lucide-arrow-up-right"
              />
            </div>
          </div>
          <ul class="grid grid-cols-2 border-t border-default lg:border-t-0 lg:border-l">
            <li
              v-for="(item, i) in ['MIT Licensed', 'Self-hostable', 'Community-driven', 'No telemetry']"
              :key="item"
              class="flex items-center gap-3 border-default p-6 sm:p-8"
              :class="[i % 2 === 0 && 'border-r', i < 2 && 'border-b']"
            >
              <UIcon name="i-lucide-check" class="size-4 shrink-0 text-highlighted" />
              <span class="text-sm text-toned">{{ item }}</span>
            </li>
          </ul>
        </div>
      </section>

      <!-- CTA -->
      <section>
        <div class="relative mx-auto max-w-6xl border-x border-default">
          <div class="bg-grid absolute inset-0 [mask-image:radial-gradient(ellipse_at_center,black,transparent_75%)]" aria-hidden="true" />
          <div class="relative flex flex-col items-center gap-6 px-4 py-20 text-center sm:px-10">
            <h2 class="text-3xl font-semibold tracking-tight text-balance text-highlighted sm:text-4xl">
              Ready to share your localhost?
            </h2>
            <p class="max-w-md text-muted">
              Create a free account and get persistent domains, subdomains, and more.
            </p>
            <div class="flex flex-wrap justify-center gap-3">
              <template v-if="loggedIn">
                <UButton to="/dashboard" label="Go to dashboard" size="lg" trailing-icon="i-lucide-arrow-right" />
              </template>
              <template v-else>
                <UButton to="/register" label="Get started for free" size="lg" trailing-icon="i-lucide-arrow-right" />
                <UButton to="/login" label="Sign in" color="neutral" variant="outline" size="lg" />
              </template>
            </div>
          </div>
        </div>
      </section>
    </main>

    <AppFooter />
  </div>
</template>
