<script lang="ts">
	import { ArrowRight, Check, Copy } from 'lucide-svelte';
	import SectionEyebrow from './section-eyebrow.svelte';
	import {
		INSTALL_COMMANDS,
		RELAY_DOMAIN,
		detectInstallOS,
		type InstallOS
	} from '$lib/site';

	// Only the one-liner installers here — "from source" lives in the Install section.
	const targets: [InstallOS, (typeof INSTALL_COMMANDS)[InstallOS]][] = [
		['unix', INSTALL_COMMANDS.unix],
		['windows', INSTALL_COMMANDS.windows]
	];

	let osTab = $state<InstallOS>(detectInstallOS());
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

	const steps = $derived([
		{
			number: '01',
			title: 'Install',
			description: 'Grab the relay CLI with a single command.',
			install: true,
			prompt: INSTALL_COMMANDS[osTab].prompt,
			command: INSTALL_COMMANDS[osTab].code
		},
		{
			number: '02',
			title: 'Login',
			description: 'Authenticate with your Relay account.',
			install: false,
			prompt: '$',
			command: 'relay login'
		},
		{
			number: '03',
			title: 'Connect',
			description: 'Forward any local port to the internet with one command.',
			install: false,
			prompt: '$',
			command: 'relay http 8080'
		},
		{
			number: '04',
			title: 'Share',
			description: 'Instantly share the URL with anyone.',
			install: false,
			prompt: null,
			command: `→ myapp.${RELAY_DOMAIN}`
		}
	]);
</script>

{#snippet cmdBox(code: string, prompt: string | null)}
	<div
		class="flex items-center gap-2 rounded-lg border border-border bg-background/60 px-3 py-2.5 font-mono text-xs"
	>
		{#if prompt}
			<span aria-hidden="true" class="shrink-0 select-none text-primary">{prompt}</span>
		{/if}
		<span class="flex-1 overflow-x-auto whitespace-nowrap text-foreground">{code}</span>
		<button
			type="button"
			onclick={() => copy(code)}
			aria-label="Copy command"
			class="shrink-0 rounded text-muted-foreground transition-colors hover:text-foreground"
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
		<div class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
			<div class="max-w-xl">
				<SectionEyebrow>How it works</SectionEyebrow>
				<h2
					class="mt-4 text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl"
				>
					Up and running in 60 seconds
				</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Four commands from a cold machine to a public URL.
				</p>
			</div>

			<!-- Platform switch drives the install command below -->
			<div
				role="tablist"
				aria-label="Installation platform"
				class="flex shrink-0 gap-1 self-start rounded-lg border border-border bg-secondary/50 p-1 md:self-auto"
			>
				{#each targets as [id, target] (id)}
					<button
						type="button"
						role="tab"
						aria-selected={osTab === id}
						onclick={() => (osTab = id)}
						class="rounded-md px-3 py-1.5 text-xs font-medium transition-colors {osTab === id
							? 'bg-card text-foreground'
							: 'text-muted-foreground hover:text-foreground'}"
					>
						{target.label}
					</button>
				{/each}
			</div>
		</div>

		<ol class="mt-12 grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
			{#each steps as step, i (step.number)}
				<li class="relative flex">
					<div
						class="flex w-full flex-col gap-3 rounded-xl border border-border bg-card p-5 transition-colors hover:border-primary/30"
					>
						<div class="flex items-center gap-3">
							<span
								class="flex size-8 items-center justify-center rounded-lg bg-primary/10 font-mono text-xs font-medium text-primary ring-1 ring-inset ring-primary/25"
							>
								{step.number}
							</span>
							<h3 class="text-base font-medium text-foreground">{step.title}</h3>
						</div>
						<p class="text-sm leading-relaxed text-muted-foreground">{step.description}</p>
						<div class="mt-auto pt-2">
							{@render cmdBox(step.command, step.prompt)}
						</div>
					</div>

					{#if i < steps.length - 1}
						<span
							aria-hidden="true"
							class="absolute -right-4 top-1/2 hidden -translate-y-1/2 text-border xl:block"
						>
							<ArrowRight class="size-4" />
						</span>
					{/if}
				</li>
			{/each}
		</ol>

		<div class="mt-8 flex flex-wrap items-center gap-x-6 gap-y-3">
			<a
				href="/docs"
				class="group inline-flex items-center gap-2 text-sm font-medium text-primary transition-opacity hover:opacity-80"
			>
				Read the full documentation
				<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
			</a>
			<p class="text-sm text-muted-foreground">{INSTALL_COMMANDS[osTab].note}</p>
		</div>
	</div>
</section>
