import {
	GlobalContainerAction,
	GlobalService,
	type GlobalContainer,
	type GlobalImage,
	type GlobalNetwork,
	type GlobalVolume,
} from '#/gen/connect/openoxide/api/v1/global_pb';
import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';

/** Docker engine calls scoped to one server; `serverId` selects the remote agent. */

const globalClient = createServiceClient(GlobalService);

export type ServerContainer = {
	id: string;
	name: string;
	image: string;
	state: string;
	status: string;
	runningFor: string;
	ports: string;
	running: boolean;
};

export type ServerImage = {
	id: string;
	repository: string;
	tag: string;
	reference: string;
	createdSince: string;
	size: string;
	sharedSize: string;
	uniqueSize: string;
};

export type ServerNetwork = {
	id: string;
	name: string;
	driver: string;
	scope: string;
	internal: string;
};

export type ServerVolume = {
	name: string;
	driver: string;
	scope: string;
	mountpoint: string;
	availability: string;
	size: string;
};

export type ServerDiskUsageRow = {
	type: string;
	total: string;
	active: string;
	size: string;
	reclaimable: string;
};

/** Short ids arrive as 64-hex digests; the table only needs the CLI prefix. */
export function shortId(value: string): string {
	return /^[0-9a-f]{13,}$/i.test(value.trim())
		? value.trim().slice(0, 12)
		: value;
}

