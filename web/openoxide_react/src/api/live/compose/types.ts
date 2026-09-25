import {
	ComposeMountType,
	ComposePatchType,
	type Compose,
	type ComposeMount,
	type ComposePatch,
} from '#/gen/connect/openoxide/api/v1/compose_pb';

/** Org-wide compose list item (all-optional fields) derived from the overview services stream — keyed under the `composes` prefix so SERVICE_COMPOSE table changes invalidate it. */
export type ComposeItemPayload = {
	id: number;
	name?: string;
	project_id?: number;
	environment_id?: number;
	status?: string;
	server_id?: number;
	server_name?: string;
	created_at?: number;
	updated_at?: number;
};

export type ComposePayload = {
	id: bigint;
	name: string;
	description?: string;
	environment_id: bigint;
	server_id?: bigint;
	status: string;
	compose_status: string;
	source_type?: string;
	repository?: string;
	owner?: string;
	branch?: string;
	gitlab_repository?: string;
	gitlab_owner?: string;
	gitlab_branch?: string;
	gitea_repository?: string;
	gitea_owner?: string;
	gitea_branch?: string;
	bitbucket_repository?: string;
	bitbucket_owner?: string;
	bitbucket_branch?: string;
	custom_git_url?: string;
	custom_git_branch?: string;
	command?: string;
	compose_path?: string;
	compose_type?: string;
	build_path?: string;
	custom_git_ssh_key_id?: bigint;
	last_error?: string;
	compose_file?: string;
	randomize: boolean;
	suffix?: string;
	created_at: bigint;
	updated_at: bigint;
};

export function toComposePayload(compose: Compose): ComposePayload {
	return {
		id: compose.id,
		name: compose.name,
		description: compose.description,
		environment_id: compose.environmentId,
		server_id: compose.serverId,
		status: compose.status,
		compose_status: compose.status,
		// DB stores 'CUSTOM' for custom-git; the source card's provider id is 'GIT' (same collapse the application payload mapper does).
		source_type:
			compose.sourceType === 'CUSTOM' ||
			compose.sourceType === 'CUSTOM_GIT'
				? 'GIT'
				: compose.sourceType,
		repository: compose.repository,
		owner: compose.owner,
		branch: compose.branch,
		gitlab_repository: compose.gitlabRepository,
		gitlab_owner: compose.gitlabOwner,
		gitlab_branch: compose.gitlabBranch,
		gitea_repository: compose.giteaRepository,
		gitea_owner: compose.giteaOwner,
		gitea_branch: compose.giteaBranch,
		bitbucket_repository: compose.bitbucketRepository,
		bitbucket_owner: compose.bitbucketOwner,
		bitbucket_branch: compose.bitbucketBranch,
		custom_git_url: compose.customGitUrl,
		custom_git_branch: compose.customGitBranch,
		command: compose.command,
		compose_path: compose.composePath,
		compose_type: compose.composeType,
		build_path: compose.buildPath,
		custom_git_ssh_key_id: compose.customGitSshKeyId,
		last_error: compose.lastError,
		compose_file: compose.composeFile,
		randomize: compose.randomize ?? false,
		suffix: compose.suffix,
		created_at: compose.createdAt,
		updated_at: compose.updatedAt,
	};
}

export type ComposeMountPayload = {
	id: number;
	mount_type: string;
	host_path?: string;
	volume_name?: string;
	file_path?: string;
	content?: string;
	mount_path: string;
	compose_id: number;
};

export type ComposePatchPayload = {
	id: number;
	patch_type: string;
	file_path: string;
	enabled: boolean;
	content: string;
	compose_id: number;
};

// Proto enums arrive as numbers. The mount/patch cards compare against the uppercase wire names ('VOLUME', 'BIND', 'FILE'), so reverse-map to those exact strings (same idiom as the applications payload mappers). UNSPECIFIED collapses to '' so the cards' own fallbacks still apply.
const enumName = (
	enumObject: Record<string | number, string | number>,
	value: string | number,
): string => {
	const name = enumObject[value];
	return typeof name === 'string' && name !== 'UNSPECIFIED' ? name : '';
};

export function toComposeMountPayload(
	mount: ComposeMount,
): ComposeMountPayload {
	return {
		id: Number(mount.id),
		mount_type: enumName(ComposeMountType, mount.mountType),
		host_path: mount.hostPath,
		volume_name: mount.volumeName,
		file_path: mount.filePath,
		content: mount.content,
		mount_path: mount.mountPath,
		compose_id: Number(mount.composeId),
	};
}

export function toComposePatchPayload(
	patch: ComposePatch,
): ComposePatchPayload {
	return {
		id: Number(patch.id),
		patch_type: enumName(ComposePatchType, patch.patchType),
		file_path: patch.filePath,
		enabled: patch.enabled,
		content: patch.content,
		compose_id: Number(patch.composeId),
	};
}
