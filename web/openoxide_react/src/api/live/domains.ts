import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {useOverviewDomains} from './overview';
import {
	DomainCertificateType,
	DomainService,
	DomainType,
	type Domain,
} from '#/gen/connect/openoxide/api/v1/domain_pb';

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

/** Create/update request fields. Keeps the legacy snake_case form-data shape (see `DomainFormData` in the domain dialog) so call sites stay untouched; the mapper converts to typed request fields. */
export type DomainInput = {
	id?: number | bigint | string;
	host?: string;
	path?: string;
	internal_path?: string;
	port?: number | string | bigint;
	https?: boolean;
	strip_path?: boolean;
	/** "LETSENCRYPT" | "CUSTOM" | "NONE" (case-insensitive). */
	certificate_type?: string;
	custom_cert_resolver?: string;
	custom_entrypoint?: string;
	middlewares?: string;
	service_name?: string;
	/** "APPLICATION" | "COMPOSE" (case-insensitive). */
	domain_type?: string;
	application_id?: number | string | bigint;
	compose_id?: number | string | bigint;
	[key: string]: unknown;
};

const domainClient = createServiceClient(DomainService);

/** Empty strings from the legacy JSON payloads mean "unset". */
function optionalString(value: unknown): string | undefined {
	if (value === undefined || value === null) return undefined;
	const text = String(value);
	return text === '' ? undefined : text;
}

function optionalBigInt(value: unknown): bigint | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	try {
		return BigInt(value as number | string | bigint);
	} catch {
		return undefined;
	}
}

function certificateTypeName(type: DomainCertificateType): string {
	switch (type) {
		case DomainCertificateType.LETSENCRYPT:
			return 'LETSENCRYPT';
		case DomainCertificateType.CUSTOM:
			return 'CUSTOM';
		case DomainCertificateType.NONE:
		case DomainCertificateType.UNSPECIFIED:
		default:
			return 'NONE';
	}
}

function parseCertificateType(
	value: unknown,
): DomainCertificateType | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	switch (String(value).toUpperCase()) {
		case 'LETSENCRYPT':
		case 'ACME':
			return DomainCertificateType.LETSENCRYPT;
		case 'CUSTOM':
			return DomainCertificateType.CUSTOM;
		case 'NONE':
			return DomainCertificateType.NONE;
		default:
			return undefined;
	}
}

function domainTypeName(type: DomainType): string {
	switch (type) {
		case DomainType.APPLICATION:
			return 'APPLICATION';
		case DomainType.COMPOSE:
			return 'COMPOSE';
		default:
			return 'UNSPECIFIED';
	}
}

function parseDomainType(value: unknown): DomainType | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	switch (String(value).toUpperCase()) {
		case 'APPLICATION':
			return DomainType.APPLICATION;
		case 'COMPOSE':
			return DomainType.COMPOSE;
		default:
			return undefined;
	}
}

/** Typed `Domain` → legacy snake_case payload. The proto message carries a single `service_id`; the application/compose split is derived from `domain_type` so payload consumers keep seeing `application_id` / `compose_id`. */
function toDomainPayload(domain: Domain): DomainPayload {
	return {
		id: domain.id,
		host: domain.host,
		path: domain.path,
		internal_path: domain.internalPath,
		port: domain.port,
		https: domain.https,
		strip_path: domain.stripPath,
		certificate_type: certificateTypeName(domain.certificateType),
		custom_cert_resolver: domain.customCertResolver,
		custom_entrypoint: domain.customEntrypoint,
		middlewares: domain.middlewares,
		domain_type: domainTypeName(domain.domainType),
		application_id:
			domain.domainType === DomainType.APPLICATION
				? domain.serviceId
				: undefined,
		compose_id:
			domain.domainType === DomainType.COMPOSE
				? domain.serviceId
				: undefined,
		service_name: domain.serviceName,
		created_at: domain.createdAt,
		updated_at: domain.updatedAt,
	};
}

/** Domains attached to a compose (legacy live: useDomainListByCompose). */
export function useDomainListByCompose(composeId: bigint) {
	return useConnectQuery<DomainPayload[]>(
		['domains', 'list', 'compose', composeId.toString()],
		async () => {
			const response = await domainClient.listComposeDomains({composeId});
			return response.domains.map(toDomainPayload);
		},
		{enabled: composeId > 0n},
	);
}

