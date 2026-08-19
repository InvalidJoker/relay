<script lang="ts">
	import { browser } from '$app/environment';
	import CodeBlock from './code-block.svelte';
	import SectionEyebrow from './section-eyebrow.svelte';
	import { INSTALL_COMMANDS, type InstallTarget } from '$lib/site';

	const targets = Object.entries(INSTALL_COMMANDS) as [
		InstallTarget,
		(typeof INSTALL_COMMANDS)[InstallTarget]
	][];

	function detectOS(): InstallTarget {
		if (!browser) return 'unix';
		return /win/i.test(navigator.userAgent) ? 'windows' : 'unix';
	}

	let active = $state<InstallTarget>(detectOS());
	const current = $derived(INSTALL_COMMANDS[active]);
</script>

<section id="install" class="scroll-mt-16 border-t border-border">
	<div class="mx-auto max-w-6xl px-5 py-20 md:py-24">
		<div class="max-w-xl">
			<SectionEyebrow>Install</SectionEyebrow>
			<h2
				class="mt-4 text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl"
			>
				One command to get running
			</h2>
			<p class="mt-4 text-pretty leading-relaxed text-muted-foreground">
				The installer detects your OS and architecture, then drops the binary on your PATH.
			</p>
		</div>

		<div class="mt-10 max-w-3xl">
			<div
				role="tablist"
				aria-label="Installation method"
				class="flex flex-wrap gap-1 rounded-lg border border-border bg-secondary/50 p-1"
			>
				{#each targets as [id, target] (id)}
					<button
						type="button"
						role="tab"
						aria-selected={active === id}
						onclick={() => (active = id)}
						class="rounded-md px-3.5 py-1.5 text-xs transition-colors {active === id
							? 'bg-card text-foreground'
							: 'text-muted-foreground hover:text-foreground'}"
					>
						{target.label}
					</button>
				{/each}
			</div>

			<div class="mt-4 space-y-4">
				<CodeBlock
					code={current.code}
					prompt={current.prompt}
					label={active === 'source' ? 'requires Rust (edition 2024 / stable)' : undefined}
				/>
				<p class="text-xs text-muted-foreground">{current.note}</p>
			</div>
		</div>
	</div>
</section>
