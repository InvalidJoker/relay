<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData, ActionData } from './$types';
	import * as Card from '$lib/components/ui/card';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Table from '$lib/components/ui/table';

	let { data, form }: { data: PageServerData; form: ActionData } = $props();
</script>

<div class="container mx-auto max-w-5xl py-10">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Dashboard</h1>
			<p class="text-muted-foreground">Manage your relay resources</p>
		</div>
		<form method="post" action="?/signOut" use:enhance>
			<Button variant="outline" type="submit">Sign out</Button>
		</form>
	</div>

	{#if form?.message}
		<div class="mb-6 rounded-md bg-destructive/15 p-4 text-sm text-destructive">
			{form.message}
		</div>
	{/if}

	<Tabs.Root value="ports" class="w-full">
		<Tabs.List class="grid w-full grid-cols-3">
			<Tabs.Trigger value="ports">Ports ({data.ports.length}/2)</Tabs.Trigger>
			<Tabs.Trigger value="domains">Domains ({data.domains.length}/1)</Tabs.Trigger>
			<Tabs.Trigger value="subdomains">Subdomains ({data.subdomains.length}/3)</Tabs.Trigger>
		</Tabs.List>

		<!-- PORTS TAB -->
		<Tabs.Content value="ports">
			<Card.Root>
				<Card.Header>
					<Card.Title>Persistent Ports</Card.Title>
					<Card.Description>Request a permanent port for your relay tunnels. Limit: 2</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.ports.length > 0}
						<Table.Root class="mb-6">
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
										<Table.Cell class="font-medium">{port.port}</Table.Cell>
										<Table.Cell>{port.description || '-'}</Table.Cell>
										<Table.Cell>{new Date(port.createdAt).toLocaleDateString()}</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removePort" use:enhance>
												<input type="hidden" name="id" value={port.id} />
												<Button variant="destructive" size="sm" type="submit">Remove</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="mb-6 text-sm text-muted-foreground text-center py-6 border rounded-lg bg-muted/20">
							No persistent ports requested yet.
						</div>
					{/if}

					<form method="post" action="?/addPort" use:enhance class="space-y-4 rounded-lg border p-4 bg-muted/10">
						<h3 class="font-medium">Request New Port</h3>
						<div class="grid gap-4 md:grid-cols-2">
							<div class="space-y-2">
								<Label for="port">Port Number</Label>
								<Input id="port" name="port" type="number" placeholder="e.g. 8080" required min="1" max="65535" disabled={data.ports.length >= 2} />
							</div>
							<div class="space-y-2">
								<Label for="description">Description (Optional)</Label>
								<Input id="description" name="description" placeholder="My web app" disabled={data.ports.length >= 2} />
							</div>
						</div>
						<Button type="submit" disabled={data.ports.length >= 2}>Add Port</Button>
					</form>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>

		<!-- DOMAINS TAB -->
		<Tabs.Content value="domains">
			<Card.Root>
				<Card.Header>
					<Card.Title>Custom Domains</Card.Title>
					<Card.Description>Map a custom domain to a specific port. Limit: 1</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.domains.length > 0}
						<Table.Root class="mb-6">
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
										<Table.Cell class="font-medium">{domain.domain}</Table.Cell>
										<Table.Cell>{new Date(domain.createdAt).toLocaleDateString()}</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removeDomain" use:enhance>
												<input type="hidden" name="id" value={domain.id} />
												<Button variant="destructive" size="sm" type="submit">Remove</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="mb-6 text-sm text-muted-foreground text-center py-6 border rounded-lg bg-muted/20">
							No custom domains added yet.
						</div>
					{/if}

					<form method="post" action="?/addDomain" use:enhance class="space-y-4 rounded-lg border p-4 bg-muted/10">
						<h3 class="font-medium">Add Custom Domain</h3>
						<div class="grid gap-4 md:grid-cols-2">
							<div class="space-y-2">
								<Label for="domain">Domain</Label>
								<Input id="domain" name="domain" placeholder="example.com" required disabled={data.domains.length >= 1} />
							</div>
						</div>
						<Button type="submit" disabled={data.domains.length >= 1}>Add Domain</Button>
					</form>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>

		<!-- SUBDOMAINS TAB -->
		<Tabs.Content value="subdomains">
			<Card.Root>
				<Card.Header>
					<Card.Title>Subdomains</Card.Title>
					<Card.Description>Request a subdomain on our domain. Limit: 3</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if data.subdomains.length > 0}
						<Table.Root class="mb-6">
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
										<Table.Cell class="font-medium">{sub.subdomain}.relay.dev</Table.Cell>
										<Table.Cell>{new Date(sub.createdAt).toLocaleDateString()}</Table.Cell>
										<Table.Cell class="text-right">
											<form method="post" action="?/removeSubdomain" use:enhance>
												<input type="hidden" name="id" value={sub.id} />
												<Button variant="destructive" size="sm" type="submit">Remove</Button>
											</form>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{:else}
						<div class="mb-6 text-sm text-muted-foreground text-center py-6 border rounded-lg bg-muted/20">
							No subdomains added yet.
						</div>
					{/if}

					<form method="post" action="?/addSubdomain" use:enhance class="space-y-4 rounded-lg border p-4 bg-muted/10">
						<h3 class="font-medium">Add Subdomain</h3>
						<div class="grid gap-4 md:grid-cols-2">
							<div class="space-y-2">
								<Label for="subdomain">Subdomain Prefix</Label>
								<div class="flex items-center space-x-2">
									<Input id="subdomain" name="subdomain" placeholder="my-app" required disabled={data.subdomains.length >= 3} />
									<span class="text-sm text-muted-foreground">.relay.dev</span>
								</div>
							</div>
						</div>
						<Button type="submit" disabled={data.subdomains.length >= 3}>Add Subdomain</Button>
					</form>
				</Card.Content>
			</Card.Root>
		</Tabs.Content>
	</Tabs.Root>
</div>
