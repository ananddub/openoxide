import {createServiceClient, useResourceListQuery, useResourceQuery} from './query';
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
