<script lang="ts">
	import { enhance } from '$app/forms';
	import { LogOut } from 'lucide-svelte';

	let {
		/** Any route whose +page.server.ts exposes a `signOut` action. */
		action = '/?/signOut',
		/** `full` stretches to the parent width — used inside the mobile menu. */
		variant = 'inline',
		labelClass = ''
	}: { action?: string; variant?: 'inline' | 'full'; labelClass?: string } = $props();
</script>

<form method="post" {action} use:enhance class={variant === 'full' ? 'w-full' : ''}>
	<button
		type="submit"
		class="inline-flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground {variant ===
		'full'
			? 'w-full justify-start'
			: ''}"
	>
		<LogOut class="size-4" />
		<span class={labelClass}>Sign out</span>
	</button>
</form>
