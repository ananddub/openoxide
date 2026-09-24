import {useState} from 'react';
import {
	useDnsProviderCreate,
	useDnsProviderUpdate,
	useDnsProviderTest,
	useDnsTestConnection,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {toast} from 'sonner';
import {useOrganizations} from '#/stores/session-context';
import {
	buildDnsCredentials,
	type DnsProviderItem,
} from './dns-provider-catalog';

export interface DnsFormValues {
	provider: DnsProviderItem | null;
	name: string;
	type: string;
	accessKey: string;
	token: string;
}

/** Create/update and connection-test actions for the DNS provider form. Reads current values lazily so the dialog only owns form state. */
export function useDnsProviderFormActions(getValues: () => DnsFormValues) {
	const {activeOrg} = useOrganizations();
	const createMutation = useDnsProviderCreate();
	const updateMutation = useDnsProviderUpdate();
	const testMutation = useDnsProviderTest();
	const testConnectionMutation = useDnsTestConnection();
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [isTesting, setIsTesting] = useState(false);

	const save = async (): Promise<boolean> => {
		const v = getValues();
		if (!v.name.trim() || (!v.provider && !v.token.trim())) {
			toast.error('Please fill in all required fields');
			return false;
		}

		setIsSubmitting(true);
		try {
			if (v.provider) {
				await updateMutation.mutateAsync({
					id: v.provider.id,
					name: v.name.trim(),
					credentials_json: v.token.trim()
						? buildDnsCredentials(v.type, v.accessKey, v.token)
						: undefined,
				});
				toast.success(`DNS Provider "${v.name}" updated`);
			} else {
				await createMutation.mutateAsync({
					organization_id: Number(activeOrg?.id || 1),
					name: v.name.trim(),
					provider_type: v.type,
					credentials_json: buildDnsCredentials(
						v.type,
						v.accessKey,
						v.token,
					),
				});
				toast.success(`DNS Provider "${v.name}" created successfully`);
			}
			return true;
		} catch (err) {
			toast.error(formatApiError(err));
			return false;
		} finally {
			setIsSubmitting(false);
		}
	};

	const reportResult = (result?: {
		success?: boolean;
		message?: string;
	}) => {
		if (result?.success) {
			toast.success(
				result.message || 'DNS API token verified successfully!',
			);
		} else {
			toast.error(result?.message || 'DNS API token verification failed');
		}
	};

	const test = async () => {
		const v = getValues();
		if (!v.provider && !v.token.trim()) {
			toast.error('Please enter an API Token to test connection');
			return;
		}

		setIsTesting(true);
		try {
			if (v.provider) {
				reportResult(await testMutation.mutateAsync({id: v.provider.id}));
			} else {
				// Call Rust backend proxy to bypass browser CORS & run official API test
				reportResult(
					await testConnectionMutation.mutateAsync({
						name: v.name.trim() || 'Test',
						provider_type: v.type,
						credentials_json: buildDnsCredentials(
							v.type,
							v.accessKey,
							v.token,
						),
					}),
				);
			}
		} catch (err: any) {
			toast.error(err?.message || formatApiError(err));
		} finally {
			setIsTesting(false);
		}
	};

	return {save, test, isSubmitting, isTesting};
}
