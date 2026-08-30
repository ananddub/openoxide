import {createServiceClient, useConnectQuery} from './query';
import {
	ApplicationService,
	ApplicationSourceType,
	ApplicationBuildType,
	type Application,
} from '#/gen/connect/openoxide/api/v1/application_pb';

export type ApplicationPayload = {
	id: bigint;
	name: string;
	description?: string;
	app_name: string;
	source_type: string;
	build_type: string;
	status: string;
	last_error?: string;
	trigger_type: string;
	environment_id: bigint;
	server_id?: bigint;
	build_server_id?: bigint;
	registry_id?: bigint;
	network_ids: string[];
	detach_rustploy_network: boolean;
	env_var?: string;
	icon?: string;
	repository?: string;
	owner?: string;
	branch?: string;
	docker_image?: string;
	registry_url?: string;
	custom_git_url?: string;
	custom_git_branch?: string;
	preview_wildcard?: string;
	preview_port?: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const applicationClient = createServiceClient(ApplicationService);

// Proto enums arrive as numbers; legacy payloads exposed lowercase value
// names ('github', 'dockerfile', ...), so reverse-map to keep the wire shape.
function toApplicationPayload(application: Application): ApplicationPayload {
	return {
		id: application.id,
		name: application.name,
		description: application.description,
		app_name: application.appName,
		source_type: (ApplicationSourceType[application.sourceType] ?? '').toLowerCase(),
		build_type: (ApplicationBuildType[application.buildType] ?? '').toLowerCase(),
		status: application.status,
		last_error: application.lastError,
		trigger_type: application.triggerType,
		environment_id: application.environmentId,
		server_id: application.serverId,
		build_server_id: application.buildServerId,
		registry_id: application.registryId,
		network_ids: application.networkIds,
		detach_rustploy_network: application.detachRustployNetwork,
		env_var: application.envVar,
		icon: application.icon,
		repository: application.repository,
		owner: application.owner,
		branch: application.branch,
		docker_image: application.dockerImage,
		registry_url: application.registryUrl,
		custom_git_url: application.customGitUrl,
		custom_git_branch: application.customGitBranch,
		preview_wildcard: application.previewWildcard,
		preview_port: application.previewPort,
		created_at: application.createdAt,
		updated_at: application.updatedAt,
	};
}

/** Single application (legacy live: useApplicationGet). */
export function useApplicationGet(id: bigint) {
	return useConnectQuery<ApplicationPayload>(
		['applications', 'get', id.toString()],
		async () => toApplicationPayload(await applicationClient.getApplication({id})),
		{enabled: id > 0n},
	);
}

/** Applications for an environment (legacy live: useApplicationListByEnvironment). */
export function useApplicationListByEnvironment(environmentId: bigint) {
	return useConnectQuery<ApplicationPayload[]>(
		['applications', 'list', environmentId.toString()],
		async () => {
			const response = await applicationClient.listApplications({
				environmentId,
			});
			return response.applications.map(toApplicationPayload);
		},
		{enabled: environmentId > 0n},
	);
}
