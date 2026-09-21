import {
	useConnectMutation,
	useResourceListQuery,
	useResourceQuery,
} from '../query';
import {remoteServerClient, serverControllerClient} from './shared';
import type {RemoteServerPayload, ServerAuditPayload} from './types';

/** Remote servers (legacy live: useRemoteServerList). */
export function useRemoteServerList() {
	return useResourceListQuery<RemoteServerPayload>(
		['servers', 'list'],
		() => remoteServerClient.listRemoteServers({}),
	);
}

/** Single remote server (legacy live: useRemoteServerGet). */
export function useRemoteServerGet(id: bigint) {
	return useResourceQuery<RemoteServerPayload>(
		['servers', 'get', id.toString()],
		() => remoteServerClient.getRemoteServer({id}),
	);
}

/**
 * SSH/agent audit for a remote server (legacy live: useRemoteServerAudit).
 * ResourceRequest takes the server id; OperationResult carries the audit
 * report JSON in its (optional) payload field.
 */
export function useRemoteServerAudit() {
	return useConnectMutation(async (id: number | bigint) => {
		const response = await serverControllerClient.audit({id: BigInt(id)});
		return JSON.parse(response.payload || '{}') as ServerAuditPayload;
	});
}
