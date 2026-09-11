import {createServiceClient, useActiveOrgId, useConnectMutation, useConnectQuery} from './query';
import {OverviewOperationsService} from '#/gen/connect/openoxide/api/v1/overview_operations_pb';
import type {OverviewServiceItemPayload} from './overview';
import {
	ApplicationService,
	ApplicationSourceType,
	ApplicationBuildType,
	type Application,
} from '#/gen/connect/openoxide/api/v1/application_pb';

/**
 * Org-wide application list item (all-optional fields) derived from the
 * overview services stream.
 */
export type ApplicationItemPayload = {
	id: number;
	name?: string;
	app_name?: string;
	project_id?: number;
	environment_id?: number;
	status?: string;
	server_id?: number;
	server_name?: string;
	created_at?: number;
	updated_at?: number;
};

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
	dockerfile?: string;
	docker_context_path?: string;
	docker_build_stage?: string;
	publish_directory?: string;
	is_static_spa?: boolean;
	railpack_version?: string;
	heroku_version?: string;
	build_path?: string;
	drop_build_path?: string;
	clean_cache?: boolean;
	auto_deploy?: boolean;
	memory_reservation?: string;
	memory_limit?: string;
	cpu_reservation?: string;
	cpu_limit?: string;
	replicas?: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const applicationClient = createServiceClient(ApplicationService);
const overviewClient = createServiceClient(OverviewOperationsService);

// Proto enums arrive as numbers. Consumers (source/build settings cards,
// preview tab) compare against the uppercase wire names ('DOCKER',
// 'NIXPACKS', 'GITHUB'), so reverse-map to those exact strings — the old
// lowercase form never matched and silently reset the cards to defaults.
// UNSPECIFIED collapses to '' so the cards' own fallbacks still apply.
const enumName = (
	enumObject: Record<string | number, string | number>,
	value: string | number,
): string => {
	const name = enumObject[value];
	return typeof name === 'string' && name !== 'UNSPECIFIED' ? name : '';
};


function toApplicationPayload(application: Application): ApplicationPayload {
	// The custom-git card's provider id is 'GIT'; the proto enum name is
	// 'CUSTOM_GIT' (DB discriminator 'CUSTOM'). Collapse both to the card id.
	const sourceTypeName = enumName(ApplicationSourceType, application.sourceType);
	const sourceType = sourceTypeName === 'CUSTOM_GIT' || sourceTypeName === 'CUSTOM' ? 'GIT' : sourceTypeName;
	return {
		id: application.id,
		name: application.name,
		description: application.description,
		app_name: application.appName,
		source_type: sourceType,
		build_type: enumName(ApplicationBuildType, application.buildType),
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
		dockerfile: application.dockerfile,
		docker_context_path: application.dockerContextPath,
		docker_build_stage: application.dockerBuildStage,
		publish_directory: application.publishDirectory,
		is_static_spa: application.isStaticSpa,
		railpack_version: application.railpackVersion,
		heroku_version: application.herokuVersion,
		build_path: application.buildPath,
		drop_build_path: application.dropBuildPath,
		clean_cache: application.cleanCache,
		auto_deploy: application.autoDeploy,
		memory_reservation: application.memoryReservation,
		memory_limit: application.memoryLimit,
		cpu_reservation: application.cpuReservation,
		cpu_limit: application.cpuLimit,
		replicas: application.replicas,
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

/**
 * Organization-wide application list (legacy app-store `applications`
 * slice). There is no org-wide Application RPC, so this derives from the
 * overview services stream — the same source the legacy realtime sync fed
 * the store from — and is keyed under the `applications` prefix so
 * SERVICE_APPLICATION table changes invalidate it.
 */
export function useApplicationList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<ApplicationItemPayload[]>(
		['applications', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.items
				.map((item) => JSON.parse(item.payload) as OverviewServiceItemPayload)
				.filter((service) => service.service_type === 'APP')
				.map((service) => ({
					id: Number(service.id),
					name: service.name,
					app_name: service.name,
					project_id: Number(service.project_id),
					environment_id: Number(service.environment_id),
					status: service.status,
					server_id:
						service.server_id === null ? undefined : Number(service.server_id),
					server_name: service.server_name ?? undefined,
					created_at: Number(service.created_at),
					updated_at: Number(service.created_at),
				} satisfies ApplicationItemPayload));
		},
	);
}

/**
 * Create an application (legacy REST: POST /applications). Source and build
 * types are the generated enum values (e.g. ApplicationSourceType.GITHUB).
 */
export function useApplicationCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			description?: string;
			environment_id: number | bigint;
			server_id?: number | bigint;
			source_type?: ApplicationSourceType;
			build_type?: ApplicationBuildType;
		}) =>
			applicationClient.createApplication({
				name: input.name,
				description: input.description,
				environmentId: BigInt(input.environment_id),
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				sourceType: input.source_type ?? ApplicationSourceType.UNSPECIFIED,
				buildType: input.build_type ?? ApplicationBuildType.UNSPECIFIED,
			}),
		{invalidate: [['applications']]},
	);
}

