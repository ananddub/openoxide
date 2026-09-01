import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
import {readResource} from './resource';
// GitProviderService is the served surface: per-kind Create/Update RPCs plus
// list/get/test. The kind discriminator ('github' | 'gitlab' | 'gitea' |
// 'bitbucket') mirrors the legacy REST {kind} path segment.
import {GitProviderService} from '#/gen/connect/openoxide/api/v1/git_provider_pb';

/** Git provider entry (backend GitProviderDto JSON). */
export type GitProviderPayload = {
	id: number;
	provider_type: string;
	name: string;
	group_name: string;
	api_url: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
	[key: string]: unknown;
};

export type GitProviderKind = 'github' | 'gitlab' | 'gitea' | 'bitbucket';

const gitProviderClient = createServiceClient(GitProviderService);

/** All configured git providers (legacy REST: GET /git-providers). */
export function useGitProviderList() {
	return useResourceListQuery<GitProviderPayload>(
		['git-providers', 'list'],
		() => gitProviderClient.listGitProviders({}),
	);
}

/** Create a git provider of the given kind (legacy REST: POST /git-providers/{kind}). */
export function useGitProviderCreate() {
	return useConnectMutation(
		(input: {kind: GitProviderKind; [field: string]: unknown}) => {
			const {kind, ...body} = input;
			const payload = JSON.stringify(body);
			const request =
				kind === 'github'
					? gitProviderClient.createGithubProvider({payload})
					: kind === 'gitlab'
						? gitProviderClient.createGitlabProvider({payload})
						: kind === 'gitea'
							? gitProviderClient.createGiteaProvider({payload})
							: gitProviderClient.createBitbucketProvider({payload});
			return request.then(response =>
				readResource<GitProviderPayload>(response),
			);
		},
	);
}

/** Update a git provider of the given kind by id (legacy REST: PUT /git-providers/{id}/{kind}). */
export function useGitProviderUpdate() {
	return useConnectMutation(
		(input: {kind: GitProviderKind; id: number | bigint; [field: string]: unknown}) => {
			const {kind, id, ...body} = input;
			const payload = JSON.stringify(body);
			const request =
				kind === 'github'
					? gitProviderClient.updateGithubProvider({id: BigInt(id), payload})
					: kind === 'gitlab'
						? gitProviderClient.updateGitlabProvider({id: BigInt(id), payload})
						: kind === 'gitea'
							? gitProviderClient.updateGiteaProvider({id: BigInt(id), payload})
							: gitProviderClient.updateBitbucketProvider({id: BigInt(id), payload});
			return request.then(response =>
				readResource<GitProviderPayload>(response),
			);
		},
	);
}

/** Delete a git provider by id (legacy REST: DELETE /git-providers/{id}). */
export function useGitProviderDelete() {
	return useConnectMutation((id: number | bigint) =>
		gitProviderClient.deleteGitProvider({id: BigInt(id)}),
	);
}

/** Test a saved provider's credentials (legacy REST: POST /git-providers/{id}/test). */
export function useGitProviderTest() {
	return useConnectMutation((id: number | bigint) =>
		gitProviderClient.testGitProvider({id: BigInt(id)}),
	);
}
