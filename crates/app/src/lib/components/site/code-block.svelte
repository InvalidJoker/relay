<script lang="ts">
	import { Check, Copy } from 'lucide-svelte';

	let {
		code,
		prompt = '$',
		label = undefined
	}: { code: string; prompt?: string | null; label?: string } = $props();

	let copied = $state(false);
	let timer: ReturnType<typeof setTimeout> | undefined;

	async function handleCopy() {
		try {
			await navigator.clipboard.writeText(code);
			copied = true;
			clearTimeout(timer);
			timer = setTimeout(() => (copied = false), 1600);
		} catch {
			// clipboard unavailable
		}
	}
</script>

<div class="group relative overflow-hidden rounded-lg border border-border bg-card">
	{#if label}
		<div class="flex items-center justify-between border-b border-border px-4 py-2">
			<span class="text-xs text-muted-foreground">{label}</span>
		</div>
	{/if}
	<div class="flex items-start gap-3 px-4 py-3.5">
		{#if prompt}
			<span aria-hidden="true" class="select-none text-sm text-primary">{prompt}</span>
		{/if}
		<code class="flex-1 overflow-x-auto whitespace-pre text-sm text-foreground">{code}</code>
		<button
			type="button"
			onclick={handleCopy}
			aria-label={copied ? 'Copied' : 'Copy command'}
			class="shrink-0 rounded-md border border-border bg-secondary p-1.5 text-muted-foreground transition-colors hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
		>
			{#if copied}
				<Check class="size-4 text-primary" />
			{:else}
				<Copy class="size-4" />
			{/if}
		</button>
	</div>
</div>
