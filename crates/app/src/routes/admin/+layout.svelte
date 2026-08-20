<script lang="ts">
	import { page } from '$app/state';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { LayoutDashboard, Users, Globe, ChevronDown, ShieldCheck } from 'lucide-svelte';
	import SignOutButton from '$lib/components/site/sign-out-button.svelte';
	import type { LayoutData } from './$types';

	let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();

	const nav = [
		{ href: '/admin', label: 'Overview', icon: LayoutDashboard },
		{ href: '/admin/users', label: 'Users', icon: Users },
		{ href: '/admin/domains', label: 'Domains', icon: Globe }
	];

	function isActive(href: string) {
		if (href === '/admin') return page.url.pathname === '/admin';
		return page.url.pathname.startsWith(href);
	}

	function getInitials(name?: string, email?: string) {
		if (name) return name.slice(0, 2).toUpperCase();
		if (email) return email.slice(0, 2).toUpperCase();
		return 'U';
	}
</script>

<div class="flex min-h-svh flex-col bg-background text-foreground">
	<!-- Topbar -->
	<header
		class="sticky top-0 z-30 flex h-[57px] items-center gap-3 border-b border-border bg-background/95 px-4 backdrop-blur-sm"
	>
		<div class="flex flex-1 items-center gap-2">
			<a href="/" class="hidden no-underline md:block">
				<RelayLogo size={20} />
			</a>
			<span class="hidden text-muted-foreground/40 md:block">/</span>
			<span class="flex items-center gap-1.5 text-sm font-semibold">
				<ShieldCheck size={15} class="text-primary" />
				Admin
			</span>
		</div>


		<div class="hidden items-center gap-2 rounded-md border border-border bg-secondary px-3 py-2 text-sm font-medium text-muted-foreground sm:flex">
            <span>{data.user?.email}</span>
        </div>

		<SignOutButton action="/dashboard?/signOut" labelClass="hidden sm:inline" />
	</header>

	<!-- Nav -->
	<nav class="border-b border-border px-4">
		<div class="mx-auto flex max-w-5xl gap-1">
			{#each nav as item}
				<a
					href={item.href}
					class="flex items-center gap-2 border-b-2 px-3 py-3 text-sm font-medium transition-colors {isActive(
						item.href
					)
						? 'border-primary text-foreground'
						: 'border-transparent text-muted-foreground hover:text-foreground'}"
				>
					<item.icon size={15} />
					{item.label}
				</a>
			{/each}
		</div>
	</nav>

	<main class="flex-1">
		{@render children()}
	</main>
</div>
