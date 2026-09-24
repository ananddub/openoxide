import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
// GitProviderService is the served surface: per-kind Create/Update RPCs plus list/get/test. The kind discriminator ('github' | 'gitlab' | 'gitea' | 'bitbucket') mirrors the legacy REST {kind} path segment.
import {
	GitProviderService,
	GitProviderType,
	type GitProvider,
	type GitProviderConfig,
	type ListGitProvidersResponse,
	type CreateGithubProviderResponse,
	type CreateGitlabProviderResponse,
	type CreateGiteaProviderResponse,
	type CreateBitbucketProviderResponse,
	type UpdateGithubProviderResponse,
	type UpdateGitlabProviderResponse,
	type UpdateGiteaProviderResponse,
	type UpdateBitbucketProviderResponse,
} from '#/gen/connect/openoxide/api/v1/git_provider_pb';

/** Git provider entry (projection of the typed GitProvider message). */
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

/** Shared create/update body. `kind` picks the per-kind RPC; the remaining fields mirror the snake_case names the dialog form already produces (`buildGitProviderPayload`). Absent secret fields keep the stored value on update; omitted `shared` means the server default (`true`). */
export type GitProviderInput = {
	kind: GitProviderKind;
	name: string;
	shared?: boolean;
	url?: string;
	internal_url?: string;
	app_name?: string;
	app_id?: number | bigint;
	client_id?: string;
	client_secret?: string;
	installation_id?: string;
	private_key?: string;
	application_id?: string;
	redirect_uri?: string;
	secret?: string;
	access_token?: string;
	refresh_token?: string;
	group_name?: string;
	scopes?: string;
	username?: string;
	email?: string;
	app_password?: string;
	api_token?: string;
	workspace?: string;
};

export type GitProviderUpdateInput = GitProviderInput & {
	id: number | bigint;
};

const gitProviderClient = createServiceClient(GitProviderService);

function kindFromEnum(providerType: GitProviderType): string {
	switch (providerType) {
		case GitProviderType.GITHUB:
			return 'github';
		case GitProviderType.GITLAB:
			return 'gitlab';
		case GitProviderType.GITEA:
			return 'gitea';
		case GitProviderType.BITBUCKET:
			return 'bitbucket';
		default:
			return '';
	}
}

// Secrets never come back from the server; the `has_*` presence flags are projected with them so the edit dialog can show "stored" indicators.
function toConfigPayload(config: GitProviderConfig | undefined) {
	return {
		url: config?.url ?? '',
		internal_url: config?.internalUrl ?? '',
		app_name: config?.appName ?? '',
		app_id: config?.appId === undefined ? '' : String(config.appId),
		client_id: config?.clientId ?? '',
		installation_id: config?.installationId ?? '',
		application_id: config?.applicationId ?? '',
		redirect_uri: config?.redirectUri ?? '',
		group_name: config?.groupName ?? '',
		scopes: config?.scopes ?? '',
		username: config?.username ?? '',
		email: config?.email ?? '',
		workspace: config?.workspace ?? '',
		has_client_secret: config?.hasClientSecret ?? false,
		has_private_key: config?.hasPrivateKey ?? false,
		has_access_token: config?.hasAccessToken ?? false,
		has_app_password: config?.hasAppPassword ?? false,
		has_api_token: config?.hasApiToken ?? false,
	};
}

function toGitProviderPayload(provider: GitProvider): GitProviderPayload {
	return {
		id: Number(provider.id),
		provider_type: kindFromEnum(provider.providerType),
		name: provider.name,
		group_name: provider.config?.groupName ?? '',
		api_url: provider.config?.url ?? '',
		organization_id: 0,
		created_at: Number(provider.createdAt),
		updated_at: Number(provider.updatedAt),
		shared: provider.shared,
		configured: provider.configured,
		webhook_configured: provider.webhookConfigured,
		config: toConfigPayload(provider.config),
	};
}

/** Input → typed per-kind create/update request (undefined fields drop out). */
function toGithubRequest(input: GitProviderInput) {
	return {
		name: input.name,
		shared: input.shared,
		appName: input.app_name,
		appId: input.app_id === undefined ? undefined : BigInt(input.app_id),
		clientId: input.client_id,
		clientSecret: input.client_secret,
		installationId: input.installation_id,
		privateKey: input.private_key,
	};
}

function toGitlabRequest(input: GitProviderInput) {
	return {
		name: input.name,
		shared: input.shared,
		url: input.url,
		internalUrl: input.internal_url,
		applicationId: input.application_id,
		redirectUri: input.redirect_uri,
		secret: input.secret,
		accessToken: input.access_token,
		refreshToken: input.refresh_token,
		groupName: input.group_name,
	};
}

