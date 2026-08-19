<script lang="ts">
	import AuthShell from '$lib/components/site/auth-shell.svelte';
	import AuthForm from '$lib/components/site/auth-form.svelte';
	import type { ActionData } from './$types';

	let { form }: { form?: ActionData } = $props();
</script>

<svelte:head>
	<title>Create account — Relay</title>
	<meta
		name="description"
		content="Create a free Relay account to reserve subdomains, ports, and custom domains."
	/>
</svelte:head>

<AuthShell>
	<AuthForm mode="register" action="?/signUpEmail" message={form?.message} />
</AuthShell>
