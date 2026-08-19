<script lang="ts">
	import SiteHeader from '$lib/components/site/site-header.svelte';
	import Hero from '$lib/components/site/hero.svelte';
	import Features from '$lib/components/site/features.svelte';
	import Install from '$lib/components/site/install.svelte';
	import Quickstart from '$lib/components/site/quickstart.svelte';
	import Usage from '$lib/components/site/usage.svelte';
	import CtaFooter from '$lib/components/site/cta-footer.svelte';
	import type { PageServerData } from './$types';

	let { data }: { data: PageServerData } = $props();
</script>

<svelte:head>
	<title>Relay — Open-source TCP tunneling</title>
	<meta
		name="description"
		content="Relay is a free, open-source TCP tunneling service. Expose local HTTP and TCP servers to the public internet in seconds — with persistent subdomains, reserved ports, and custom domains."
	/>
</svelte:head>

<main class="min-h-screen bg-background">
	<SiteHeader user={data.user} />
	<Hero user={data.user} />
	<Features />
	<Install />
	<Quickstart />
	<Usage />
	<CtaFooter user={data.user} />
</main>
