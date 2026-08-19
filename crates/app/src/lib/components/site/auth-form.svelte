<script lang="ts">
	import { enhance } from '$app/forms';
	import { ArrowRight, Eye, EyeOff, LoaderCircle } from 'lucide-svelte';

	let {
		mode,
		action,
		message = undefined
	}: { mode: 'login' | 'register'; action: string; message?: string } = $props();

	const isRegister = $derived(mode === 'register');

	let showPassword = $state(false);
	let submitting = $state(false);
</script>

<div class="w-full max-w-sm">
	<div class="mb-8">
		<h1 class="text-2xl font-semibold tracking-tight text-foreground">
			{isRegister ? 'Create your account' : 'Welcome back'}
		</h1>
		<p class="mt-2 text-sm text-muted-foreground">
			{isRegister
				? 'Reserve your subdomains, ports, and custom domain.'
				: 'Sign in to manage your tunnels and domains.'}
		</p>
	</div>

	<form
		method="POST"
		{action}
		use:enhance={() => {
			submitting = true;
			return async ({ update }) => {
				await update();
				submitting = false;
			};
		}}
		class="space-y-4"
	>
		{#if isRegister}
			<div>
				<label for="name" class="mb-1.5 block text-sm font-medium text-foreground">Name</label>
				<input
					id="name"
					name="name"
					type="text"
					autocomplete="name"
					required
					placeholder="Ada Lovelace"
					class="w-full rounded-md border border-input bg-background px-3 py-2.5 text-sm text-foreground placeholder:text-muted-foreground/60 focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring/40"
				/>
			</div>
		{/if}

		<div>
			<label for="email" class="mb-1.5 block text-sm font-medium text-foreground">Email</label>
			<input
				id="email"
				name="email"
				type="email"
				autocomplete="email"
				required
				placeholder="you@example.com"
				class="w-full rounded-md border border-input bg-background px-3 py-2.5 text-sm text-foreground placeholder:text-muted-foreground/60 focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring/40"
			/>
		</div>

		<div>
			<label for="password" class="mb-1.5 block text-sm font-medium text-foreground">Password</label>
			<div class="relative">
				<input
					id="password"
					name="password"
					type={showPassword ? 'text' : 'password'}
					autocomplete={isRegister ? 'new-password' : 'current-password'}
					required
					placeholder={isRegister ? 'At least 8 characters' : '••••••••'}
					class="w-full rounded-md border border-input bg-background px-3 py-2.5 pr-10 text-sm text-foreground placeholder:text-muted-foreground/60 focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring/40"
				/>
				<button
					type="button"
					onclick={() => (showPassword = !showPassword)}
					aria-label={showPassword ? 'Hide password' : 'Show password'}
					class="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-muted-foreground transition-colors hover:text-foreground"
				>
					{#if showPassword}
						<EyeOff class="size-4" />
					{:else}
						<Eye class="size-4" />
					{/if}
				</button>
			</div>
		</div>

		{#if message}
			<p class="rounded-md border border-destructive/40 bg-destructive/10 px-3 py-2 text-sm text-destructive">
				{message}
			</p>
		{/if}

		<button
			type="submit"
			disabled={submitting}
			class="group flex w-full items-center justify-center gap-2 rounded-md bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-70"
		>
			{#if submitting}
				<LoaderCircle class="size-4 animate-spin" />
			{:else}
				{isRegister ? 'Create account' : 'Sign in'}
				<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
			{/if}
		</button>
	</form>

	<p class="mt-6 text-center text-sm text-muted-foreground">
		{isRegister ? 'Already have an account? ' : "Don't have an account? "}
		<a
			href={isRegister ? '/login' : '/register'}
			class="font-medium text-primary underline-offset-4 hover:underline"
		>
			{isRegister ? 'Sign in' : 'Sign up'}
		</a>
	</p>
</div>
