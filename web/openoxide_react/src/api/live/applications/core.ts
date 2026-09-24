import {
	ApplicationSourceType,
	ApplicationBuildType,
} from '#/gen/connect/openoxide/api/v1/application_pb';
import {applicationClient, big, idMutation, inputMutation} from './shared';

/** Create an application (legacy REST: POST /applications). Source and build types are the generated enum values (e.g. ApplicationSourceType.GITHUB). */
export const useApplicationCreate = inputMutation(
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
			serverId: big(input.server_id),
			sourceType: input.source_type ?? ApplicationSourceType.UNSPECIFIED,
			buildType: input.build_type ?? ApplicationBuildType.UNSPECIFIED,
		}),
);

/** Move an application to another environment (MoveApplication RPC). */
export const useApplicationMove = inputMutation(
	(input: {id: number | bigint; target_environment_id: number | bigint}) =>
		applicationClient.moveApplication({
			id: BigInt(input.id),
			targetEnvironmentId: BigInt(input.target_environment_id),
		}),
	// The move changes the service's environment — refresh env-scoped lists too, which the default 'applications' prefix already covers.
	[['applications'], ['environments'], ['overview']],
);

/** Update an application by id (legacy REST: PATCH /applications/{id}). */
export const useApplicationUpdate = inputMutation(
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
			serverId: big(input.server_id),
			buildServerId: big(input.build_server_id),
			registryId: big(input.registry_id),
			networkIds: input.network_ids ?? [],
			detachRustployNetwork: input.detach_rustploy_network,
		}),
);

/** Persist build-type details (dockerfile, context, publish dir, versions,
 * clean cache, autodeploy) via the dedicated UpdateBuildConfig RPC. */
export const useApplicationUpdateBuildConfig = inputMutation(
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
);

/** Persist container resource limits via the dedicated UpdateResourceConfig
 * RPC (the backend also live-applies memory/CPU to the running service). */
export const useApplicationUpdateResourceConfig = inputMutation(
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
			replicas: big(input.replicas),
		}),
);

/** Delete an application by id (legacy REST: DELETE /applications/{id}). */
export const useApplicationDelete = idMutation(id =>
	applicationClient.deleteApplication({id}),
);
