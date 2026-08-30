import {createServiceClient, useResourceListQuery} from './query';
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

/** Organization-wide services (legacy live: useOverviewServices). */
export function useOverviewServices(organizationId: bigint) {
	return useResourceListQuery<OverviewServiceItemPayload>(
		['overview-services', 'list', organizationId.toString()],
		() => overviewClient.listServices({organizationId}),
	);
}

/** Organization-wide domains (legacy live: useOverviewDomains). */
export function useOverviewDomains(organizationId: bigint) {
	return useResourceListQuery<OverviewDomainItemPayload>(
		['overview-domains', 'list', organizationId.toString()],
		() => overviewClient.listDomains({organizationId}),
	);
}

/** Organization-wide backups (legacy live: useOverviewBackups). */
export function useOverviewBackups(organizationId: bigint) {
	return useResourceListQuery<OverviewBackupItemPayload>(
		['overview-backups', 'list', organizationId.toString()],
		() => overviewClient.listBackups({organizationId}),
	);
}
