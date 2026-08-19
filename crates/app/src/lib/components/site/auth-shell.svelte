<script lang="ts">
	import { Check } from 'lucide-svelte';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { RELAY_DOMAIN } from '$lib/site';

	let { children }: { children: import('svelte').Snippet } = $props();

	const perks = [
		'Reserve up to 3 persistent subdomains',
		'Lock in up to 2 fixed remote TCP ports',
		'Bring your own custom domain'
	];
</script>

<main class="flex min-h-svh bg-background">
	<!-- Brand panel -->
	<aside
		class="relative hidden w-1/2 flex-col justify-between overflow-hidden border-r border-border bg-card p-10 lg:flex"
	>
		<div
			aria-hidden="true"
			class="relay-grid pointer-events-none absolute inset-0 opacity-50"
			style="mask-image: radial-gradient(ellipse 80% 80% at 30% 20%, black, transparent 80%); -webkit-mask-image: radial-gradient(ellipse 80% 80% at 30% 20%, black, transparent 80%);"
		></div>

		<div class="relative">
			<a href="/" aria-label="Relay home"><RelayLogo size={28} /></a>
		</div>

		<div class="relative">
			<div class="rounded-xl border border-border bg-background/60 p-5 backdrop-blur">
				<div class="mb-4 flex items-center gap-2">
					<span
						class="size-2 rounded-full bg-primary"
						style="animation: relay-pulse 1.8s ease-in-out infinite"
					></span>
					<span class="text-xs text-muted-foreground">tunnel active</span>
				</div>
				<div class="space-y-1.5 text-sm">
					<p class="text-foreground">
						<span class="text-primary">$</span> relay http 3000 --subdomain ada
					</p>
					<p class="break-all text-muted-foreground">
						&nbsp;&nbsp;forwarding <span class="text-primary">https://ada.{RELAY_DOMAIN}</span>
					</p>
					<p class="text-muted-foreground">
						&nbsp;&nbsp;<span class="text-accent">✓</span> reserved to your account
					</p>
				</div>
			</div>

			<h2 class="mt-8 text-balance text-2xl font-semibold tracking-tight text-foreground">
				One account. Every tunnel.
			</h2>
			<ul class="mt-5 space-y-3">
				{#each perks as perk (perk)}
					<li class="flex items-center gap-3 text-sm text-muted-foreground">
						<span
							class="flex size-5 items-center justify-center rounded-full bg-primary/15 text-primary ring-1 ring-inset ring-primary/30"
						>
							<Check class="size-3" />
						</span>
						{perk}
					</li>
				{/each}
			</ul>
		</div>

		<p class="relative text-xs text-muted-foreground">
			Open-source TCP tunneling · free forever
		</p>
	</aside>

	<!-- Form panel -->
	<section class="flex flex-1 flex-col">
		<div class="flex items-center justify-between border-b border-border px-6 py-4 lg:hidden">
			<a href="/" aria-label="Relay home"><RelayLogo size={26} /></a>
		</div>
		<div class="flex flex-1 items-center justify-center px-6 py-12">
			{@render children()}
		</div>
	</section>
</main>
