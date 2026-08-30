import {createServiceClient, useConnectQuery} from './query';
import {ComposeService, type Compose} from '#/gen/connect/openoxide/api/v1/compose_pb';

export type ComposePayload = {
	id: bigint;
	name: string;
	description?: string;
	environment_id: bigint;
	server_id?: bigint;
	status: string;
	source_type?: string;
	repository?: string;
	branch?: string;
	compose_file?: string;
	created_at: bigint;
	updated_at: bigint;
};

const composeClient = createServiceClient(ComposeService);

function toComposePayload(compose: Compose): ComposePayload {
	return {
		id: compose.id,
		name: compose.name,
		description: compose.description,
		environment_id: compose.environmentId,
		server_id: compose.serverId,
		status: compose.status,
		source_type: compose.sourceType,
		repository: compose.repository,
		branch: compose.branch,
		compose_file: compose.composeFile,
		created_at: compose.createdAt,
		updated_at: compose.updatedAt,
	};
}

/** Single compose (legacy live: useComposeGet). */
export function useComposeGet(id: bigint) {
	return useConnectQuery<ComposePayload>(
		['composes', 'get', id.toString()],
		async () => toComposePayload(await composeClient.getCompose({id})),
		{enabled: id > 0n},
	);
}

/** Composes for an environment (legacy live: useComposeListByEnvironment). */
export function useComposeListByEnvironment(environmentId: bigint) {
	return useConnectQuery<ComposePayload[]>(
		['composes', 'list', environmentId.toString()],
		async () => {
			const response = await composeClient.listComposes({
				environmentId,
			});
			return response.composes.map(toComposePayload);
		},
		{enabled: environmentId > 0n},
	);
}
