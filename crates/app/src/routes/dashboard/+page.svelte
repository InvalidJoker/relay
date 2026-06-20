<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData, ActionData } from './$types';
	import * as Card from '$lib/components/ui/card';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Table from '$lib/components/ui/table';
	import { Plug, Globe, Link2, Plus, Trash2, AlertCircle } from 'lucide-svelte';
	import { env } from '$env/dynamic/public';
	import {Alert} from "$lib/components/ui/alert";

	let { data, form }: { data: PageServerData; form: ActionData } = $props();

	const stats = $derived([
		{
			label: 'Persistent Ports',
			used: data.ports.length,
			max: 2,
			icon: Plug,
			color: '#818cf8'
		},
		{
			label: 'Custom Domains',
			used: data.domains.length,
			max: 1,
			icon: Globe,
			color: '#6366f1'
		},
		{
			label: 'Subdomains',
			used: data.subdomains.length,
			max: 3,
			icon: Link2,
			color: '#4f46e5'
		}
	]);
</script>

<div class="page">
	<Alert variant="destructive">
		<AlertCircle size={16} />
		This dashboard is temporary, and will be replaced soon
	</Alert>
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1 class="page-title">Dashboard</h1>
			<p class="page-subtitle">Manage your relay resources and tunnels</p>
		</div>
	</div>

	{#if form?.message}
		<div class="error-banner">
			<AlertCircle size={16} />
			{form.message}
		</div>
	{/if}

	<!-- Stats row -->
	<div class="stats-grid">
		{#each stats as stat}
			<div class="stat-card">
				<div class="stat-header">
					<span class="stat-label">{stat.label}</span>
					<div class="stat-icon" style="--icon-color: {stat.color}">
						<stat.icon size={16} />
					</div>
				</div>
				<div class="stat-value">
					<span class="stat-used" style="color: {stat.color}">{stat.used}</span>
					<span class="stat-sep">/</span>
					<span class="stat-max">{stat.max}</span>
				</div>
				<div class="stat-bar">
					<div
						class="stat-bar-fill"
						style="width: {(stat.used / stat.max) * 100}%; background: {stat.color}"
					></div>
				</div>
			</div>
		{/each}
	</div>

	<!-- Tabs -->
	<Tabs.Root value="ports" class="tabs-root">
		<Tabs.List class="tabs-list">
			<Tabs.Trigger value="ports" class="tab-trigger">
				<Plug size={15} />
				Ports <span class="tab-count">{data.ports.length}/2</span>
			</Tabs.Trigger>
			<Tabs.Trigger value="domains" class="tab-trigger">
				<Globe size={15} />
				Domains <span class="tab-count">{data.domains.length}/1</span>
			</Tabs.Trigger>
			<Tabs.Trigger value="subdomains" class="tab-trigger">
				<Link2 size={15} />
				Subdomains <span class="tab-count">{data.subdomains.length}/3</span>
			</Tabs.Trigger>
		</Tabs.List>

		<!-- PORTS TAB -->
		<Tabs.Content value="ports">
			<Card.Root class="tab-card">
				<Card.Header>
					<Card.Title>Persistent Ports</Card.Title>
					<Card.Description>
						Reserve a fixed <strong>remote port</strong> on the relay server so your tunnel is always
						reachable at the same address. Allowed range: {data.portRange.start}–{data.portRange.end}.
						Limit: 2
					</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.ports.length > 0}
						<Table.Root class="resource-table">
							<Table.Header>
								<Table.Row>
									<Table.Head>Port</Table.Head>
									<Table.Head>Description</Table.Head>
									<Table.Head>Created</Table.Head>
									<Table.Head class="text-right">Actions</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each data.ports as port}
									<Table.Row>
										<Table.Cell>
											<span class="resource-code">{port.port}</span>
										</Table.Cell>
										<Table.Cell class="text-muted-foreground">
											{port.description || '—'}
										</Table.Cell>
										<Table.Cell class="text-muted-foreground">
											{new Date(port.createdAt).toLocaleDateString()}
										</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removePort" use:enhance>
												<input type="hidden" name="id" value={port.id} />
												<Button variant="ghost" size="sm" type="submit" class="remove-btn">
													<Trash2 size={14} />
												</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="empty-state">
							<Plug size={28} class="empty-icon" />
							<p class="empty-title">No ports reserved yet</p>
							<p class="empty-desc">Reserve a port below to get a permanent tunnel address.</p>
						</div>
					{/if}

					<div class="add-form">
						<h3 class="form-title">
							<Plus size={16} />
							Request New Port
						</h3>
						<form method="post" action="?/addPort" use:enhance class="form-grid">
							<div class="form-field">
								<Label for="port">Remote Port</Label>
								<Input
									id="port"
									name="port"
									type="number"
									placeholder="e.g. {data.portRange.start}"
									required
									min={data.portRange.start}
									max={data.portRange.end}
									disabled={data.ports.length >= 2}
								/>
							</div>
							<div class="form-field">
								<Label for="description">Description (Optional)</Label>
								<Input
									id="description"
									name="description"
									placeholder="My web app"
									disabled={data.ports.length >= 2}
								/>
							</div>
							<div class="form-submit">
								<Button type="submit" disabled={data.ports.length >= 2}>Add Port</Button>
							</div>
						</form>
					</div>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>

		<!-- DOMAINS TAB -->
		<Tabs.Content value="domains">
			<Card.Root class="tab-card">
				<Card.Header>
					<Card.Title>Custom Domains</Card.Title>
					<Card.Description>Map your own domain to a relay tunnel. Limit: 1</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.domains.length > 0}
						<Table.Root class="resource-table">
							<Table.Header>
								<Table.Row>
									<Table.Head>Domain</Table.Head>
									<Table.Head>Created</Table.Head>
									<Table.Head class="text-right">Actions</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each data.domains as domain}
									<Table.Row>
										<Table.Cell>
											<span class="resource-code">{domain.domain}</span>
										</Table.Cell>
										<Table.Cell class="text-muted-foreground">
											{new Date(domain.createdAt).toLocaleDateString()}
										</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removeDomain" use:enhance>
												<input type="hidden" name="id" value={domain.id} />
												<Button variant="ghost" size="sm" type="submit" class="remove-btn">
													<Trash2 size={14} />
												</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="empty-state">
							<Globe size={28} class="empty-icon" />
							<p class="empty-title">No custom domains yet</p>
							<p class="empty-desc">Add your own domain to get a branded tunnel URL.</p>
						</div>
					{/if}

					<div class="add-form">
						<h3 class="form-title">
							<Plus size={16} />
							Add Custom Domain
						</h3>
						<form method="post" action="?/addDomain" use:enhance class="form-grid">
							<div class="form-field">
								<Label for="domain">Domain</Label>
								<Input
									id="domain"
									name="domain"
									placeholder="example.com"
									required
									disabled={data.domains.length >= 1}
								/>
							</div>
							<div class="form-submit">
								<Button type="submit" disabled={data.domains.length >= 1}>Add Domain</Button>
							</div>
						</form>
					</div>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>

		<!-- SUBDOMAINS TAB -->
		<Tabs.Content value="subdomains">
			<Card.Root class="tab-card">
				<Card.Header>
					<Card.Title>Subdomains</Card.Title>
					<Card.Description>
						Claim subdomains on {env.PUBLIC_RELAY_DOMAIN} for your tunnels. Limit: 3
					</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.subdomains.length > 0}
						<Table.Root class="resource-table">
							<Table.Header>
								<Table.Row>
									<Table.Head>Subdomain</Table.Head>
									<Table.Head>Created</Table.Head>
									<Table.Head class="text-right">Actions</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each data.subdomains as sub}
									<Table.Row>
										<Table.Cell>
											<span class="resource-code">{sub.subdomain}.{env.PUBLIC_RELAY_DOMAIN}</span>
										</Table.Cell>
										<Table.Cell class="text-muted-foreground">
											{new Date(sub.createdAt).toLocaleDateString()}
										</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removeSubdomain" use:enhance>
												<input type="hidden" name="id" value={sub.id} />
												<Button variant="ghost" size="sm" type="submit" class="remove-btn">
													<Trash2 size={14} />
												</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="empty-state">
							<Link2 size={28} class="empty-icon" />
							<p class="empty-title">No subdomains claimed yet</p>
							<p class="empty-desc">Claim a subdomain below and share a permanent URL.</p>
						</div>
					{/if}

					<div class="add-form">
						<h3 class="form-title">
							<Plus size={16} />
							Add Subdomain
						</h3>
						<form method="post" action="?/addSubdomain" use:enhance class="form-grid">
							<div class="form-field">
								<Label for="subdomain">Subdomain Prefix</Label>
								<div class="subdomain-input-wrap">
									<Input
										id="subdomain"
										name="subdomain"
										placeholder="my-app"
										required
										disabled={data.subdomains.length >= 3}
									/>
									<span class="subdomain-suffix">.{env.PUBLIC_RELAY_DOMAIN}</span>
								</div>
							</div>
							<div class="form-submit">
								<Button type="submit" disabled={data.subdomains.length >= 3}>Add Subdomain</Button>
							</div>
						</form>
					</div>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>
	</Tabs.Root>
