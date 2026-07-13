<script lang="ts">
	import LoginForm from './login-form.svelte';

	let { data, form } = $props();
</script>

<div class="relative flex min-h-svh items-center justify-center bg-background p-6">
	<div class="dot-grid dot-grid-fade pointer-events-none absolute inset-0"></div>
	<div class="relative w-full max-w-4xl">
		<LoginForm {form} />
	</div>
</div>