<script lang="ts">
	import { ArrowRight } from 'lucide-svelte';
	import CodeBlock from './code-block.svelte';
	import GithubIcon from './github-icon.svelte';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { INSTALL_COMMANDS, REPO_URL } from '$lib/site';

	let { user = null }: { user?: unknown } = $props();
</script>

<section class="border-t border-border">
	<div class="mx-auto max-w-6xl px-5 py-20 md:py-24">
		<div class="relative overflow-hidden rounded-2xl border border-border bg-card p-8 md:p-12">
			<div
				aria-hidden="true"
				class="relay-grid pointer-events-none absolute inset-0 opacity-40"
				style="mask-image: radial-gradient(ellipse 70% 90% at 50% 0%, black, transparent 80%); -webkit-mask-image: radial-gradient(ellipse 70% 90% at 50% 0%, black, transparent 80%);"
			></div>
			<div class="relative mx-auto max-w-2xl text-center">
				<h2 class="text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl">
					Ship your localhost in one line
				</h2>
				<p class="mt-4 text-pretty leading-relaxed text-muted-foreground">
					Free, open-source, and ready when you are. Create an account to lock in your domains, then
					install the CLI.
				</p>
				<div class="mx-auto mt-8 max-w-xl text-left">
					<CodeBlock code={INSTALL_COMMANDS.unix.code} />
				</div>
				<div class="mt-6 flex flex-col justify-center gap-3 sm:flex-row">
					<a
						href={user ? '/dashboard' : '/register'}
						class="group inline-flex items-center justify-center gap-2 rounded-md bg-primary px-5 py-2.5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
					>
						{user ? 'Go to dashboard' : 'Create free account'}
						<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
					</a>
					<a
						href={REPO_URL}
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center justify-center gap-2 rounded-md border border-border bg-secondary px-5 py-2.5 text-sm font-medium text-foreground transition-colors hover:bg-muted"
					>
						<GithubIcon class="size-4" />
						Star on GitHub
					</a>
				</div>
			</div>
		</div>
	</div>
</section>

<footer class="border-t border-border">
	<div
		class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-5 py-8 sm:flex-row"
	>
		<RelayLogo size={24} />
		<div class="flex flex-wrap items-center justify-center gap-x-5 gap-y-2 text-xs text-muted-foreground">
			<a href="/docs" class="transition-colors hover:text-foreground">Docs</a>
			<a href={user ? '/dashboard' : '/login'} class="transition-colors hover:text-foreground">
				{user ? 'Dashboard' : 'Sign in'}
			</a>
			<span>
				Open-source TCP tunneling. Inspired by
				<a
					href="https://github.com/ekzhang/bore"
					target="_blank"
					rel="noopener noreferrer"
					class="text-foreground underline-offset-4 hover:underline">bore</a
				>.
			</span>
		</div>
		<a
			href={REPO_URL}
			target="_blank"
			rel="noopener noreferrer"
			class="text-muted-foreground transition-colors hover:text-foreground"
			aria-label="Relay on GitHub"
		>
			<GithubIcon class="size-5" />
		</a>
	</div>
</footer>
