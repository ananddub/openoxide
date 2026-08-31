import {createServiceClient, useConnectMutation, useConnectQuery} from './query';
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

/** Create a compose (legacy REST: POST /composes). */
export function useComposeCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			description?: string;
			environment_id: number | bigint;
			server_id?: number | bigint;
			compose_file?: string;
		}) =>
			composeClient.createCompose({
				name: input.name,
				description: input.description,
				environmentId: BigInt(input.environment_id),
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				composeFile: input.compose_file,
			}),
	);
}

/** Update a compose by id (legacy REST: PATCH /composes/{id}). */
export function useComposeUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			compose_file?: string;
			server_id?: number | bigint;
			network_ids?: string[];
		}) =>
			composeClient.updateCompose({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				composeFile: input.compose_file,
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				networkIds: input.network_ids ?? [],
			}),
	);
}

/** Delete a compose by id (legacy REST: DELETE /composes/{id}). */
export function useComposeDelete() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.deleteCompose({id: BigInt(id)}),
	);
}

/** Deploy a compose by id (legacy REST: POST /composes/{id}/deploy). */
export function useComposeDeploy() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.deploy({id: BigInt(id)}),
	);
}

/** Redeploy a compose by id. */
export function useComposeRedeploy() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.redeploy({id: BigInt(id)}),
	);
}

/** Reload a compose's configuration by id. */
export function useComposeReload() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.reload({id: BigInt(id)}),
	);
}

/** Start a stopped compose by id (legacy REST: POST /composes/{id}/start). */
export function useComposeStart() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.start({id: BigInt(id)}),
	);
}

/** Stop a running compose by id (legacy REST: POST /composes/{id}/stop). */
export function useComposeStop() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.stop({id: BigInt(id)}),
	);
}

/** Cancel the running deployment of a compose by id (legacy REST: POST /compose/{id}/cancel). */
export function useComposeCancel() {
	return useConnectMutation((id: number | bigint) =>
		composeClient.cancel({id: BigInt(id)}),
	);
}

/** Set a compose's source to inline raw content (legacy REST: PATCH /compose/{id}/source/raw). */
export function useComposeSetRawSource() {
	return useConnectMutation(
		(input: {id: number | bigint; content: string; file_name?: string}) =>
			composeClient.setRawSource({
				id: BigInt(input.id),
				content: input.content,
				fileName: input.file_name,
			}),
	);
}

type ComposeSourceInput = {
	id: number | bigint;
	owner: string;
	repository: string;
	branch?: string;
	build_path?: string;
	provider_id?: number | bigint;
};

function toSetSourceRequest(input: ComposeSourceInput) {
	return {
		id: BigInt(input.id),
		owner: input.owner,
		repository: input.repository,
		branch: input.branch,
		buildPath: input.build_path,
		providerId:
			input.provider_id !== undefined ? BigInt(input.provider_id) : undefined,
	};
}

/** Point a compose at a GitHub repository (legacy REST: PATCH /compose/{id}/source/github). */
export function useComposeSetGithubSource() {
	return useConnectMutation((input: ComposeSourceInput) =>
		composeClient.setGithubSource(toSetSourceRequest(input)),
	);
}

/** Point a compose at a GitLab repository (legacy REST: PATCH /compose/{id}/source/gitlab). */
export function useComposeSetGitlabSource() {
	return useConnectMutation((input: ComposeSourceInput) =>
		composeClient.setGitlabSource(toSetSourceRequest(input)),
	);
}

/** Point a compose at a Gitea repository (legacy REST: PATCH /compose/{id}/source/gitea). */
export function useComposeSetGiteaSource() {
	return useConnectMutation((input: ComposeSourceInput) =>
		composeClient.setGiteaSource(toSetSourceRequest(input)),
	);
}

/** Point a compose at a Bitbucket repository (legacy REST: PATCH /compose/{id}/source/bitbucket). */
export function useComposeSetBitbucketSource() {
	return useConnectMutation((input: ComposeSourceInput) =>
		composeClient.setBitbucketSource(toSetSourceRequest(input)),
	);
}

/**
 * Point a compose at a custom Git URL (legacy REST: PATCH /compose/{id}/source/git).
 * The backend only consumes the URL and branch from the shared source request.
 */
export function useComposeSetCustomGitSource() {
	return useConnectMutation(
		(input: {id: number | bigint; url: string; branch?: string}) =>
			composeClient.setCustomGitSource({
				id: BigInt(input.id),
				repository: input.url,
				branch: input.branch,
			}),
	);
}
