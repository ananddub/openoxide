import {createServiceClient, useConnectQuery} from './query';
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
// environments payload exposed (see EnvironmentResponseDto in types/api.d.ts).
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
