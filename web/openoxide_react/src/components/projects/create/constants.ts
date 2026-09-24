import {Box, Layers2, Database, LayoutTemplate} from 'lucide-react';

export type ServiceKind =
	| 'application'
	| 'compose'
	| 'database'
	| 'template';

export const OPTIONS: {
	id: ServiceKind;
	title: string;
	description: string;
	icon: typeof Box;
}[] = [
	{
		id: 'application',
		title: 'Application',
		description: 'Git repo, Dockerfile, Nixpacks ya static build.',
		icon: Box,
	},
	{
		id: 'compose',
		title: 'Compose Stack',
		description: 'Multi-container app from docker-compose.yml.',
		icon: Layers2,
	},
	{
		id: 'database',
		title: 'Database',
		description: 'PostgreSQL, MySQL, MariaDB, MongoDB, Redis.',
		icon: Database,
	},
	{
		id: 'template',
		title: 'Template',
		description: 'Ready-made app blueprints — one-click deploy.',
		icon: LayoutTemplate,
	},
];

export const DB_KINDS = [
	{
		id: 'postgres',
		label: 'PostgreSQL',
		description: 'The advanced open-source relational database.',
		defaultImage: 'postgres:17',
	},
	{
		id: 'mysql',
		label: 'MySQL',
		description: 'The world’s most popular open-source database.',
		defaultImage: 'mysql:8',
	},
	{
		id: 'mariadb',
		label: 'MariaDB',
		description: 'Community-driven drop-in MySQL replacement.',
		defaultImage: 'mariadb:11',
	},
	{
		id: 'mongodb',
		label: 'MongoDB',
		description: 'Document database for modern applications.',
		defaultImage: 'mongo:7',
	},
	{
		id: 'redis',
		label: 'Redis',
		description: 'In-memory data store, cache and message broker.',
		defaultImage: 'redis:7',
	},
	{
		id: 'libsql',
		label: 'LibSQL',
		description: 'SQLite-compatible embedded database by Turso.',
		defaultImage: 'ghcr.io/turso-labs/libsql:latest',
	},
];

export const TYPE_INFO: Record<
	ServiceKind,
	{heading: string; steps: string[]}
> = {
	application: {
		heading: 'Before you deploy an app',
		steps: [
			'Connect a Git provider or keep the repo public for auto-deploy.',
			'Add environment variables later in the service Environment tab.',
			'After creation, attach a domain from the Domains tab.',
		],
	},
	compose: {
		heading: 'About Compose Stacks',
		steps: [
			'Paste a docker-compose file or point at a repo path.',
			'Each service in the file gets its own container and logs.',
			'Swarm mode runs the stack across your cluster nodes.',
		],
	},
	database: {
		heading: 'About Databases',
		steps: [
			'Credentials are generated inside the service env file.',
			'Databases are private by default — publish a port only if needed.',
			'Schedule backups from the Backups tab after creation.',
		],
	},
	template: {
		heading: 'About Templates',
		steps: [
			'Opens the template gallery — pick an app and install it as a Compose Stack.',
			'Environment variables from the template are pre-filled; review them before deploying.',
			'Attach domains and mounts afterwards from the compose page tabs.',
		],
	},
};
