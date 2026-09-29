import {
	siBitbucket,
	siCaddy,
	siCloudflare,
	siDigitalocean,
	siDocker,
	siGithub,
	siGitlab,
	siGrafana,
	siHetzner,
	siHostinger,
	siLetsencrypt,
	siMariadb,
	siMinio,
	siMongodb,
	siMysql,
	siNamecheap,
	siNginx,
	siPostgresql,
	siRedis,
	siSwarm,
	siUptimekuma,
	siVercel,
	siWireguard,
} from 'simple-icons';

/** Monochrome 24x24 glyphs, drawn with currentColor so the wall stays inside the ink palette. */
export type Brand = {name: string; path: string};

const brand = (icon: {title: string; path: string}): Brand => ({
	name: icon.title,
	path: icon.path,
});

export const brands: Brand[] = [
	brand(siDocker),
	brand(siSwarm),
	brand(siGithub),
	brand(siGitlab),
	brand(siBitbucket),
	brand(siPostgresql),
	brand(siMysql),
	brand(siMariadb),
	brand(siMongodb),
	brand(siRedis),
	brand(siNginx),
	brand(siCaddy),
	brand(siCloudflare),
	brand(siLetsencrypt),
	brand(siWireguard),
	brand(siHetzner),
	brand(siDigitalocean),
	brand(siVercel),
	brand(siHostinger),
	brand(siNamecheap),
	brand(siMinio),
	brand(siGrafana),
	brand(siUptimekuma),
];

export type Stat = {value: string; label: string; note: string};

/** Each number is countable in the repository: provider modules, proto enum arms, engine defaults, builder paths. */
export const stats: Stat[] = [
	{
		value: '72',
		label: 'DNS providers',
		note: 'modules in crates/dns-update',
	},
	{
		value: '12',
		label: 'Notification channels',
		note: 'Slack, ntfy, Gotify, email and more',
	},
	{
		value: '6',
		label: 'Database engines',
		note: 'lifecycle, backups, restores',
	},
	{
		value: '7',
		label: 'Build paths',
		note: 'Dockerfile, Nixpacks, static and more',
	},
];

export type SpecRow = {key: string; value: string};

export const panelSpec: SpecRow[] = [
	{key: 'panel', value: 'dubeyanand/openoxide'},
	{key: 'agent', value: 'dubeyanand/openoxide-agent'},
	{key: 'api', value: 'Connect RPC over HTTP/2'},
	{key: 'port', value: '4000'},
	{key: 'state', value: 'one SQLite file on disk'},
	{key: 'stack', value: 'Axum, Tokio, React 19'},
];

export type Feature = {
	id: string;
	eyebrow: string;
	title: string;
	body: string;
	bullets: string[];
	/** Screenshots are captured from a running panel, so each row names its own light and dark frame. */
	shots: {
		url: string;
		light: string;
		dark: string;
		alt: string;
	}[];
};

