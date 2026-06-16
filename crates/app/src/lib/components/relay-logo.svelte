<script lang="ts">
	let { size = 28, class: className = '' }: { size?: number; class?: string } = $props();
</script>

<div class="relay-logo {className}" style="display: flex; align-items: center; gap: 8px;">
	<svg
		width={size}
		height={size}
		viewBox="0 0 32 32"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
		aria-hidden="true"
	>
		<!-- Relay icon: two interconnected arcs suggesting a relay/tunnel -->
		<defs>
			<linearGradient id="relay-grad" x1="0" y1="0" x2="32" y2="32" gradientUnits="userSpaceOnUse">
				<stop offset="0%" stop-color="#818cf8" />
				<stop offset="100%" stop-color="#a78bfa" />
			</linearGradient>
		</defs>
		<!-- Left node -->
		<circle cx="6" cy="16" r="4" fill="url(#relay-grad)" />
		<!-- Right node -->
		<circle cx="26" cy="16" r="4" fill="url(#relay-grad)" />
		<!-- Top arc -->
		<path
			d="M10 16 Q16 6 22 16"
			stroke="url(#relay-grad)"
			stroke-width="2.5"
			fill="none"
			stroke-linecap="round"
		/>
		<!-- Bottom arc -->
		<path
			d="M10 16 Q16 26 22 16"
			stroke="url(#relay-grad)"
			stroke-width="2.5"
			fill="none"
			stroke-linecap="round"
			opacity="0.6"
		/>
	</svg>
	<span
		style="font-size: {size * 0.75}px; font-weight: 700; letter-spacing: -0.02em; line-height: 1;"
		class="relay-wordmark"
	>
		relay
	</span>
</div>

<style>
	.relay-wordmark {
		background: linear-gradient(135deg, #818cf8, #a78bfa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
</style>