/** Update an application by id (legacy REST: PATCH /applications/{id}). */
export function useApplicationUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			build_type?: ApplicationBuildType;
			trigger_type?: string;
			env_var?: string;
			server_id?: number | bigint;
			build_server_id?: number | bigint;
			registry_id?: number | bigint;
			network_ids?: string[];
			detach_rustploy_network?: boolean;
		}) =>
			applicationClient.updateApplication({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				buildType: input.build_type,
				triggerType: input.trigger_type,
				envVar: input.env_var,
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				buildServerId: input.build_server_id !== undefined ? BigInt(input.build_server_id) : undefined,
				registryId: input.registry_id !== undefined ? BigInt(input.registry_id) : undefined,
				networkIds: input.network_ids ?? [],
				detachRustployNetwork: input.detach_rustploy_network,
			}),
		{invalidate: [['applications']]},
	);
}

/** Persist build-type details (dockerfile, context, publish dir, versions,
 * clean cache, autodeploy) via the dedicated UpdateBuildConfig RPC. */
export function useApplicationUpdateBuildConfig() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			build_args?: string;
			build_secrets?: string;
			dockerfile?: string;
			context_path?: string;
			build_stage?: string;
			publish_directory?: string;
			static_spa?: boolean;
			create_env_file?: boolean;
			command?: string;
			args?: string;
			build_path?: string;
			clean_cache?: boolean;
			enable_submodules?: boolean;
			watch_paths?: string;
			railpack_version?: string;
			heroku_version?: string;
			auto_deploy?: boolean;
		}) =>
			applicationClient.updateBuildConfig({
				id: BigInt(input.id),
				buildArgs: input.build_args,
				buildSecrets: input.build_secrets,
				dockerfile: input.dockerfile,
				contextPath: input.context_path,
				buildStage: input.build_stage,
				publishDirectory: input.publish_directory,
				staticSpa: input.static_spa,
				createEnvFile: input.create_env_file,
				command: input.command,
				args: input.args,
				buildPath: input.build_path,
				cleanCache: input.clean_cache,
				enableSubmodules: input.enable_submodules,
				watchPaths: input.watch_paths,
				railpackVersion: input.railpack_version,
				herokuVersion: input.heroku_version,
				autoDeploy: input.auto_deploy,
			}),
		{invalidate: [['applications']]},
	);
}

/** Persist container resource limits via the dedicated UpdateResourceConfig
 * RPC (the backend also live-applies memory/CPU to the running service). */
export function useApplicationUpdateResourceConfig() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			memory_reservation?: string;
			memory_limit?: string;
			cpu_reservation?: string;
			cpu_limit?: string;
			replicas?: number | bigint;
		}) =>
			applicationClient.updateResourceConfig({
				id: BigInt(input.id),
				memoryReservation: input.memory_reservation,
				memoryLimit: input.memory_limit,
				cpuReservation: input.cpu_reservation,
				cpuLimit: input.cpu_limit,
				replicas: input.replicas !== undefined ? BigInt(input.replicas) : undefined,
			}),
		{invalidate: [['applications']]},
	);
}

/** Delete an application by id (legacy REST: DELETE /applications/{id}). */
export function useApplicationDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.deleteApplication({id: BigInt(id)}),
		{invalidate: [['applications']]},
	);
}

/** Deploy an application by id (legacy REST: POST /applications/{id}/deploy). */
export function useApplicationDeploy() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.deploy({id: BigInt(id)}),
		// Deploys also insert a DEPLOYMENTS row, so refresh that list too.
		{invalidate: [['applications'], ['deployments']]},
	);
}

/** Redeploy an application by id. */
export function useApplicationRedeploy() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.redeploy({id: BigInt(id)}),
		{invalidate: [['applications'], ['deployments']]},
	);
}

