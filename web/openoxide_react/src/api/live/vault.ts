import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
	useResourceListQuery,
} from './query';
import {VaultService} from '#/gen/connect/openoxide/api/v1/vault_pb';

export type VaultProviderAssignment = {
	project_id: number;
	environment_ids: number[];
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

/** Vault providers (legacy live: useVaultList). */
export function useVaultList() {
	return useResourceListQuery<VaultProviderPayload>(
		['vault_providers', 'list'],
		() => vaultClient.listProviders({}),
	);
}

/**
 * Create a vault provider. The vault controller scopes strictly by the
 * `organization_id` on the request (no header fallback), so the input
 * carries it explicitly; provider fields ride in the payload.
 */
export function useVaultProviderCreate() {
	return useConnectMutation(
		(input: {
			organization_id: number | bigint;
			[field: string]: unknown;
		}) => {
			const {organization_id, ...body} = input;
			return vaultClient.createProvider({
				organizationId: BigInt(organization_id),
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['vault_providers']]},
	);
}

/** Update a vault provider by id (fields to change in the payload). */
export function useVaultProviderUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			organization_id?: number | bigint;
			[field: string]: unknown;
		}) => {
			const {id, organization_id, ...body} = input;
			return vaultClient.updateProvider({
				id: BigInt(id),
				organizationId:
					organization_id !== undefined
						? BigInt(organization_id)
						: undefined,
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['vault_providers']]},
	);
}

/** Delete a vault provider by id. */
export function useVaultProviderDelete() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			vaultClient.deleteProvider({
				id: BigInt(input.id),
				organizationId:
					input.organization_id !== undefined
						? BigInt(input.organization_id)
						: undefined,
			}),
		{invalidate: [['vault_providers']]},
	);
}

/** Test the connection of a stored vault provider by id. */
export function useVaultProviderTest() {
	return useConnectMutation(
		(input: {id: number | bigint; organization_id?: number | bigint}) =>
			vaultClient.testProvider({
				id: BigInt(input.id),
				organizationId:
					input.organization_id !== undefined
						? BigInt(input.organization_id)
						: undefined,
			}),
	);
}

/**
 * Test unsaved vault credentials (provider_type/api_url/auth_token in the
 * payload; nothing is persisted).
 */
export function useVaultTestConnection() {
	return useConnectMutation((input: Record<string, unknown>) =>
		vaultClient.testConnection({payload: JSON.stringify(input)}),
	);
}

/**
 * Secret names held by a provider. Values never reach the browser — the
 * backend returns names only, which is what `${{vault.<name>.<secret>}}`
 * references are built from.
 */
export function useVaultSecrets(providerId: number | null) {
	return useConnectQuery<string[]>(
		['vault_providers', 'secrets', providerId ?? 0],
		async () => {
			const response = await vaultClient.listSecrets({
				id: BigInt(providerId ?? 0),
			});
			try {
				const parsed = JSON.parse(response.items?.[0]?.payload || '{}');
				return Array.isArray(parsed?.secrets)
					? (parsed.secrets as string[])
					: [];
			} catch {
				return [];
			}
		},
		{enabled: providerId !== null},
	);
}
