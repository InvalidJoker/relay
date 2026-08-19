<script lang="ts">
	let {
		size = 28,
		showWordmark = true,
		class: className = ''
	}: { size?: number; showWordmark?: boolean; class?: string } = $props();

	const mark = $derived(Math.round(size));
	const glyph = $derived(Math.round(size * 0.64));
</script>

<span class="inline-flex items-center gap-2.5 {className}">
	<span
		class="relative inline-flex items-center justify-center rounded-lg bg-primary/12 ring-1 ring-inset ring-primary/30"
		style="width: {mark}px; height: {mark}px"
	>
		<svg
			width={glyph}
			height={glyph}
			viewBox="0 0 24 24"
			fill="none"
			aria-hidden="true"
			class="text-primary"
		>
			<!-- routing path: local -> node -> public -->
			<path
				d="M3 17 L9 17 L13 7 L21 7"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
			<circle cx="3" cy="17" r="2" fill="currentColor" />
			<circle cx="21" cy="7" r="2" class="fill-accent" />
		</svg>
	</span>
	{#if showWordmark}
		<span
			class="font-semibold tracking-tight text-foreground"
			style="font-size: {Math.round(size * 0.56)}px"
		>
			relay
		</span>
	{/if}
</span>
