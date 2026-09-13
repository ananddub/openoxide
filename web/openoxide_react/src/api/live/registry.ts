import {createServiceClient, useConnectMutation, useResourceListQuery, useResourceQuery} from './query';
import {RegistryService} from '#/gen/connect/openoxide/api/v1/registry_pb';

export type RegistryPayload = {
	id: string;
	registry_name: string;
	image_prefix?: string;
	username: string;
	password: string;
	registry_url: string;
	registry_type: string;
	created_at: number;
	updated_at: number;
};

const registryClient = createServiceClient(RegistryService);

/** Container registries (legacy live: useRegistryList). */
export function useRegistryList() {
	return useResourceListQuery<RegistryPayload>(
		['registries', 'list'],
		() => registryClient.listRegistries({}),
	);
}

/** Single registry (legacy live: useRegistryGet). */
export function useRegistryGet(id: bigint) {
	return useResourceQuery<RegistryPayload>(
		['registries', 'get', id.toString()],
		() => registryClient.getRegistry({id}),
	);
}

/** Create a container registry (credentials in the payload). */
export function useRegistryCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			registryClient.createRegistry({payload: JSON.stringify(input)}),
		{invalidate: [['registries']]},
	);
}

/** Update a registry by id (fields to change in the payload). */
export function useRegistryUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return registryClient.updateRegistry({id: BigInt(id), payload: JSON.stringify(body)});
		},
		{invalidate: [['registries']]},
	);
}

/** Delete a registry by id. */
export function useRegistryDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			registryClient.deleteRegistry({id: BigInt(id)}),
		{invalidate: [['registries']]},
	);
}

/**
 * Test registry credentials. Pass a saved registry id (stored credentials
 * are used) or raw {registry_url, username, password} (nothing persisted —
 * legacy REST: POST /registries/test-raw).
 */
export function useRegistryTest() {
	return useConnectMutation(
		(
			input:
				| number
				| bigint
				| {registry_url: string; username: string; password: string},
		) =>
			registryClient.testRegistry({
				id: typeof input === 'object' ? undefined : BigInt(input),
				payload: JSON.stringify(typeof input === 'object' ? input : {}),
			}),
	);
}

export function useRegistryRotateCredentials() {
	return useConnectMutation(
		(input: {id: number | bigint; username: string; password: string}) =>
			registryClient.rotateCredentials({
				id: BigInt(input.id),
				payload: JSON.stringify({username: input.username, password: input.password}),
			}),
		{invalidate: [['registries']]},
	);
}
