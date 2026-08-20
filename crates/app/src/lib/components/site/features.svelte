<script lang="ts">
	import { Globe, Lock, Network, Pin, Settings2, TerminalSquare } from 'lucide-svelte';
	import SectionEyebrow from './section-eyebrow.svelte';
	import { RELAY_DOMAIN } from '$lib/site';

	const features = [
		{
			icon: Globe,
			title: 'HTTP tunnels',
			body: `Forward any local web server to a public HTTPS URL on ${RELAY_DOMAIN} — with a random name, or a subdomain of your choice.`
		},
		{
			icon: Network,
			title: 'Raw TCP tunnels',
			body: 'Expose game servers, databases, or any TCP service. Reserve a fixed remote port when you need one that never changes.'
		},
		{
			icon: Pin,
			title: 'Persistent domains',
			body: 'Reserve subdomains and bring your own custom domains from your account, so the URL you share stays the same between sessions.'
		},
		{
			icon: Lock,
			title: 'Basic auth',
			body: 'Protect any HTTP tunnel with a username and password using a single pair of flags. Only the right people get in.'
		},
		{
			icon: Settings2,
			title: 'Config files',
			body: 'Save a tunnel with --save and bring it back later with relay run. No need to retype your flags every time.'
		},
		{
			icon: TerminalSquare,
			title: 'Self-hostable',
			body: 'Run your own relay server and point the CLI at it with relay login <url>. Fully open-source, written in Rust.'
		}
	];
</script>

<section id="features" class="scroll-mt-16 border-t border-border">
	<div class="mx-auto max-w-6xl px-5 py-20 md:py-24">
		<div class="max-w-xl">
			<SectionEyebrow>Features</SectionEyebrow>
			<h2
				class="mt-4 text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl"
			>
				Everything you need to share local work
			</h2>
			<p class="mt-4 text-pretty leading-relaxed text-muted-foreground">
				A small, fast CLI that does one thing well — get your machine online without the setup
				overhead.
			</p>
		</div>

		<div
			class="mt-12 grid gap-px overflow-hidden rounded-xl border border-border bg-border sm:grid-cols-2 lg:grid-cols-3"
		>
			{#each features as feature (feature.title)}
				<div class="group relative bg-card p-6 transition-colors hover:bg-secondary/50">
					<span
						class="absolute inset-x-0 top-0 h-px scale-x-0 bg-gradient-to-r from-primary to-signal transition-transform duration-300 group-hover:scale-x-100"
					></span>
					<div
						class="flex size-10 items-center justify-center rounded-lg bg-primary/10 ring-1 ring-inset ring-primary/25"
					>
						<feature.icon class="size-5 text-primary" />
					</div>
					<h3 class="mt-4 font-medium text-foreground">{feature.title}</h3>
					<p class="mt-2 text-sm leading-relaxed text-muted-foreground">{feature.body}</p>
				</div>
			{/each}
		</div>
	</div>
</section>
