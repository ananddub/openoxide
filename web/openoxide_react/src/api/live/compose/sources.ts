import {useConnectMutation} from '../query';
import {big, composeClient} from './shared';

/** Set a compose's source to inline raw content (legacy REST: PATCH /compose/{id}/source/raw). */
export function useComposeSetRawSource() {
	return useConnectMutation(
		(input: {id: number | bigint; content: string; file_name?: string}) =>
			composeClient.setRawSource({
				id: BigInt(input.id),
				content: input.content,
				fileName: input.file_name,
			}),
		{invalidate: [['composes']]},
	);
}

type ComposeSourceInput = {
	id: number | bigint;
	owner: string;
	repository: string;
	branch?: string;
	build_path?: string;
	provider_id?: number | bigint;
};

function toSetSourceRequest(input: ComposeSourceInput) {
	return {
		id: BigInt(input.id),
		owner: input.owner,
		repository: input.repository,
		branch: input.branch,
		buildPath: input.build_path,
		providerId: big(input.provider_id),
	};
}

// The four hosted-forge setters share the same request shape; only the RPC
// method differs, so build them from one factory.
const forgeSource = (
	call: (req: ReturnType<typeof toSetSourceRequest>) => Promise<unknown>,
) =>
	function useComposeForgeSource() {
		return useConnectMutation(
			(input: ComposeSourceInput) => call(toSetSourceRequest(input)),
			{invalidate: [['composes']]},
		);
	};

/** Point a compose at a GitHub repository (legacy REST: PATCH /compose/{id}/source/github). */
export const useComposeSetGithubSource = forgeSource(req =>
	composeClient.setGithubSource(req),
);

/** Point a compose at a GitLab repository (legacy REST: PATCH /compose/{id}/source/gitlab). */
export const useComposeSetGitlabSource = forgeSource(req =>
	composeClient.setGitlabSource(req),
);

/** Point a compose at a Gitea repository (legacy REST: PATCH /compose/{id}/source/gitea). */
export const useComposeSetGiteaSource = forgeSource(req =>
	composeClient.setGiteaSource(req),
);

/** Point a compose at a Bitbucket repository (legacy REST: PATCH /compose/{id}/source/bitbucket). */
export const useComposeSetBitbucketSource = forgeSource(req =>
	composeClient.setBitbucketSource(req),
);

/**
 * Point a compose at a custom Git URL (legacy REST: PATCH /compose/{id}/source/git).
 * The shared source request carries the build path and (for private repos)
 * the SSH key id alongside url/branch.
 */
export function useComposeSetCustomGitSource() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			url: string;
			branch?: string;
			build_path?: string;
			ssh_key_id?: number | bigint;
		}) =>
			composeClient.setCustomGitSource({
				id: BigInt(input.id),
				repository: input.url,
				branch: input.branch,
				buildPath: input.build_path,
				sshKeyId: big(input.ssh_key_id),
			}),
		{invalidate: [['composes']]},
	);
}
