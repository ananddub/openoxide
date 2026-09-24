import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	DestinationService,
	type Destination,
} from '#/gen/connect/openoxide/api/v1/destination_pb';

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

function toDestinationPayload(
	destination: Destination,
): DestinationPayload {
	return {
		id: destination.id,
		name: destination.name,
		provider: destination.provider,
		access_key: destination.accessKey,
		secret_access_key: destination.secretAccessKey,
		bucket: destination.bucket,
		region: destination.region,
		endpoint: destination.endpoint,
		organization_id: Number(destination.organizationId),
		created_at: Number(destination.createdAt),
		updated_at: Number(destination.updatedAt),
	};
}

/** Backup destinations (legacy live: useDestinationList). */
export function useDestinationList() {
	return useConnectQuery<DestinationPayload[]>(
		['destinations', 'list'],
		async () => {
			const response = await destinationClient.listDestinations({});
			return response.destinations.map(toDestinationPayload);
		},
	);
}

/** Create a backup destination (provider credentials as typed fields). */
export function useDestinationCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			provider: string;
			access_key: string;
			secret_access_key: string;
			bucket: string;
			region: string;
			endpoint: string;
			[field: string]: unknown;
		}) =>
			destinationClient.createDestination({
				name: input.name,
				provider: input.provider,
				accessKey: input.access_key,
				secretAccessKey: input.secret_access_key,
				bucket: input.bucket,
				region: input.region,
				endpoint: input.endpoint,
			}),
		{invalidate: [['destinations']]},
	);
}

/** Update a backup destination by its UUID id (absent fields keep their
 * stored value). */
export function useDestinationUpdate() {
	return useConnectMutation(
		(input: {
			id: string;
			name?: string;
			provider?: string;
			access_key?: string;
			secret_access_key?: string;
			bucket?: string;
			region?: string;
			endpoint?: string;
			[field: string]: unknown;
		}) =>
			destinationClient.updateDestination({
				id: input.id,
				name: input.name,
				provider: input.provider,
				accessKey: input.access_key,
				secretAccessKey: input.secret_access_key,
				bucket: input.bucket,
				region: input.region,
				endpoint: input.endpoint,
			}),
		{invalidate: [['destinations']]},
	);
}

/** Delete a backup destination by its UUID id. */
export function useDestinationDelete() {
	return useConnectMutation(
		(id: string) => destinationClient.deleteDestination({id}),
		{invalidate: [['destinations']]},
	);
}

/** Test unsaved destination credentials (raw provider fields; legacy REST:
 * POST /destinations/test). */
export function useDestinationTest() {
	return useConnectMutation(
		(input: {
			provider: string;
			access_key: string;
			secret_access_key: string;
			bucket: string;
			region: string;
			endpoint: string;
			[field: string]: unknown;
		}) =>
			destinationClient.testDestination({
				provider: input.provider,
				accessKey: input.access_key,
				secretAccessKey: input.secret_access_key,
				bucket: input.bucket,
				region: input.region,
				endpoint: input.endpoint,
			}),
	);
}
