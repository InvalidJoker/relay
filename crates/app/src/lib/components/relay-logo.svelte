<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';


	let { size = 28, class: className = '' }: { size?: number; class?: string } = $props();
</script>

<div class="relay-logo {className}">
	<img src={favicon} alt="Relay" width={size} height={size} />
	<span class="relay-wordmark">Relay</span>
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

		background: linear-gradient(135deg, #818cf8, #6366f1);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}
</style>