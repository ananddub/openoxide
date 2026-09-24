import {
	createServiceClient,
	useActiveOrgId,
	useConnectQuery,
} from './query';
import {
	OverviewService,
	OverviewBackupStatus,
	OverviewBackupType,
	OverviewServiceType,
	type OverviewBackupItem,
	type OverviewDomainItem,
	type OverviewServiceItem,
} from '#/gen/connect/openoxide/api/v1/overview_pb';

/** Overview payloads mirror the legacy live shape (snake_case JSON payload of the overview DTOs): `service_type` is "APP" | "COMPOSE" | "DATABASE" — the proto carries numeric enums, so the mappers below resolve them back to member names (consumers compare uppercase names or lowercase-contains). */
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
	internal_path?: string;
	custom_entrypoint?: string;
	custom_cert_resolver?: string;
	strip_path: boolean;
	middlewares: string;
	domain_type: string;
	certificate_type: string;
	domain_service_name?: string;
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

/** Enum value → member name ('APP', 'DATABASE', 'ENABLED', …). */
function enumName<E extends Record<number, string>>(
	enumeration: E,
	value: number,
): string {
	return enumeration[value] ?? '';
}

function toServiceItemPayload(
	item: OverviewServiceItem,
): OverviewServiceItemPayload {
	return {
		id: item.id,
		name: item.name,
		service_type: enumName(OverviewServiceType, item.serviceType),
		status: item.status,
		created_at: item.createdAt,
		project_id: item.projectId,
		project_name: item.projectName,
		environment_id: item.environmentId,
		environment_name: item.environmentName,
		db_kind: item.dbKind ?? null,
		server_id: item.serverId ?? null,
		server_name: item.serverName ?? null,
	};
}

function toDomainItemPayload(
	item: OverviewDomainItem,
): OverviewDomainItemPayload {
	return {
		id: item.id,
		host: item.host,
		path: item.path ?? null,
		port: item.port ?? null,
		https: item.https,
		application_id: item.applicationId ?? null,
		compose_id: item.composeId ?? null,
		service_name: item.serviceName,
		project_name: item.projectName,
		internal_path: item.internalPath,
		custom_entrypoint: item.customEntrypoint,
		custom_cert_resolver: item.customCertResolver,
		strip_path: item.stripPath,
		middlewares: item.middlewares,
		domain_type: item.domainType,
		certificate_type: item.certificateType,
		domain_service_name: item.domainServiceName,
	};
}

function toBackupItemPayload(
	item: OverviewBackupItem,
): OverviewBackupItemPayload {
	return {
		id: item.id,
		name: item.name,
		backup_type: enumName(OverviewBackupType, item.backupType),
		status: enumName(OverviewBackupStatus, item.status),
		destination: item.destination,
		created_at: item.createdAt,
	};
}

/** Organization-wide services (legacy live: useOverviewServices). Keyed under the `overview-services` prefix so the table-watch stream invalidates it on any SERVICES / SERVICE_APPLICATION / SERVICE_COMPOSE / SERVICE_DATABASE table change while mounted. */
export function useOverviewServices() {
	const orgId = useActiveOrgId();
	return useConnectQuery<OverviewServiceItemPayload[]>(
		['overview-services', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.services.map(toServiceItemPayload);
		},
	);
}

/** Organization-wide domains (legacy live: useOverviewDomains). Same invalidation story via the `overview-domains` prefix (DOMAINS table). */
export function useOverviewDomains() {
	const orgId = useActiveOrgId();
	return useConnectQuery<OverviewDomainItemPayload[]>(
		['overview-domains', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationDomains({
				organizationId: orgId,
			});
			return response.domains.map(toDomainItemPayload);
		},
	);
}

/** Organization-wide backups (legacy live: useOverviewBackups). Invalidated via the `overview-backups` prefix (BACKUPS / BACKUP_EXECUTIONS / VOLUME_BACKUPS tables). */
export function useOverviewBackups() {
	const orgId = useActiveOrgId();
	return useConnectQuery<OverviewBackupItemPayload[]>(
		['overview-backups', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationBackups({
				organizationId: orgId,
			});
			return response.backups.map(toBackupItemPayload);
		},
	);
}
