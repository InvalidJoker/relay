<script lang="ts">
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import GithubIcon from './github-icon.svelte';
	import SignOutButton from './sign-out-button.svelte';
	import { LayoutDashboard, Menu, X } from 'lucide-svelte';
	import { REPO_URL } from '$lib/site';

	let { user = null }: { user?: { name?: string | null; email?: string } | null } = $props();

	const navLinks = [
		{ href: '/#features', label: 'Features' },
		{ href: '/#quickstart', label: 'Quickstart' },
		{ href: '/docs', label: 'Docs' }
	];

	let open = $state(false);
</script>

<header class="sticky top-0 z-50 border-b border-border/60 bg-background/70 backdrop-blur-xl">
	<div class="mx-auto flex h-16 max-w-6xl items-center justify-between gap-3 px-5">
		<a href="/" aria-label="Relay home" class="shrink-0">
			<RelayLogo size={28} />
		</a>

		<nav
			class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-1 rounded-full border border-border/70 bg-secondary/40 px-1.5 py-1 md:flex"
		>
			{#each navLinks as link (link.href)}
				<a
					href={link.href}
					class="rounded-full px-3.5 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-card hover:text-foreground"
				>
					{link.label}
				</a>
			{/each}
		</nav>

		<div class="flex items-center gap-2">
			<a
				href={REPO_URL}
				target="_blank"
				rel="noopener noreferrer"
				aria-label="Relay on GitHub"
				class="inline-flex size-9 items-center justify-center rounded-md border border-border bg-secondary text-muted-foreground transition-colors hover:text-foreground"
			>
				<GithubIcon class="size-4" />
			</a>

			{#if user}
				<a
					href="/dashboard"
					class="inline-flex items-center gap-1.5 rounded-md bg-primary px-3.5 py-2 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
				>
					<LayoutDashboard class="size-4" />
					Dashboard
				</a>
				<div class="hidden sm:block">
					<SignOutButton />
				</div>
			{:else}
				<a
					href="/login"
					class="hidden rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground sm:inline-flex"
				>
					Sign in
				</a>
				<a
					href="/register"
					class="inline-flex items-center rounded-md bg-primary px-3.5 py-2 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
				>
					Sign up
				</a>
			{/if}

			<button
				type="button"
				aria-label={open ? 'Close menu' : 'Open menu'}
				onclick={() => (open = !open)}
				class="inline-flex size-9 items-center justify-center rounded-md border border-border bg-secondary text-muted-foreground transition-colors hover:text-foreground md:hidden"
			>
				{#if open}
					<X class="size-4" />
				{:else}
					<Menu class="size-4" />
				{/if}
			</button>
		</div>
	</div>

	{#if open}
		<nav class="border-t border-border bg-background px-5 py-3 md:hidden">
			<ul class="flex flex-col">
				{#each navLinks as link (link.href)}
					<li>
						<a
							href={link.href}
							onclick={() => (open = false)}
							class="block rounded-md px-2 py-2.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
						>
							{link.label}
						</a>
					</li>
				{/each}
				{#if user}
					<li>
						<SignOutButton variant="full" />
					</li>
				{:else}
					<li>
						<a
							href="/login"
							class="block rounded-md px-2 py-2.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
						>
							Sign in
						</a>
					</li>
				{/if}
			</ul>
		</nav>
	{/if}
</header>
