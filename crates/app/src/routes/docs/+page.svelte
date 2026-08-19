<script lang="ts">
	import SiteHeader from '$lib/components/site/site-header.svelte';
	import DocsNav from '$lib/components/site/docs-nav.svelte';
	import CodeBlock from '$lib/components/site/code-block.svelte';
	import GithubIcon from '$lib/components/site/github-icon.svelte';
	import RelayLogo from '$lib/components/relay-logo.svelte';
	import { ArrowRight, Info, TriangleAlert } from 'lucide-svelte';
	import { INSTALL_COMMANDS, PORT_RANGE, RELAY_DOMAIN, REPO_URL, TCP_DOMAIN } from '$lib/site';
	import type { PageServerData } from './$types';

	let { data }: { data: PageServerData } = $props();

	const sections = [
		{ id: 'overview', title: 'What is Relay?', group: 'Getting started' },
		{ id: 'architecture', title: 'How it works', group: 'Getting started' },
		{ id: 'install', title: 'Install the CLI', group: 'Getting started' },
		{ id: 'login', title: 'Authentication', group: 'Getting started' },
		{ id: 'http', title: 'HTTP tunnels', group: 'Tunnels' },
		{ id: 'tcp', title: 'TCP tunnels', group: 'Tunnels' },
		{ id: 'basic-auth', title: 'Protecting a tunnel', group: 'Tunnels' },
		{ id: 'config', title: 'Config files & relay run', group: 'Tunnels' },
		{ id: 'cli', title: 'CLI reference', group: 'Reference' },
		{ id: 'account', title: 'Account & limits', group: 'Reference' },
		{ id: 'domains', title: 'Subdomains & custom domains', group: 'Reference' },
		{ id: 'self-hosting', title: 'Self-hosting', group: 'Operations' },
		{ id: 'troubleshooting', title: 'Troubleshooting', group: 'Operations' },
		{ id: 'faq', title: 'FAQ', group: 'Operations' }
	];

	const cliCommands = [
		{
			command: 'relay login [SERVER]',
			description:
				'Authenticate against a Relay server using the OAuth 2.0 device-code flow. Defaults to the public server; pass a URL to use your own.'
		},
		{
			command: 'relay http <PORT>',
			description:
				'Forward a local HTTP server. Flags: --subdomain <NAME>, --username <USER> + --password <PASS>, --save.'
		},
		{
			command: 'relay tcp <PORT> [REMOTE_PORT]',
			description:
				'Forward a local TCP port. Pass a remote port you reserved to get a fixed public address. Flag: --save.'
		},
		{
			command: 'relay run',
			description:
				'Start the tunnel described by a relay.toml in the current directory. Flag: -p, --path <FILE>.'
		}
	];

	const globalFlags = [
		{
			flag: '-c, --config <FILE>',
			description:
				'Use a different credentials file instead of ~/.config/relay.toml. Applies to every command.'
		},
		{ flag: '--help', description: 'Print usage for relay or any subcommand.' },
		{ flag: '--version', description: 'Print the CLI version.' },
		{
			flag: 'DEBUG=1',
			description: 'Environment variable — raises the log level from INFO to DEBUG.'
		}
	];

	const appEnv = [
		{ name: 'DATABASE_URL', description: 'Postgres connection string used by Drizzle.' },
		{ name: 'ORIGIN', description: 'Public origin of the web app, e.g. https://relay.example.com.' },
		{ name: 'BETTER_AUTH_SECRET', description: 'Secret for session signing. 32+ random characters.' },
		{
			name: 'PUBLIC_RELAY_DOMAIN',
			description: 'Domain that HTTP tunnels are published under, e.g. relay.example.com.'
		},
		{
			name: 'PUBLIC_TCP_DOMAIN',
			description: 'Hostname advertised for TCP tunnels, e.g. tcp.example.com.'
		},
		{ name: 'RELAY_URL', description: 'Host the CLI should connect to for the control channel.' },
		{
			name: 'PUBLIC_PORT_RANGE_START / _END',
			description: 'Remote port range users may reserve. Must match the relay server.'
		},
		{
			name: 'REQUIRE_AUTH',
			description: 'Set to "false" to let unauthenticated clients open tunnels. Defaults to true.'
		}
	];

	const relayEnv = [
		{ name: 'BACKEND_URL', description: 'URL of the web app, used to authorize tunnels.' },
		{
			name: 'PORT_RANGE_START / PORT_RANGE_END',
			description: 'Remote ports the server may hand out. Defaults 10000–20000.'
		},
		{ name: 'HTTP_PORT', description: 'Port for the plain HTTP proxy. Defaults to 80.' },
		{ name: 'HTTPS_PORT', description: 'Port for the TLS proxy. Defaults to 443.' },
		{ name: 'DEBUG', description: 'Set to 1 for debug logging.' }
	];

	const configExample = `# HTTP tunnel
type = "Http"
port = 3000
domain = "myapp"

[auth]
username = "alice"
password = "secret"`;

	const tcpConfigExample = `# TCP tunnel
type = "Tcp"
port = 25565
remote_port = ${PORT_RANGE.start}`;

	const credentialsExample = `server = "https://${RELAY_DOMAIN}/"
secret = "<your access token>"`;

	const composeExample = `services:
  postgres:
    image: postgres:alpine
    environment:
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=backend
    volumes:
      - postgres_data:/var/lib/postgresql

  app:
    image: ghcr.io/invalidjoker/relay-app:main
    restart: unless-stopped
    depends_on:
      - postgres
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@postgres:5432/backend
      - ORIGIN=https://${RELAY_DOMAIN}
      - BETTER_AUTH_SECRET=change-me
      - PUBLIC_RELAY_DOMAIN=${RELAY_DOMAIN}
      - PUBLIC_TCP_DOMAIN=${TCP_DOMAIN}
      - RELAY_URL=${RELAY_DOMAIN}
      - PUBLIC_PORT_RANGE_START=${PORT_RANGE.start}
      - PUBLIC_PORT_RANGE_END=${PORT_RANGE.end}

  relay:
    image: ghcr.io/invalidjoker/relay-relay:main
    restart: unless-stopped
    environment:
      - BACKEND_URL=https://${RELAY_DOMAIN}
      - PORT_RANGE_START=${PORT_RANGE.start}
      - PORT_RANGE_END=${PORT_RANGE.end}
    ports:
      - "2550:2550/tcp"
      - "${PORT_RANGE.start}-${PORT_RANGE.end}:${PORT_RANGE.start}-${PORT_RANGE.end}/tcp"

volumes:
  postgres_data:`;
