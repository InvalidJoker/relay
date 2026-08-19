<script lang="ts">
	import { ArrowRight } from 'lucide-svelte';
	import GithubIcon from './github-icon.svelte';
	import TunnelVisual from './tunnel-visual.svelte';
	import { REPO_URL, RELAY_DOMAIN } from '$lib/site';

	let { user = null }: { user?: unknown } = $props();

	const publicUrl = `https://myapp.${RELAY_DOMAIN}`;
</script>

<section class="relative overflow-hidden">
	<div
		aria-hidden="true"
		class="relay-grid pointer-events-none absolute inset-0 opacity-60"
		style="mask-image: radial-gradient(ellipse 90% 70% at 50% -10%, black, transparent 75%); -webkit-mask-image: radial-gradient(ellipse 90% 70% at 50% -10%, black, transparent 75%);"
	></div>

	<div
		class="relative mx-auto grid max-w-6xl gap-12 px-5 py-20 md:py-28 lg:grid-cols-[1.05fr_0.95fr] lg:items-center"
	>
		<div>
			<div
				class="inline-flex items-center gap-2 rounded-full border border-border bg-secondary/60 px-3 py-1 text-xs text-muted-foreground"
			>
				<span class="relative flex size-2">
					<span
						class="absolute inline-flex size-full animate-ping rounded-full bg-primary opacity-60"
					></span>
					<span class="relative inline-flex size-2 rounded-full bg-primary"></span>
				</span>
				open-source · self-hostable · written in Rust
			</div>

			<h1
				class="mt-6 text-balance text-4xl font-semibold leading-[1.05] tracking-tight text-foreground sm:text-5xl lg:text-6xl"
			>
				Put localhost on the
				<span class="relative whitespace-nowrap text-primary">
					public internet
					<span
						aria-hidden="true"
						class="absolute -bottom-1 left-0 h-px w-full bg-gradient-to-r from-primary via-accent to-transparent"
					></span>
				</span>
			</h1>

			<p class="mt-5 max-w-md text-pretty text-lg leading-relaxed text-muted-foreground">
				Relay forwards your local HTTP and TCP servers to a shareable URL in one command. Create an
				account to reserve persistent subdomains, ports, and custom domains.
			</p>

			<div class="mt-8 flex flex-col gap-3 sm:flex-row">
				<a
					href={user ? '/dashboard' : '/register'}
					class="group inline-flex items-center justify-center gap-2 rounded-md bg-primary px-5 py-2.5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
				>
					{user ? 'Go to dashboard' : 'Create free account'}
					<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
				</a>
				<a
					href={REPO_URL}
					target="_blank"
					rel="noopener noreferrer"
					class="inline-flex items-center justify-center gap-2 rounded-md border border-border bg-secondary px-5 py-2.5 text-sm font-medium text-foreground transition-colors hover:bg-muted"
				>
					<GithubIcon class="size-4" />
					View source
				</a>
			</div>

			<dl class="mt-10 flex gap-8 border-t border-border/60 pt-6">
				<div>
					<dt class="text-xs text-muted-foreground">protocols</dt>
					<dd class="mt-1 text-lg font-semibold text-foreground">
						HTTP<span class="text-muted-foreground">&nbsp;+ TCP</span>
					</dd>
				</div>
				<div>
					<dt class="text-xs text-muted-foreground">setup</dt>
					<dd class="mt-1 text-lg font-semibold text-foreground">
						1<span class="text-muted-foreground">&nbsp;cmd</span>
					</dd>
				</div>
				<div>
					<dt class="text-xs text-muted-foreground">price</dt>
					<dd class="mt-1 text-lg font-semibold text-foreground">$0</dd>
				</div>
			</dl>
		</div>

		<div class="space-y-4">
			<!-- Terminal demo -->
			<div class="overflow-hidden rounded-xl border border-border bg-card shadow-2xl shadow-black/40">
				<div class="flex items-center gap-2 border-b border-border px-4 py-3">
					<span class="size-3 rounded-full bg-muted"></span>
					<span class="size-3 rounded-full bg-muted"></span>
					<span class="size-3 rounded-full bg-muted"></span>
					<span class="ml-2 text-xs text-muted-foreground">relay — zsh</span>
				</div>
				<div class="space-y-1.5 p-4 text-sm leading-relaxed">
					<p class="text-foreground">
						<span class="text-primary">$</span> relay http 3000 --subdomain myapp
					</p>
					<p class="text-muted-foreground">&nbsp;&nbsp;connecting to relay edge...</p>
					<p class="text-muted-foreground">
						&nbsp;&nbsp;tunnel established <span class="text-accent">✓</span>
					</p>
					<p class="break-all text-foreground">
						&nbsp;&nbsp;forwarding&nbsp;&nbsp;<span class="text-primary">{publicUrl}</span>
					</p>
					<p class="text-foreground">
						&nbsp;&nbsp;<span class="text-muted-foreground">→</span> http://localhost:3000
					</p>
					<p class="pt-1 text-foreground">
						<span class="text-primary">$</span>
						<span
							aria-hidden="true"
							class="inline-block h-4 w-2 translate-y-0.5 bg-foreground"
							style="animation: relay-blink 1.1s step-end infinite"
						></span>
					</p>
				</div>
			</div>

			<TunnelVisual publicLabel="myapp.{RELAY_DOMAIN}" />
		</div>
	</div>
</section>
