import {
	createServiceClient,
	useConnectMutation,
	useResourceListQuery,
} from './query';
import {useOverviewDomains} from './overview';
// Only DomainOperationsService is served by the backend (DomainService has
// no controller), so the live layer targets the operations variant.
import {DomainOperationsService} from '#/gen/connect/openoxide/api/v1/domain_operations_pb';

type LiveRow = Record<string, any>;

/** Legacy domain payload shape (snake_case REST DTO) with optional extras. */
export type DomainPayload = {
	id: bigint;
	host: string;
	path?: string;
	internal_path?: string;
	port?: bigint;
	https: boolean;
	strip_path: boolean;
	certificate_type: string;
	custom_cert_resolver?: string;
	custom_entrypoint?: string;
	middlewares: string;
	domain_type: string;
	application_id?: bigint;
	compose_id?: bigint;
	service_name?: string;
	project_name?: string;
	created_at: bigint;
	updated_at: bigint;
	[key: string]: unknown;
};

const domainClient = createServiceClient(DomainOperationsService);

/** Payload ids deserialize from JSON as numbers; coerce before comparing. */
function payloadIdEquals(value: unknown, expected: bigint): boolean {
	if (value === undefined || value === null) return false;
	try {
		return BigInt(value as string | number | bigint) === expected;
	} catch {
		return false;
	}
}

/**
 * Domains attached to a compose (legacy live: useDomainListByCompose).
 * ListDomains has no server-side compose filter; the attachment lives
 * inside each JSON payload, so fetch all and filter client-side on
 * `compose_id`.
 */
export function useDomainListByCompose(composeId: bigint) {
	const query = useResourceListQuery<DomainPayload>(
		['domains', 'list', 'compose', composeId.toString()],
		() => domainClient.listDomains({}),
	);
	return {
		...query,
		data: query.data?.filter(domain =>
			payloadIdEquals(domain.compose_id, composeId),
		),
	};
}

/**
 * Domains attached to an application (legacy live: useDomainListByApplication).
 * Same reason as compose: only the payload carries `application_id`, so
 * filter client-side after parsing.
 */
export function useDomainListByApplication(applicationId: bigint) {
	const query = useResourceListQuery<DomainPayload>(
		['domains', 'list', 'application', applicationId.toString()],
		() => domainClient.listDomains({}),
	);
	return {
		...query,
		data: query.data?.filter(domain =>
			payloadIdEquals(domain.application_id, applicationId),
		),
	};
}

/** Create a domain (host, route, certificate and application_id/compose_id linkage in the payload). */
export function useDomainCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			domainClient.createDomain({payload: JSON.stringify(input)}),
		{invalidate: [['domains']]},
	);
}

/** Update a domain by id (fields to change in the payload). */
export function useDomainUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return domainClient.updateDomain({
				id: BigInt(id),
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['domains']]},
	);
}

/** Delete a domain by id. */
export function useDomainDelete() {
	return useConnectMutation(
		(id: number | bigint) => domainClient.deleteDomain({id: BigInt(id)}),
		{invalidate: [['domains']]},
	);
}

/**
 * Legacy domain item shape the former zustand `domains` slice carried
 * (`domain` alias for `host`, numeric ids, uppercased certificate_type).
 */
export type DomainItemPayload = {
	id: number;
	domain: string;
	host: string;
	path: string;
	internal_path: string;
	port: number;
	https: boolean;
	strip_path: boolean;
	certificate_type: string;
	custom_cert_resolver?: string;
	custom_entrypoint?: string;
	middlewares: string;
	domain_type?: string;
	application_id?: number;
	compose_id?: number;
	service_name?: string;
	project_name?: string;
	created_at: number;
	updated_at: number;
};

function toDomainItem(d: LiveRow): DomainItemPayload {
	return {
		id: Number(d.id),
		domain: d.host,
		host: d.host,
		path: d.path || '/',
		internal_path: d.internal_path || '/',
		port: d.port || 80,
		https: !!d.https,
		strip_path: !!d.strip_path,
		certificate_type: String(d.certificate_type || 'NONE').toUpperCase(),
		custom_cert_resolver: d.custom_cert_resolver || undefined,
		custom_entrypoint: d.custom_entrypoint || undefined,
		middlewares: d.middlewares || '[]',
		domain_type: d.domain_type,
		application_id: d.application_id
			? Number(d.application_id)
			: undefined,
		compose_id: d.compose_id ? Number(d.compose_id) : undefined,
		// The domain's own compose service name (not the joined app display
		// name) — the edit dialog's service selector matches against this.
		service_name: d.domain_service_name || d.service_name,
		project_name: d.project_name,
		created_at: Number(d.created_at || 0),
		updated_at: Number(d.updated_at || 0),
	};
}

/**
 * Legacy-shaped domains for the active organization (the former zustand
 * `domains` slice data). Built over useOverviewDomains, whose key prefix
 * `['overview-domains']` is invalidated by the table-watch DOMAINS entry,
 * so mutations and backend changes refetch automatically.
 */
export function useDomainItems(organizationId?: bigint) {
	void organizationId;
	const query = useOverviewDomains();
	return {
		...query,
		data: query.data?.map(d => toDomainItem(d as unknown as LiveRow)),
	};
}
