import {useState} from 'react';
import {
	useVaultProviderCreate,
	useVaultProviderUpdate,
	useVaultProviderTest,
	useVaultTestConnection,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {toast} from 'sonner';
import {useOrganizations} from '#/stores/session-context';
import {
	requireVaultAssignments,
	vaultDefaultApiUrl,
	type VaultProviderItem,
	type VaultProviderType,
} from './vault-provider-catalog';

export interface VaultFormValues {
	provider: VaultProviderItem | null;
	name: string;
	type: VaultProviderType;
	url: string;
	token: string;
	namespace: string;
	configJson: () => string;
	assignmentsJson: string;
}

const TOKEN_REQUIRED_TYPES = ['HASHICORP', 'DOPPLER'];

/** Create/update and connection-test actions for the vault provider form. Reads current values lazily so the dialog only owns form state. */
export function useVaultProviderFormActions(
	getValues: () => VaultFormValues,
) {
	const {activeOrg} = useOrganizations();
	const createMutation = useVaultProviderCreate();
	const updateMutation = useVaultProviderUpdate();
	const testMutation = useVaultProviderTest();
	const testConnectionMutation = useVaultTestConnection();
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [isTesting, setIsTesting] = useState(false);

	const tokenMissing = () => {
		const v = getValues();
		return (
			!v.provider &&
			TOKEN_REQUIRED_TYPES.includes(v.type) &&
			!v.token.trim()
		);
	};

	const save = async (): Promise<boolean> => {
		const v = getValues();
		if (!v.name.trim() || tokenMissing()) {
			toast.error('Please fill in all required fields');
			return false;
		}

		setIsSubmitting(true);
		try {
			const assignments = requireVaultAssignments(v.assignmentsJson);
			if (v.provider) {
				await updateMutation.mutateAsync({
					id: v.provider.id,
					name: v.name.trim(),
					api_url: v.url.trim() || undefined,
					...(v.token.trim() ? {auth_token: v.token.trim()} : {}),
					namespace: v.namespace.trim() || undefined,
					config_json: v.configJson(),
					assignments,
				});
				toast.success(`Vault Provider "${v.name}" updated`);
			} else {
				await createMutation.mutateAsync({
					organization_id: Number(activeOrg?.id || 1),
					name: v.name.trim(),
					provider_type: v.type,
					api_url: vaultDefaultApiUrl(v.type, v.url),
					auth_token: v.token.trim(),
					namespace: v.namespace.trim() || undefined,
					config_json: v.configJson(),
					assignments,
				});
				toast.success(`Vault Provider "${v.name}" created successfully`);
			}
			return true;
		} catch (err) {
			toast.error(formatApiError(err));
			return false;
		} finally {
			setIsSubmitting(false);
		}
	};

	const test = async () => {
		const v = getValues();
		if (tokenMissing()) {
			toast.error('Please enter a Token / Access Key to test connection');
			return;
		}

		setIsTesting(true);
		try {
			// Unsaved providers are pinged via the Rust backend proxy (CORS-safe).
			const result = v.provider
				? await testMutation.mutateAsync({id: v.provider.id})
				: await testConnectionMutation.mutateAsync({
						name: v.name.trim() || 'Test',
						provider_type: v.type,
						api_url: v.url.trim(),
						auth_token: v.token.trim(),
						namespace: v.namespace.trim() || undefined,
						config_json: v.configJson(),
					});
			if (result?.success) {
				toast.success(result.message || 'Connection successful!');
			} else {
				toast.error(result?.message || 'Connection test failed!');
			}
		} catch (err: any) {
			toast.error(err?.message || formatApiError(err));
		} finally {
			setIsTesting(false);
		}
	};

	return {save, test, isSubmitting, isTesting};
}