/** Domains attached to an application (legacy live: useDomainListByApplication). */
export function useDomainListByApplication(applicationId: bigint) {
	return useConnectQuery<DomainPayload[]>(
		['domains', 'list', 'application', applicationId.toString()],
		async () => {
			const response = await domainClient.listApplicationDomains({
				applicationId,
			});
			return response.domains.map(toDomainPayload);
		},
		{enabled: applicationId > 0n},
	);
}

/** Create a domain (host, route, certificate and application_id/compose_id linkage). */
export function useDomainCreate() {
	return useConnectMutation(
		(input: DomainInput) =>
			domainClient.createDomain({
				host: String(input.host ?? ''),
				https: Boolean(input.https),
				port: optionalBigInt(input.port),
				path: optionalString(input.path),
				internalPath: optionalString(input.internal_path),
				customEntrypoint: optionalString(input.custom_entrypoint),
				serviceName: optionalString(input.service_name),
				customCertResolver: optionalString(input.custom_cert_resolver),
				stripPath: Boolean(input.strip_path),
				middlewares: optionalString(input.middlewares),
				domainType: parseDomainType(input.domain_type),
				certificateType: parseCertificateType(input.certificate_type),
				applicationId: optionalBigInt(input.application_id),
				composeId: optionalBigInt(input.compose_id),
			}),
		{invalidate: [['domains']]},
	);
}

/** Update a domain by id (only the patchable fields are sent). */
export function useDomainUpdate() {
	return useConnectMutation(
		(input: DomainInput & {id: number | bigint}) => {
			const {id, ...body} = input;
			return domainClient.updateDomain({
				id: BigInt(id),
				host: optionalString(body.host),
				https: body.https === undefined ? undefined : Boolean(body.https),
				port: optionalBigInt(body.port),
				path: optionalString(body.path),
				internalPath: optionalString(body.internal_path),
				customEntrypoint: optionalString(body.custom_entrypoint),
				serviceName: optionalString(body.service_name),
				customCertResolver: optionalString(body.custom_cert_resolver),
				stripPath:
					body.strip_path === undefined
						? undefined
						: Boolean(body.strip_path),
				middlewares: optionalString(body.middlewares),
				certificateType: parseCertificateType(body.certificate_type),
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

export type DnsValidationResult = {
	isValid: boolean;
	resolvedIps: string[];
	expectedIps: string[];
	error?: string;
};

/** Resolve a domain's DNS and compare against the server it should point to ("Validate DNS"). */
export function useValidateDomainDns() {
	return useConnectMutation(
		(input: {
			host: string;
			serverId?: number | bigint | string;
		}): Promise<DnsValidationResult> => {
			const serverId = optionalBigInt(input.serverId);
			return domainClient
				.validateDomainDns({host: input.host, serverId})
				.then(r => ({
					isValid: r.isValid,
					resolvedIps: r.resolvedIps,
					expectedIps: r.expectedIps,
					error: r.error,
				}));
		},
		{invalidate: []},
	);
}

/** Legacy domain item shape the former zustand `domains` slice carried (`domain` alias for `host`, numeric ids, uppercased certificate_type). */
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
		// The domain's own compose service name (not the joined app display name) — the edit dialog's service selector matches against this.
		service_name: d.domain_service_name || d.service_name,
		project_name: d.project_name,
		created_at: Number(d.created_at || 0),
		updated_at: Number(d.updated_at || 0),
	};
}

/** Legacy-shaped domains for the active organization (the former zustand `domains` slice data). Built over useOverviewDomains, whose key prefix `['overview-domains']` is invalidated by the table-watch DOMAINS entry, so mutations and backend changes refetch automatically. The overview item keeps `project_name` and `domain_service_name`, which the plain Domain message does not carry. */
export function useDomainItems(organizationId?: bigint) {
	void organizationId;
	const query = useOverviewDomains();
	return {
		...query,
		data: query.data?.map(d => toDomainItem(d as unknown as LiveRow)),
	};
}
