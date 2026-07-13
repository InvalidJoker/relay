<script lang="ts">
	import { Monitor, Globe } from 'lucide-svelte';
	import favicon from '$lib/assets/favicon.svg';

	let {
		remote = 'myapp.relay',
		class: className = ''
	}: { remote?: string; class?: string } = $props();
</script>

<!-- Signature motif: a request "hops" from localhost, through relay, out to the world -->
<div class="hop {className}">
	<div class="node">
		<div class="node-dot">
			<Monitor size={20} strokeWidth={1.75} />
		</div>
		<span class="node-label">localhost:8080</span>
	</div>

	<div class="wire" aria-hidden="true">
		<span class="pulse"></span>
	</div>

	<div class="node node--brand">
		<div class="node-dot node-dot--brand">
			<img src={favicon} alt="" width="22" height="22" />
		</div>
		<span class="node-label">relay</span>
	</div>

	<div class="wire" aria-hidden="true">
		<span class="pulse pulse--2"></span>
	</div>

	<div class="node">
		<div class="node-dot node-dot--signal">
			<Globe size={20} strokeWidth={1.75} />
		</div>
		<span class="node-label">{remote}</span>
	</div>
</div>

<style>
	.hop {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
	}

	.node {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.55rem;
		flex-shrink: 0;
	}

	.node-dot {
		width: 52px;
		height: 52px;
		border-radius: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--card);
		border: 1px solid var(--border);
		color: var(--muted-foreground);
		box-shadow: 0 1px 2px oklch(0 0 0 / 0.05);
	}

	.node-dot--brand {
		border-color: color-mix(in oklch, var(--brand) 45%, transparent);
		background: color-mix(in oklch, var(--brand) 12%, var(--card));
		color: var(--brand);
	}

	.node-dot--signal {
		border-color: color-mix(in oklch, var(--signal) 45%, transparent);
		background: color-mix(in oklch, var(--signal) 14%, var(--card));
		color: color-mix(in oklch, var(--signal) 72%, var(--foreground));
	}

	.node-label {
		font-size: 0.72rem;
		font-family: 'SF Mono', ui-monospace, 'Fira Code', monospace;
		color: var(--muted-foreground);
		white-space: nowrap;
	}

	.wire {
		position: relative;
		flex: 1 1 auto;
		min-width: 36px;
		max-width: 120px;
		height: 2px;
		margin-bottom: 1.6rem;
		border-radius: 999px;
		background-image: linear-gradient(
			to right,
			color-mix(in oklch, var(--brand) 55%, transparent) 0 6px,
			transparent 6px 12px
		);
		background-size: 12px 2px;
		background-repeat: repeat-x;
	}

	.pulse {
		position: absolute;
		top: 50%;
		left: 0;
		width: 7px;
		height: 7px;
		border-radius: 999px;
		background: var(--brand);
		box-shadow: 0 0 10px 1px var(--brand);
		transform: translate(-50%, -50%);
		animation: travel 2.4s ease-in-out infinite;
	}

	.pulse--2 {
		background: var(--signal);
		box-shadow: 0 0 10px 1px var(--signal);
		animation-delay: 1.2s;
	}

	@keyframes travel {
		0% {
			left: 0;
			opacity: 0;
		}
		15% {
			opacity: 1;
		}
		85% {
			opacity: 1;
		}
		100% {
			left: 100%;
			opacity: 0;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.pulse {
			animation: none;
			left: 50%;
		}
	}

	@media (max-width: 420px) {
		.node-label {
			font-size: 0.62rem;
		}
		.node-dot {
			width: 44px;
			height: 44px;
		}
	}
</style>