</div>

<style>
	/* ── PAGE ── */
	.page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2.5rem 1.75rem;
		display: flex;
		flex-direction: column;
		gap: 1.75rem;
	}

	.page-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
	}

	.page-title {
		font-size: 1.625rem;
		font-weight: 800;
		letter-spacing: -0.025em;
		color: rgba(255, 255, 255, 0.95);
		margin: 0;
	}

	.page-subtitle {
		font-size: 0.9rem;
		color: rgba(255, 255, 255, 0.4);
		margin: 0.2rem 0 0;
	}

	/* ── ERROR BANNER ── */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.125rem;
		border-radius: 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.25);
		color: #fca5a5;
		font-size: 0.875rem;
	}

	/* ── STATS ── */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	.stat-card {
		padding: 1.25rem;
		border-radius: 0.875rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.07);
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		transition: background 0.2s, border-color 0.2s;
	}

	.stat-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.1);
	}

	.stat-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.stat-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.45);
	}

	.stat-icon {
		width: 28px;
		height: 28px;
		border-radius: 0.5rem;
		background: color-mix(in oklch, var(--icon-color) 15%, transparent);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--icon-color);
	}

	.stat-value {
		display: flex;
		align-items: baseline;
		gap: 0.2rem;
		font-size: 1.5rem;
		font-weight: 800;
		letter-spacing: -0.02em;
	}

	.stat-used {
		font-size: 1.75rem;
	}

	.stat-sep,
	.stat-max {
		font-size: 1.25rem;
		color: rgba(255, 255, 255, 0.25);
		font-weight: 500;
	}

	.stat-bar {
		height: 3px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.07);
		overflow: hidden;
	}

	.stat-bar-fill {
		height: 100%;
		border-radius: 999px;
		transition: width 0.4s ease;
	}

	/* ── TABS ── */
	:global(.tabs-root) {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	:global(.tabs-list) {
		display: flex;
		gap: 0.25rem;
		background: rgba(255, 255, 255, 0.04) !important;
		border: 1px solid rgba(255, 255, 255, 0.07) !important;
		border-radius: 0.75rem !important;
		padding: 0.3rem !important;
	}

	:global(.tab-trigger) {
		display: flex !important;
		align-items: center !important;
		gap: 0.375rem !important;
		font-size: 0.875rem !important;
	}

	.tab-count {
		font-size: 0.75rem;
		opacity: 0.6;
		font-weight: 500;
	}

	:global(.tab-card) {
		background: rgba(255, 255, 255, 0.02) !important;
		border: 1px solid rgba(255, 255, 255, 0.07) !important;
		border-radius: 0.875rem !important;
	}

	/* ── TABLE ── */
	:global(.resource-table) {
		margin-bottom: 1.5rem;
	}

	.resource-code {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 0.875rem;
		color: #a5b4fc;
		background: rgba(99, 102, 241, 0.1);
		padding: 0.2rem 0.5rem;
		border-radius: 0.375rem;
	}

	:global(.remove-btn) {
		color: rgba(255, 100, 100, 0.6) !important;
		transition: color 0.15s, background 0.15s !important;
	}

	:global(.remove-btn:hover) {
		color: rgb(248, 113, 113) !important;
		background: rgba(239, 68, 68, 0.1) !important;
	}

	/* ── EMPTY STATE ── */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2.5rem;
		border-radius: 0.75rem;
		border: 1px dashed rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.015);
		margin-bottom: 1.5rem;
		gap: 0.5rem;
	}

	:global(.empty-icon) {
		color: rgba(255, 255, 255, 0.2) !important;
		margin-bottom: 0.25rem;
	}

	.empty-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.5);
		margin: 0;
	}

	.empty-desc {
		font-size: 0.8125rem;
		color: rgba(255, 255, 255, 0.3);
		margin: 0;
		text-align: center;
	}

	/* ── ADD FORM ── */
	.add-form {
		border-radius: 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.07);
		background: rgba(255, 255, 255, 0.02);
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-title {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.9rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.7);
		margin: 0;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1rem;
		align-items: end;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.form-submit {
		display: flex;
		align-items: flex-end;
	}

	.subdomain-input-wrap {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.subdomain-suffix {
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.35);
		white-space: nowrap;
	}

	/* ── RESPONSIVE ── */
	@media (max-width: 640px) {
		.page {
			padding: 1.5rem 1rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
