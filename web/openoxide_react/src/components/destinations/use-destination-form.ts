import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useDestinationCreate,
	useDestinationTest,
	useDestinationUpdate,
} from '#/api/live';
import type {DestinationResponse} from '#/types/api-helpers';
import {
	matchProvider,
	PROVIDERS,
} from '#/components/destinations/destinations-shared';

export {PROVIDERS};

export type TestStatus = 'idle' | 'testing' | 'success' | 'failed';

interface Args {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
	editingDestination?: DestinationResponse | null;
}

/** State, mutations and handlers for the destination create/edit sheet. */
export function useDestinationForm({
	isOpen,
	onClose,
	onSuccess,
	editingDestination,
}: Args) {
	const [name, setName] = useState('');
	const [provider, setProvider] = useState('aws');
	const [bucket, setBucket] = useState('');
	const [region, setRegion] = useState('us-east-1');
	const [endpoint, setEndpoint] = useState('');
	const [accessKey, setAccessKey] = useState('');
	const [secretKey, setSecretKey] = useState('');
	const [testStatus, setTestStatus] = useState<TestStatus>('idle');
	const [submitting, setSubmitting] = useState(false);

	const createMutation = useDestinationCreate();
	const updateMutation = useDestinationUpdate();
	const testMutation = useDestinationTest();

	useEffect(() => {
		if (editingDestination) {
			setName(editingDestination.name || '');
			setProvider(
				matchProvider(editingDestination.provider?.toLocaleLowerCase()),
			);
			setBucket(editingDestination.bucket || '');
			setRegion(editingDestination.region || 'us-east-1');
			setEndpoint(editingDestination.endpoint || '');
			setAccessKey(editingDestination.access_key || '');
			setSecretKey(editingDestination.secret_access_key || '');
		} else {
			setName('');
			setProvider('aws');
			setBucket('');
			setRegion('us-east-1');
			setEndpoint('');
			setAccessKey('');
			setSecretKey('');
		}
		setTestStatus('idle');
	}, [editingDestination, isOpen]);

	const handleProviderChange = (newProvider: string) => {
		setProvider(newProvider);
		if (newProvider === 'aws' && !region) setRegion('us-east-1');
		if (newProvider === 'r2' && !region) setRegion('auto');
	};

	const handleTestConnection = async () => {
		if (!bucket || !accessKey) {
			toast.error(
				'Bucket and Access Key ID are required to test connection',
			);
			return;
		}
		setTestStatus('testing');
		try {
			if (!secretKey) {
				toast.error('Secret Access Key is required to test connection');
				setTestStatus('idle');
				return;
			}
			await Promise.race([
				testMutation.mutateAsync({
					provider: provider || 'aws',
					bucket,
					region: region || 'us-east-1',
					endpoint: endpoint || '',
					access_key: accessKey,
					secret_access_key: secretKey,
				}),
				new Promise<never>((_, reject) =>
					setTimeout(
						() => reject(new Error('S3 connection test timed out')),
						15_000,
					),
				),
			]);
			setTestStatus('success');
			toast.success('S3 Storage Destination connection test passed!');
			setTimeout(() => setTestStatus('idle'), 3000);
		} catch (err: unknown) {
			setTestStatus('failed');
			toast.error(formatApiError(err));
			setTimeout(() => setTestStatus('idle'), 3000);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !bucket || !accessKey) {
			toast.error(
				'Please fill in required fields (Name, Bucket, Access Key)',
			);
			return;
		}

		setSubmitting(true);
		try {
			if (editingDestination) {
				await updateMutation.mutateAsync({
					id: String(editingDestination.id),
					name,
					provider,
					bucket,
					region,
					endpoint,
					access_key: accessKey,
					...(secretKey ? {secret_access_key: secretKey} : {}),
				});
				toast.success('S3 Storage Destination updated');
			} else {
				if (!secretKey) {
					toast.error('Secret Access Key is required for new destination');
					return;
				}
				await createMutation.mutateAsync({
					name,
					provider,
					bucket,
					region,
					endpoint,
					access_key: accessKey,
					secret_access_key: secretKey,
					organization_id: 1,
				});
				toast.success('S3 Storage Destination added successfully');
			}
			onSuccess();
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return {
		name,
		setName,
		provider,
		bucket,
		setBucket,
		region,
		setRegion,
		endpoint,
		setEndpoint,
		accessKey,
		setAccessKey,
		secretKey,
		setSecretKey,
		testStatus,
		submitting,
		handleProviderChange,
		handleTestConnection,
		handleSubmit,
	};
}
