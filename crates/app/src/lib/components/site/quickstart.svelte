<script lang="ts">
	import { browser } from '$app/environment';
	import { Check, Copy } from 'lucide-svelte';
	import SectionEyebrow from './section-eyebrow.svelte';
	import { INSTALL_COMMANDS, RELAY_DOMAIN, type InstallTarget } from '$lib/site';

	// Only the one-liner installers here — "from source" lives in the Install section.
	type OS = Extract<InstallTarget, 'unix' | 'windows'>;
	const targets: [OS, (typeof INSTALL_COMMANDS)[OS]][] = [
		['unix', INSTALL_COMMANDS.unix],
		['windows', INSTALL_COMMANDS.windows]
	];

	function detectOS(): OS {
		if (!browser) return 'unix';
		return /win/i.test(navigator.userAgent) ? 'windows' : 'unix';
	}

	let osTab = $state<OS>(detectOS());
	let copied = $state<string | null>(null);
	let timer: ReturnType<typeof setTimeout> | undefined;

	async function copy(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copied = text;
			clearTimeout(timer);
			timer = setTimeout(() => {
				if (copied === text) copied = null;
			}, 1600);
		} catch {
			// clipboard unavailable
		}
	}

	const steps = [
		{
			number: '01',
			title: 'Install',
			description: 'Grab the relay CLI with a single command.',
			install: true,
			command: ''
		},
		{
			number: '02',
			title: 'Connect',
			description: 'Forward any local port to the internet with one command.',
			install: false,
			command: 'relay http 8080'
		},
		{
			number: '03',
			title: 'Share',
			description: 'Instantly share the URL with anyone.',
			install: false,
			command: `→ myapp.${RELAY_DOMAIN}`
		}
	];
</script>

{#snippet cmdBox(code: string)}
	<div
		class="flex items-start gap-2 rounded-lg border border-border bg-background/60 px-3 py-2 text-xs"
	>
		<span class="break-all text-foreground">{code}</span>
		<button
			type="button"
			onclick={() => copy(code)}
			aria-label="Copy command"
			class="ml-auto shrink-0 text-muted-foreground transition-colors hover:text-foreground"
		>
			{#if copied === code}
				<Check class="size-3.5 text-primary" />
			{:else}
				<Copy class="size-3.5" />
			{/if}
		</button>
	</div>
{/snippet}

<!-- Up and running — kept from the original Relay homepage -->
<section id="quickstart" class="relative scroll-mt-16 overflow-hidden border-t border-border">
	<div
		aria-hidden="true"
		class="relay-grid pointer-events-none absolute inset-0 opacity-40"
		style="mask-image: radial-gradient(ellipse 70% 90% at 50% 0%, black, transparent 80%); -webkit-mask-image: radial-gradient(ellipse 70% 90% at 50% 0%, black, transparent 80%);"
	></div>

	<div class="relative mx-auto max-w-6xl px-5 py-20 md:py-24">
		<div class="max-w-xl">
			<SectionEyebrow>How it works</SectionEyebrow>
			<h2
				class="mt-4 text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl"
			>
				Up and running in 60 seconds
			</h2>
			<p class="mt-4 leading-relaxed text-muted-foreground">Three commands. That's it.</p>
		</div>

		<div class="mt-12 grid gap-4 md:grid-cols-3">
			{#each steps as step (step.number)}
				<div class="flex flex-col gap-3 rounded-xl border border-border bg-card p-6">
					<div class="flex items-center gap-3">
						<span
							class="rounded-md bg-primary/10 px-2 py-0.5 text-sm text-primary ring-1 ring-inset ring-primary/25"
						>
							{step.number}
						</span>
						<h3 class="text-lg font-medium text-foreground">{step.title}</h3>
					</div>
					<p class="text-sm leading-relaxed text-muted-foreground">{step.description}</p>

					<div class="mt-auto flex flex-col gap-2 pt-2">
						{#if step.install}
							<div class="flex gap-1 rounded-lg border border-border bg-secondary/50 p-1">
								{#each targets as [id, target] (id)}
									<button
										type="button"
										onclick={() => (osTab = id)}
										class="flex-1 rounded-md px-2 py-1 text-xs font-medium transition-colors {osTab ===
										id
											? 'bg-card text-foreground'
											: 'text-muted-foreground hover:text-foreground'}"
									>
										{target.label}
									</button>
								{/each}
							</div>
							{@render cmdBox(INSTALL_COMMANDS[osTab].code)}
						{:else}
							{@render cmdBox(step.command)}
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>
