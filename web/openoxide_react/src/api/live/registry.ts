import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	RegistryService,
	type Registry,
} from '#/gen/connect/openoxide/api/v1/registry_pb';

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

function toRegistryPayload(registry: Registry): RegistryPayload {
	return {
		id: registry.id.toString(),
		registry_name: registry.registryName,
		image_prefix: registry.imagePrefix,
		username: registry.username,
		password: registry.password,
		registry_url: registry.registryUrl,
		registry_type: registry.registryType,
		created_at: Number(registry.createdAt),
		updated_at: Number(registry.updatedAt),
	};
}

/** Container registries (legacy live: useRegistryList). */
export function useRegistryList() {
	return useConnectQuery<RegistryPayload[]>(
		['registries', 'list'],
		async () => {
			const response = await registryClient.listRegistries({});
			return response.registries.map(toRegistryPayload);
		},
	);
}

/** Single registry (legacy live: useRegistryGet). */
export function useRegistryGet(id: bigint) {
	return useConnectQuery<RegistryPayload>(
		['registries', 'get', id.toString()],
		async () => {
			const response = await registryClient.getRegistry({id});
			if (!response.registry) {
				throw new Error(`Registry ${id.toString()} not found`);
			}
			return toRegistryPayload(response.registry);
		},
	);
}

/** Create a container registry. `name` is accepted as a legacy alias for
 * `registry_name`; `registry_type` defaults to the 'custom' label. */
export function useRegistryCreate() {
	return useConnectMutation(
		(input: {
			registry_name?: string;
			/** Legacy alias for `registry_name`. */
			name?: string;
			image_prefix?: string;
			username: string;
			password: string;
			registry_url: string;
			registry_type?: string;
			[field: string]: unknown;
		}) =>
			registryClient.createRegistry({
				registryName: input.registry_name ?? input.name ?? '',
				imagePrefix: input.image_prefix,
				username: input.username,
				password: input.password,
				registryUrl: input.registry_url,
				registryType: input.registry_type ?? 'custom',
			}),
		{invalidate: [['registries']]},
	);
}

/** Update a registry by id (absent fields keep their stored value;
 * `image_prefix` present-but-empty clears it). */
export function useRegistryUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			registry_name?: string;
			/** Legacy alias for `registry_name`. */
			name?: string;
			image_prefix?: string;
			username?: string;
			password?: string;
			registry_url?: string;
			registry_type?: string;
			[field: string]: unknown;
		}) =>
			registryClient.updateRegistry({
				id: BigInt(input.id),
				registryName: input.registry_name ?? input.name,
				imagePrefix: input.image_prefix,
				username: input.username,
				password: input.password,
				registryUrl: input.registry_url,
				registryType: input.registry_type,
			}),
		{invalidate: [['registries']]},
	);
}

/** Delete a registry by id (the payload exposes ids as strings). */
export function useRegistryDelete() {
	return useConnectMutation(
		(id: number | bigint | string) =>
			registryClient.deleteRegistry({id: BigInt(id)}),
		{invalidate: [['registries']]},
	);
}

/** Test registry credentials. Pass a saved registry id (stored credentials are used) or raw {registry_url, username, password} (nothing persisted — legacy REST: POST /registries/test-raw). */
export function useRegistryTest() {
	return useConnectMutation(
		(
			input:
				| number
				| bigint
				| {
						registry_url: string;
						username: string;
						password: string;
				  },
		) =>
			typeof input === 'object'
				? registryClient.testRegistry({
						registryUrl: input.registry_url,
						username: input.username,
						password: input.password,
					})
				: registryClient.testRegistry({
						id: BigInt(input),
						registryUrl: '',
						username: '',
						password: '',
					}),
	);
}

/** Replace a saved registry's stored username/password. */
export function useRegistryRotateCredentials() {
	return useConnectMutation(
		(input: {id: number | bigint; username: string; password: string}) =>
			registryClient.rotateCredentials({
				registryId: BigInt(input.id),
				username: input.username,
				password: input.password,
			}),
		{invalidate: [['registries']]},
	);
}