export function mapContainer(row: GlobalContainer): ServerContainer {
	const state = row.state.toLowerCase();
	return {
		id: shortId(row.id),
		name: row.names.replace(/^\//, '').split(',')[0] || row.id,
		image: row.image,
		state: row.state,
		status: row.status,
		runningFor: row.runningFor,
		ports: row.ports,
		running: state === 'running',
	};
}

export function mapImage(row: GlobalImage): ServerImage {
	const reference =
		row.repository && row.tag
			? `${row.repository}:${row.tag}`
			: row.repository;
	return {
		id: shortId(row.id),
		repository: row.repository,
		tag: row.tag,
		reference: reference || shortId(row.id),
		createdSince: row.createdSince,
		size: row.size,
		sharedSize: row.sharedSize,
		uniqueSize: row.uniqueSize,
	};
}

export function mapNetwork(row: GlobalNetwork): ServerNetwork {
	return {
		id: shortId(row.id),
		name: row.name,
		driver: row.driver,
		scope: row.scope,
		internal: row.internal,
	};
}

export function mapVolume(row: GlobalVolume): ServerVolume {
	return {
		name: row.name,
		driver: row.driver,
		scope: row.scope,
		mountpoint: row.mountpoint,
		availability: row.availability,
		size: row.size,
	};
}

/** `docker system df --format {{json .}}` rows; key casing varies across CLI versions. */
export function mapDiskUsage(dataJson: string): ServerDiskUsageRow[] {
	let parsed: unknown;
	try {
		parsed = JSON.parse(dataJson || '{}');
	} catch {
		return [];
	}
	const items = (parsed as {items?: unknown}).items;
	if (!Array.isArray(items)) return [];
	const text = (
		row: Record<string, unknown>,
		...keys: string[]
	): string => {
		for (const key of keys) {
			const value = row[key];
			if (typeof value === 'string' && value) return value;
			if (typeof value === 'number') return String(value);
		}
		return '—';
	};
	return items.map(raw => {
		const row = (raw ?? {}) as Record<string, unknown>;
		return {
			type: text(row, 'Type', 'type'),
			total: text(
				row,
				'Total count',
				'TotalCount',
				'total_count',
				'total',
			),
			active: text(row, 'Active', 'active'),
			size: text(row, 'Size', 'size'),
			reclaimable: text(row, 'Reclaimable', 'reclaimable'),
		};
	});
}

export function useServerContainers(
	serverId: bigint,
	options?: {enabled?: boolean},
) {
	return useConnectQuery<ServerContainer[]>(
		['global', 'containers', String(serverId)],
		async () => {
			const response = await globalClient.listContainers({serverId});
			return response.containers.map(mapContainer);
		},
		options,
	);
}

export function useServerImages(
	serverId: bigint,
	options?: {enabled?: boolean},
) {
	return useConnectQuery<ServerImage[]>(
		['global', 'images', String(serverId)],
		async () => {
			const response = await globalClient.listImages({serverId});
			return response.images.map(mapImage);
		},
		options,
	);
}

export function useServerNetworks(
	serverId: bigint,
	options?: {enabled?: boolean},
) {
	return useConnectQuery<ServerNetwork[]>(
		['global', 'networks', String(serverId)],
		async () => {
			const response = await globalClient.listNetworks({serverId});
			return response.networks.map(mapNetwork);
		},
		options,
	);
}

export function useServerVolumes(
	serverId: bigint,
	options?: {enabled?: boolean},
) {
	return useConnectQuery<ServerVolume[]>(
		['global', 'volumes', String(serverId)],
		async () => {
			const response = await globalClient.listVolumes({serverId});
			return response.volumes.map(mapVolume);
		},
		options,
	);
}

export function useServerDiskUsage(
	serverId: bigint,
	options?: {enabled?: boolean},
) {
	return useConnectQuery<ServerDiskUsageRow[]>(
		['global', 'disk-usage', String(serverId)],
		async () => {
			const response = await globalClient.diskUsage({serverId});
			return mapDiskUsage(response.dataJson);
		},
		options,
	);
}

/** Raw `docker inspect` document for one resource on this server's engine. */
export function useServerInspect(
	kind: 'container' | 'image' | 'network' | 'volume',
	serverId: bigint,
	id: string | null,
) {
	return useConnectQuery<string>(
		['global', 'inspect', kind, String(serverId), id ?? ''],
		async () => {
			const target = id ?? '';
			const response =
				kind === 'container'
					? await globalClient.inspectContainer({
							serverId,
							containerId: target,
						})
					: kind === 'image'
						? await globalClient.inspectImage({serverId, imageId: target})
						: kind === 'network'
							? await globalClient.inspectNetwork({
									serverId,
									networkId: target,
								})
							: await globalClient.inspectVolume({
									serverId,
									volumeId: target,
								});
			return response.dataJson;
		},
		{enabled: id !== null},
	);
}

export type ContainerActionKind =
	| 'start'
	| 'stop'
	| 'restart'
	| 'kill'
	| 'pause'
	| 'unpause';

const ACTION_BY_KIND: Record<ContainerActionKind, GlobalContainerAction> =
	{
		start: GlobalContainerAction.START,
		stop: GlobalContainerAction.STOP,
		restart: GlobalContainerAction.RESTART,
		kill: GlobalContainerAction.KILL,
		pause: GlobalContainerAction.PAUSE,
		unpause: GlobalContainerAction.UNPAUSE,
	};

/** Lifecycle action on one container; the reply carries the engine's stdout/stderr. */
export function useContainerActionMutation(serverId: bigint) {
	return useConnectMutation(
		async (input: {containerId: string; action: ContainerActionKind}) => {
			const response = await globalClient.containerAction({
				serverId,
				containerId: input.containerId,
				action: ACTION_BY_KIND[input.action],
			});
			return {
				stdout: response.output?.stdout ?? '',
				stderr: response.output?.stderr ?? '',
			};
		},
		{invalidate: [['global', 'containers', String(serverId)]]},
	);
}

/** Remove a container; `force` kills it first, `volumes` drops its anonymous volumes. */
export function useContainerRemoveMutation(serverId: bigint) {
	return useConnectMutation(
		async (input: {
			containerId: string;
			force: boolean;
			volumes: boolean;
		}) =>
			globalClient.removeContainer({
				serverId,
				...input,
			}),
		{invalidate: [['global', 'containers', String(serverId)]]},
	);
}

export type ServerPrunePayload = {
	containers: boolean;
	images: boolean;
	networks: boolean;
	volumes: boolean;
};

/** Prune unused engine resources on this server; one output entry per selected kind. */
export function useServerPruneMutation(serverId: bigint) {
	return useConnectMutation(
		async (input: ServerPrunePayload) => {
			const response = await globalClient.prune({serverId, ...input});
			return response.operations
				.map(operation => operation.stdout.trim())
				.filter(Boolean)
				.join('\n');
		},
		{
			invalidate: [
				['global', 'containers', String(serverId)],
				['global', 'images', String(serverId)],
				['global', 'networks', String(serverId)],
				['global', 'volumes', String(serverId)],
				['global', 'disk-usage', String(serverId)],
			],
		},
	);
}
