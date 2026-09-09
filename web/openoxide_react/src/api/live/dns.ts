import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
import {DnsService} from '#/gen/connect/openoxide/api/v1/dns_pb';

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

const dnsClient = createServiceClient(DnsService);

/** DNS providers (legacy live: useDnsList). */
export function useDnsList() {
	return useResourceListQuery<DnsProviderPayload>(
		['dns_providers', 'list'],
		() => dnsClient.listProviders({}),
	);
}

/** DNS zones for a provider by id (legacy REST: GET /dns-providers/{id}/zones). */
export function useDnsZoneList(providerId: bigint) {
	return useResourceListQuery<DnsZonePayload>(
		['dns', 'zones', providerId.toString()],
		() => dnsClient.listZones({id: providerId}),
	);
}

/** DNS records of a zone (legacy REST: GET /dns-providers/{id}/zones/{zone_id}/records). */
export function useDnsRecordList(providerId: bigint, zoneId: string) {
	return useResourceListQuery<Record<string, unknown>>(
		['dns', 'records', providerId.toString(), zoneId],
		() =>
			dnsClient.listRecords({
				id: providerId,
				payload: JSON.stringify({zone_id: zoneId}),
			}),
	);
}

/**
 * Create a DNS provider. The DNS controller scopes strictly by the
 * `organization_id` on the request (no header fallback), so the input
 * carries it explicitly; provider fields ride in the payload.
 */
export function useDnsProviderCreate() {
	return useConnectMutation(
		(input: {organization_id: number | bigint; [field: string]: unknown}) => {
			const {organization_id, ...body} = input;
			return dnsClient.createProvider({
				organizationId: BigInt(organization_id),
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['dns_providers']]},
	);
}

/** Update a DNS provider by id (fields to change in the payload). */
export function useDnsProviderUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint; [field: string]: unknown}) => {
			const {id, organization_id, ...body} = input;
			return dnsClient.updateProvider({
				id: BigInt(id),
				organizationId: organization_id !== undefined ? BigInt(organization_id) : undefined,
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['dns_providers']]},
	);
}

/** Delete a DNS provider by id. */
export function useDnsProviderDelete() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			dnsClient.deleteProvider({
				id: BigInt(input.id),
				organizationId: input.organization_id !== undefined ? BigInt(input.organization_id) : undefined,
			}),
		{invalidate: [['dns_providers']]},
	);
}

/** Test the connection of a stored DNS provider by id. */
export function useDnsProviderTest() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			dnsClient.testProvider({
				id: BigInt(input.id),
				organizationId: input.organization_id !== undefined ? BigInt(input.organization_id) : undefined,
			}),
		{invalidate: [['dns_providers']]},
	);
}

/**
 * Test unsaved DNS credentials (provider_type/credentials_json in the
 * payload; nothing is persisted).
 */
export function useDnsTestConnection() {
	return useConnectMutation((input: Record<string, unknown>) =>
		dnsClient.testConnection({payload: JSON.stringify(input)}),
	);
}

/**
 * Create (upsert) a DNS record on a provider; `provider_id` targets the
 * provider and zone/record fields ride in the payload.
 */
export function useDnsRecordCreate() {
	return useConnectMutation(
		(input: {provider_id: number | bigint; organization_id?: number | bigint; [field: string]: unknown}) => {
			const {provider_id, organization_id, ...body} = input;
			return dnsClient.createRecord({
				id: BigInt(provider_id),
				organizationId: organization_id !== undefined ? BigInt(organization_id) : undefined,
				payload: JSON.stringify(body),
			});
		},
		// Records live at the DNS provider (no CDC table), so the zone and
		// record queries must be invalidated directly.
		{invalidate: [['dns']]},
	);
}

/**
 * Delete a DNS record on a provider; `provider_id` targets the provider and
 * `{zone_id, name}` ride in the payload.
 */
export function useDnsRecordDelete() {
	return useConnectMutation(
		(input: {provider_id: number | bigint; organization_id?: number | bigint; zone_id: string; name: string}) =>
			dnsClient.deleteRecord({
				id: BigInt(input.provider_id),
				organizationId: input.organization_id !== undefined ? BigInt(input.organization_id) : undefined,
				payload: JSON.stringify({zone_id: input.zone_id, name: input.name}),
			}),
		{invalidate: [['dns']]},
	);
}
