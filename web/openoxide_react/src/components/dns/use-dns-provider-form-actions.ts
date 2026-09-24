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
	dnsProviderFields,
	type DnsProviderItem,
} from './dns-provider-catalog';

export interface DnsFormValues {
	provider: DnsProviderItem | null;
	name: string;
	type: string;
	values: Record<string, string>;
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

	const missingFields = (v: DnsFormValues) =>
		dnsProviderFields(v.type).filter(f => !(v.values[f.key] ?? '').trim());

	const save = async (): Promise<boolean> => {
		const v = getValues();
		if (!v.name.trim()) {
			toast.error('Please fill in all required fields');
			return false;
		}
		if (!v.provider && missingFields(v).length > 0) {
			toast.error('Please fill in all credential fields');
			return false;
		}

		setIsSubmitting(true);
		try {
			if (v.provider) {
				// Masked secrets round-trip untouched; the backend keeps the
				// stored value for any field sent back as the mask.
				await updateMutation.mutateAsync({
					id: v.provider.id,
					name: v.name.trim(),
					credentials_json: buildDnsCredentials(v.type, v.values),
				});
				toast.success(`DNS Provider "${v.name}" updated`);
			} else {
				await createMutation.mutateAsync({
					organization_id: Number(activeOrg?.id || 1),
					name: v.name.trim(),
					provider_type: v.type,
					credentials_json: buildDnsCredentials(v.type, v.values),
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
				result.message || 'DNS API credentials verified successfully!',
			);
		} else {
			toast.error(
				result?.message || 'DNS API credential verification failed',
			);
		}
	};

	const test = async () => {
		const v = getValues();
		if (!v.provider && missingFields(v).length > 0) {
			toast.error('Please fill in all credential fields to test');
			return;
		}

		setIsTesting(true);
		try {
			// While editing, only test the live form when a real (unmasked)
			// secret is present; masked values would fail at the provider.
			const editedSecret = v.provider
				? dnsProviderFields(v.type).some(
						f =>
							f.secret &&
							(v.values[f.key] ?? '').trim() &&
							!(v.values[f.key] ?? '').includes('********'),
					)
				: false;
			if (v.provider && !editedSecret) {
				reportResult(await testMutation.mutateAsync({id: v.provider.id}));
			} else {
				// Call Rust backend proxy to bypass browser CORS & run official API test
				reportResult(
					await testConnectionMutation.mutateAsync({
						name: v.name.trim() || 'Test',
						provider_type: v.type,
						credentials_json: buildDnsCredentials(v.type, v.values),
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
