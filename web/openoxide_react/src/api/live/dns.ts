import {createServiceClient, useResourceListQuery} from './query';
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

const dnsClient = createServiceClient(DnsService);

/** DNS providers (legacy live: useDnsList). */
export function useDnsList() {
	return useResourceListQuery<DnsProviderPayload>(
		['dns_providers', 'list'],
		() => dnsClient.listProviders({}),
	);
}
