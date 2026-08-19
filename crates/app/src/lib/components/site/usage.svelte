<script lang="ts">
	import { ArrowRight } from 'lucide-svelte';
	import CodeBlock from './code-block.svelte';
	import SectionEyebrow from './section-eyebrow.svelte';

	const steps = [
		{
			step: '01',
			title: 'Authenticate once',
			body: 'Log in with the OAuth 2.0 device-code flow — no browser redirect. Credentials are saved to ~/.config/relay.toml.',
			code: 'relay login'
		},
		{
			step: '02',
			title: 'Expose an HTTP server',
			body: 'Forward a local port to a public URL. Pick a subdomain you reserved, or protect the tunnel with basic auth.',
			code: 'relay http 3000 --subdomain myapp'
		},
		{
			step: '03',
			title: 'Or forward raw TCP',
			body: 'Great for game servers and databases. Ask for a remote port you reserved if you need a fixed one.',
			code: 'relay tcp 25565 8080'
		},
		{
			step: '04',
			title: 'Save & restart anytime',
			body: 'Add --save to write a relay.toml next to your project, then bring the tunnel back with a single command.',
			code: 'relay run'
		}
	];

	const configExample = `# HTTP example
type = "Http"
port = 3000
domain = "myapp"

# TCP example
# type = "Tcp"
# port = 25565
# remote_port = 8080`;
</script>

<section id="usage" class="scroll-mt-16 border-t border-border">
	<div class="mx-auto max-w-6xl px-5 py-20 md:py-24">
		<div class="max-w-xl">
			<SectionEyebrow>Usage</SectionEyebrow>
			<h2
				class="mt-4 text-balance text-3xl font-semibold tracking-tight text-foreground sm:text-4xl"
			>
				From install to live in four steps
			</h2>
		</div>

		<div class="mt-12 grid gap-x-10 gap-y-10 md:grid-cols-2">
			{#each steps as s (s.step)}
				<div class="flex flex-col gap-4">
					<div class="flex items-baseline gap-3">
						<span class="text-sm text-primary">{s.step}</span>
						<h3 class="text-lg font-medium text-foreground">{s.title}</h3>
					</div>
					<p class="text-sm leading-relaxed text-muted-foreground">{s.body}</p>
					<CodeBlock code={s.code} />
				</div>
			{/each}
		</div>

		<div class="mt-14 max-w-3xl">
			<h3 class="text-sm text-muted-foreground">relay.toml</h3>
			<div class="mt-3 overflow-hidden rounded-lg border border-border bg-card">
				<pre class="overflow-x-auto p-4 text-sm leading-relaxed text-muted-foreground">{configExample}</pre>
			</div>
			<a
				href="/docs"
				class="group mt-6 inline-flex items-center gap-2 text-sm font-medium text-primary transition-opacity hover:opacity-80"
			>
				Read the full documentation
				<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
			</a>
		</div>
	</div>
</section>
