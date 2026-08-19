<script lang="ts">
	import { enhance } from '$app/forms';
	import { Plus, Trash2 } from 'lucide-svelte';
	import type { ComponentType, Snippet, SvelteComponent } from 'svelte';

	export interface ResourceItem {
		id: string;
		value: string;
		prefix?: string;
		suffix?: string;
		createdAt?: Date | string | null;
	}

	let {
		icon: Icon,
		title,
		description,
		limit = null,
		items,
		addAction,
		removeAction,
		inputName,
		placeholder,
		inputPrefix = undefined,
		inputSuffix = undefined,
		inputType = 'text',
		min = undefined,
		max = undefined,
		addLabel = 'Add',
		emptyText = 'Nothing reserved yet.',
		extraFields = undefined
	}: {
		icon: ComponentType<SvelteComponent<{ class?: string }>>;
		title: string;
		description: string;
		limit?: number | null;
		items: ResourceItem[];
		addAction: string;
		removeAction: string;
		inputName: string;
		placeholder: string;
		inputPrefix?: string;
		inputSuffix?: string;
		inputType?: string;
		min?: number;
		max?: number;
		addLabel?: string;
		emptyText?: string;
		extraFields?: Snippet<[boolean]>;
	} = $props();

	const atLimit = $derived(limit !== null && items.length >= limit);

	function formatDate(value: Date | string | null | undefined) {
		if (!value) return null;
		return new Date(value).toLocaleDateString();
	}
</script>

<section class="overflow-hidden rounded-xl border border-border bg-card">
	<div class="flex flex-wrap items-start justify-between gap-3 border-b border-border p-5">
		<div class="flex items-start gap-3">
			<div
				class="flex size-9 items-center justify-center rounded-lg bg-primary/10 ring-1 ring-inset ring-primary/25"
			>
				<Icon class="size-4 text-primary" />
			</div>
			<div>
				<h3 class="font-medium text-foreground">{title}</h3>
				<p class="mt-0.5 text-sm text-muted-foreground">{description}</p>
			</div>
		</div>
		<span
			class="shrink-0 rounded-full border border-border bg-secondary px-2.5 py-1 text-xs text-muted-foreground"
		>
			{limit === null ? `${items.length} active` : `${items.length} / ${limit}`}
		</span>
	</div>

	<div class="divide-y divide-border">
		{#if items.length === 0}
			<p class="px-5 py-6 text-center text-sm text-muted-foreground">{emptyText}</p>
		{/if}
		{#each items as item (item.id)}
			<div class="flex items-center justify-between gap-3 px-5 py-3.5">
				<div class="flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1">
					<span class="truncate font-mono text-sm text-foreground">
						{#if item.prefix}<span class="text-muted-foreground">{item.prefix}</span>{/if}{item.value}{#if item.suffix}<span
								class="text-muted-foreground">{item.suffix}</span
							>{/if}
					</span>
					{#if formatDate(item.createdAt)}
						<span class="shrink-0 text-xs text-muted-foreground">
							added {formatDate(item.createdAt)}
						</span>
					{/if}
				</div>
				<form method="post" action={removeAction} use:enhance class="shrink-0">
					<input type="hidden" name="id" value={item.id} />
					<button
						type="submit"
						aria-label="Remove {item.value}"
						class="rounded-md p-1.5 text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive"
					>
						<Trash2 class="size-4" />
					</button>
				</form>
			</div>
		{/each}
	</div>

	<form method="post" action={addAction} use:enhance class="border-t border-border bg-secondary/30 p-4">
		<div class="flex flex-col gap-2 sm:flex-row">
			<div
				class="flex flex-1 items-center rounded-md border border-input bg-background focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/40"
			>
				{#if inputPrefix}
					<span class="pl-3 font-mono text-sm text-muted-foreground">{inputPrefix}</span>
				{/if}
				<input
					name={inputName}
					type={inputType}
					{min}
					{max}
					required
					disabled={atLimit}
					placeholder={atLimit ? 'Limit reached' : placeholder}
					class="w-full border-0 bg-transparent px-3 py-2 font-mono text-sm text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-0 disabled:cursor-not-allowed disabled:opacity-60"
				/>
				{#if inputSuffix}
					<span class="pr-3 font-mono text-sm text-muted-foreground">{inputSuffix}</span>
				{/if}
			</div>

			{#if extraFields}
				{@render extraFields(atLimit)}
			{/if}

			<button
				type="submit"
				disabled={atLimit}
				class="inline-flex items-center justify-center gap-1.5 rounded-md bg-primary px-4 py-2 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
			>
				<Plus class="size-4" />
				{addLabel}
			</button>
		</div>
	</form>
</section>
