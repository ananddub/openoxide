import {createServiceClient, useResourceListQuery, useResourceQuery} from './query';
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
