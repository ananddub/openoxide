import {createServiceClient, useConnectMutation, useResourceListQuery, useResourceQuery} from './query';
import {SshKeyService} from '#/gen/connect/openoxide/api/v1/ssh_key_pb';

export type SshKeyPayload = {
	id: number;
	name: string;
	description?: string;
	public_key: string;
	private_key?: string;
	has_private_key: boolean;
	last_used_at?: number;
	created_at: number;
	updated_at: number;
};

const sshKeyClient = createServiceClient(SshKeyService);

/** SSH keys (legacy live: useSshKeyList). */
export function useSshKeyList() {
	return useResourceListQuery<SshKeyPayload>(
		['ssh_keys', 'list'],
		() => sshKeyClient.listKeys({}),
	);
}

/** Single SSH key (legacy live: useSshKeyGet). */
export function useSshKeyGet(id: bigint) {
	return useResourceQuery<SshKeyPayload>(
		['ssh_keys', 'get', id.toString()],
		() => sshKeyClient.getKey({id}),
	);
}

/** Create an SSH key from provided key material (name/description/public_key in the payload). */
export function useSshKeyCreate() {
	return useConnectMutation((input: {name: string; description?: string; public_key: string; private_key?: string}) =>
		sshKeyClient.createKey({payload: JSON.stringify(input)}),
	);
}

/** Generate and store a new key pair server-side (name/description/key_type in the payload). */
export function useSshKeyGenerate() {
	return useConnectMutation((input: {name: string; description?: string; key_type?: string}) =>
		sshKeyClient.generateKey({payload: JSON.stringify(input)}),
	);
}

/**
 * Generate an in-memory key pair without storing it (key_type in the
 * payload; response payload holds `{private_key, public_key}`).
 */
export function useSshKeyGeneratePair() {
	return useConnectMutation((input: {key_type?: string}) =>
		sshKeyClient.generatePair({payload: JSON.stringify(input)}),
	);
}

/** Update an SSH key by id (fields to change in the payload). */
export function useSshKeyUpdate() {
	return useConnectMutation((input: {id: number | bigint; [field: string]: unknown}) => {
		const {id, ...body} = input;
		return sshKeyClient.updateKey({id: BigInt(id), payload: JSON.stringify(body)});
	});
}

/** Delete an SSH key by id. */
export function useSshKeyDelete() {
	return useConnectMutation((id: number | bigint) =>
		sshKeyClient.deleteKey({id: BigInt(id)}),
	);
}
