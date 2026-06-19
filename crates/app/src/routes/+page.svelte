<script lang="ts">
	import RelayLogo from '$lib/components/relay-logo.svelte';
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
		ChevronRight,
		LayoutDashboard,
		Menu,
		X,
		LogIn
	} from 'lucide-svelte';
	import type { PageServerData } from './$types';
	import { env } from '$env/dynamic/public';

	import Icon from '@iconify/svelte';

	let { data }: { data: PageServerData } = $props();

	let mobileMenuOpen = $state(false);

	const features = [
		{
			icon: Globe,
			title: 'Persistent Domains',
			description:
				'Your friends never need to update the URL. Get a permanent domain that sticks across sessions.',
			gradient: 'from-indigo-500 to-violet-500'
		},
		{
			icon: Link,
			title: 'Custom Domains',
			description:
				'Bring your own domain and map it directly to your relay tunnel. Full ownership.',
			gradient: 'from-violet-500 to-purple-500'
		},
		{
			icon: Zap,
			title: 'Subdomains',
			description:
				'Claim up to 3 subdomains on our domain. Share clean, memorable URLs instantly.',
			gradient: 'from-purple-500 to-pink-500'
		},
		{
			icon: Plug,
			title: 'Persistent Ports',
			description:
				'Reserve specific ports so your tunnels are always reachable on the same address.',
			gradient: 'from-pink-500 to-rose-500'
		},
		{
			icon: Lock,
			title: 'Open Source',
			description:
				"No black boxes. Relay's source is fully open. Audit it, contribute, or self-host.",
			gradient: 'from-emerald-500 to-teal-500'
		},
		{
			icon: Server,
			title: 'Self-Hostable',
			description:
				'Run your own Relay server. Total control over your infrastructure and data.',
			gradient: 'from-teal-500 to-cyan-500'
		}
	];

	const steps = [
		{
			number: '01',
			title: 'Install',
			description: 'Download the relay CLI with a single command.',
			code: 'cargo install relay-cli'
		},
		{
			number: '02',
			title: 'Connect',
			description: 'Forward any local port to the internet with one command.',
			code: 'relay http 8080'
		},
		{
			number: '03',
			title: 'Share',
			description: 'Instantly share the URL with anyone.',
			code: `→ myapp.${env.PUBLIC_RELAY_DOMAIN}`
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
<nav class="sticky top-0 z-50 border-b border-white/[0.06] bg-background/80 backdrop-blur-xl">
	<div class="mx-auto flex h-[58px] max-w-6xl items-center gap-6 px-4">
		<a href="/" class="shrink-0 no-underline">
			<RelayLogo size={24} />
		</a>

		<!-- Desktop nav -->
		<div class="hidden items-center gap-1 md:flex">
			<Button variant="ghost" href="#features" size="sm" class="text-muted-foreground hover:text-foreground">Features</Button>
			<Button variant="ghost" href="#how-it-works" size="sm" class="text-muted-foreground hover:text-foreground">How it works</Button>
			<Button variant="ghost" href="https://github.com/InvalidJoker/relay" size="sm" class="gap-1.5 text-muted-foreground hover:text-foreground">
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

				<!-- action="?/signOut" -->
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

		<!-- Mobile hamburger -->
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

	<!-- Mobile menu -->
	{#if mobileMenuOpen}
		<div class="border-t border-white/[0.06] px-4 pb-4 pt-2">
			<div class="flex flex-col gap-1">
				<Button variant="ghost" href="#features" class="justify-start" onclick={() => (mobileMenuOpen = false)}>Features</Button>
				<Button variant="ghost" href="#how-it-works" class="justify-start" onclick={() => (mobileMenuOpen = false)}>How it works</Button>
				<Button variant="ghost" href="https://github.com/InvalidJoker/relay" class="justify-start gap-2">
					<Icon icon="simple-icons:github" width="15" height="15" /> GitHub
				</Button>
				<Separator class="my-2 opacity-30" />
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
<section class="hero-section relative overflow-hidden">
	<div class="hero-glow-1 pointer-events-none absolute"></div>
	<div class="hero-glow-2 pointer-events-none absolute"></div>

	<div class="relative mx-auto flex max-w-3xl flex-col items-center gap-7 px-4 py-32 text-center">
		<Badge variant="outline" class="gap-2 rounded-full border-indigo-500/30 bg-indigo-500/10 px-3 py-1 text-indigo-300">
			<span class="size-1.5 animate-pulse rounded-full bg-indigo-400"></span>
			Open source & free to use
		</Badge>

		<h1 class="text-balance text-5xl font-extrabold leading-[1.1] tracking-tight text-foreground md:text-6xl">
			Share your local server<br />
			<span class="hero-gradient">with the world.</span>
		</h1>

		<p class="max-w-xl text-lg leading-relaxed text-muted-foreground">
			Relay tunnels your localhost to the internet instantly — with persistent domains, custom URLs,
			and zero configuration.
		</p>

		<div class="flex flex-wrap justify-center gap-3">
			{#if data.user}
				<Button
					href="/dashboard"
					size="lg"
				>
					<LayoutDashboard size={18} />
					Go to Dashboard
				</Button>
				<Button
					href="https://github.com/InvalidJoker/relay"
					variant="outline"
					size="lg"
					class="gap-2 border-white/10 hover:-translate-y-0.5 transition-all"
				>
					<Icon icon="simple-icons:github" width="18" height="18" />
					View on GitHub
				</Button>
			{:else}
				<Button
					href="/register"
					size="lg"
				>
					Start tunneling free
					<ArrowRight size={18} />
				</Button>
				<Button
					href="https://github.com/InvalidJoker/relay"
					variant="outline"
					size="lg"
					class="gap-2 border-white/10 hover:-translate-y-0.5 transition-all"
				>
					<Icon icon="simple-icons:github" width="18" height="18" />
					View on GitHub
				</Button>
			{/if}
		</div>
	</div>
</section>

<!-- FEATURES -->
<section id="features" class="border-t border-white/[0.06] py-24">
	<div class="mx-auto max-w-6xl px-4">
		<div class="mb-14 flex flex-col items-center gap-4 text-center">
			<Badge variant="outline" class="rounded-full border-indigo-500/30 bg-indigo-500/10 text-indigo-300">Features</Badge>
			<h2 class="text-4xl font-extrabold tracking-tight">Everything you need to tunnel</h2>
			<p class="max-w-md text-muted-foreground">Built for developers who want reliability, flexibility, and freedom.</p>
		</div>

		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each features as feature}
				<Card.Root class="group/feat transition-all duration-200 hover:-translate-y-1 hover:shadow-lg hover:shadow-indigo-500/5">
					<Card.Content class="flex flex-col gap-3 pt-6">
						<div class="flex size-10 items-center justify-center rounded-xl bg-gradient-to-br {feature.gradient}">
							<feature.icon size={18} class="text-white" />
						</div>
						<h3 class="font-bold">{feature.title}</h3>
						<p class="text-sm text-muted-foreground leading-relaxed">{feature.description}</p>
					</Card.Content>
				</Card.Root>
			{/each}
		</div>
	</div>
</section>

<!-- HOW IT WORKS -->
<section id="how-it-works" class="border-t border-white/[0.06] py-24">
	<div class="relative mx-auto max-w-6xl px-4">
		<div class="hiw-glow pointer-events-none absolute inset-0"></div>
		<div class="mb-14 flex flex-col items-center gap-4 text-center">
			<Badge variant="outline" class="rounded-full border-indigo-500/30 bg-indigo-500/10 text-indigo-300">How it works</Badge>
			<h2 class="text-4xl font-extrabold tracking-tight">Up and running in 60 seconds</h2>
			<p class="text-muted-foreground">Three commands. That's it.</p>
		</div>

		<div class="flex flex-col items-stretch gap-4 md:flex-row">
			{#each steps as step, i}
				<Card.Root class="flex-1 text-center">
					<Card.Content class="flex flex-col items-center gap-3 pt-6">
						<Badge variant="outline" class="rounded-full border-indigo-500/30 bg-indigo-500/10 text-indigo-300 font-mono">{step.number}</Badge>
						<h3 class="text-lg font-bold">{step.title}</h3>
						<p class="text-sm text-muted-foreground leading-relaxed max-w-[200px]">{step.description}</p>
						<div class="flex items-center gap-2 rounded-lg border border-white/[0.08] bg-black/30 px-3 py-2 font-mono text-xs text-indigo-300">
							<Terminal size={12} />
							{step.code}
						</div>
					</Card.Content>
				</Card.Root>
				{#if i < steps.length - 1}
					<div class="hidden items-center text-muted-foreground/30 md:flex">
						<ChevronRight size={20} />
					</div>
				{/if}
			{/each}
		</div>
	</div>
</section>

<!-- OPEN SOURCE -->
<section class="border-t border-white/[0.06] py-16">
	<div class="mx-auto max-w-6xl px-4">
		<Card.Root class="relative overflow-hidden border-indigo-500/20 bg-indigo-500/[0.04]">
			<div class="os-glow pointer-events-none absolute inset-0"></div>
			<Card.Content class="flex flex-col gap-8 pt-8 md:flex-row md:items-center md:gap-12">
				<div class="flex flex-col gap-4">
					<div class="flex items-center gap-4">
						<div class="flex size-14 items-center justify-center rounded-2xl border border-indigo-500/25 bg-indigo-500/15 text-indigo-300">
							<Icon icon="simple-icons:github" width="28" height="28" />
						</div>
						<div>
							<h2 class="text-2xl font-extrabold tracking-tight">Built in the open</h2>
							<p class="text-sm text-muted-foreground">MIT licensed, no black boxes</p>
						</div>
					</div>
					<p class="text-muted-foreground leading-relaxed">
						Relay is MIT-licensed and fully open source. Star us on GitHub, file issues, contribute
						code, or fork it to build your own tunneling service.
					</p>
					<div class="flex flex-wrap gap-3">
						{#each ['MIT Licensed', 'Self-hostable', 'Community-driven', 'No telemetry'] as check}
							<Badge variant="outline" class="gap-1.5 border-white/10 text-muted-foreground">
								<CheckCircle size={12} class="text-green-400" />
								{check}
							</Badge>
						{/each}
					</div>
				</div>
				<div class="shrink-0">
					<Button
						href="https://github.com/InvalidJoker/relay"
						variant="outline"
						size="lg"
						class="gap-2 border-white/10"
					>
						<Icon icon="simple-icons:github" width="18" height="18" />
						View on GitHub
						<ArrowRight size={16} />
					</Button>
				</div>
			</Card.Content>
		</Card.Root>
	</div>
</section>

<!-- CTA BANNER -->
<section class="cta-section border-t border-indigo-500/15 py-24">
	<div class="cta-glow pointer-events-none absolute inset-0"></div>
	<div class="relative mx-auto flex max-w-2xl flex-col items-center gap-5 px-4 text-center">
		<h2 class="text-4xl font-extrabold tracking-tight">Ready to share your localhost?</h2>
		<p class="text-muted-foreground">
			Create a free account and get persistent domains, subdomains, and more.
		</p>
		<div class="flex flex-wrap justify-center gap-3 pt-2">
			{#if data.user}
				<Button
					href="/dashboard"
					size="lg"
				>
					<LayoutDashboard size={18} />
					Go to Dashboard
				</Button>
			{:else}
				<Button
					href="/register"
					size="lg"
				>
					Get started for free
					<ArrowRight size={18} />
				</Button>
				<Button href="/login" variant="outline" size="lg" class="gap-2 border-white/10">
					<LogIn size={16} />
					Sign in
				</Button>
			{/if}
		</div>
	</div>
</section>

<!-- FOOTER -->
<footer class="border-t border-white/[0.06] py-8">
	<div class="mx-auto max-w-6xl px-4">
		<div class="flex flex-wrap items-center gap-6">
			<div class="mr-auto flex flex-col gap-1">
				<RelayLogo size={20} />
				<p class="text-xs text-muted-foreground">Open source TCP tunneling for developers.</p>
			</div>
			<div class="flex gap-4">
				<Button variant="link" href="https://github.com/InvalidJoker/relay" class="text-muted-foreground h-auto p-0 text-sm">GitHub</Button>
				{#if data.user}
					<Button variant="link" href="/dashboard" class="text-muted-foreground h-auto p-0 text-sm">Dashboard</Button>
				{:else}
					<Button variant="link" href="/login" class="text-muted-foreground h-auto p-0 text-sm">Sign in</Button>
					<Button variant="link" href="/register" class="text-muted-foreground h-auto p-0 text-sm">Register</Button>
				{/if}
			</div>
		</div>
		<Separator class="my-6 opacity-20" />
		<p class="text-center text-xs text-muted-foreground">© 2025 Relay. MIT License.</p>
	</div>
</footer>

<style>
	/* Gradient text */
	.hero-gradient {
		background: linear-gradient(135deg, #818cf8 0%, #a78bfa 50%, #c084fc 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	/* Hero background glows */
	.hero-section {
		background: hsl(var(--background));
		min-height: 90vh;
		display: flex;
		align-items: center;
	}

	.hero-glow-1 {
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, oklch(0.55 0.2 274 / 0.15) 0%, transparent 70%);
		top: -120px;
		left: -120px;
		border-radius: 50%;
		filter: blur(80px);
	}

	.hero-glow-2 {
		width: 500px;
		height: 500px;
		background: radial-gradient(circle, oklch(0.55 0.2 290 / 0.1) 0%, transparent 70%);
		bottom: -80px;
		right: -80px;
		border-radius: 50%;
		filter: blur(80px);
	}

	/* How it works glow */
	.hiw-glow {
		background: radial-gradient(ellipse at center, oklch(0.55 0.2 274 / 0.06) 0%, transparent 70%);
	}

	/* Open source glow */
	.os-glow {
		background: radial-gradient(circle at 80% 20%, oklch(0.55 0.2 274 / 0.12) 0%, transparent 60%);
	}

	/* CTA section */
	.cta-section {
		position: relative;
		overflow: hidden;
		background: linear-gradient(
			135deg,
			oklch(0.18 0.05 270) 0%,
			oklch(0.14 0.04 285) 50%,
			oklch(0.12 0.02 270) 100%
		);
	}

	.cta-glow {
		background: radial-gradient(ellipse at center, oklch(0.55 0.2 274 / 0.15) 0%, transparent 70%);
	}

	/* Terminal */
	.terminal {
		border-radius: 0.875rem;
		background: oklch(0.08 0.01 270 / 0.9);
		border: 1px solid oklch(1 0 0 / 0.08);
		overflow: hidden;
		box-shadow:
			0 25px 60px oklch(0 0 0 / 0.5),
			inset 0 1px 0 oklch(1 0 0 / 0.06);
		backdrop-filter: blur(8px);
	}

	.terminal-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: oklch(1 0 0 / 0.04);
		border-bottom: 1px solid oklch(1 0 0 / 0.06);
	}

	.terminal-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		display: block;
	}

	.terminal-body {
		padding: 1.25rem 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.terminal-line {
		line-height: 1.5;
		opacity: 0;
		animation: fadeInLine 0.3s ease forwards;
	}

	.terminal-line:first-child {
		opacity: 1;
		animation: none;
	}

	.terminal-fade {
		animation: fadeInLine 0.4s ease forwards;
	}

	@keyframes fadeInLine {
		from {
			opacity: 0;
			transform: translateY(3px);
		}
		to {
			opacity: 1;
			transform: none;
		}
	}

	@keyframes blink {
		0%, 100% { opacity: 1; }
		50% { opacity: 0; }
	}
</style>