import * as simpleIcons from 'simple-icons';

export type IconResult = {
	path?: string;
	paths?: string[];
	color: string;
	viewBox?: string;
};

// Official Redpanda Data Brand Icon (3 paths, official #E14226 color)
const REDPANDA_PATHS = [
	"M114.3,126.4c-2.6-4.8-4.9-9.7-7-14.7c7.8-3.4,14.7-8.3,20.5-14.5c9-8.9,13.6-21.5,13.6-37.7c0-18.8-6.1-33.4-18.4-43.9C110.6,5.2,94.8-0.4,78.7,0H0v179.7h38.3v-64.5h14.6l8.9-0.2c3.3,8.1,10.7,25.8,15,35.2L114.3,126.4z M38.3,35.9h35.3c17.7,0,26.5,7.2,26.6,21.5c0,15.4-9.9,23.1-29.6,23.1H38.3V35.9z",
	"M153.9,166.8l-10-7.1l-41.5,27l14.5,5.7l25.3-0.1l18.4-13.1L153.9,166.8z",
	"M120.6,136.8l-38.5,24.3l10.8,17l41-25.9L120.6,136.8z",
];

const ALIASES: Record<string, string> = {
	app: 'docker',
	web: 'docker',
	container: 'docker',
	docker: 'docker',
	postgres: 'postgresql',
	postgresql: 'postgresql',
	pg: 'postgresql',
	mongo: 'mongodb',
	mongodb: 'mongodb',
	elastic: 'elasticsearch',
	elasticsearch: 'elasticsearch',
	prom: 'prometheus',
	prometheus: 'prometheus',
	golang: 'go',
	go: 'go',
	node: 'nodedotjs',
	nodejs: 'nodedotjs',
	nats: 'natsdotio',
	mosquitto: 'eclipsemosquitto',
	mqtt: 'mqtt',
	kafka: 'apachekafka',
	apachekafka: 'apachekafka',
	caddy: 'caddy',
	nginx: 'nginx',
	redis: 'redis',
};

// Cache all icons array once
const ALL_ICONS = Object.values(simpleIcons).filter(
	(i): i is simpleIcons.SimpleIcon => Boolean(i && typeof i === 'object' && 'slug' in i && 'path' in i),
);

/**
 * Checks if the given image string is a valid Docker image tag
 * and NOT a local build path like '.', './worker', etc.
 */
export function isValidDockerImage(image?: string): boolean {
	if (!image) return false;
	const trimmed = image.trim();
	if (
		!trimmed ||
		trimmed === '.' ||
		trimmed === 'build' ||
		trimmed.startsWith('./') ||
		trimmed.startsWith('../') ||
		trimmed.startsWith('/')
	) {
		return false;
	}
	return true;
}

export function resolveLocalIcon(image?: string): IconResult | null {
	// Rule: If image name is NOT a valid Docker image (e.g. '.', './dir'), DO NOT search by container name!
	if (!isValidDockerImage(image)) {
		return null;
	}

	const target = image!.trim();

	let raw = target.split(':')[0].split('@')[0];
	if (raw.includes('/')) raw = raw.split('/').pop() || raw;
	const clean = raw.toLowerCase().replace(/[^a-z0-9]/g, '');

	if (!clean) return null;

	// Redpanda special case
	if (clean.includes('redpanda') || clean.includes('vectorized')) {
		return {paths: REDPANDA_PATHS, color: '#E14226', viewBox: '0 0 160.5 192.4'};
	}

	const aliasTarget = ALIASES[clean];

	// 1. Direct slug match or alias match
	let matched = ALL_ICONS.find(
		i => i.slug === clean || (aliasTarget && i.slug === aliasTarget),
	);
	if (matched) return {path: matched.path, color: '#' + matched.hex, viewBox: '0 0 24 24'};

	// 2. Title match
	matched = ALL_ICONS.find(i => i.title.toLowerCase().replace(/[^a-z0-9]/g, '') === clean);
	if (matched) return {path: matched.path, color: '#' + matched.hex, viewBox: '0 0 24 24'};

	// No fuzzy match: if not exact, return null to fall back safely to Docker 🐳 logo
	return null;
}
