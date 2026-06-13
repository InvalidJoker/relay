<script lang="ts">
    import RegisterForm from './register-form.svelte';

    let { form } = $props();
</script>

<div class="bg-muted flex min-h-svh items-center justify-center p-6">
    <div class="w-full max-w-4xl">
        <RegisterForm {form} />
    </div>
</div>