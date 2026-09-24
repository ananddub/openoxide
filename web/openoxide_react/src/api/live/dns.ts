import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	DnsProviderType,
	DnsService,
	type DnsProvider,
	type DnsRecord,
	type DnsZone,
} from '#/gen/connect/openoxide/api/v1/dns_pb';

export type DnsProviderPayload = {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

/** DNS zone entry (backend DnsZoneDto JSON). */
export type DnsZonePayload = {
	id: string;
	name: string;
	status?: string | null;
	[key: string]: unknown;
};

/** DNS record entry (backend DnsRecordDto JSON). */
export type DnsRecordPayload = {
	id: string;
	zone_id: string;
	record_type: string;
	name: string;
	content: string;
	ttl?: number;
	proxied?: boolean;
};

const dnsClient = createServiceClient(DnsService);

/** Wire string the UI catalogs compare against (e.g. 'CLOUDFLARE'). */
function dnsProviderTypeToWire(providerType: DnsProviderType): string {
	switch (providerType) {
		case DnsProviderType.CLOUDFLARE:
			return 'CLOUDFLARE';
		case DnsProviderType.ROUTE53:
			return 'ROUTE53';
		case DnsProviderType.HETZNER:
			return 'HETZNER';
		case DnsProviderType.DIGITALOCEAN:
			return 'DIGITALOCEAN';
		case DnsProviderType.GODADDY:
			return 'GODADDY';
		default:
			return '';
	}
}

/** Accepts catalog labels case-insensitively ('cloudflare', 'ROUTE53', ...)
 * or the numeric generated enum value directly. */
function dnsProviderTypeFromWire(
	providerType: string | DnsProviderType | undefined,
): DnsProviderType {
	if (typeof providerType === 'number') return providerType;
	switch (providerType?.toUpperCase()) {
		case 'CLOUDFLARE':
			return DnsProviderType.CLOUDFLARE;
		case 'ROUTE53':
			return DnsProviderType.ROUTE53;
		case 'HETZNER':
			return DnsProviderType.HETZNER;
		case 'DIGITALOCEAN':
			return DnsProviderType.DIGITALOCEAN;
		case 'GODADDY':
			return DnsProviderType.GODADDY;
		default:
			return DnsProviderType.UNSPECIFIED;
	}
}

function toDnsProviderPayload(provider: DnsProvider): DnsProviderPayload {
	return {
		id: Number(provider.id),
		name: provider.name,
		provider_type: dnsProviderTypeToWire(provider.providerType),
		credentials_json: provider.credentialsJson,
		organization_id: Number(provider.organizationId),
		created_at: Number(provider.createdAt),
		updated_at: Number(provider.updatedAt),
	};
}

function toDnsZonePayload(zone: DnsZone): DnsZonePayload {
	return {
		id: zone.id,
		name: zone.name,
		status: zone.status,
	};
}

function toDnsRecordPayload(record: DnsRecord): DnsRecordPayload {
	return {
		id: record.id,
		zone_id: record.zoneId,
		record_type: record.recordType,
		name: record.name,
		content: record.content,
		ttl: record.ttl,
		proxied: record.proxied,
	};
}

/** DNS providers (legacy live: useDnsList). */
export function useDnsList() {
	return useConnectQuery<DnsProviderPayload[]>(
		['dns_providers', 'list'],
		async () => {
			const response = await dnsClient.listProviders({});
			return response.providers.map(toDnsProviderPayload);
		},
	);
}

/** DNS zones for a provider by id (legacy REST: GET /dns-providers/{id}/zones). */
export function useDnsZoneList(providerId: bigint) {
	return useConnectQuery<DnsZonePayload[]>(
		['dns', 'zones', providerId.toString()],
		async () => {
			const response = await dnsClient.listZones({providerId});
			return response.zones.map(toDnsZonePayload);
		},
	);
}

/** DNS records of a zone (legacy REST: GET /dns-providers/{id}/zones/{zone_id}/records). */
export function useDnsRecordList(providerId: bigint, zoneId: string) {
	return useConnectQuery<DnsRecordPayload[]>(
		['dns', 'records', providerId.toString(), zoneId],
		async () => {
			const response = await dnsClient.listRecords({
				providerId,
				zoneId,
			});
			return response.records.map(toDnsRecordPayload);
		},
		{enabled: zoneId !== ''},
	);
}

/** Create a DNS provider. Organization scoping derives from the session on the server; `organization_id` is accepted for call-site compatibility and ignored. */
export function useDnsProviderCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			provider_type: string | DnsProviderType;
			credentials_json: string;
			organization_id?: number | bigint;
			[field: string]: unknown;
		}) =>
			dnsClient.createProvider({
				name: input.name,
				providerType: dnsProviderTypeFromWire(input.provider_type),
				credentialsJson: input.credentials_json,
			}),
		{invalidate: [['dns_providers']]},
	);
}

/** Update a DNS provider by id (absent fields keep their stored value). */
export function useDnsProviderUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			provider_type?: string | DnsProviderType;
			credentials_json?: string;
			organization_id?: number | bigint;
			[field: string]: unknown;
		}) =>
			dnsClient.updateProvider({
				id: BigInt(input.id),
				name: input.name,
				providerType:
					input.provider_type !== undefined
						? dnsProviderTypeFromWire(input.provider_type)
						: undefined,
				credentialsJson: input.credentials_json,
			}),
		{invalidate: [['dns_providers']]},
	);
}

/** Delete a DNS provider by id. */
export function useDnsProviderDelete() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			dnsClient.deleteProvider({id: BigInt(input.id)}),
		{invalidate: [['dns_providers']]},
	);
}

/** Test the connection of a stored DNS provider by id. */
export function useDnsProviderTest() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			dnsClient.testProvider({id: BigInt(input.id)}),
		{invalidate: [['dns_providers']]},
	);
}

/** Test unsaved DNS credentials (provider_type/credentials_json; nothing is persisted). Extra fields (e.g. a display name) are ignored. */
export function useDnsTestConnection() {
	return useConnectMutation(
		(input: {
			provider_type: string | DnsProviderType;
			credentials_json: string;
			[field: string]: unknown;
		}) =>
			dnsClient.testConnection({
				providerType: dnsProviderTypeFromWire(input.provider_type),
				credentialsJson: input.credentials_json,
			}),
	);
}

/** Create (upsert) a DNS record on a provider. */
export function useDnsRecordCreate() {
	return useConnectMutation(
		(input: {
			provider_id: number | bigint;
			zone_id: string;
			record_type: string;
			name: string;
			content: string;
			ttl?: number;
			proxied?: boolean;
			organization_id?: number | bigint;
		}) =>
			dnsClient.createRecord({
				providerId: BigInt(input.provider_id),
				zoneId: input.zone_id,
				recordType: input.record_type,
				name: input.name,
				content: input.content,
				ttl: input.ttl,
				proxied: input.proxied,
			}),
		// Records live at the DNS provider (no CDC table), so the zone and record queries must be invalidated directly.
		{invalidate: [['dns']]},
	);
}

/** Delete a DNS record on a provider, targeted by zone + record name. */
export function useDnsRecordDelete() {
	return useConnectMutation(
		(input: {
			provider_id: number | bigint;
			zone_id: string;
			name: string;
			organization_id?: number | bigint;
		}) =>
			dnsClient.deleteRecord({
				providerId: BigInt(input.provider_id),
				zoneId: input.zone_id,
				name: input.name,
			}),
		{invalidate: [['dns']]},
	);
}
