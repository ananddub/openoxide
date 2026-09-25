import {big, composeClient, idMutation, inputMutation} from './shared';

/** Create a compose (legacy REST: POST /composes). */
export const useComposeCreate = inputMutation(
	(input: {
		name: string;
		description?: string;
		environment_id: number | bigint;
		server_id?: number | bigint;
		compose_file?: string;
		/** Canonical ComposeType: 'DOCKER-COMPOSE' | 'STACK'. */
		compose_type?: string;
	}) =>
		composeClient.createCompose({
			name: input.name,
			description: input.description,
			environmentId: BigInt(input.environment_id),
			serverId: big(input.server_id),
			composeFile: input.compose_file,
			composeType: input.compose_type,
		}),
);

/** Update a compose by id (legacy REST: PATCH /composes/{id}). */
export const useComposeUpdate = inputMutation(
	(input: {
		id: number | bigint;
		name?: string;
		description?: string;
		compose_file?: string;
		server_id?: number | bigint;
		network_ids?: string[];
		command?: string;
		randomize?: boolean;
	}) =>
		composeClient.updateCompose({
			id: BigInt(input.id),
			name: input.name,
			description: input.description,
			composeFile: input.compose_file,
			serverId: big(input.server_id),
			networkIds: input.network_ids ?? [],
			command: input.command,
			randomize: input.randomize,
		}),
);

/** Delete a compose by id (legacy REST: DELETE /composes/{id}). */
export const useComposeDelete = idMutation(id =>
	composeClient.deleteCompose({id}),
);

/** Move a compose stack to another environment (MoveCompose RPC). */
export const useComposeMove = inputMutation(
	(input: {id: number | bigint; target_environment_id: number | bigint}) =>
		composeClient.moveCompose({
			id: BigInt(input.id),
			targetEnvironmentId: BigInt(input.target_environment_id),
		}),
	// The move changes the service's environment — refresh env-scoped lists too, which the default 'composes' prefix already covers.
	[['composes'], ['environments'], ['overview']],
);

/** Install a raw docker-compose template as a new RAW compose (InstallTemplate RPC); `env_var` is seeded as .env-style KEY=VALUE lines. */
export const useComposeInstallTemplate = inputMutation(
	(input: {
		template: string;
		environment_id: number | bigint;
		name?: string;
		server_id?: number | bigint;
		env_var?: string;
	}) =>
		composeClient.installTemplate({
			template: input.template,
			environmentId: BigInt(input.environment_id),
			name: input.name,
			serverId: big(input.server_id),
			envVar: input.env_var,
		}),
);