function toGiteaRequest(input: GitProviderInput) {
	return {
		name: input.name,
		shared: input.shared,
		url: input.url,
		internalUrl: input.internal_url,
		redirectUri: input.redirect_uri,
		clientId: input.client_id,
		clientSecret: input.client_secret,
		accessToken: input.access_token,
		refreshToken: input.refresh_token,
		scopes: input.scopes,
	};
}

function toBitbucketRequest(input: GitProviderInput) {
	return {
		name: input.name,
		shared: input.shared,
		username: input.username,
		email: input.email,
		appPassword: input.app_password,
		apiToken: input.api_token,
		workspace: input.workspace,
	};
}

function fromCreateResponse(
	response:
		| CreateGithubProviderResponse
		| CreateGitlabProviderResponse
		| CreateGiteaProviderResponse
		| CreateBitbucketProviderResponse,
): GitProviderPayload {
	if (!response.provider) {
		throw new Error('Git provider response contained no provider');
	}
	const payload = toGitProviderPayload(response.provider);
	// The generated message uses camelCase; the legacy payload key was snake_case, so expose both spellings for consumers of the create form.
	return {...payload, webhook_secret: response.webhookSecret};
}

function fromUpdateResponse(
	response:
		| UpdateGithubProviderResponse
		| UpdateGitlabProviderResponse
		| UpdateGiteaProviderResponse
		| UpdateBitbucketProviderResponse,
): GitProviderPayload {
	if (!response.provider) {
		throw new Error('Git provider response contained no provider');
	}
	return toGitProviderPayload(response.provider);
}

/** All configured git providers (legacy REST: GET /git-providers). */
export function useGitProviderList() {
	return useConnectQuery<GitProviderPayload[]>(
		['git-providers', 'list'],
		async () => {
			const response: ListGitProvidersResponse =
				await gitProviderClient.listGitProviders({});
			return response.providers.map(toGitProviderPayload);
		},
	);
}

/** Create a git provider of the given kind (legacy REST: POST /git-providers/{kind}). */
export function useGitProviderCreate() {
	return useConnectMutation(
		async (input: GitProviderInput): Promise<GitProviderPayload> => {
			const {kind} = input;
			if (kind === 'github') {
				return fromCreateResponse(
					await gitProviderClient.createGithubProvider(
						toGithubRequest(input),
					),
				);
			}
			if (kind === 'gitlab') {
				return fromCreateResponse(
					await gitProviderClient.createGitlabProvider(
						toGitlabRequest(input),
					),
				);
			}
			if (kind === 'gitea') {
				return fromCreateResponse(
					await gitProviderClient.createGiteaProvider(
						toGiteaRequest(input),
					),
				);
			}
			return fromCreateResponse(
				await gitProviderClient.createBitbucketProvider(
					toBitbucketRequest(input),
				),
			);
		},
		{invalidate: [['git-providers']]},
	);
}

/** Update a git provider of the given kind by id (legacy REST: PUT /git-providers/{id}/{kind}). */
export function useGitProviderUpdate() {
	return useConnectMutation(
		async (input: GitProviderUpdateInput): Promise<GitProviderPayload> => {
			const {kind, id} = input;
			if (kind === 'github') {
				return fromUpdateResponse(
					await gitProviderClient.updateGithubProvider({
						id: BigInt(id),
						...toGithubRequest(input),
					}),
				);
			}
			if (kind === 'gitlab') {
				return fromUpdateResponse(
					await gitProviderClient.updateGitlabProvider({
						id: BigInt(id),
						...toGitlabRequest(input),
					}),
				);
			}
			if (kind === 'gitea') {
				return fromUpdateResponse(
					await gitProviderClient.updateGiteaProvider({
						id: BigInt(id),
						...toGiteaRequest(input),
					}),
				);
			}
			return fromUpdateResponse(
				await gitProviderClient.updateBitbucketProvider({
					id: BigInt(id),
					...toBitbucketRequest(input),
				}),
			);
		},
		{invalidate: [['git-providers']]},
	);
}

/** Delete a git provider by id (legacy REST: DELETE /git-providers/{id}). */
export function useGitProviderDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			gitProviderClient.deleteGitProvider({id: BigInt(id)}),
		{invalidate: [['git-providers']]},
	);
}

/** Test a saved provider's credentials (legacy REST: POST /git-providers/{id}/test). */
export function useGitProviderTest() {
	return useConnectMutation(
		(id: number | bigint) =>
			gitProviderClient.testGitProvider({id: BigInt(id)}),
		{invalidate: [['git-providers']]},
	);
}
