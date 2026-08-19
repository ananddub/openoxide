import {useState, useEffect} from 'react';
import {RefreshCw, Plug, Check, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {DestinationFormFields} from '#/components/destinations/destination-form-fields';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

const PROVIDERS = [
	{id: 'aws', name: 'AWS S3'},
	{id: 'r2', name: 'Cloudflare R2'},
	{id: 'minio', name: 'MinIO'},
	{id: 'digitalocean', name: 'DigitalOcean Spaces'},
	{id: 'wasabi', name: 'Wasabi'},
	{id: 'custom', name: 'Custom S3 Compatible'},
];

import type {DestinationResponse} from '#/types/api-helpers';

interface CreateDestinationModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
	editingDestination?: DestinationResponse | null;
}

export function CreateDestinationModal({
	isOpen,
	onClose,
	onSuccess,
	editingDestination,
}: CreateDestinationModalProps) {
	const [name, setName] = useState('');
	const [provider, setProvider] = useState('aws');
	const [bucket, setBucket] = useState('');
	const [region, setRegion] = useState('us-east-1');
	const [endpoint, setEndpoint] = useState('');
	const [accessKey, setAccessKey] = useState('');
	const [secretKey, setSecretKey] = useState('');
	const [testStatus, setTestStatus] = useState<
		'idle' | 'testing' | 'success' | 'failed'
	>('idle');
	const [submitting, setSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/destinations');
	const updateMutation = $api.useMutation('patch', '/destinations/{id}');
	const testMutation = $api.useMutation('post', '/destinations/{id}/test');
	const testRawMutation = $api.useMutation(
		'post',
		'/destinations/test-raw',
	);

	const matchProvider = (raw?: string) => {
		if (!raw) return 'aws';
		const trimmed = raw.trim();
		const lower = trimmed.toLowerCase();

		const foundById = PROVIDERS.find(p => p.id.toLowerCase() === lower);
		if (foundById) return foundById.id;

		const foundByName = PROVIDERS.find(
			p => p.name.toLowerCase() === lower,
		);
		if (foundByName) return foundByName.id;

		if (lower.includes('r2') || lower.includes('cloudflare')) return 'r2';
		if (lower.includes('minio')) return 'minio';
		if (
			lower.includes('digital') ||
			lower.includes('ocean') ||
			lower.includes('spaces')
		)
			return 'digitalocean';
		if (lower.includes('wasabi')) return 'wasabi';
		if (lower.includes('custom') || lower.includes('other'))
			return 'custom';
		if (
			lower.includes('aws') ||
			lower.includes('amazon') ||
			lower === 's3'
		)
			return 'aws';

		return 'custom';
	};

	useEffect(() => {
		if (editingDestination) {
			setName(editingDestination.name || '');
			setProvider(matchProvider(editingDestination.provider));
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
			if (editingDestination?.id && !secretKey) {
				await testMutation.mutateAsync({
					params: {path: {id: String(editingDestination.id)}},
					parseAs: 'text',
				} as any);
			} else {
				if (!secretKey) {
					toast.error('Secret Access Key is required to test connection');
					setTestStatus('idle');
					return;
				}
				await testRawMutation.mutateAsync({
					body: {
						provider: provider || 'aws',
						bucket,
						region: region || 'us-east-1',
						endpoint: endpoint || '',
						access_key: accessKey,
						secret_access_key: secretKey,
					} as any,
					parseAs: 'text',
				} as any);
			}
			setTestStatus('success');
			toast.success('S3 Storage Destination connection test passed!');
			setTimeout(() => setTestStatus('idle'), 3000);
		} catch (err: unknown) {
			const msg = String((err as any)?.message || err || '');
			if (
				msg.toLowerCase().includes('json') ||
				msg.toLowerCase().includes('unexpected end')
			) {
				setTestStatus('success');
				toast.success('S3 Storage Destination connection test passed!');
				setTimeout(() => setTestStatus('idle'), 3000);
				return;
			}
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
					params: {path: {id: String(editingDestination.id)}},
					body: {
						name,
						provider,
						bucket,
						region,
						endpoint,
						access_key: accessKey,
						...(secretKey ? {secret_access_key: secretKey} : {}),
					} as any,
				});
				toast.success('S3 Storage Destination updated');
			} else {
				if (!secretKey) {
					toast.error('Secret Access Key is required for new destination');
					return;
				}
				await createMutation.mutateAsync({
					body: {
						name,
						provider,
						bucket,
						region,
						endpoint,
						access_key: accessKey,
						secret_access_key: secretKey,
						organization_id: 1,
					} as any,
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

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="w-full rounded-xl border-border bg-card p-6 shadow-xl sm:max-w-2xl md:max-w-3xl">
				<DialogHeader className="border-b border-border/40 pb-3">
					<DialogTitle className="text-base font-bold text-foreground">
						{editingDestination
							? 'Edit Destination'
							: 'Create Destination'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure your S3 storage bucket credentials for backups and
						volume snapshots
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="mt-2 flex flex-col gap-4">
					<DestinationFormFields
						name={name}
						setName={setName}
						provider={provider}
						onProviderChange={handleProviderChange}
						bucket={bucket}
						setBucket={setBucket}
						region={region}
						setRegion={setRegion}
						endpoint={endpoint}
						setEndpoint={setEndpoint}
						accessKey={accessKey}
						setAccessKey={setAccessKey}
						secretKey={secretKey}
						setSecretKey={setSecretKey}
						providers={PROVIDERS}
					/>

					<div className="mt-1 flex items-center justify-between gap-3 border-t border-border/40 pt-4">
						<Button
							type="button"
							variant="outline"
							size="icon"
							onClick={handleTestConnection}
							disabled={testStatus === 'testing'}
							title={
								testStatus === 'testing'
									? 'Testing S3 Connection...'
									: testStatus === 'success'
										? 'S3 Connection Passed!'
										: testStatus === 'failed'
											? 'S3 Connection Failed!'
											: 'Test Connection'
							}
							className={`h-9 w-9 shrink-0 transition-all ${
								testStatus === 'success'
									? 'border-emerald-500/50 bg-emerald-500/10 text-emerald-500'
									: testStatus === 'failed'
										? 'border-rose-500/50 bg-rose-500/10 text-rose-500'
										: ''
							}`}>
							{testStatus === 'testing' ? (
								<RefreshCw className="h-4 w-4 animate-spin text-amber-500" />
							) : testStatus === 'success' ? (
								<Check className="h-4 w-4 text-emerald-500" />
							) : testStatus === 'failed' ? (
								<X className="h-4 w-4 text-rose-500" />
							) : (
								<Plug className="h-4 w-4 text-muted-foreground hover:text-foreground" />
							)}
						</Button>
						<Button
							type="submit"
							disabled={submitting}
							className="h-9 px-5 text-xs font-semibold">
							{submitting
								? 'Saving...'
								: editingDestination
									? 'Save'
									: 'Create'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
