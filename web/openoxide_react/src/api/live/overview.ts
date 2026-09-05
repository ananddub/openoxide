import {createServiceClient, useActiveOrgId, useResourceListQuery} from './query';
import {OverviewService} from '#/gen/connect/openoxide/api/v1/overview_pb';

/**
 * Overview payloads mirror the legacy live shape (snake_case JSON payload of
 * the overview DTOs): `service_type` is "APP" | "COMPOSE" | "DATABASE".
 */
export type OverviewServiceItemPayload = {
	id: bigint;
	name: string;
	service_type: string;
	status: string;
	created_at: bigint;
	project_id: bigint;
	project_name: string;
	environment_id: bigint;
	environment_name: string;
	db_kind: string | null;
	server_id: bigint | null;
	server_name: string | null;
};

export type OverviewDomainItemPayload = {
	id: bigint;
	host: string;
	path: string | null;
	port: number | null;
	https: boolean;
	application_id: bigint | null;
	compose_id: bigint | null;
	service_name: string;
	project_name: string;
};

export type OverviewBackupItemPayload = {
	id: bigint;
	name: string;
	backup_type: string;
	status: string;
	destination: string;
	created_at: bigint;
};

const overviewClient = createServiceClient(OverviewService);

/**
 * Organization-wide services (legacy live: useOverviewServices). Keyed under
 * the `overview-services` prefix so the table-watch stream invalidates it on
 * any SERVICES / SERVICE_APPLICATION / SERVICE_COMPOSE / SERVICE_DATABASE
 * table change while mounted.
 */
export function useOverviewServices() {
	const orgId = useActiveOrgId();
	return useResourceListQuery<OverviewServiceItemPayload>(
		['overview-services', orgId.toString()],
		() => overviewClient.listServices({organizationId: orgId}),
	);
}

/**
 * Organization-wide domains (legacy live: useOverviewDomains). Same
 * invalidation story via the `overview-domains` prefix (DOMAINS table).
 */
export function useOverviewDomains() {
	const orgId = useActiveOrgId();
	return useResourceListQuery<OverviewDomainItemPayload>(
		['overview-domains', orgId.toString()],
		() => overviewClient.listDomains({organizationId: orgId}),
	);
}

/**
 * Organization-wide backups (legacy live: useOverviewBackups). Invalidated
 * via the `overview-backups` prefix (BACKUPS / BACKUP_EXECUTIONS /
 * VOLUME_BACKUPS tables).
 */
export function useOverviewBackups() {
	const orgId = useActiveOrgId();
	return useResourceListQuery<OverviewBackupItemPayload>(
		['overview-backups', orgId.toString()],
		() => overviewClient.listBackups({organizationId: orgId}),
	);
}
