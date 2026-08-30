import {createServiceClient, useResourceListQuery} from './query';
import {DestinationService} from '#/gen/connect/openoxide/api/v1/destination_pb';

export type DestinationPayload = {
	id: string;
	name: string;
	provider: string;
	access_key: string;
	secret_access_key: string;
	bucket: string;
	region: string;
	endpoint: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

const destinationClient = createServiceClient(DestinationService);

/** Backup destinations (legacy live: useDestinationList). */
export function useDestinationList() {
	return useResourceListQuery<DestinationPayload>(
		['destinations', 'list'],
		() => destinationClient.listDestinations({}),
	);
}
