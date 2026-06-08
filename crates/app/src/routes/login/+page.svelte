<script lang="ts">
	import LoginForm from './login-form.svelte';

	let { data, form } = $props();
</script>

<div class="bg-muted flex min-h-svh items-center justify-center p-6">
	<div class="w-full max-w-md">
		<LoginForm {form} />
	</div>
</div>