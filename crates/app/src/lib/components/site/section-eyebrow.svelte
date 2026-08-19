<script lang="ts">
	let { children }: { children: import('svelte').Snippet } = $props();
</script>

<span class="inline-flex items-center gap-2 text-xs uppercase tracking-[0.2em] text-primary">
	<span class="h-px w-6 bg-primary"></span>
	{@render children()}
</span>
