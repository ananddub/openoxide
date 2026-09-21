import type {ContainerItem} from './container-inspect-modal';

/** Checks whether a raw docker container is actively running. */
const isContainerRunning = (c: any): boolean => {
	const state = String(c.state || c.State || '').toLowerCase();
	const status = String(c.status || c.Status || '').toLowerCase();
	if (
		state.includes('exited') ||
		state.includes('dead') ||
		state.includes('stopped')
	)
		return false;
	if (status.includes('exited') || status.includes('dead')) return false;
	return (
		state.includes('running') ||
		state.includes('up') ||
		status.includes('up') ||
		status.includes('running')
	);
};

const rawNameOf = (c: any): string =>
	String(
		c.names || c.Names || c.name || c.Name || c.id || c.ID || 'container',
	).replace(/^\//, '');

const matchesProject = (
	c: any,
	rawApp: string,
	cleanApp: string,
): boolean => {
	const n = rawNameOf(c).toLowerCase();
	const l = String(c.labels || c.Labels || '').toLowerCase();
	const cleanN = n.replace(/[^a-z0-9]/g, '');
	const isNameMatch =
		n.startsWith(`${rawApp}-`) ||
		n.startsWith(`${rawApp}_`) ||
		n.includes(`-${rawApp}-`) ||
		cleanN.startsWith(cleanApp);
	const isLabelMatch =
		l.includes(`com.docker.compose.project=${rawApp}`) ||
		l.includes(`compose.project=${rawApp}`) ||
		l.includes(rawApp);
	return isNameMatch || isLabelMatch;
};

const resolveServiceName = (
	cleanName: string,
	serviceNames: string[],
): string => {
	const found = serviceNames.find(
		s =>
			cleanName.toLowerCase().includes(`-${s.toLowerCase()}-`) ||
			cleanName.toLowerCase().endsWith(`-${s.toLowerCase()}`),
	);
	if (found) return found;
	const parts = cleanName.split(/[-_]/);
	if (parts.length >= 2 && !isNaN(Number(parts[parts.length - 1])))
		return parts[parts.length - 2];
	return parts.pop() || 'service';
};

const toContainerItem = (
	c: any,
	appName: string,
	serviceNames: string[],
): ContainerItem => {
	const isRunning = isContainerRunning(c);
	const cleanName = rawNameOf(c).split(',')[0].trim();
	const containerId = String(c.id || c.ID || 'id').slice(0, 12);
	return {
		id: containerId,
		name: cleanName,
		service: resolveServiceName(cleanName, serviceNames),
		image: c.image || c.Image || `${appName}:latest`,
		status: isRunning ? 'running' : 'stopped',
		statusText:
			c.status || c.Status || (isRunning ? 'Up (healthy)' : 'Exited'),
		ports: c.ports || c.Ports || 'N/A',
		networks:
			c.networks || c.Networks
				? [c.networks || c.Networks]
				: [`${appName}_default`],
		mounts:
			c.mounts || c.Mounts
				? [{source: c.mounts || c.Mounts, destination: '/app', mode: 'rw'}]
				: [],
		env: {COMPOSE_PROJECT: appName, CONTAINER_ID: containerId},
	};
};

/**
 * Filters raw docker containers down to those belonging to this compose
 * project (by name prefix or compose label), dedupes by name, and maps each
 * to a presentation `ContainerItem`.
 */
export function buildComposeContainers(
	compose: any,
	rawContainers: any[],
	serviceNames: string[],
): ContainerItem[] {
	const appName = compose?.app_name || compose?.name || '';
	const rawApp = appName.toLowerCase().trim();
	if (!rawApp) return [];
	const cleanApp = rawApp.replace(/[^a-z0-9]/g, '');

	const all = Array.isArray(rawContainers) ? rawContainers : [];
	const matched = all.filter((c: any) =>
		matchesProject(c, rawApp, cleanApp),
	);
	if (matched.length === 0) return [];

	const running = matched.filter(isContainerRunning);
	const active = running.length > 0 ? running : matched;

	const unique = new Map<string, any>();
	active.forEach((c: any) => {
		const name = rawNameOf(c);
		if (!unique.has(name)) unique.set(name, c);
	});

	return Array.from(unique.values()).map((c: any) =>
		toContainerItem(c, appName, serviceNames),
	);
}
