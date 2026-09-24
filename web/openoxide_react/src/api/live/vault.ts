import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	VaultProviderType,
	VaultService,
	type VaultProvider,
} from '#/gen/connect/openoxide/api/v1/vault_pb';

export type VaultProviderAssignment = {
	project_id: number;
	environment_ids: number[];
};

/** Assignment as accepted from form/JSON state (numbers or bigints). */
type VaultAssignmentInput = {
	project_id: number | bigint;
	environment_ids: Array<number | bigint>;
};

export type VaultProviderPayload = {
	id: number;
	name: string;
	provider_type: string;
	api_url: string;
	auth_token: string;
	namespace?: string;
	config_json?: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
	assignments: VaultProviderAssignment[];
};

const vaultClient = createServiceClient(VaultService);

/** Wire string the UI catalogs compare against (e.g. 'HASHICORP'). */
function vaultProviderTypeToWire(providerType: VaultProviderType): string {
	switch (providerType) {
		case VaultProviderType.HASHICORP:
			return 'HASHICORP';
		case VaultProviderType.INFISICAL:
			return 'INFISICAL';
		case VaultProviderType.DOPPLER:
			return 'DOPPLER';
		case VaultProviderType.AWS:
			return 'AWS';
		case VaultProviderType.AWS_PARAMETER_STORE:
			return 'AWS_PARAMETER_STORE';
		case VaultProviderType.SCALEWAY:
			return 'SCALEWAY';
		case VaultProviderType.AZURE:
			return 'AZURE';
		default:
			return '';
	}
}

/** Accepts catalog labels case-insensitively ('hashicorp', 'DOPPLER', ...)
 * or the numeric generated enum value directly. */
function vaultProviderTypeFromWire(
	providerType: string | VaultProviderType | undefined,
): VaultProviderType {
	if (typeof providerType === 'number') return providerType;
	switch (providerType?.toUpperCase()) {
		case 'HASHICORP':
			return VaultProviderType.HASHICORP;
		case 'INFISICAL':
			return VaultProviderType.INFISICAL;
		case 'DOPPLER':
			return VaultProviderType.DOPPLER;
		case 'AWS':
			return VaultProviderType.AWS;
		case 'AWS_PARAMETER_STORE':
			return VaultProviderType.AWS_PARAMETER_STORE;
		case 'SCALEWAY':
			return VaultProviderType.SCALEWAY;
		case 'AZURE':
			return VaultProviderType.AZURE;
		default:
			return VaultProviderType.UNSPECIFIED;
	}
}

function toAssignmentMessage(assignment: VaultAssignmentInput): {
	projectId: bigint;
	environmentIds: bigint[];
} {
	return {
		projectId: BigInt(assignment.project_id),
		environmentIds: assignment.environment_ids.map(BigInt),
	};
}

function toVaultProviderPayload(
	provider: VaultProvider,
): VaultProviderPayload {
	return {
		id: Number(provider.id),
		name: provider.name,
		provider_type: vaultProviderTypeToWire(provider.providerType),
		api_url: provider.apiUrl,
		auth_token: provider.authToken,
		namespace: provider.namespace,
		config_json: provider.configJson,
		organization_id: Number(provider.organizationId),
		created_at: Number(provider.createdAt),
		updated_at: Number(provider.updatedAt),
		assignments: provider.assignments.map(assignment => ({
			project_id: Number(assignment.projectId),
			environment_ids: assignment.environmentIds.map(Number),
		})),
	};
}

/** Vault providers (legacy live: useVaultList). */
export function useVaultList() {
	return useConnectQuery<VaultProviderPayload[]>(
		['vault_providers', 'list'],
		async () => {
			const response = await vaultClient.listProviders({});
			return response.providers.map(toVaultProviderPayload);
		},
	);
}

/** Create a vault provider. Organization scoping derives from the session on the server; `organization_id` is accepted for call-site compatibility and ignored. */
export function useVaultProviderCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			provider_type: string | VaultProviderType;
			api_url: string;
			auth_token: string;
			namespace?: string;
			config_json?: string;
			assignments?: VaultAssignmentInput[];
			organization_id?: number | bigint;
			[field: string]: unknown;
		}) =>
			vaultClient.createProvider({
				name: input.name,
				providerType: vaultProviderTypeFromWire(input.provider_type),
				apiUrl: input.api_url,
				authToken: input.auth_token,
				namespace: input.namespace,
				configJson: input.config_json,
				assignments: (input.assignments ?? []).map(toAssignmentMessage),
			}),
		{invalidate: [['vault_providers']]},
	);
}

/** Update a vault provider by id. Absent fields keep their stored value; `auth_token` present-but-empty clears the stored token; `assignments` must be the full replacement list (absent means "leave unchanged"). */
export function useVaultProviderUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			api_url?: string;
			auth_token?: string;
			namespace?: string;
			config_json?: string;
			assignments?: VaultAssignmentInput[];
			organization_id?: number | bigint;
			[field: string]: unknown;
		}) =>
			vaultClient.updateProvider({
				id: BigInt(input.id),
				name: input.name,
				apiUrl: input.api_url,
				authToken: input.auth_token,
				namespace: input.namespace,
				configJson: input.config_json,
				assignments:
					input.assignments !== undefined
						? {
								assignments: input.assignments.map(toAssignmentMessage),
							}
						: undefined,
			}),
		{invalidate: [['vault_providers']]},
	);
}

/** Delete a vault provider by id. */
export function useVaultProviderDelete() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			vaultClient.deleteProvider({id: BigInt(input.id)}),
		{invalidate: [['vault_providers']]},
	);
}

/** Test the connection of a stored vault provider by id. */
export function useVaultProviderTest() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			vaultClient.testProvider({id: BigInt(input.id)}),
	);
}

/** Test unsaved vault credentials (provider_type/api_url/auth_token; nothing is persisted). Extra fields (e.g. a display name) are ignored. */
export function useVaultTestConnection() {
	return useConnectMutation(
		(input: {
			provider_type: string | VaultProviderType;
			api_url: string;
			auth_token: string;
			namespace?: string;
			config_json?: string;
			[field: string]: unknown;
		}) =>
			vaultClient.testConnection({
				providerType: vaultProviderTypeFromWire(input.provider_type),
				apiUrl: input.api_url,
				authToken: input.auth_token,
				namespace: input.namespace,
				configJson: input.config_json,
			}),
	);
}

/** Secret names held by a provider. Values never reach the browser — the backend returns names only, which is what `${{vault.<name>.<secret>}}` references are built from. */
export function useVaultSecrets(providerId: number | null) {
	return useConnectQuery<string[]>(
		['vault_providers', 'secrets', providerId ?? 0],
		async () => {
			const response = await vaultClient.listSecrets({
				providerId: BigInt(providerId ?? 0),
			});
			return response.secrets;
		},
		{enabled: providerId !== null},
	);
}
