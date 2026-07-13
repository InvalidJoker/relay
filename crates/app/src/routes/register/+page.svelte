<script lang="ts">
    import RegisterForm from './register-form.svelte';

    let { form } = $props();
</script>

<div class="relative flex min-h-svh items-center justify-center bg-background p-6">
    <div class="dot-grid dot-grid-fade pointer-events-none absolute inset-0"></div>
    <div class="relative w-full max-w-4xl">
        <RegisterForm {form} />
    </div>
</div>