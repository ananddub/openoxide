import {createServiceClient, useResourceListQuery} from './query';
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
