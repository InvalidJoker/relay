<script lang="ts">
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import GithubIcon from '$lib/components/site/github-icon.svelte';
	import SignOutButton from '$lib/components/site/sign-out-button.svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronDown, ShieldCheck } from 'lucide-svelte';
	import { REPO_URL } from '$lib/site';
	import type { LayoutServerData } from './$types';
	import { emailOTP } from 'better-auth/plugins';

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

				<div class="hidden items-center gap-2 rounded-md border border-border bg-secondary px-3 py-2 text-sm font-medium text-muted-foreground sm:flex">
                    <span>{data.user?.email}</span>
                </div>

				<SignOutButton action="/dashboard?/signOut" labelClass="hidden sm:inline" />
			</div>
		</div>
	</header>

	<main>
		{@render children()}
	</main>
</div>
