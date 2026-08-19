<script lang="ts">
	import { Globe, Laptop, Server } from 'lucide-svelte';

	let { publicLabel = 'myapp.relay' }: { publicLabel?: string } = $props();
</script>

{#snippet flowLine(reverse: boolean)}
	<div class="relative h-px flex-1 overflow-hidden bg-border">
		<div
			class="absolute top-1/2 h-px w-8 -translate-y-1/2 bg-primary"
			style="animation: relay-flow 2.4s linear infinite; animation-delay: {reverse
				? '1.2s'
				: '0s'}; box-shadow: 0 0 8px 1px var(--primary);"
		></div>
	</div>
{/snippet}

{#snippet node(Icon: typeof Globe, label: string, sub: string, accent: boolean)}
	<div class="flex flex-col items-center gap-2 text-center">
		<div
			class="flex size-12 items-center justify-center rounded-lg border {accent
				? 'border-primary/40 bg-primary/10 text-primary'
				: 'border-border bg-secondary text-foreground'}"
		>
			<Icon class="size-5" />
		</div>
		<div>
			<p class="font-mono text-xs font-medium text-foreground">{label}</p>
			<p class="font-mono text-[11px] text-muted-foreground">{sub}</p>
		</div>
	</div>
{/snippet}

<div class="rounded-xl border border-border bg-card/60 p-6">
	<div class="mb-5 flex items-center gap-2">
		<span
			class="size-2 rounded-full bg-primary"
			style="animation: relay-pulse 1.8s ease-in-out infinite"
		></span>
		<span class="font-mono text-xs text-muted-foreground">tunnel active</span>
	</div>
	<div class="flex items-center justify-between gap-2">
		{@render node(Laptop, 'localhost', ':3000', false)}
		{@render flowLine(false)}
		{@render node(Server, 'relay', 'edge', true)}
		{@render flowLine(true)}
		{@render node(Globe, 'public', publicLabel, false)}
	</div>
</div>
