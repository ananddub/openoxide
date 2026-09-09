import {createServiceClient, useConnectMutation, useConnectQuery} from './query';
import {EnvironmentService, type Environment} from '#/gen/connect/openoxide/api/v1/environment_pb';

export type EnvironmentPayload = {
	id: bigint;
	name: string;
	description?: string;
	env_var: string;
	is_default: boolean;
	project_id: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const environmentClient = createServiceClient(EnvironmentService);

// `environmentVariables` maps to `env_var`, the field name the legacy
// environments payload exposed by the Environment proto message.
function toEnvironmentPayload(environment: Environment): EnvironmentPayload {
	return {
		id: environment.id,
		name: environment.name,
		description: environment.description,
		env_var: environment.environmentVariables,
		is_default: environment.isDefault,
		project_id: environment.projectId,
		created_at: environment.createdAt,
		updated_at: environment.updatedAt,
	};
}

/** Single environment (legacy live: useEnvironmentGet). */
export function useEnvironmentGet(id: bigint) {
	return useConnectQuery<EnvironmentPayload>(
		['environments', 'get', id.toString()],
		async () => toEnvironmentPayload(await environmentClient.getEnvironment({id})),
		{enabled: id > 0n},
	);
}

/** Environments for a project (legacy live: useEnvironmentListByProject). */
export function useEnvironmentListByProject(projectId: bigint) {
	return useConnectQuery<EnvironmentPayload[]>(
		['environments', 'list', projectId.toString()],
		async () => {
			const response = await environmentClient.listEnvironments({
				projectId,
			});
			return response.environments.map(toEnvironmentPayload);
		},
		{enabled: projectId > 0n},
	);
}

/** Create an environment in a project (legacy REST: POST /environments). */
export function useEnvironmentCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			description?: string;
			environment_variables?: string;
			is_default?: boolean;
			project_id: number | bigint;
		}) =>
			environmentClient.createEnvironment({
				name: input.name,
				description: input.description,
				environmentVariables: input.environment_variables ?? '',
				isDefault: input.is_default ?? false,
				projectId: BigInt(input.project_id),
			}),
		{invalidate: [['environments']]},
	);
}

/** Update an environment by id (legacy REST: PATCH /environments/{id}). */
export function useEnvironmentUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			environment_variables?: string;
			is_default?: boolean;
		}) =>
			environmentClient.updateEnvironment({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				environmentVariables: input.environment_variables,
				isDefault: input.is_default,
			}),
		{invalidate: [['environments']]},
	);
}

/** Mark an environment as its project's default by id. */
export function useEnvironmentSetDefault() {
	return useConnectMutation(
		(id: number | bigint) =>
			environmentClient.setDefaultEnvironment({id: BigInt(id)}),
		{invalidate: [['environments']]},
	);
}

/** Delete an environment by id (legacy REST: DELETE /environments/{id}). */
export function useEnvironmentDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			environmentClient.deleteEnvironment({id: BigInt(id)}),
		{invalidate: [['environments']]},
	);
}