export const features: Feature[] = [
	{
		id: 'deployments',
		eyebrow: 'Deployments',
		title: 'Every build stays readable',
		body: 'Queued builds, live output, exit reasons and durations, all in one ledger. Open a deployment and the exact stream that produced it is already on screen.',
		bullets: [
			'Build queue with memory and CPU limits per deploy',
			'Rollback to any earlier release without a rebuild',
			'Search across titles, descriptions and log lines',
		],
		shots: [
			{
				url: 'panel.example.com/Deployments',
				light: '/screenshots/deployments-light.webp',
				dark: '/screenshots/deployments-dark.webp',
				alt: 'Deployment history with status, duration and log search',
			},
		],
	},
	{
		id: 'architecture',
		eyebrow: 'Architecture',
		title: 'The stack, drawn',
		body: 'Each service, its mounts, its ports and the directory it writes into, rendered as a graph you can click. No guessing which container owns which volume.',
		bullets: [
			'Per service CPU, memory, disk and network gauges',
			'Domains, backups, schedules and logs on the same record',
			'Compose and application projects share one view',
		],
		shots: [
			{
				url: 'panel.example.com/projects/1/compose/1',
				light: '/screenshots/arch-light.webp',
				dark: '/screenshots/arch-dark.webp',
				alt: 'Production architecture graph for a Compose project',
			},
			{
				url: 'panel.example.com/projects/1',
				light: '/screenshots/stack-light.webp',
				dark: '/screenshots/stack-dark.webp',
				alt: 'Project page listing every service with health and restart state',
			},
		],
	},
	{
		id: 'networking',
		eyebrow: 'Domains and TLS',
		title: 'Certificates that renew themselves',
		body: 'Point a domain at the panel, pick a DNS provider, and the wildcard certificate is issued, installed and renewed. Traefik config stays a file you can read.',
		bullets: [
			'ACME DNS challenges through 72 providers',
			'Redirects, middlewares and per route TLS',
			'Live Traefik dynamic config with validation',
		],
		shots: [
			{
				url: 'panel.example.com/dns',
				light: '/screenshots/dns-light.webp',
				dark: '/screenshots/dns-dark.webp',
				alt: 'DNS provider list with wildcard readiness',
			},
			{
				url: 'panel.example.com/infra',
				light: '/screenshots/infra-light.webp',
				dark: '/screenshots/infra-dark.webp',
				alt: 'Web server page with panel address, TLS status and disk usage',
			},
		],
	},
	{
		id: 'catalogue',
		eyebrow: 'Catalogue',
		title: 'Self-hosted apps, one click',
		body: 'A registry of Compose blueprints. Pick one, set a few variables, and it lands in a project with its own domain, backups and logs.',
		bullets: [
			'Import, export and patch any stack from the UI',
			'Secrets and env files kept out of the image',
			'Isolated deployments per service group',
		],
		shots: [
			{
				url: 'panel.example.com/templates',
				light: '/screenshots/templates-light.webp',
				dark: '/screenshots/templates-dark.webp',
				alt: 'Template catalogue of one-click self-hosted applications',
			},
		],
	},
	{
		id: 'fleet',
		eyebrow: 'Fleet',
		title: 'More than the machine it runs on',
		body: 'Add a server over SSH and the panel installs its agent, opens a WireGuard path if you want one, and starts shipping metrics. Swarm joins the same way.',
		bullets: [
			'One small Rust agent per monitored host',
			'Managed WireGuard networks and Swarm overlays',
			'Disk, image and build cache pruning from the host view',
		],
		shots: [
			{
				url: 'panel.example.com/remote-servers',
				light: '/screenshots/servers-light.webp',
				dark: '/screenshots/servers-dark.webp',
				alt: 'Remote server list with agent health',
			},
		],
	},
	{
		id: 'keyboard',
		eyebrow: 'Everyday use',
		title: 'Two keystrokes to anywhere',
		body: 'The palette reaches projects, deployments, queues and servers. Everything else is a click behind it, and nothing is hidden behind a wizard.',
		bullets: [
			'Command palette over every object in the panel',
			'Organizations, roles, sessions, 2FA and audit trails',
			'Notification rules wired to alert conditions',
		],
		shots: [
			{
				url: 'panel.example.com/projects',
				light: '/screenshots/palette-light.webp',
				dark: '/screenshots/palette-dark.webp',
				alt: 'Command palette open over the projects list',
			},
		],
	},
];

export type CapabilityGroup = {
	title: string;
	glyph: 'Boxes' | 'Globe' | 'Database' | 'Activity';
	items: string[];
};

export const capabilityGroups: CapabilityGroup[] = [
	{
		title: 'Compute',
		glyph: 'Boxes',
		items: [
			'Git apps, Dockerfiles and Compose stacks',
			'Nixpacks, Railpack and Cloud Native Buildpacks',
			'Build queue with resource limits and rollback',
			'Live build streams and container logs',
		],
	},
	{
		title: 'Networking',
		glyph: 'Globe',
		items: [
			'Traefik routes, middlewares and redirects',
			'Wildcard TLS through ACME DNS challenges',
			'Managed WireGuard private networks',
			'Swarm overlay networks across hosts',
		],
	},
	{
		title: 'Data',
		glyph: 'Database',
		items: [
			'Postgres, MySQL, MariaDB, MongoDB, Redis, LibSQL',
			'Scheduled dumps shipped to S3 destinations',
			'Volume backups and Compose config snapshots',
			'Migration workflows tied to a deployment',
		],
	},
	{
		title: 'Operations',
		glyph: 'Activity',
		items: [
			'One monitoring agent per host, over gRPC',
			'Alert rules routed to 12 notification channels',
			'Cron style schedules and background runners',
			'Organizations, sessions, 2FA and audit trails',
		],
	},
];

export type InstallStep = {comment: string; command: string};

/** Mirrors docker-compose.yml in the repository root: external network, panel on 4000, agent per host. */
export const installSteps: InstallStep[] = [
	{
		comment: 'the compose file expects this network to exist',
		command: 'docker network create openoxide-network',
	},
	{
		comment:
			'set SECRET_KEY, JWT_ACCESS_SECRET, JWT_REFRESH_SECRET, METRICS_TOKEN first',
		command: 'docker compose up -d openoxide',
	},
	{
		comment: 'then one agent on every host you want watched',
		command: 'docker compose -f docker-compose.agent.yml up -d',
	},
];
