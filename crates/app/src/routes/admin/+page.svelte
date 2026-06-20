<script lang="ts">
	import type { PageServerData } from './$types';
	import { Badge } from '$lib/components/ui/badge';
	import { Users, ShieldCheck, Ban, UserPlus, Plug, Globe, Link2 } from 'lucide-svelte';

	let { data }: { data: PageServerData } = $props();

	const cards = $derived([
		{ label: 'Total Users', value: data.stats.totalUsers, icon: Users, color: '#818cf8' },
		{ label: 'New (7d)', value: data.stats.newUsers, icon: UserPlus, color: '#34d399' },
		{ label: 'Admins', value: data.stats.adminUsers, icon: ShieldCheck, color: '#6366f1' },
		{ label: 'Banned', value: data.stats.bannedUsers, icon: Ban, color: '#f87171' },
		{ label: 'Ports', value: data.stats.totalPorts, icon: Plug, color: '#818cf8' },
		{ label: 'Custom Domains', value: data.stats.totalDomains, icon: Globe, color: '#6366f1' },
		{ label: 'Subdomains', value: data.stats.totalSubdomains, icon: Link2, color: '#4f46e5' }
	]);
</script>

<div class="mx-auto flex max-w-5xl flex-col gap-6 px-4 py-8">
	<div>
		<h1 class="text-2xl font-extrabold tracking-tight">Overview</h1>
		<p class="text-sm text-muted-foreground">Platform-wide usage and statistics</p>
	</div>

	<!-- Stat cards -->
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4">
		{#each cards as card}
			<div class="flex flex-col gap-3 rounded-xl border border-border bg-card/40 p-4">
				<div class="flex items-center justify-between">
					<span class="text-xs font-medium text-muted-foreground">{card.label}</span>
					<div
						class="flex size-7 items-center justify-center rounded-lg"
						style="color: {card.color}; background: color-mix(in oklch, {card.color} 15%, transparent)"
					>
						<card.icon size={15} />
					</div>
				</div>
				<span class="text-2xl font-extrabold tracking-tight" style="color: {card.color}">
					{card.value}
				</span>
			</div>
		{/each}
	</div>

	<div class="grid gap-6 lg:grid-cols-2">
		<!-- Recent users -->
		<section class="flex flex-col gap-3 rounded-xl border border-border bg-card/30 p-5">
			<h2 class="text-sm font-semibold">Recent signups</h2>
			{#if data.recentUsers.length > 0}
				<ul class="flex flex-col divide-y divide-border">
					{#each data.recentUsers as u}
						<li class="flex items-center justify-between gap-3 py-2.5">
							<div class="min-w-0">
								<p class="truncate text-sm font-medium">{u.name || u.email}</p>
								<p class="truncate text-xs text-muted-foreground">{u.email}</p>
							</div>
							<div class="flex shrink-0 items-center gap-2">
								{#if u.role === 'admin'}
									<Badge variant="secondary">admin</Badge>
								{/if}
								{#if u.banned}
									<Badge variant="destructive">banned</Badge>
								{/if}
								<span class="text-xs text-muted-foreground">
									{new Date(u.createdAt).toLocaleDateString()}
								</span>
							</div>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="py-6 text-center text-sm text-muted-foreground">No users yet</p>
			{/if}
		</section>

		<!-- Top users by resources -->
		<section class="flex flex-col gap-3 rounded-xl border border-border bg-card/30 p-5">
			<h2 class="text-sm font-semibold">Top users by resources</h2>
			{#if data.topUsers.length > 0}
				<ul class="flex flex-col divide-y divide-border">
					{#each data.topUsers as u}
						<li class="flex items-center justify-between gap-3 py-2.5">
							<div class="min-w-0">
								<p class="truncate text-sm font-medium">{u.name || u.email}</p>
								<p class="truncate text-xs text-muted-foreground">{u.email}</p>
							</div>
							<Badge variant="outline">{u.resources} resources</Badge>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="py-6 text-center text-sm text-muted-foreground">No reserved resources yet</p>
			{/if}
		</section>
	</div>
</div>
