<script lang="ts">
	import { enhance } from '$app/forms';
	import * as Card from '$lib/components/ui/card';
	import {
		FieldGroup,
		Field,
		FieldLabel,
		FieldDescription,
		FieldSeparator
	} from '$lib/components/ui/field';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import RelayHop from '$lib/components/relay-hop.svelte';

	import type { ActionData } from './$types';

	let { form }: { form?: ActionData } = $props();
</script>

<Card.Root class="overflow-hidden p-0">
	<Card.Content class="grid p-0 md:grid-cols-2">
		<form
				method="POST"
				action="?/signInEmail"
				use:enhance
				class="p-6 md:p-8"
		>
			<FieldGroup>
				<div class="flex flex-col items-center gap-2 text-center">
					<h1 class="text-2xl font-bold">Welcome back</h1>
					<p class="text-muted-foreground">
						Sign in to your account
					</p>
				</div>

				<Field>
					<FieldLabel for="email">Email</FieldLabel>
					<Input
							id="email"
							name="email"
							type="email"
							placeholder="m@example.com"
							required
					/>
				</Field>

				<Field>
					<FieldLabel for="password">Password</FieldLabel>
					<Input
							id="password"
							name="password"
							type="password"
							required
					/>
				</Field>

				{#if form?.message}
					<p class="text-sm text-destructive">
						{form.message}
					</p>
				{/if}

				<Field>
					<Button type="submit" class="w-full">
						Login
					</Button>
				</Field>

				<FieldDescription class="text-center">
					Don't have an account?
					<a
							href="/register"
							class="font-medium underline"
					>
						Register
					</a>
				</FieldDescription>
			</FieldGroup>
		</form>

		<div class="bg-muted relative hidden flex-col items-center justify-center gap-8 overflow-hidden p-8 md:flex">
			<div class="dot-grid dot-grid-fade pointer-events-none absolute inset-0"></div>
			<div class="brand-halo pointer-events-none absolute left-1/2 top-1/2 size-64 -translate-x-1/2 -translate-y-1/2 rounded-full"></div>
			<div class="relative w-full max-w-xs">
				<RelayHop remote="myapp.relay" />
			</div>
			<div class="relative text-center">
				<p class="text-lg font-bold tracking-tight">Tunnel localhost to the world</p>
				<p class="mt-1 text-sm text-muted-foreground">One command. A public URL. Zero config.</p>
			</div>
		</div>
	</Card.Content>
</Card.Root>