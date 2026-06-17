<script lang="ts">
	import { enhance } from '$app/forms';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import {
		LogOut,
		ChevronDown
	} from 'lucide-svelte';
	import type { LayoutServerData } from './$types';

	let { data, children }: { data: LayoutServerData; children: import('svelte').Snippet } =
		$props();

	function getInitials(name?: string, email?: string) {
		if (name) return name.slice(0, 2).toUpperCase();
		if (email) return email.slice(0, 2).toUpperCase();
		return 'U';
	}
</script>

<div class="flex min-h-svh bg-background text-foreground">
	<!-- Main content -->
	<div class="flex min-w-0 flex-1 flex-col">
		<!-- Topbar -->
		<header class="sticky top-0 z-30 flex h-[57px] items-center gap-3 border-b border-border bg-background/95 px-4 backdrop-blur-sm">
			<!-- Breadcrumb -->
			<div class="flex flex-1 items-center gap-2">
				<a href="/" class="hidden no-underline md:block">
					<RelayLogo size={20} />
				</a>
				<span class="text-muted-foreground/40 hidden md:block">/</span>
				<span class="text-sm font-semibold">Dashboard</span>
			</div>

			<!-- User dropdown -->
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
							<span class="text-xs text-muted-foreground truncate">{data.user.email}</span>
						</div>
					</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<form method="post" action="?/signOut" use:enhance>
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

		<!-- Page slot -->
		<main class="flex-1">
			{@render children()}
		</main>
	</div>
</div>
