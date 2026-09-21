import {useConnectMutation} from '../query';
import {remoteServerClient, serverIdMutation} from './shared';

/** Create a remote server (SSH connection fields in the payload). */
export function useRemoteServerCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			remoteServerClient.createRemoteServer({
				payload: JSON.stringify(input),
			}),
		{invalidate: [['servers']]},
	);
}

/** Update a remote server by id (fields to change in the payload). */
export function useRemoteServerUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return remoteServerClient.updateRemoteServer({
				id: BigInt(id),
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['servers']]},
	);
}

/** Delete a remote server by id. */
export const useRemoteServerDelete = serverIdMutation(id =>
	remoteServerClient.deleteRemoteServer({id}),
);

/** Mark a remote server active by id. */
export const useRemoteServerActivate = serverIdMutation(id =>
	remoteServerClient.activate({id}),
);

/** Mark a remote server inactive by id. */
export const useRemoteServerDeactivate = serverIdMutation(id =>
	remoteServerClient.deactivate({id}),
);

/** Refresh the stored last-test timestamp for a remote server by id. */
export const useRemoteServerTestConnection = serverIdMutation(id =>
	remoteServerClient.testConnection({id}),
);
