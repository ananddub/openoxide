import {
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
	command?: string;
	args?: string;
	watch_paths?: string;
	enable_submodules?: boolean;
	created_at: bigint;
	updated_at: bigint;
};

// ── Advanced sub-resource payload shapes ──────────────────────────

export type ApplicationPortPayload = {
	id: number;
	published_port: number;
	target_port: number;
	protocol: string;
	publish_mode: string;
	application_id: number;
};

export type ApplicationMountPayload = {
	id: number;
	mount_type: string;
	host_path?: string;
	volume_name?: string;
	file_path?: string;
	content?: string;
	mount_path: string;
	application_id: number;
};

export type ApplicationRedirectPayload = {
	id: number;
	regex: string;
	replacement: string;
	permanent: number;
	application_id: number;
};

export type ApplicationSecurityPayload = {
	id: number;
	username: string;
	has_password: boolean;
	application_id: number;
};

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

export function toApplicationPayload(
	application: Application,
): ApplicationPayload {
	// The custom-git card's provider id is 'GIT'; the proto enum name is
	// 'CUSTOM_GIT' (DB discriminator 'CUSTOM'). Collapse both to the card id.
	const sourceTypeName = enumName(
		ApplicationSourceType,
		application.sourceType,
	);
	const sourceType =
		sourceTypeName === 'CUSTOM_GIT' || sourceTypeName === 'CUSTOM'
			? 'GIT'
			: sourceTypeName;
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
		command: application.command,
		args: application.args,
		watch_paths: application.watchPaths,
		enable_submodules: application.enableSubmodules,
		created_at: application.createdAt,
		updated_at: application.updatedAt,
	};
}
