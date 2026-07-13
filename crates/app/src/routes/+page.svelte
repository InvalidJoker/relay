<script lang="ts">
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import RelayHop from '$lib/components/relay-hop.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Card from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';
	import {
		Globe,
		ArrowRight,
		Zap,
		Lock,
		Link,
		Server,
		Plug,
		Terminal,
		CheckCircle,
		Check,
		Copy,
		LayoutDashboard,
		Menu,
		X,
		LogIn
	} from 'lucide-svelte';
	import type { PageServerData } from './$types';
	import { env } from '$env/dynamic/public';
	import { browser } from '$app/environment';

	import Icon from '@iconify/svelte';

	let { data }: { data: PageServerData } = $props();

	let mobileMenuOpen = $state(false);
	let copied = $state<string | null>(null);

	const relayDomain = env.PUBLIC_RELAY_DOMAIN || 'relay.invalidjoker.dev';
	const repo = 'https://github.com/InvalidJoker/relay';

	const installCommands = {
		unix: {
			label: 'macOS / Linux',
			code: 'curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh'
		},
		windows: {
			label: 'Windows',
			code: 'irm https://raw.githubusercontent.com/InvalidJoker/relay/main/install.ps1 | iex'
		}
	} as const;

	type OS = keyof typeof installCommands;

	function detectOS(): OS {
		if (!browser) return 'unix';
		return /win/i.test(navigator.userAgent) ? 'windows' : 'unix';
	}

	let osTab = $state<OS>(detectOS());

	async function copy(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copied = text;
			setTimeout(() => {
				if (copied === text) copied = null;
			}, 1600);
		} catch {
			// clipboard unavailable — ignore
		}
	}

	const features = [
		{
			icon: Globe,
			title: 'Persistent Domains',
			description:
				'Your friends never need to update the URL. Get a permanent domain that sticks across sessions.'
		},
		{
			icon: Link,
			title: 'Custom Domains',
			description: 'Bring your own domain and map it directly to your relay tunnel. Full ownership.'
		},
		{
			icon: Zap,
			title: 'Subdomains',
			description: 'Claim up to 3 subdomains on our domain. Share clean, memorable URLs instantly.'
		},
		{
			icon: Plug,
			title: 'Remote Ports',
			description:
				'Reserve a fixed remote port so your tunnels are always reachable on the same address.'
		},
		{
			icon: Lock,
			title: 'Open Source',
			description: "No black boxes. Relay's source is fully open. Audit it, contribute, or self-host."
		},
		{
			icon: Server,
			title: 'Self-Hostable',
			description: 'Run your own Relay server. Total control over your infrastructure and data.'
		}
	];

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
			command: `→ myapp.${relayDomain}`
		}
	];
</script>

<svelte:head>
	<title>Relay — Open Source TCP Tunneling</title>
	<meta
		name="description"
		content="Relay is a free, open-source TCP tunneling service with persistent domains, custom domains, subdomains, and self-hosting support."
	/>
</svelte:head>

