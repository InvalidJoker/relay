<script lang="ts">
	import { enhance } from '$app/forms';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import GithubIcon from '$lib/components/site/github-icon.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronDown, LogOut, ShieldCheck } from 'lucide-svelte';
	import { REPO_URL } from '$lib/site';
	import type { LayoutServerData } from './$types';

	let { data, children }: { data: LayoutServerData; children: import('svelte').Snippet } = $props();

	function getInitials(name?: string | null, email?: string) {
		if (name) return name.slice(0, 2).toUpperCase();
		if (email) return email.slice(0, 2).toUpperCase();
		return 'U';
	}
</script>

<div class="min-h-svh bg-background text-foreground">
	<header class="sticky top-0 z-50 border-b border-border bg-background/70 backdrop-blur-xl">
		<div class="mx-auto flex h-16 max-w-5xl items-center justify-between gap-3 px-5">
			<div class="flex items-center gap-3">
				<a href="/" aria-label="Relay home"><RelayLogo size={28} /></a>
				<span
					class="hidden rounded-full border border-border bg-secondary px-2.5 py-0.5 text-[11px] text-muted-foreground sm:inline"
				>
					dashboard
				</span>
			</div>

			<div class="flex items-center gap-2">
				<a
					href="/docs"
					class="hidden rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground sm:inline-flex"
				>
					Docs
				</a>
				<a
					href={REPO_URL}
					target="_blank"
					rel="noopener noreferrer"
					aria-label="Relay on GitHub"
					class="inline-flex size-9 items-center justify-center rounded-md border border-border bg-secondary text-muted-foreground transition-colors hover:text-foreground"
				>
					<GithubIcon class="size-4" />
				</a>

				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<button
								{...props}
								class="inline-flex items-center gap-2 rounded-md border border-border bg-secondary px-2.5 py-1.5 text-sm text-foreground transition-colors hover:bg-muted"
							>
								<span
									class="flex size-6 items-center justify-center rounded-full bg-primary/15 text-xs font-semibold text-primary"
								>
									{getInitials(data.user.name, data.user.email)}
								</span>
								<span class="hidden sm:inline">{data.user.name || data.user.email}</span>
								<ChevronDown class="size-3.5 text-muted-foreground" />
							</button>
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
						{#if data.user.role === 'admin'}
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a href="/admin" class="flex w-full items-center gap-2" {...props}>
										<ShieldCheck class="size-3.5" />
										Admin panel
									</a>
								{/snippet}
							</DropdownMenu.Item>
							<DropdownMenu.Separator />
						{/if}
						<form method="post" action="/dashboard?/signOut" use:enhance>
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<button type="submit" class="flex w-full items-center gap-2" {...props}>
										<LogOut class="size-3.5" />
										Sign out
									</button>
								{/snippet}
							</DropdownMenu.Item>
						</form>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</header>

	<main>
		{@render children()}
	</main>
</div>
