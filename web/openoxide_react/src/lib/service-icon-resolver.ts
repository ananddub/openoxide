export type IconResult = {
	path?: string;
	paths?: string[];
	color: string;
	viewBox?: string;
	/** 'git' = source-build (repo/Dockerfile) → render the Git icon,
	 *  'docker' = local image without a brand match. */
	kind?: 'git' | 'docker';
	/** Brand slug whose real multi-color artwork lives in
	 *  #/components/icons/db — render that component instead of `path`. */
	brand?: string;
};

// Real glyph data pulled from tree-shaken simple-icons modules — the previous hand-written paths here were approximations and rendered as wrong/garbage logos for redis/mongo/nginx/node.
import {
	siDocker,
	siRedis,
	siMongodb,
	siNginx,
	siNodedotjs,
} from 'simple-icons';

// Curated SVG Path & Brand Color Map for instant 0ms lookup without loading 5.7MB simple-icons bundle
const CURATED_ICONS: Record<string, IconResult> = {
	docker: {
		path: siDocker.path,
		color: `#${siDocker.hex}`,
		viewBox: '0 0 24 24',
	},
	postgresql: {
		brand: 'postgresql',
		path: siDocker.path,
		color: '#336791',
		viewBox: '0 0 24 24',
	},
	redis: {
		path: siRedis.path,
		color: `#${siRedis.hex}`,
		viewBox: '0 0 24 24',
	},
	mongodb: {
		path: siMongodb.path,
		color: `#${siMongodb.hex}`,
		viewBox: '0 0 24 24',
	},
	nginx: {
		path: siNginx.path,
		color: `#${siNginx.hex}`,
		viewBox: '0 0 24 24',
	},
	nodedotjs: {
		path: siNodedotjs.path,
		color: `#${siNodedotjs.hex}`,
		viewBox: '0 0 24 24',
	},
};

const REDPANDA_PATHS = [
	'M114.3,126.4c-2.6-4.8-4.9-9.7-7-14.7c7.8-3.4,14.7-8.3,20.5-14.5c9-8.9,13.6-21.5,13.6-37.7c0-18.8-6.1-33.4-18.4-43.9C110.6,5.2,94.8-0.4,78.7,0H0v179.7h38.3v-64.5h14.6l8.9-0.2c3.3,8.1,10.7,25.8,15,35.2L114.3,126.4z M38.3,35.9h35.3c17.7,0,26.5,7.2,26.6,21.5c0,15.4-9.9,23.1-29.6,23.1H38.3V35.9z',
	'M153.9,166.8l-10-7.1l-41.5,27l14.5,5.7l25.3-0.1l18.4-13.1L153.9,166.8z',
	'M120.6,136.8l-38.5,24.3l10.8,17l41-25.9L120.6,136.8z',
];

const ALIASES: Record<string, string> = {
	app: 'docker',
	web: 'docker',
	container: 'docker',
	docker: 'docker',
	postgres: 'postgresql',
	postgresql: 'postgresql',
	pg: 'postgresql',
	mysql: 'mysql',
	mariadb: 'mariadb',
	maria: 'mariadb',
	mongo: 'mongodb',
	mongodb: 'mongodb',
	redis: 'redis',
	nginx: 'nginx',
	node: 'nodedotjs',
	nodejs: 'nodedotjs',
};

// Brands whose real artwork ships as React components in #/components/icons/db — no path here, just color for the tile tint.
const BRAND_ICONS: Record<string, IconResult> = {
	mysql: {brand: 'mysql', color: '#4479A1'},
	mariadb: {brand: 'mariadb', color: '#7b8cde'},
};

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

// App nodes often carry a git repo URL or a buildpack name instead of a Docker image — those must render the Git mark, not a Docker whale (or a broken CDN lookup for the repo's last path segment).
const GIT_HOSTS = [
	'github.com',
	'gitlab.com',
	'bitbucket.org',
	'dev.azure.com',
	'gitea.',
	'git.',
];

const BUILD_PACKS = [
	'nixpacks',
	'dokher',
	'packonomic',
	'packonic',
	'crawtor',
	'crautorks',
	'railpack',
	'buildpacks',
	'dockerfile',
];

export function isGitSource(image?: string): boolean {
	if (!image) return false;
	const t = image.trim().toLowerCase();
	if (!t) return false;
	if (t.startsWith('http://') || t.startsWith('https://')) return true;
	if (t.startsWith('git@') || t.endsWith('.git')) return true;
	if (GIT_HOSTS.some(h => t.includes(h))) return true;
	const clean = t.replace(/[^a-z0-9]/g, '');
	return BUILD_PACKS.some(b => clean.includes(b));
}

export function resolveLocalIcon(image?: string): IconResult | null {
	if (isGitSource(image)) {
		return {color: '#F05032', kind: 'git'};
	}
	if (!isValidDockerImage(image)) {
		return null;
	}

	const target = image!.trim();
	let raw = target.split(':')[0].split('@')[0];
	if (raw.includes('/')) raw = raw.split('/').pop() || raw;
	const clean = raw.toLowerCase().replace(/[^a-z0-9]/g, '');

	if (!clean) return null;

	if (clean.includes('redpanda') || clean.includes('vectorized')) {
		return {
			paths: REDPANDA_PATHS,
			color: '#E14226',
			viewBox: '0 0 160.5 192.4',
		};
	}

	const targetSlug = ALIASES[clean] || clean;
	if (CURATED_ICONS[targetSlug]) {
		return CURATED_ICONS[targetSlug];
	}
	if (BRAND_ICONS[targetSlug]) {
		return BRAND_ICONS[targetSlug];
	}

	// Default fallback to Docker 🐳 icon if not in curated list
	return CURATED_ICONS.docker;
}
