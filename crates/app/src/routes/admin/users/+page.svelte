<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, PageServerData, ActionData } from './$types';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import {
		MoreHorizontal,
		Search,
		ShieldCheck,
		ShieldOff,
		Ban,
		CircleCheck,
		Trash2,
		AlertCircle
	} from 'lucide-svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();

	type Row = PageServerData['users'][number];

	let banTarget = $state<Row | null>(null);
	let deleteTarget = $state<Row | null>(null);
</script>

<div class="mx-auto flex max-w-5xl flex-col gap-6 px-4 py-8">
	<div class="flex flex-wrap items-end justify-between gap-3">
		<div>
			<h1 class="text-2xl font-extrabold tracking-tight">Users</h1>
			<p class="text-sm text-muted-foreground">{data.total} total user{data.total === 1 ? '' : 's'}</p>
		</div>
		<form method="get" class="flex items-center gap-2">
			<div class="relative">
				<Search
					size={15}
					class="pointer-events-none absolute top-1/2 left-2.5 -translate-y-1/2 text-muted-foreground"
				/>
				<Input
					name="q"
					value={data.q}
					placeholder="Search name or email"
					class="w-56 pl-8"
				/>
			</div>
			<Button type="submit" variant="secondary">Search</Button>
		</form>
	</div>

	{#if form?.message}
		<div
			class="flex items-center gap-2 rounded-lg border border-destructive/25 bg-destructive/10 px-4 py-3 text-sm text-destructive"
		>
			<AlertCircle size={16} />
			{form.message}
		</div>
	{/if}

	<div class="rounded-xl border border-border bg-card/30">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>User</Table.Head>
					<Table.Head>Role</Table.Head>
					<Table.Head>Status</Table.Head>
					<Table.Head>Resources</Table.Head>
					<Table.Head>Joined</Table.Head>
					<Table.Head class="text-right">Actions</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each data.users as u (u.id)}
					{@const isSelf = u.id === data.user.id}
					<Table.Row>
						<Table.Cell>
							<div class="flex min-w-0 flex-col">
								<span class="flex items-center gap-1.5 font-medium">
									{u.name || '—'}
									{#if isSelf}<span class="text-xs text-muted-foreground">(you)</span>{/if}
								</span>
								<span class="truncate text-xs text-muted-foreground">{u.email}</span>
							</div>
						</Table.Cell>
						<Table.Cell>
							{#if u.role === 'admin'}
								<Badge variant="secondary">admin</Badge>
							{:else}
								<span class="text-sm text-muted-foreground">user</span>
							{/if}
						</Table.Cell>
						<Table.Cell>
							{#if u.banned}
								<Badge variant="destructive">banned</Badge>
							{:else if u.emailVerified}
								<Badge variant="outline">verified</Badge>
							{:else}
								<span class="text-sm text-muted-foreground">unverified</span>
							{/if}
						</Table.Cell>
						<Table.Cell class="text-muted-foreground">{u.resources}</Table.Cell>
						<Table.Cell class="text-muted-foreground">
							{new Date(u.createdAt).toLocaleDateString()}
						</Table.Cell>
						<Table.Cell class="text-right">
							<DropdownMenu.Root>
								<DropdownMenu.Trigger>
									{#snippet child({ props })}
										<Button variant="ghost" size="sm" disabled={isSelf} {...props}>
											<MoreHorizontal size={16} />
										</Button>
									{/snippet}
								</DropdownMenu.Trigger>
								<DropdownMenu.Content align="end" class="w-44">
									{#if u.role === 'admin'}
										<form method="post" action="?/setRole" use:enhance>
											<input type="hidden" name="id" value={u.id} />
											<input type="hidden" name="role" value="user" />
											<DropdownMenu.Item>
												{#snippet child({ props })}
													<button type="submit" class="flex w-full items-center gap-2" {...props}>
														<ShieldOff size={14} />
														Revoke admin
													</button>
												{/snippet}
											</DropdownMenu.Item>
										</form>
									{:else}
										<form method="post" action="?/setRole" use:enhance>
											<input type="hidden" name="id" value={u.id} />
											<input type="hidden" name="role" value="admin" />
											<DropdownMenu.Item>
												{#snippet child({ props })}
													<button type="submit" class="flex w-full items-center gap-2" {...props}>
														<ShieldCheck size={14} />
														Make admin
													</button>
												{/snippet}
											</DropdownMenu.Item>
										</form>
									{/if}

									{#if u.banned}
										<form method="post" action="?/unban" use:enhance>
											<input type="hidden" name="id" value={u.id} />
											<DropdownMenu.Item>
												{#snippet child({ props })}
													<button type="submit" class="flex w-full items-center gap-2" {...props}>
														<CircleCheck size={14} />
														Unban
													</button>
												{/snippet}
											</DropdownMenu.Item>
										</form>
									{:else}
										<DropdownMenu.Item onSelect={() => (banTarget = u)}>
											<Ban size={14} />
											Ban user
										</DropdownMenu.Item>
									{/if}

									<DropdownMenu.Separator />
									<DropdownMenu.Item variant="destructive" onSelect={() => (deleteTarget = u)}>
										<Trash2 size={14} />
										Delete
									</DropdownMenu.Item>
								</DropdownMenu.Content>
							</DropdownMenu.Root>
						</Table.Cell>
					</Table.Row>
				{:else}
					<Table.Row>
						<Table.Cell colspan={6} class="py-10 text-center text-muted-foreground">
							No users found.
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>

<!-- Ban dialog -->
<AlertDialog.Root open={banTarget !== null} onOpenChange={(o) => !o && (banTarget = null)}>
	<AlertDialog.Content>
		<form method="post" action="?/ban" use:enhance={() => async ({ update }) => {
			banTarget = null;
			await update();
		}}>
			<input type="hidden" name="id" value={banTarget?.id} />
			<AlertDialog.Header>
				<AlertDialog.Title>Ban {banTarget?.name || banTarget?.email}?</AlertDialog.Title>
				<AlertDialog.Description>
					The user will be signed out immediately and unable to sign in until unbanned.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<div class="my-4 flex flex-col gap-2">
				<Label for="reason">Reason (optional)</Label>
				<Input id="reason" name="reason" placeholder="e.g. Abuse of service" />
			</div>
			<AlertDialog.Footer>
				<AlertDialog.Cancel type="button">Cancel</AlertDialog.Cancel>
				<AlertDialog.Action type="submit" class="bg-destructive text-white hover:bg-destructive/90">
					Ban user
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</form>
	</AlertDialog.Content>
</AlertDialog.Root>

<!-- Delete dialog -->
<AlertDialog.Root open={deleteTarget !== null} onOpenChange={(o) => !o && (deleteTarget = null)}>
	<AlertDialog.Content>
		<form method="post" action="?/remove" use:enhance={() => async ({ update }) => {
			deleteTarget = null;
			await update();
		}}>
			<input type="hidden" name="id" value={deleteTarget?.id} />
			<AlertDialog.Header>
				<AlertDialog.Title>Delete {deleteTarget?.name || deleteTarget?.email}?</AlertDialog.Title>
				<AlertDialog.Description>
					This permanently deletes the user and all their ports, domains and subdomains. This cannot
					be undone.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer>
				<AlertDialog.Cancel type="button">Cancel</AlertDialog.Cancel>
				<AlertDialog.Action type="submit" class="bg-destructive text-white hover:bg-destructive/90">
					Delete
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</form>
	</AlertDialog.Content>
</AlertDialog.Root>
