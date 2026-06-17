<script lang="ts">
	import { Zap } from 'lucide-svelte';

	let { size = 28, class: className = '' }: { size?: number; class?: string } = $props();
</script>

<div class="relay-logo {className}">
	<svg width="0" height="0" style="position: absolute">
		<defs>
			<linearGradient id="relay-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
				<stop offset="0%" stop-color="#818cf8" />
				<stop offset="100%" stop-color="#a78bfa" />
			</linearGradient>
		</defs>
	</svg>

	<Zap
			size={size}
			fill="url(#relay-gradient)"
			style="stroke: url(#relay-gradient);"
	/>

	<span
			style="font-size: {size * 0.75}px"
			class="relay-wordmark"
	>
		relay
	</span>
</div>

<style>
	.relay-logo {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.relay-wordmark {
		font-weight: 700;
		letter-spacing: -0.02em;
		line-height: 1;

		background: linear-gradient(135deg, #818cf8, #a78bfa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
</style>