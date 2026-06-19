<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData, ActionData } from './$types';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { env } from '$env/dynamic/public';
	import { Globe, Link2, Plug, Trash2, AlertCircle } from 'lucide-svelte';

	let { data, form }: { data: PageServerData; form: ActionData } = $props();

	const relayDomain = env.PUBLIC_RELAY_DOMAIN || 'relay.invalidjoker.dev';
</script>

{#snippet owner(o: { name: string; email: string })}
	<div class="flex flex-col">
		<span class="text-sm">{o.name || '—'}</span>
		<span class="text-xs text-muted-foreground">{o.email}</span>
	</div>
{/snippet}

<div class="mx-auto flex max-w-5xl flex-col gap-6 px-4 py-8">
	<div>
		<h1 class="text-2xl font-extrabold tracking-tight">Domains &amp; Ports</h1>
		<p class="text-sm text-muted-foreground">All reserved resources across every user</p>
	</div>

	{#if form?.message}
		<div
			class="flex items-center gap-2 rounded-lg border border-destructive/25 bg-destructive/10 px-4 py-3 text-sm text-destructive"
		>
			<AlertCircle size={16} />
			{form.message}
		</div>
	{/if}

	<Tabs.Root value="domains" class="flex flex-col gap-4">
		<Tabs.List>
			<Tabs.Trigger value="domains" class="gap-1.5">
				<Globe size={15} /> Domains <span class="text-xs opacity-60">{data.domains.length}</span>
			</Tabs.Trigger>
			<Tabs.Trigger value="subdomains" class="gap-1.5">
				<Link2 size={15} /> Subdomains <span class="text-xs opacity-60">{data.subdomains.length}</span>
			</Tabs.Trigger>
			<Tabs.Trigger value="ports" class="gap-1.5">
				<Plug size={15} /> Ports <span class="text-xs opacity-60">{data.ports.length}</span>
			</Tabs.Trigger>
		</Tabs.List>

		<!-- Domains -->
		<Tabs.Content value="domains">
			<div class="rounded-xl border border-border bg-card/30">
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Domain</Table.Head>
							<Table.Head>Owner</Table.Head>
							<Table.Head>Created</Table.Head>
							<Table.Head class="text-right">Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each data.domains as d (d.id)}
							<Table.Row>
								<Table.Cell>
									<span class="rounded bg-indigo-500/10 px-1.5 py-0.5 font-mono text-sm text-indigo-300">
										{d.domain}
									</span>
								</Table.Cell>
								<Table.Cell>{@render owner(d.owner)}</Table.Cell>
								<Table.Cell class="text-muted-foreground">
									{new Date(d.createdAt).toLocaleDateString()}
								</Table.Cell>
								<Table.Cell class="text-right">
									<form method="post" action="?/removeDomain" use:enhance>
										<input type="hidden" name="id" value={d.id} />
										<Button variant="ghost" size="sm" type="submit" class="text-destructive">
											<Trash2 size={14} />
										</Button>
									</form>
								</Table.Cell>
							</Table.Row>
						{:else}
							<Table.Row>
								<Table.Cell colspan={4} class="py-10 text-center text-muted-foreground">
									No custom domains.
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		</Tabs.Content>

		<!-- Subdomains -->
		<Tabs.Content value="subdomains">
			<div class="rounded-xl border border-border bg-card/30">
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Subdomain</Table.Head>
							<Table.Head>Owner</Table.Head>
							<Table.Head>Created</Table.Head>
							<Table.Head class="text-right">Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each data.subdomains as s (s.id)}
							<Table.Row>
								<Table.Cell>
									<span class="rounded bg-indigo-500/10 px-1.5 py-0.5 font-mono text-sm text-indigo-300">
										{s.subdomain}.{relayDomain}
									</span>
								</Table.Cell>
								<Table.Cell>{@render owner(s.owner)}</Table.Cell>
								<Table.Cell class="text-muted-foreground">
									{new Date(s.createdAt).toLocaleDateString()}
								</Table.Cell>
								<Table.Cell class="text-right">
									<form method="post" action="?/removeSubdomain" use:enhance>
										<input type="hidden" name="id" value={s.id} />
										<Button variant="ghost" size="sm" type="submit" class="text-destructive">
											<Trash2 size={14} />
										</Button>
									</form>
								</Table.Cell>
							</Table.Row>
						{:else}
							<Table.Row>
								<Table.Cell colspan={4} class="py-10 text-center text-muted-foreground">
									No subdomains.
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		</Tabs.Content>

		<!-- Ports -->
		<Tabs.Content value="ports">
			<div class="rounded-xl border border-border bg-card/30">
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Port</Table.Head>
							<Table.Head>Description</Table.Head>
							<Table.Head>Owner</Table.Head>
							<Table.Head>Created</Table.Head>
							<Table.Head class="text-right">Actions</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each data.ports as p (p.id)}
							<Table.Row>
								<Table.Cell>
									<span class="rounded bg-indigo-500/10 px-1.5 py-0.5 font-mono text-sm text-indigo-300">
										{p.port}
									</span>
								</Table.Cell>
								<Table.Cell class="text-muted-foreground">{p.description || '—'}</Table.Cell>
								<Table.Cell>{@render owner(p.owner)}</Table.Cell>
								<Table.Cell class="text-muted-foreground">
									{new Date(p.createdAt).toLocaleDateString()}
								</Table.Cell>
								<Table.Cell class="text-right">
									<form method="post" action="?/removePort" use:enhance>
										<input type="hidden" name="id" value={p.id} />
										<Button variant="ghost" size="sm" type="submit" class="text-destructive">
											<Trash2 size={14} />
										</Button>
									</form>
								</Table.Cell>
							</Table.Row>
						{:else}
							<Table.Row>
								<Table.Cell colspan={5} class="py-10 text-center text-muted-foreground">
									No reserved ports.
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		</Tabs.Content>
	</Tabs.Root>
</div>
