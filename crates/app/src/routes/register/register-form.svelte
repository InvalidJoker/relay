<script lang="ts">
    import { enhance } from '$app/forms';
    import * as Card from '$lib/components/ui/card';
    import {
        FieldGroup,
        Field,
        FieldLabel,
        FieldDescription
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
                action="?/signUpEmail"
                use:enhance
                class="p-6 md:p-8"
        >
            <FieldGroup>
                <div class="flex flex-col items-center gap-2 text-center">
                    <h1 class="text-2xl font-bold">Create account</h1>
                    <p class="text-muted-foreground">
                        Register a new account
                    </p>
                </div>

                <Field>
                    <FieldLabel for="name">Name</FieldLabel>
                    <Input
                            id="name"
                            name="name"
                            required
                    />
                </Field>

                <Field>
                    <FieldLabel for="email">Email</FieldLabel>
                    <Input
                            id="email"
                            name="email"
                            type="email"
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
                        Register
                    </Button>
                </Field>

                <FieldDescription class="text-center">
                    Already have an account?
                    <a
                            href="/login"
                            class="font-medium underline"
                    >
                        Login
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
                <p class="text-lg font-bold tracking-tight">Persistent domains, free</p>
                <p class="mt-1 text-sm text-muted-foreground">Claim subdomains and reserved ports in seconds.</p>
            </div>
        </div>
    </Card.Content>
</Card.Root>