<!-- NAVBAR -->
<nav class="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-xl">
	<div class="mx-auto flex h-14 max-w-6xl items-center gap-6 px-4">
		<a href="/" class="shrink-0 no-underline">
			<RelayLogo size={24} />
		</a>

		<div class="hidden items-center gap-1 md:flex">
			<Button variant="ghost" href="#features" size="sm" class="text-muted-foreground">Features</Button>
			<Button variant="ghost" href="#how-it-works" size="sm" class="text-muted-foreground">
				How it works
			</Button>
			<Button variant="ghost" href={repo} size="sm" class="gap-1.5 text-muted-foreground">
				<Icon icon="simple-icons:github" width="14" height="14" />
				GitHub
			</Button>
		</div>

		<div class="ml-auto hidden items-center gap-2 md:flex">
			{#if data.user}
				<Button href="/dashboard" size="sm">
					<LayoutDashboard size={14} />
					Dashboard
				</Button>
				<form method="post" action="?/signOut">
					<Button type="submit" variant="outline" size="sm" class="gap-1.5">
						<LogIn size={14} class="rotate-180" />
						Sign out
					</Button>
				</form>
			{:else}
				<Button variant="ghost" href="/login" size="sm">Sign in</Button>
				<Button href="/register" size="sm">
					Get Started
					<ArrowRight size={14} />
				</Button>
			{/if}
		</div>

		<Button
			variant="ghost"
			size="icon-sm"
			class="ml-auto md:hidden"
			onclick={() => (mobileMenuOpen = !mobileMenuOpen)}
		>
			{#if mobileMenuOpen}
				<X size={18} />
			{:else}
				<Menu size={18} />
			{/if}
		</Button>
	</div>

	{#if mobileMenuOpen}
		<div class="border-t border-border px-4 pb-4 pt-2 md:hidden">
			<div class="flex flex-col gap-1">
				<Button variant="ghost" href="#features" class="justify-start" onclick={() => (mobileMenuOpen = false)}>
					Features
				</Button>
				<Button variant="ghost" href="#how-it-works" class="justify-start" onclick={() => (mobileMenuOpen = false)}>
					How it works
				</Button>
				<Button variant="ghost" href={repo} class="justify-start gap-2">
					<Icon icon="simple-icons:github" width="15" height="15" /> GitHub
				</Button>
				<Separator class="my-2" />
				{#if data.user}
					<Button href="/dashboard">
						<LayoutDashboard size={15} /> Dashboard
					</Button>
				{:else}
					<Button variant="outline" href="/login" class="mb-1">Sign in</Button>
					<Button href="/register">
						Get Started <ArrowRight size={15} />
					</Button>
				{/if}
			</div>
		</div>
	{/if}
</nav>

<!-- HERO -->
<section class="relative overflow-hidden">
	<div class="dot-grid dot-grid-fade pointer-events-none absolute inset-0"></div>
	<div class="brand-halo pointer-events-none absolute left-1/2 top-[-120px] size-[520px] -translate-x-1/2 rounded-full"></div>

	<div class="relative mx-auto flex max-w-3xl flex-col items-center gap-5 px-4 pb-16 pt-10 text-center md:pt-14">
		<Badge variant="outline" class="gap-2 rounded-full border-border bg-card/70 px-3 py-1 text-foreground backdrop-blur">
			<span class="relative flex size-2">
				<span class="absolute inline-flex size-full animate-ping rounded-full bg-signal opacity-70"></span>
				<span class="relative inline-flex size-2 rounded-full bg-signal"></span>
			</span>
			Open source &amp; free to use
		</Badge>

		<h1 class="text-balance text-5xl font-extrabold leading-[1.05] tracking-tight md:text-6xl">
			Your localhost,<br />
			<span class="brand-text">out in the world.</span>
		</h1>

		<p class="max-w-xl text-lg leading-relaxed text-muted-foreground">
			Relay tunnels any local port to a public URL in one command — with persistent domains,
			custom URLs, and zero configuration.
		</p>

		<div class="flex flex-wrap justify-center gap-3">
			{#if data.user}
				<Button href="/dashboard" size="lg">
					<LayoutDashboard size={18} />
					Go to Dashboard
				</Button>
			{:else}
				<Button href="/register" size="lg">
					Start tunneling free
					<ArrowRight size={18} />
				</Button>
			{/if}
			<Button href={repo} variant="outline" size="lg" class="gap-2 bg-card/70 backdrop-blur">
				<Icon icon="simple-icons:github" width="18" height="18" />
				View on GitHub
			</Button>
		</div>

		<!-- Showcase: the relay hop, then the terminal that produces it -->
		<Card.Root class="mt-3 w-full max-w-2xl overflow-hidden p-0 text-left shadow-xl shadow-brand/5">
			<div class="dot-grid px-6 py-8">
				<RelayHop remote="myapp.{relayDomain}" />
			</div>
			<div class="flex items-center gap-2 border-y border-border bg-muted/40 px-4 py-2.5">
				<span class="size-3 rounded-full bg-red-400/80"></span>
				<span class="size-3 rounded-full bg-yellow-400/80"></span>
				<span class="size-3 rounded-full bg-green-400/80"></span>
				<span class="ml-2 inline-flex items-center gap-1.5 text-xs text-muted-foreground">
					<Terminal size={12} /> relay
				</span>
			</div>
			<div class="flex flex-col gap-1.5 bg-card p-5 font-mono text-sm">
				<p><span class="text-brand">$</span> relay http 8080</p>
				<p>
					<span class="text-signal">INFO</span> <span class="text-muted-foreground">Reaching out to relay...</span>
				</p>
				<p>
					<span class="text-signal">INFO</span> <span class="text-muted-foreground">Tunnel established at</span>
					<span class="brand-text font-semibold"> myapp.{relayDomain}</span>
				</p>
			</div>
		</Card.Root>
	</div>
</section>

<!-- FEATURES -->
<section id="features" class="border-t border-border py-24">
	<div class="mx-auto max-w-6xl px-4">
		<div class="mb-14 flex flex-col items-center gap-4 text-center">
			<Badge variant="outline" class="rounded-full border-border text-muted-foreground">Features</Badge>
			<h2 class="text-4xl font-extrabold tracking-tight">Everything you need to tunnel</h2>
			<p class="max-w-md text-muted-foreground">
				Built for developers who want reliability, flexibility, and freedom.
			</p>
		</div>

		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each features as feature}
				<Card.Root class="transition-colors duration-200 hover:border-brand/40">
					<Card.Content class="flex flex-col gap-3">
						<div
							class="flex size-11 items-center justify-center rounded-xl bg-brand/10 text-brand ring-1 ring-inset ring-brand/15"
						>
							<feature.icon size={19} />
						</div>
						<h3 class="font-bold">{feature.title}</h3>
						<p class="text-sm leading-relaxed text-muted-foreground">{feature.description}</p>
					</Card.Content>
				</Card.Root>
			{/each}
		</div>
	</div>
</section>

{#snippet cmdBox(code: string)}
	<div class="flex items-start gap-2 rounded-lg border border-border bg-muted/40 px-3 py-2 font-mono text-xs">
		<Terminal size={12} class="mt-0.5 shrink-0 text-muted-foreground" />
		<span class="break-all text-foreground">{code}</span>
		<button
			type="button"
			onclick={() => copy(code)}
			aria-label="Copy command"
			class="ml-auto shrink-0 text-muted-foreground transition-colors hover:text-foreground"
		>
			{#if copied === code}
				<Check size={13} class="text-signal" />
			{:else}
				<Copy size={13} />
			{/if}
		</button>
	</div>
{/snippet}

<!-- HOW IT WORKS -->
<section id="how-it-works" class="relative overflow-hidden border-t border-border bg-muted/30 py-24">
	<div class="dot-grid dot-grid-fade pointer-events-none absolute inset-0 opacity-70"></div>
	<div class="relative mx-auto max-w-6xl px-4">
		<div class="mb-14 flex flex-col items-center gap-4 text-center">
			<Badge variant="outline" class="rounded-full border-border text-muted-foreground">How it works</Badge>
			<h2 class="text-4xl font-extrabold tracking-tight">Up and running in 60 seconds</h2>
			<p class="text-muted-foreground">Three commands. That's it.</p>
		</div>

		<div class="grid gap-4 md:grid-cols-3">
			{#each steps as step}
				<Card.Root class="flex flex-col">
					<Card.Content class="flex flex-1 flex-col gap-3">
						<div class="flex items-center gap-3">
							<span
								class="flex size-9 items-center justify-center rounded-lg bg-brand/10 font-mono text-sm font-bold text-brand ring-1 ring-inset ring-brand/15"
							>
								{step.number}
							</span>
							<h3 class="text-lg font-bold">{step.title}</h3>
						</div>
						<p class="text-sm leading-relaxed text-muted-foreground">{step.description}</p>

						<div class="mt-auto flex flex-col gap-2 pt-2">
							{#if step.install}
								<div class="flex gap-1 rounded-lg border border-border bg-muted/40 p-1">
									{#each Object.entries(installCommands) as [key, cmd]}
										<button
											type="button"
											onclick={() => (osTab = key as OS)}
											class="flex-1 rounded-md px-2 py-1 text-xs font-medium transition-colors {osTab === key
												? 'bg-card text-foreground shadow-sm'
												: 'text-muted-foreground hover:text-foreground'}"
										>
											{cmd.label}
										</button>
									{/each}
								</div>
								{@render cmdBox(installCommands[osTab].code)}
							{:else}
								{@render cmdBox(step.command)}
							{/if}
						</div>
					</Card.Content>
				</Card.Root>
			{/each}
		</div>
	</div>
</section>

<!-- OPEN SOURCE -->
<section class="border-t border-border py-16">
	<div class="mx-auto max-w-6xl px-4">
		<Card.Root class="relative overflow-hidden">
			<Card.Content class="relative flex flex-col gap-8 md:flex-row md:items-center md:gap-12">
				<div class="flex flex-col gap-4">
					<div class="flex items-center gap-4">
						<div
							class="flex size-14 items-center justify-center rounded-2xl bg-foreground text-background"
						>
							<Icon icon="simple-icons:github" width="26" height="26" />
						</div>
						<div>
							<h2 class="text-2xl font-extrabold tracking-tight">Built in the open</h2>
							<p class="text-sm text-muted-foreground">MIT licensed, no black boxes</p>
						</div>
					</div>
					<p class="leading-relaxed text-muted-foreground">
						Relay is MIT-licensed and fully open source. Star us on GitHub, file issues, contribute
						code, or fork it to build your own tunneling service.
					</p>
					<div class="flex flex-wrap gap-3">
						{#each ['MIT Licensed', 'Self-hostable', 'Community-driven', 'No telemetry'] as check}
							<Badge variant="outline" class="gap-1.5 text-muted-foreground">
								<CheckCircle size={12} class="text-signal" />
								{check}
							</Badge>
						{/each}
					</div>
				</div>
				<div class="shrink-0">
					<Button href={repo} variant="outline" size="lg" class="gap-2">
						<Icon icon="simple-icons:github" width="18" height="18" />
						View on GitHub
						<ArrowRight size={16} />
					</Button>
				</div>
			</Card.Content>
		</Card.Root>
	</div>
</section>

<!-- FOOTER -->
<footer class="border-t border-border py-8">
	<div class="mx-auto max-w-6xl px-4">
		<div class="flex flex-wrap items-center gap-6">
			<div class="mr-auto flex flex-col gap-1">
				<RelayLogo size={20} />
				<p class="text-xs text-muted-foreground">Open source TCP tunneling for developers.</p>
			</div>
			<div class="flex gap-4">
				<Button variant="link" href={repo} class="h-auto p-0 text-sm text-muted-foreground">GitHub</Button>
				{#if data.user}
					<Button variant="link" href="/dashboard" class="h-auto p-0 text-sm text-muted-foreground">
						Dashboard
					</Button>
				{:else}
					<Button variant="link" href="/login" class="h-auto p-0 text-sm text-muted-foreground">Sign in</Button>
					<Button variant="link" href="/register" class="h-auto p-0 text-sm text-muted-foreground">Register</Button>
				{/if}
			</div>
		</div>
		<Separator class="my-6" />
		<p class="text-center text-xs text-muted-foreground">© 2025 Relay. MIT License.</p>
	</div>
</footer>

