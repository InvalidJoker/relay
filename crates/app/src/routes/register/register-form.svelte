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
                    <p class="text-sm text-red-500">
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

        <div class="bg-muted relative hidden md:block">
            <img
                    src="/placeholder.svg"
                    alt=""
                    class="absolute inset-0 h-full w-full object-cover"
            />
        </div>
    </Card.Content>
</Card.Root>