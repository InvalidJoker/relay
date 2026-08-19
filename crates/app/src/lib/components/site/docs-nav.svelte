<script lang="ts">
	import { onMount } from 'svelte';

	let {
		sections
	}: { sections: { id: string; title: string; group: string }[] } = $props();

	let active = $state('');

	const groups = $derived(
		sections.reduce<{ name: string; items: { id: string; title: string }[] }[]>((acc, section) => {
			const last = acc.at(-1);
			if (last && last.name === section.group) last.items.push(section);
			else acc.push({ name: section.group, items: [section] });
			return acc;
		}, [])
	);

	onMount(() => {
		active = sections[0]?.id ?? '';

		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					if (entry.isIntersecting) active = entry.target.id;
				}
			},
			{ rootMargin: '-80px 0px -70% 0px', threshold: 0 }
		);

		for (const section of sections) {
			const el = document.getElementById(section.id);
			if (el) observer.observe(el);
		}

		return () => observer.disconnect();
	});
</script>

<nav aria-label="Documentation sections" class="flex flex-col gap-6 text-sm">
	{#each groups as group (group.name)}
		<div>
			<p class="mb-2 text-xs uppercase tracking-[0.18em] text-muted-foreground/70">{group.name}</p>
			<ul class="flex flex-col gap-0.5 border-l border-border">
				{#each group.items as item (item.id)}
					<li>
						<a
							href="#{item.id}"
							aria-current={active === item.id ? 'true' : undefined}
							class="-ml-px block border-l py-1.5 pl-3 transition-colors {active === item.id
								? 'border-primary font-medium text-foreground'
								: 'border-transparent text-muted-foreground hover:border-border hover:text-foreground'}"
						>
							{item.title}
						</a>
					</li>
				{/each}
			</ul>
		</div>
	{/each}
</nav>
