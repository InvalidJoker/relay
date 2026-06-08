<script lang="ts">
    import RegisterForm from './register-form.svelte';

    let { form } = $props();
</script>

<div class="bg-muted flex min-h-svh items-center justify-center p-6">
    <div class="w-full max-w-md">
        <RegisterForm {form} />
    </div>
</div>