</script>

<svelte:head>
	<title>Documentation — Relay</title>
	<meta
		name="description"
		content="Complete Relay documentation: installing the CLI, authenticating, HTTP and TCP tunnels, config files, account limits, custom domains, and self-hosting."
	/>
</svelte:head>

{#snippet callout(tone: 'info' | 'warn', title: string, body: string)}
	<div
		class="my-6 flex gap-3 rounded-lg border p-4 {tone === 'info'
			? 'border-primary/25 bg-primary/5'
			: 'border-accent/30 bg-accent/5'}"
	>
		{#if tone === 'info'}
			<Info class="mt-0.5 size-4 shrink-0 text-primary" />
		{:else}
			<TriangleAlert class="mt-0.5 size-4 shrink-0 text-accent" />
		{/if}
		<div>
			<p class="text-sm font-medium text-foreground">{title}</p>
			<p class="mt-1 text-sm leading-relaxed text-muted-foreground">{body}</p>
		</div>
	</div>
{/snippet}

{#snippet envTable(caption: string, rows: { name: string; description: string }[])}
	<div class="my-6 overflow-hidden rounded-xl border border-border">
		<div class="border-b border-border bg-secondary/40 px-4 py-2.5 text-xs text-muted-foreground">
			{caption}
		</div>
		<dl class="divide-y divide-border">
			{#each rows as row (row.name)}
				<div class="grid gap-1 px-4 py-3 sm:grid-cols-[minmax(0,14rem)_1fr] sm:gap-4">
					<dt class="text-sm font-medium text-primary">{row.name}</dt>
					<dd class="text-sm leading-relaxed text-muted-foreground">{row.description}</dd>
				</div>
			{/each}
		</dl>
	</div>
{/snippet}

<div class="min-h-screen bg-background">
	<SiteHeader user={data.user} />

	<!-- Docs hero -->
	<section class="relative overflow-hidden border-b border-border">
		<div
			aria-hidden="true"
			class="relay-grid pointer-events-none absolute inset-0 opacity-50"
			style="mask-image: radial-gradient(ellipse 80% 80% at 30% 0%, black, transparent 80%); -webkit-mask-image: radial-gradient(ellipse 80% 80% at 30% 0%, black, transparent 80%);"
		></div>
		<div class="relative mx-auto max-w-6xl px-5 py-14 md:py-16">
			<span class="inline-flex items-center gap-2 text-xs uppercase tracking-[0.2em] text-primary">
				<span class="h-px w-6 bg-primary"></span>
				Documentation
			</span>
			<h1 class="mt-4 text-balance text-4xl font-semibold tracking-tight text-foreground sm:text-5xl">
				Everything about Relay
			</h1>
			<p class="mt-4 max-w-2xl text-pretty text-lg leading-relaxed text-muted-foreground">
				How the tunnels work, every CLI command and flag, what your account gets you, and how to run
				the whole stack yourself.
			</p>
		</div>
	</section>

	<div class="mx-auto grid max-w-6xl gap-10 px-5 py-12 lg:grid-cols-[16rem_minmax(0,1fr)]">
		<aside class="hidden lg:block">
			<div class="sticky top-24 max-h-[calc(100vh-8rem)] overflow-y-auto pb-8">
				<DocsNav {sections} />
			</div>
		</aside>

		<main class="min-w-0 max-w-3xl">
			<!-- OVERVIEW -->
			<section id="overview" class="scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">What is Relay?</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Relay is an open-source tunneling service written in Rust. It takes a server running on
					your machine — a dev server, a game server, a database — and makes it reachable from the
					public internet, without touching your router, firewall, or DNS.
				</p>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					It ships as three parts: the <span class="text-foreground">relay CLI</span> you run
					locally, the <span class="text-foreground">relay server</span> that accepts tunnels and
					proxies public traffic, and the
					<span class="text-foreground">web app</span> where you sign in and reserve subdomains, ports,
					and custom domains. All three are MIT licensed and self-hostable.
				</p>
				<ul class="mt-6 grid gap-3 sm:grid-cols-2">
					{#each [{ t: 'HTTP tunnels', d: `Public URL on ${RELAY_DOMAIN}, random or reserved name.` }, { t: 'TCP tunnels', d: `Raw TCP on ${TCP_DOMAIN}, with an optional fixed remote port.` }, { t: 'Basic auth', d: 'Username/password gate in front of any HTTP tunnel.' }, { t: 'Config files', d: 'Save a tunnel to relay.toml and restart it with one command.' }] as item (item.t)}
						<li class="rounded-lg border border-border bg-card p-4">
							<p class="text-sm font-medium text-foreground">{item.t}</p>
							<p class="mt-1 text-sm text-muted-foreground">{item.d}</p>
						</li>
					{/each}
				</ul>
			</section>

			<!-- ARCHITECTURE -->
			<section id="architecture" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">How it works</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					The CLI opens a long-lived control connection to the relay server on TCP port
					<span class="text-foreground">2550</span> and sends a hello message containing your access
					token and what you want to expose. The server asks the web app whether you're allowed to
					have it, then starts publishing traffic to you.
				</p>
				<ol class="mt-6 space-y-4">
					{#each [{ n: '1', t: 'Handshake', d: 'The CLI connects to port 2550 and sends your token plus the requested subdomain or remote port.' }, { n: '2', t: 'Authorization', d: 'The relay server calls the web app to check the token and confirm the subdomain, custom domain, or port belongs to your account.' }, { n: '3', t: 'Publish', d: 'For HTTP, the server registers your hostname in its routing table. For TCP, it binds the requested remote port (or picks a free one in the configured range).' }, { n: '4', t: 'Proxy', d: 'Each incoming request opens a fresh stream over the control connection; the CLI dials your local port and pipes bytes both ways.' }] as step (step.n)}
						<li class="flex gap-4">
							<span
								class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-sm text-primary ring-1 ring-inset ring-primary/25"
							>
								{step.n}
							</span>
							<div>
								<p class="font-medium text-foreground">{step.t}</p>
								<p class="mt-1 text-sm leading-relaxed text-muted-foreground">{step.d}</p>
							</div>
						</li>
					{/each}
				</ol>
				<p class="mt-6 leading-relaxed text-muted-foreground">
					HTTP requests are routed by the <span class="text-foreground">Host</span> header, so a
					tunnel is only reachable at the exact hostname it registered. Unknown hostnames get a
					<span class="text-foreground">404 Not Found</span>.
				</p>
			</section>

			<!-- INSTALL -->
			<section id="install" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Install the CLI</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					The installers detect your OS and CPU architecture and drop a single
					<span class="text-foreground">relay</span> binary on your PATH.
				</p>

				<h3 class="mt-8 text-lg font-medium text-foreground">macOS / Linux</h3>
				<p class="mt-2 text-sm leading-relaxed text-muted-foreground">
					Installs to <span class="text-foreground">/usr/local/bin</span>.
				</p>
				<div class="mt-3"><CodeBlock code={INSTALL_COMMANDS.unix.code} /></div>
				<p class="mt-4 text-sm leading-relaxed text-muted-foreground">
					Pin a version or pick a different directory with environment variables:
				</p>
				<div class="mt-3 space-y-3">
					<CodeBlock
						code="RELAY_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh"
					/>
					<CodeBlock
						code="RELAY_INSTALL_DIR=~/.local/bin curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh"
					/>
				</div>

				<h3 class="mt-8 text-lg font-medium text-foreground">Windows</h3>
				<p class="mt-2 text-sm leading-relaxed text-muted-foreground">
					Run in PowerShell. Installs to <span class="text-foreground">%LOCALAPPDATA%\relay\bin</span
					> and adds it to your user PATH.
				</p>
				<div class="mt-3 space-y-3">
					<CodeBlock code={INSTALL_COMMANDS.windows.code} prompt=">" />
					<CodeBlock
						code="irm https://raw.githubusercontent.com/InvalidJoker/relay/main/install.ps1 | iex -Version v0.1.0"
						prompt=">"
					/>
				</div>

				<h3 class="mt-8 text-lg font-medium text-foreground">From source</h3>
				<p class="mt-2 text-sm leading-relaxed text-muted-foreground">
					Requires a stable Rust toolchain (edition 2024). The binary lands at
					<span class="text-foreground">target/release/relay</span>.
				</p>
				<div class="mt-3">
					<CodeBlock code={INSTALL_COMMANDS.source.code} prompt={null} />
				</div>
			</section>

			<!-- LOGIN -->
			<section id="login" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Authentication</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Before opening a tunnel, the CLI needs an access token. Relay uses the OAuth 2.0
					device-code flow, so there's no browser redirect and no pasting secrets into your shell.
				</p>
				<div class="mt-4"><CodeBlock code="relay login" /></div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					The CLI prints a short user code and a URL. Open the URL in any browser, sign in, enter the
					code, and the CLI picks up the token automatically — it polls the server until you approve
					or the code expires.
				</p>
				<p class="mt-4 leading-relaxed text-muted-foreground">Point it at your own server instead:</p>
				<div class="mt-3">
					<CodeBlock code="relay login https://relay.example.com" />
				</div>

				<h3 class="mt-8 text-lg font-medium text-foreground">Where credentials live</h3>
				<p class="mt-2 leading-relaxed text-muted-foreground">
					Credentials are written to <span class="text-foreground">~/.config/relay.toml</span>:
				</p>
				<div class="mt-3 overflow-hidden rounded-lg border border-border bg-card">
					<pre class="overflow-x-auto p-4 text-sm leading-relaxed text-muted-foreground">{credentialsExample}</pre>
				</div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Keep a second identity — say a personal account and a self-hosted server — in separate
					files and select one per command:
				</p>
				<div class="mt-3">
					<CodeBlock code="relay --config ~/.config/relay-work.toml http 3000" />
				</div>
				{@render callout(
					'warn',
					'Treat this file as a secret',
					'The token in relay.toml grants access to your account. Do not commit it, and do not confuse it with the relay.toml that --save writes into your project directory — that one only describes the tunnel.'
				)}
			</section>

			<!-- HTTP -->
			<section id="http" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">HTTP tunnels</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Forward a local web server and get a public URL back:
				</p>
				<div class="mt-4"><CodeBlock code="relay http 3000" /></div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Without <span class="text-foreground">--subdomain</span>, the server generates a readable
					random name like
					<span class="text-foreground">swiftfalcon.{RELAY_DOMAIN}</span> for the session. Ask for a
					specific name — a subdomain or a custom domain you reserved on your account:
				</p>
				<div class="mt-3 space-y-3">
					<CodeBlock code="relay http 3000 --subdomain myapp" />
					<CodeBlock code="relay http 3000 --subdomain tunnel.example.com" />
				</div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					The same flag takes both: the server first looks for a subdomain you own, then a custom
					domain. If neither is yours, the tunnel is refused with
					<span class="text-foreground">403 Forbidden</span>.
				</p>
				{@render callout(
					'info',
					'One hostname, one tunnel',
					'A hostname can only be claimed by one running tunnel at a time. Stop the old process before starting a new one on the same name.'
				)}
			</section>

			<!-- TCP -->
			<section id="tcp" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">TCP tunnels</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Anything that speaks TCP works — Minecraft servers, Postgres, SSH, a custom protocol:
				</p>
				<div class="mt-4"><CodeBlock code="relay tcp 25565" /></div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					With no remote port, the server picks a free one from its pool
					({PORT_RANGE.start}–{PORT_RANGE.end} on this deployment) and prints the public address. To
					always get the same port, reserve one in the dashboard and pass it explicitly:
				</p>
				<div class="mt-3">
					<CodeBlock code="relay tcp 25565 {PORT_RANGE.start}" />
				</div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Your players then connect to <span class="text-foreground"
						>{TCP_DOMAIN}:{PORT_RANGE.start}</span
					>. Requesting a port you haven't reserved is rejected — that's what stops two people
					fighting over the same number.
				</p>
			</section>

			<!-- BASIC AUTH -->
			<section id="basic-auth" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Protecting a tunnel</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					HTTP tunnels can sit behind HTTP basic auth. Both flags are required together:
				</p>
				<div class="mt-4">
					<CodeBlock code="relay http 3000 --username alice --password secret" />
				</div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					The relay server checks credentials itself and returns
					<span class="text-foreground">401 Unauthorized</span> before any request reaches your
					machine. Comparison is constant-time, so it doesn't leak the password through timing.
				</p>
				{@render callout(
					'warn',
					'Basic auth is not encryption',
					'Credentials are only as private as the connection carrying them. Share the https:// URL, not the http:// one, and treat basic auth as a gate for casual traffic rather than a security boundary for sensitive data.'
				)}
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Raw TCP tunnels have no equivalent — authentication there is whatever your own service
					does.
				</p>
			</section>

			<!-- CONFIG -->
			<section id="config" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">
					Config files & relay run
				</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Add <span class="text-foreground">--save</span> to any http or tcp command and the CLI
					writes a <span class="text-foreground">relay.toml</span> into the current directory describing
					that tunnel:
				</p>
				<div class="mt-4"><CodeBlock code="relay http 3000 --subdomain myapp --save" /></div>
				<p class="mt-4 leading-relaxed text-muted-foreground">Bring it back later:</p>
				<div class="mt-3 space-y-3">
					<CodeBlock code="relay run" />
					<CodeBlock code="relay run --path ./deploy/relay.toml" />
				</div>

				<h3 class="mt-8 text-lg font-medium text-foreground">File format</h3>
				<div class="mt-3 overflow-hidden rounded-lg border border-border bg-card">
					<pre class="overflow-x-auto p-4 text-sm leading-relaxed text-muted-foreground">{configExample}</pre>
				</div>
				<div class="mt-3 overflow-hidden rounded-lg border border-border bg-card">
					<pre class="overflow-x-auto p-4 text-sm leading-relaxed text-muted-foreground">{tcpConfigExample}</pre>
				</div>
				{@render envTable('relay.toml keys', [
					{ name: 'type', description: 'Either "Http" or "Tcp". Decides which fields are read.' },
					{ name: 'port', description: 'The local port to forward. Required.' },
					{
						name: 'remote_port',
						description: 'TCP only. A remote port reserved to your account; omit to get any free port.'
					},
					{
						name: 'domain',
						description: 'HTTP only. A reserved subdomain or custom domain; omit for a random name.'
					},
					{
						name: '[auth]',
						description: 'HTTP only. A table with username and password for basic auth.'
					}
				])}
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Because it holds no secrets beyond the basic-auth pair, this file is safe to commit
					alongside a project — teammates who are signed in can run
					<span class="text-foreground">relay run</span> and get the same setup.
				</p>
			</section>

			<!-- CLI REFERENCE -->
			<section id="cli" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">CLI reference</h2>
				<div class="mt-6 overflow-hidden rounded-xl border border-border">
					<div
						class="border-b border-border bg-secondary/40 px-4 py-2.5 text-xs text-muted-foreground"
					>
						Commands
					</div>
					<dl class="divide-y divide-border">
						{#each cliCommands as row (row.command)}
							<div class="grid gap-1 px-4 py-3 sm:grid-cols-[minmax(0,16rem)_1fr] sm:gap-4">
								<dt class="text-sm font-medium text-primary">{row.command}</dt>
								<dd class="text-sm leading-relaxed text-muted-foreground">{row.description}</dd>
							</div>
						{/each}
					</dl>
				</div>
				{@render envTable('Global flags & environment', globalFlags.map((f) => ({ name: f.flag, description: f.description })))}
			</section>

			<!-- ACCOUNT -->
			<section id="account" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Account & limits</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					An account is free and exists so reservations can be tied to someone. From the
					<a href="/dashboard" class="text-primary underline-offset-4 hover:underline">dashboard</a>
					you can hold:
				</p>
				<div class="mt-6 grid gap-4 sm:grid-cols-3">
					{#each [{ n: '3', l: 'Subdomains', d: `Names under ${RELAY_DOMAIN}.` }, { n: '2', l: 'Reserved ports', d: `Fixed remote TCP ports in ${PORT_RANGE.start}–${PORT_RANGE.end}.` }, { n: '1', l: 'Custom domain', d: 'A domain you own, pointed at the relay.' }] as item (item.l)}
						<div class="rounded-xl border border-border bg-card p-5">
							<p class="text-3xl font-semibold text-foreground">{item.n}</p>
							<p class="mt-1 text-sm font-medium text-foreground">{item.l}</p>
							<p class="mt-1 text-sm text-muted-foreground">{item.d}</p>
						</div>
					{/each}
				</div>
				<p class="mt-6 leading-relaxed text-muted-foreground">
					These limits apply to the public deployment; administrators are exempt, and on your own
					server you decide. Unauthenticated tunnels can be allowed entirely by setting
					<span class="text-foreground">REQUIRE_AUTH=false</span> on the web app.
				</p>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Releasing a reservation frees the name or port for everyone else, so only hold what you
					actually use.
				</p>
			</section>

			<!-- DOMAINS -->
			<section id="domains" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">
					Subdomains & custom domains
				</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					A <span class="text-foreground">subdomain</span> is a name under
					<span class="text-foreground">{RELAY_DOMAIN}</span> — reserve
					<span class="text-foreground">myapp</span> in the dashboard and
					<span class="text-foreground">myapp.{RELAY_DOMAIN}</span> is yours until you release it. Nothing
					else to configure.
				</p>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					A <span class="text-foreground">custom domain</span> is one you already own. Add it in the dashboard,
					then point it at the relay server with DNS:
				</p>
				<div class="mt-4 overflow-hidden rounded-xl border border-border">
					<div
						class="border-b border-border bg-secondary/40 px-4 py-2.5 text-xs text-muted-foreground"
					>
						DNS record
					</div>
					<div class="grid gap-1 px-4 py-3 sm:grid-cols-[minmax(0,10rem)_1fr] sm:gap-4">
						<span class="text-sm font-medium text-primary">A / CNAME</span>
						<span class="text-sm leading-relaxed text-muted-foreground">
							Point <span class="text-foreground">tunnel.example.com</span> at the relay server's IP
							(A record), or at
							<span class="text-foreground">{RELAY_DOMAIN}</span> (CNAME). Traffic then arrives with your
							hostname in the Host header, which is how the server finds your tunnel.
						</span>
					</div>
				</div>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					Start the tunnel by passing the full domain, exactly as you added it:
				</p>
				<div class="mt-3"><CodeBlock code="relay http 3000 --subdomain tunnel.example.com" /></div>
				{@render callout(
					'info',
					'Certificates for custom domains',
					'The public deployment terminates TLS in front of the relay. If you self-host, the relay binary only serves a self-signed certificate on its HTTPS port — put a reverse proxy that manages certificates in front of it before exposing custom domains.'
				)}
			</section>

			<!-- SELF-HOSTING -->
			<section id="self-hosting" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Self-hosting</h2>
				<p class="mt-4 leading-relaxed text-muted-foreground">
					A full deployment is three containers: Postgres, the web app (accounts, dashboard,
					authorization API), and the relay server (control channel plus proxies). Published images
					are built from main.
				</p>
				<div class="mt-4 overflow-hidden rounded-lg border border-border bg-card">
					<div
						class="border-b border-border bg-secondary/40 px-4 py-2.5 text-xs text-muted-foreground"
					>
						compose.yml
					</div>
					<pre class="overflow-x-auto p-4 text-sm leading-relaxed text-muted-foreground">{composeExample}</pre>
				</div>

				<h3 class="mt-8 text-lg font-medium text-foreground">Ports to open</h3>
				{@render envTable('Relay server', [
					{ name: '2550/tcp', description: 'Control channel — the port the CLI connects to. Required.' },
					{ name: '80 / 443', description: 'HTTP and HTTPS proxies for tunnelled web traffic.' },
					{
						name: `${PORT_RANGE.start}–${PORT_RANGE.end}`,
						description: 'Remote ports handed out to TCP tunnels. Publish the whole range.'
					}
				])}

				<h3 class="mt-8 text-lg font-medium text-foreground">Web app environment</h3>
				{@render envTable('app', appEnv)}

				<h3 class="mt-8 text-lg font-medium text-foreground">Relay server environment</h3>
				{@render envTable('relay', relayEnv)}

				{@render callout(
					'warn',
					'Keep the port ranges in sync',
					'PORT_RANGE_* on the relay and PUBLIC_PORT_RANGE_* on the web app must match, or the dashboard will let people reserve ports the relay will never hand out.'
				)}

				<h3 class="mt-8 text-lg font-medium text-foreground">DNS for your deployment</h3>
				<p class="mt-2 leading-relaxed text-muted-foreground">
					Point a wildcard record — <span class="text-foreground">*.{RELAY_DOMAIN}</span> — at the relay
					server so every subdomain resolves, and an A record for the TCP hostname. The web app itself
					can live anywhere reachable by both the browser and the relay container.
				</p>

				<h3 class="mt-8 text-lg font-medium text-foreground">Pointing the CLI at it</h3>
				<div class="mt-3"><CodeBlock code="relay login https://relay.example.com" /></div>
			</section>

			<!-- TROUBLESHOOTING -->
			<section id="troubleshooting" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">Troubleshooting</h2>
				<dl class="mt-6 divide-y divide-border overflow-hidden rounded-xl border border-border">
					{#each [{ q: 'Config file not found, please login first', a: 'The CLI could not read ~/.config/relay.toml. Run relay login, or pass --config with the path to an existing credentials file.' }, { q: '403 Forbidden: You do not have access to this domain', a: "The subdomain or custom domain isn't reserved to your account. Add it in the dashboard first, and pass it exactly as stored." }, { q: '403 Forbidden: You do not have access to this port', a: 'You asked for a remote TCP port you have not reserved. Reserve it in the dashboard, or drop the argument to get any free port.' }, { q: '404 Not Found: Subdomain not registered', a: 'Traffic reached the relay for a hostname with no running tunnel. Check the tunnel is still up and that DNS points at the right server.' }, { q: '401 Unauthorized on every request', a: 'The tunnel was started with --username/--password. Send those credentials, or restart without the flags.' }, { q: 'Tunnel timeout / client disconnected', a: 'The relay could not reach your local port. Confirm your server is actually listening on the port you forwarded, on localhost.' }, { q: 'Want more detail', a: 'Set DEBUG=1 before the command to raise the CLI log level to DEBUG. The relay server honours the same variable.' }] as item (item.q)}
						<div class="px-4 py-4">
							<dt class="text-sm font-medium text-foreground">{item.q}</dt>
							<dd class="mt-1.5 text-sm leading-relaxed text-muted-foreground">{item.a}</dd>
						</div>
					{/each}
				</dl>
			</section>

			<!-- FAQ -->
			<section id="faq" class="mt-16 scroll-mt-24">
				<h2 class="text-2xl font-semibold tracking-tight text-foreground">FAQ</h2>
				<dl class="mt-6 space-y-6">
					{#each [{ q: 'Is it really free?', a: 'Yes. The public server is free to use within the limits above, and the whole stack is MIT licensed if you would rather run it yourself.' }, { q: 'Does a tunnel survive a reboot?', a: 'No — a tunnel lives as long as the CLI process. What persists is the address: your reserved subdomain, custom domain, or remote port stays yours, so relay run brings the same URL back.' }, { q: 'Can I run several tunnels at once?', a: 'Yes, one process per tunnel. Each needs its own hostname or remote port.' }, { q: 'Which protocols work?', a: 'HTTP/1.1 through the HTTP proxy, and anything TCP through a raw tunnel. Traffic that needs UDP — most voice chat, QUIC — is not supported.' }, { q: 'Where does my traffic go?', a: 'Through whichever relay server you are logged into. On the public deployment that is our machine; self-host if the data should never leave your own infrastructure.' }, { q: 'What is it built on?', a: 'Rust for the CLI and server (tokio, hyper, rustls), SvelteKit and Postgres for the web app. Design and protocol take inspiration from bore.' }] as item (item.q)}
						<div>
							<dt class="font-medium text-foreground">{item.q}</dt>
							<dd class="mt-1.5 leading-relaxed text-muted-foreground">{item.a}</dd>
						</div>
					{/each}
				</dl>
			</section>

			<!-- FOOTER CTA -->
			<div class="mt-16 flex flex-col gap-4 rounded-xl border border-border bg-card p-6 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<p class="font-medium text-foreground">Something missing?</p>
					<p class="mt-1 text-sm text-muted-foreground">
						Open an issue or a pull request — the docs live in the same repository as the code.
					</p>
				</div>
				<a
					href={REPO_URL}
					target="_blank"
					rel="noopener noreferrer"
					class="inline-flex shrink-0 items-center justify-center gap-2 rounded-md border border-border bg-secondary px-4 py-2.5 text-sm font-medium text-foreground transition-colors hover:bg-muted"
				>
					<GithubIcon class="size-4" />
					Open on GitHub
				</a>
			</div>
		</main>
	</div>

	<footer class="border-t border-border">
		<div
			class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-5 py-8 sm:flex-row"
		>
			<RelayLogo size={24} />
			<a
				href="/"
				class="group inline-flex items-center gap-2 text-sm text-muted-foreground transition-colors hover:text-foreground"
			>
				Back to home
				<ArrowRight class="size-4 transition-transform group-hover:translate-x-0.5" />
			</a>
		</div>
	</footer>
</div>
