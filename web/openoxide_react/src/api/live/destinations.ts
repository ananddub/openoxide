import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
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

/** Create a backup destination (provider credentials in the payload). */
export function useDestinationCreate() {
	return useConnectMutation((input: Record<string, unknown>) =>
		destinationClient.createDestination({payload: JSON.stringify(input)}),
	);
}

/**
 * Update a backup destination. Destination ids are UUID strings the backend
 * reads from the payload JSON, so the id is merged back into the body.
 */
export function useDestinationUpdate() {
	return useConnectMutation((input: {id: string; [field: string]: unknown}) => {
		const {id, ...body} = input;
		return destinationClient.updateDestination({payload: JSON.stringify({...body, id})});
	});
}

/** Delete a backup destination by its UUID id (carried in the payload). */
export function useDestinationDelete() {
	return useConnectMutation((id: string) =>
		destinationClient.deleteDestination({payload: JSON.stringify({id})}),
	);
}

/** Test unsaved destination credentials (raw provider fields in the payload; legacy REST: POST /destinations/test). */
export function useDestinationTest() {
	return useConnectMutation((input: Record<string, unknown>) =>
		destinationClient.testDestination({payload: JSON.stringify(input)}),
	);
}
