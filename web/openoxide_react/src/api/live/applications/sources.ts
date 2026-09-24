// --------------------------------------------------------------------------- Source-setting mutations (legacy REST: PATCH /applications/{id}/source/*). Each hook takes the application id plus the provider-specific fields the legacy REST body carried (snake_case in, camelCase to the proto). ---------------------------------------------------------------------------

import {applicationClient, big, inputMutation} from './shared';

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
		providerId: big(input.provider_id),
		autoDeploy: input.auto_deploy,
		sshKeyId: big(input.ssh_key_id),
	};
}

/** Point an application at a Docker image. */
export const useApplicationSetDockerSource = inputMutation(
	(input: DockerSourceInput) =>
		applicationClient.setDockerSource({
			id: BigInt(input.id),
			image: input.image,
			username: input.username,
			password: input.password,
			registryUrl: input.registry_url,
		}),
);

/** Point an application at a plain Git URL. */
export const useApplicationSetCustomGitSource = inputMutation(
	(input: CustomGitSourceInput) =>
		applicationClient.setCustomGitSource({
			...toGitFields(input),
			url: input.url,
		}),
);

/** GitHub / GitLab / Gitea / Bitbucket repository sources share one shape. */
const gitSource = (
	call: (fields: ReturnType<typeof toGitFields>) => Promise<unknown>,
) => inputMutation((input: GitSourceInput) => call(toGitFields(input)));

export const useApplicationSetGithubSource = gitSource(fields =>
	applicationClient.setGithubSource(fields),
);

export const useApplicationSetGitlabSource = gitSource(fields =>
	applicationClient.setGitlabSource(fields),
);

export const useApplicationSetGiteaSource = gitSource(fields =>
	applicationClient.setGiteaSource(fields),
);

export const useApplicationSetBitbucketSource = gitSource(fields =>
	applicationClient.setBitbucketSource(fields),
);

/** Switch an application to drop-upload (manual file) source. */
export const useApplicationSetDropSource = inputMutation(
	(input: {id: number | bigint; build_path?: string}) =>
		applicationClient.setDropSource({
			id: BigInt(input.id),
			buildPath: input.build_path,
		}),
);
