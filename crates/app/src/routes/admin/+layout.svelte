<script lang="ts">
	import { page } from '$app/state';
	import { enhance } from '$app/forms';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { LayoutDashboard, Users, Globe, LogOut, ChevronDown, ShieldCheck } from 'lucide-svelte';
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
				<ShieldCheck size={15} class="text-indigo-400" />
				Admin
			</span>
		</div>

		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Button variant="ghost" size="sm" class="gap-2" {...props}>
						<div
							class="flex size-7 items-center justify-center rounded-full bg-gradient-to-br from-indigo-500 to-violet-600 text-[11px] font-bold text-white"
						>
							{getInitials(data.user.name, data.user.email)}
						</div>
						<span class="hidden text-sm font-medium sm:block">
							{data.user.name || data.user.email}
						</span>
						<ChevronDown size={14} class="text-muted-foreground" />
					</Button>
				{/snippet}
			</DropdownMenu.Trigger>

			<DropdownMenu.Content align="end" class="w-56">
				<DropdownMenu.Label class="font-normal">
					<div class="flex flex-col gap-0.5">
						<span class="font-semibold">{data.user.name || 'User'}</span>
						<span class="truncate text-xs text-muted-foreground">{data.user.email}</span>
					</div>
				</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a href="/dashboard" class="flex w-full items-center gap-2 text-muted-foreground" {...props}>
							<LayoutDashboard size={14} />
							Dashboard
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<form method="post" action="/dashboard?/signOut" use:enhance>
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<button
								type="submit"
								class="flex w-full items-center gap-2 text-muted-foreground"
								{...props}
							>
								<LogOut size={14} />
								Sign out
							</button>
						{/snippet}
					</DropdownMenu.Item>
				</form>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
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
						? 'border-indigo-400 text-foreground'
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
