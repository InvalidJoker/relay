<script lang="ts">
	import { Globe, Link2, Network, TriangleAlert } from 'lucide-svelte';
	import ResourceSection from '$lib/components/dashboard/resource-section.svelte';
	import CodeBlock from '$lib/components/site/code-block.svelte';
	import { RELAY_DOMAIN, TCP_DOMAIN } from '$lib/site';
	import type { ActionData, PageServerData } from './$types';

	let { data, form }: { data: PageServerData; form: ActionData } = $props();

	const PORT_LIMIT = 2;
	const DOMAIN_LIMIT = 1;
	const SUBDOMAIN_LIMIT = 3;

	const stats = $derived([
		{ label: 'Subdomains', value: `${data.subdomains.length}/${SUBDOMAIN_LIMIT}` },
		{ label: 'Reserved ports', value: `${data.ports.length}/${PORT_LIMIT}` },
		{ label: 'Custom domains', value: `${data.domains.length}/${DOMAIN_LIMIT}` }
	]);

	const firstName = $derived(data.user?.name?.split(' ')[0] ?? null);
</script>

<svelte:head>
	<title>Dashboard — Relay</title>
</svelte:head>

<div class="mx-auto max-w-5xl px-5 py-10">
	<div class="mb-8">
		<h1 class="text-2xl font-semibold tracking-tight text-foreground">
			{firstName ? `Welcome back, ${firstName}` : 'Welcome back'}
		</h1>
		<p class="mt-1 text-muted-foreground">
			Manage the domains and ports reserved to your account.
		</p>
	</div>

	{#if form?.message}
		<div
			class="mb-6 flex items-start gap-3 rounded-lg border border-destructive/40 bg-destructive/10 px-4 py-3 text-sm text-destructive"
		>
			<TriangleAlert class="mt-0.5 size-4 shrink-0" />
			{form.message}
		</div>
	{/if}

	<div class="mb-8 grid gap-4 sm:grid-cols-3">
		{#each stats as stat (stat.label)}
			<div class="rounded-xl border border-border bg-card p-5">
				<p class="text-xs uppercase tracking-widest text-muted-foreground">{stat.label}</p>
				<p class="mt-2 text-2xl font-semibold text-foreground">{stat.value}</p>
			</div>
		{/each}
	</div>

	<div class="space-y-6">
		<!-- Connect the CLI -->
		<section class="overflow-hidden rounded-xl border border-border bg-card">
			<div class="border-b border-border p-5">
				<h3 class="font-medium text-foreground">Connect the CLI</h3>
				<p class="mt-0.5 text-sm text-muted-foreground">
					Sign in from your terminal with the device-code flow, then start a tunnel. Credentials are
					stored in ~/.config/relay.toml.
				</p>
			</div>
			<div class="space-y-3 p-5">
				<CodeBlock code="relay login" />
				<CodeBlock code="relay http 3000 --subdomain myapp" />
				<a
					href="/docs"
					class="inline-block text-sm font-medium text-primary underline-offset-4 hover:underline"
				>
					Read the documentation
				</a>
			</div>
		</section>

		<ResourceSection
			icon={Link2}
			title="Persistent subdomains"
			description="Reserve up to {SUBDOMAIN_LIMIT} subdomains that always route to your tunnels."
			limit={SUBDOMAIN_LIMIT}
			items={data.subdomains.map((s) => ({
				id: s.id,
				value: s.subdomain,
				suffix: `.${RELAY_DOMAIN}`,
				createdAt: s.createdAt
			}))}
			addAction="?/addSubdomain"
			removeAction="?/removeSubdomain"
			inputName="subdomain"
			placeholder="myapp"
			inputSuffix=".{RELAY_DOMAIN}"
			addLabel="Reserve"
			emptyText="No subdomains claimed yet."
		/>

		<ResourceSection
			icon={Network}
			title="Reserved TCP ports"
			description="Lock in up to {PORT_LIMIT} fixed remote ports in the range {data.portRange
				.start}–{data.portRange.end}."
			limit={PORT_LIMIT}
			items={data.ports.map((p) => ({
				id: p.id,
				value: String(p.port),
				prefix: `${TCP_DOMAIN}:`,
				createdAt: p.createdAt
			}))}
			addAction="?/addPort"
			removeAction="?/removePort"
			inputName="port"
			inputType="number"
			min={data.portRange.start}
			max={data.portRange.end}
			placeholder={String(data.portRange.start)}
			inputPrefix="{TCP_DOMAIN}:"
			addLabel="Reserve"
			emptyText="No ports reserved yet."
		>
			{#snippet extraFields(disabled: boolean)}
				<input
					name="description"
					{disabled}
					placeholder="Description (optional)"
					class="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground/60 focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring/40 disabled:cursor-not-allowed disabled:opacity-60"
				/>
			{/snippet}
		</ResourceSection>

		<ResourceSection
			icon={Globe}
			title="Custom domains"
			description="Point a domain you own at Relay and serve tunnels from it."
			limit={DOMAIN_LIMIT}
			items={data.domains.map((d) => ({
				id: d.id,
				value: d.domain,
				createdAt: d.createdAt
			}))}
			addAction="?/addDomain"
			removeAction="?/removeDomain"
			inputName="domain"
			placeholder="tunnel.example.com"
			addLabel="Add domain"
			emptyText="No custom domains yet."
		/>
	</div>
</div>
