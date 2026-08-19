<script lang="ts">
	import { Check, Copy } from 'lucide-svelte';

	let {
		code,
		prompt = '$',
		label = undefined
	}: { code: string; prompt?: string | null; label?: string } = $props();

	let copied = $state(false);
	let timer: ReturnType<typeof setTimeout> | undefined;

	const lines = $derived(code.split('\n'));

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
			<span class="font-mono text-xs text-muted-foreground">{label}</span>
		</div>
	{/if}

	<div class="flex items-start gap-3 px-4 py-3.5">
		<div class="min-w-0 flex-1 overflow-x-auto">
			<pre class="font-mono text-[13px] leading-6 tracking-tight text-foreground">{#each lines as line, i (i)}<span
						class="block"
						>{#if prompt}<span class="mr-2 select-none text-primary">{i === 0
									? prompt
									: ' '.repeat(prompt.length)}</span>{/if}{line}</span
					>{/each}</pre>
		</div>
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