/** Rebuild an application by id. */
export function useApplicationRebuild() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.rebuild({id: BigInt(id)}),
		{invalidate: [['applications'], ['deployments']]},
	);
}

/** Reload an application's configuration by id. */
export function useApplicationReload() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.reload({id: BigInt(id)}),
		{invalidate: [['applications']]},
	);
}

/** Start a stopped application by id (legacy REST: POST /applications/{id}/start). */
export function useApplicationStart() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.start({id: BigInt(id)}),
		{invalidate: [['applications']]},
	);
}

/** Stop a running application by id (legacy REST: POST /applications/{id}/stop). */
export function useApplicationStop() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.stop({id: BigInt(id)}),
		{invalidate: [['applications']]},
	);
}

/** Cancel the running deployment of an application by id. */
export function useApplicationCancel() {
	return useConnectMutation(
		(id: number | bigint) =>
			applicationClient.cancel({id: BigInt(id)}),
		{invalidate: [['applications'], ['deployments']]},
	);
}

// ---------------------------------------------------------------------------
// Source-setting mutations (legacy REST: PATCH /applications/{id}/source/*).
// Each hook takes the application id plus the provider-specific fields the
// legacy REST body carried (snake_case in, camelCase to the proto).
// ---------------------------------------------------------------------------

export type GitSourceInput = {
	id: number | bigint;
	repository: string;
	owner: string;
	branch?: string;
	build_path?: string;
	provider_id?: number | bigint;
	auto_deploy?: boolean;
	ssh_key_id?: number | bigint;
};

export type DockerSourceInput = {
	id: number | bigint;
	image: string;
	username?: string;
	password?: string;
	registry_url?: string;
};

export type CustomGitSourceInput = GitSourceInput & {url: string};

function toGitFields(input: GitSourceInput) {
	return {
		id: BigInt(input.id),
		repository: input.repository,
		owner: input.owner,
		branch: input.branch,
		buildPath: input.build_path,
		providerId: input.provider_id !== undefined ? BigInt(input.provider_id) : undefined,
		autoDeploy: input.auto_deploy,
		sshKeyId: input.ssh_key_id !== undefined ? BigInt(input.ssh_key_id) : undefined,
	};
}

/** Point an application at a Docker image. */
export function useApplicationSetDockerSource() {
	return useConnectMutation(
		(input: DockerSourceInput) =>
			applicationClient.setDockerSource({
				id: BigInt(input.id),
				image: input.image,
				username: input.username,
				password: input.password,
				registryUrl: input.registry_url,
			}),
		{invalidate: [['applications']]},
	);
}

/** Point an application at a plain Git URL. */
export function useApplicationSetCustomGitSource() {
	return useConnectMutation(
		(input: CustomGitSourceInput) =>
			applicationClient.setCustomGitSource({
				...toGitFields(input),
				url: input.url,
			}),
		{invalidate: [['applications']]},
	);
}

/** Point an application at a GitHub repository. */
export function useApplicationSetGithubSource() {
	return useConnectMutation(
		(input: GitSourceInput) =>
			applicationClient.setGithubSource(toGitFields(input)),
		{invalidate: [['applications']]},
	);
}

/** Point an application at a GitLab repository. */
export function useApplicationSetGitlabSource() {
	return useConnectMutation(
		(input: GitSourceInput) =>
			applicationClient.setGitlabSource(toGitFields(input)),
		{invalidate: [['applications']]},
	);
}

/** Point an application at a Gitea repository. */
export function useApplicationSetGiteaSource() {
	return useConnectMutation(
		(input: GitSourceInput) =>
			applicationClient.setGiteaSource(toGitFields(input)),
		{invalidate: [['applications']]},
	);
}

/** Point an application at a Bitbucket repository. */
export function useApplicationSetBitbucketSource() {
	return useConnectMutation(
		(input: GitSourceInput) =>
			applicationClient.setBitbucketSource(toGitFields(input)),
		{invalidate: [['applications']]},
	);
}

/** Switch an application to drop-upload (manual file) source. */
export function useApplicationSetDropSource() {
	return useConnectMutation(
		(input: {id: number | bigint; build_path?: string}) =>
			applicationClient.setDropSource({
				id: BigInt(input.id),
				buildPath: input.build_path,
			}),
		{invalidate: [['applications']]},
	);
}
