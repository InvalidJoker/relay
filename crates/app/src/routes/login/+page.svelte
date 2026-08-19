<script lang="ts">
	import AuthShell from '$lib/components/site/auth-shell.svelte';
	import AuthForm from '$lib/components/site/auth-form.svelte';
	import type { ActionData } from './$types';

	let { form }: { form?: ActionData } = $props();
</script>

<svelte:head>
	<title>Sign in — Relay</title>
	<meta name="description" content="Sign in to your Relay account to manage tunnels and domains." />
</svelte:head>

<AuthShell>
	<AuthForm mode="login" action="?/signInEmail" message={form?.message} />
</AuthShell>
