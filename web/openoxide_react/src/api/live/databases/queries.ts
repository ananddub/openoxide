import {
	useActiveOrgId,
	useConnectQuery,
	useResourceListQuery,
} from '../query';
import type {OverviewServiceItemPayload} from '../overview';
import {
	DatabaseKind,
	type AdvancedConfig,
} from '#/gen/connect/openoxide/api/v1/database_pb';
import {
	databaseClient,
	databaseNetworkClient,
	overviewClient,
} from './shared';
import {
	toDatabasePayload,
	toMountPayload,
	toSwarmSettingsPayload,
	type DatabaseItemPayload,
	type DatabaseMountPayload,
	type DatabaseAdvancedConfigPayload,
	type DatabaseNetworkPayload,
	type DatabasePayload,
	type DatabaseSwarmSettingsPayload,
} from './types';

/**
 * Organization-wide database list (legacy app-store `databases` slice).
 * There is no org-wide Database RPC, so this derives from the overview
 * services stream and is keyed under the `databases` prefix so
 * SERVICE_DATABASE table changes invalidate it.
 */
export function useDatabaseList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<DatabaseItemPayload[]>(
		['databases', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.items
				.map(
					item => JSON.parse(item.payload) as OverviewServiceItemPayload,
				)
				.filter(service => service.service_type === 'DATABASE')
				.map(
					service =>
						({
							id: Number(service.id),
							name: service.name,
							kind: (service.db_kind ?? '').toLowerCase() || undefined,
							project_id: Number(service.project_id),
							environment_id: Number(service.environment_id),
							status: service.status,
							server_id:
								service.server_id === null
									? undefined
									: Number(service.server_id),
							server_name: service.server_name ?? undefined,
							created_at: Number(service.created_at),
							updated_at: Number(service.created_at),
						}) satisfies DatabaseItemPayload,
				);
		},
	);
}

/**
 * Single database of any kind (legacy live: per-kind getters).
 * GetDatabaseRequest requires the kind discriminator alongside the id.
 */
export function useDatabaseGet(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabasePayload>(
		['databases', 'get', kind.toString(), id.toString()],
		async () =>
			toDatabasePayload(await databaseClient.getDatabase({kind, id})),
		{enabled: id > 0n},
	);
}

// Per-kind thin wrappers preserving the legacy hook surface; each just
// resolves to useDatabaseGet with its DatabaseKind.
export function usePostgresGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.POSTGRES, id);
}

export function useMysqlGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MYSQL, id);
}

export function useMariadbGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MARIADB, id);
}

export function useMongoGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MONGO, id);
}

export function useRedisGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.REDIS, id);
}

export function useLibsqlGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.LIBSQL, id);
}

/** Swarm settings for a database (legacy REST: GET /database-management/{kind}/{id}/swarm-settings). */
export function useDatabaseSwarmSettings(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabaseSwarmSettingsPayload>(
		['databases', 'swarm-settings', kind.toString(), id.toString()],
		async () =>
			toSwarmSettingsPayload(
				await databaseClient.getSwarmSettings({kind, id}),
			),
		{enabled: id > 0n},
	);
}

/** Mounts attached to a database (legacy REST: GET /database-management/{kind}/{id}/mounts). */
export function useDatabaseMountList(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabaseMountPayload[]>(
		['databases', 'mounts', kind.toString(), id.toString()],
		async () => {
			const response = await databaseClient.listMounts({kind, id});
			return response.mounts.map(toMountPayload);
		},
		{enabled: id > 0n},
	);
}

/** Postgres advanced configuration (legacy REST: GET /database-management/postgres/{id}/advanced-config). */
export function useDatabasePostgresAdvancedConfig(id: bigint) {
	return useConnectQuery<DatabaseAdvancedConfigPayload>(
		['databases', 'postgres-config', id.toString()],
		async () => {
			const config: AdvancedConfig =
				await databaseClient.getPostgresAdvancedConfig({
					kind: DatabaseKind.POSTGRES,
					id,
				});
			return {
				settings: config.config
					? (JSON.parse(config.config) as Record<string, string>)
					: {},
				replication_role: config.replicationRole || 'STANDALONE',
			};
		},
		{enabled: id > 0n},
	);
}

/** Database networks across servers (legacy REST: GET /database-networks). */
export function useDatabaseNetworkList() {
	return useResourceListQuery<DatabaseNetworkPayload>(
		['database-networks', 'list'],
		() => databaseNetworkClient.listNetworks({}),
	);
}
