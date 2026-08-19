import { env } from '$env/dynamic/public';

export const REPO_URL = 'https://github.com/InvalidJoker/relay';

/** Domain that HTTP tunnels are published under, e.g. `relay.koder.wtf`. */
export const RELAY_DOMAIN = env.PUBLIC_RELAY_DOMAIN || 'relay.koder.wtf';

/** Domain that raw TCP tunnels are published under. */
export const TCP_DOMAIN = env.PUBLIC_TCP_DOMAIN || RELAY_DOMAIN;

export const PORT_RANGE = {
	start: Number(env.PUBLIC_PORT_RANGE_START) || 10000,
	end: Number(env.PUBLIC_PORT_RANGE_END) || 20000
};

export const INSTALL_COMMANDS = {
	unix: {
		label: 'macOS / Linux',
		prompt: '$',
		code: 'curl -fsSL https://raw.githubusercontent.com/InvalidJoker/relay/main/install.sh | sh',
		note: 'Downloads the binary for your OS and architecture and installs it to /usr/local/bin.'
	},
	windows: {
		label: 'Windows',
		prompt: '>',
		code: 'irm https://raw.githubusercontent.com/InvalidJoker/relay/main/install.ps1 | iex',
		note: 'Run in PowerShell. Installs to %LOCALAPPDATA%\\relay\\bin and updates your user PATH.'
	},
	source: {
		label: 'From source',
		prompt: null,
		code: `git clone https://github.com/InvalidJoker/relay.git
cd relay
cargo build --release -p relay_cli`,
		note: 'Requires Rust (edition 2024 / stable). The binary lands at target/release/relay.'
	}
} as const;

export type InstallTarget = keyof typeof INSTALL_COMMANDS;
