import {createServiceClient, useConnectMutation, useConnectQuery, useResourceListQuery} from './query';
import {ProjectService} from '#/gen/connect/openoxide/api/v1/project_pb';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
// `User` is declared in common.proto, so it is exported from common_pb
// (auth_pb only imports it for its own message fields).
import type {User} from '#/gen/connect/openoxide/api/v1/common_pb';
import type {Project} from '#/gen/connect/openoxide/api/v1/project_pb';

export type ProjectPayload = {
	id: bigint;
	name: string;
	description?: string;
	env_var: string;
	organization_id: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const projectClient = createServiceClient(ProjectService);
const authClient = createServiceClient(AuthService);

/** Projects for an organization (legacy live: useProjectListByOrganization). */
export function useProjectListByOrganization(organizationId: bigint) {
	return useConnectQuery<ProjectPayload[]>(
		['projects', 'list', organizationId.toString()],
		async () => {
			const response = await projectClient.listProjects({
				organizationId,
			});
			return response.projects.map(
				(project: Project): ProjectPayload => ({
					id: project.id,
					name: project.name,
					description: project.description,
					env_var: project.environmentVariables,
					organization_id: project.organizationId,
					created_at: project.createdAt,
					updated_at: project.updatedAt,
				}),
			);
		},
		{enabled: organizationId > 0n},
	);
}

/** Single project (legacy live: useProjectGet). */
export function useProjectGet(id: bigint) {
	return useConnectQuery<ProjectPayload>(
		['projects', 'get', id.toString()],
		async () => {
			const project = await projectClient.getProject({id});
			return {
				id: project.id,
				name: project.name,
				description: project.description,
				env_var: project.environmentVariables,
				organization_id: project.organizationId,
				created_at: project.createdAt,
				updated_at: project.updatedAt,
			};
		},
		{enabled: id > 0n},
	);
}

/** Create a project in an organization (legacy REST: POST /projects). */
export function useProjectCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			description?: string;
			environment_variables?: string;
			organization_id: number | bigint;
		}) =>
			projectClient.createProject({
				name: input.name,
				description: input.description,
				environmentVariables: input.environment_variables ?? '',
				organizationId: BigInt(input.organization_id),
			}),
	);
}

/** Update a project by id (legacy REST: PATCH /projects/{id}). */
export function useProjectUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			environment_variables?: string;
		}) =>
			projectClient.updateProject({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				environmentVariables: input.environment_variables,
			}),
	);
}

/** Delete a project by id (legacy REST: DELETE /projects/{id}). */
export function useProjectDelete() {
	return useConnectMutation((id: number | bigint) =>
		projectClient.deleteProject({id: BigInt(id)}),
	);
}

export type WhoAmIPayload = {
	user_id: bigint;
	// The proto marks email optional, so it may be absent on the wire.
	email?: string;
	first_name?: string;
	last_name?: string;
	avatar?: string;
};

/** Current user (legacy live: useAuthWhoAmI). */
export function useAuthWhoAmI() {
	return useConnectQuery<WhoAmIPayload>(['auth', 'whoami'], async () => {
		// WhoAmI returns the common `User` message; its id field is `id`.
		const user: User = await authClient.whoAmI({});
		return {
			user_id: user.id,
			email: user.email,
			first_name: user.firstName,
			last_name: user.lastName,
			avatar: user.avatar,
		};
	});
}

// Re-export so agents building sibling modules share the same helpers.
export {useResourceListQuery, useConnectQuery};
