import {createServiceClient, useResourceListQuery} from './query';
import {DomainService} from '#/gen/connect/openoxide/api/v1/domain_pb';

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

const domainClient = createServiceClient(DomainService);

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
		data: query.data?.filter((domain) => payloadIdEquals(domain.compose_id, composeId)),
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
		data: query.data?.filter((domain) => payloadIdEquals(domain.application_id, applicationId)),
	};
}
