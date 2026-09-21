import {createServiceClient, useConnectMutation} from '../query';
import {RemoteServerService} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {ServerControllerService} from '#/gen/connect/openoxide/api/v1/server_controller_pb';

export const remoteServerClient = createServiceClient(RemoteServerService);
export const serverControllerClient = createServiceClient(
	ServerControllerService,
);

/** Builds an id-only mutation hook scoped to the `servers` cache. */
export const serverIdMutation = (
	call: (id: bigint) => Promise<unknown>,
	invalidate: string[][] = [['servers']],
) =>
	function useServerIdMutation() {
		return useConnectMutation((id: number | bigint) => call(BigInt(id)), {
			invalidate,
		});
	};
