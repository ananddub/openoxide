import {useState, useEffect} from 'react';
import {HardDrive, RefreshCw, ShieldCheck} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DestinationFormFields} from './destination-form-fields';

interface CreateDestinationModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
	editingDestination?: any | null;
}

const PROVIDERS = [
	{id: 'aws', name: 'AWS S3', region: 'us-east-1', endpoint: 'https://s3.us-east-1.amazonaws.com'},
	{id: 'cloudflare_r2', name: 'Cloudflare R2', region: 'auto', endpoint: 'https://<accountid>.r2.cloudflarestorage.com'},
	{id: 'minio', name: 'MinIO Storage', region: 'us-east-1', endpoint: 'http://localhost:9000'},
	{id: 'wasabi', name: 'Wasabi S3', region: 'us-east-1', endpoint: 'https://s3.wasabisys.com'},
	{id: 'digitalocean', name: 'DigitalOcean Spaces', region: 'nyc3', endpoint: 'https://nyc3.digitaloceanspaces.com'},
	{id: 'custom', name: 'Custom S3 Compatible', region: 'us-east-1', endpoint: ''},
];

export function CreateDestinationModal({isOpen, onClose, onSuccess, editingDestination}: CreateDestinationModalProps) {
	const [name, setName] = useState('');
	const [provider, setProvider] = useState('aws');
	const [bucket, setBucket] = useState('');
	const [region, setRegion] = useState('us-east-1');
	const [endpoint, setEndpoint] = useState('');
	const [accessKey, setAccessKey] = useState('');
	const [secretKey, setSecretKey] = useState('');
	const [testing, setTesting] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/destinations');
	const patchMutation = $api.useMutation('patch', '/destinations/{id}');
	const testRawMutation = $api.useMutation('post', '/destinations/test-raw');

	useEffect(() => {
		if (editingDestination) {
			setName(editingDestination.name || '');
			setProvider(editingDestination.provider || 'aws');
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
			setEndpoint('https://s3.us-east-1.amazonaws.com');
			setAccessKey('');
			setSecretKey('');
		}
	}, [editingDestination, isOpen]);

	const handleProviderChange = (val: string) => {
		setProvider(val);
		const found = PROVIDERS.find(p => p.id === val);
		if (found && !editingDestination) {
			setRegion(found.region);
			setEndpoint(found.endpoint);
		}
	};

	const handleTestConnection = async () => {
		if (!bucket || !accessKey || !secretKey) {
			toast.error('Bucket name, Access Key and Secret Key are required to test connection');
			return;
		}
		setTesting(true);
		try {
			await testRawMutation.mutateAsync({
				body: {
					name: name || 'Test',
					provider,
					bucket,
					region,
					endpoint,
					access_key: accessKey,
					secret_access_key: secretKey,
					organization_id: 1,
				} as any,
			});
			toast.success('S3 Connection & Bucket Write Permissions Verified Successfully!');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setTesting(false);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !bucket || !accessKey || !secretKey) {
			toast.error('Please fill in all required fields');
			return;
		}
		setSubmitting(true);
		try {
			if (editingDestination) {
				await patchMutation.mutateAsync({
					params: {path: {id: String(editingDestination.id)}},
					body: {
						name,
						provider,
						bucket,
						region,
						endpoint,
						access_key: accessKey,
						secret_access_key: secretKey,
					} as any,
				});
				toast.success('S3 Storage Destination updated');
			} else {
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
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-w-2xl bg-card border-border p-6 shadow-2xl rounded-2xl">
				<DialogHeader className="pb-4 border-b border-border/50">
					<DialogTitle className="text-lg font-extrabold text-foreground flex items-center gap-2.5">
						<HardDrive className="w-5.5 h-5.5 text-primary" />
						{editingDestination ? 'Edit S3 Destination' : 'Add S3 Destination'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground mt-0.5">
						Configure S3 compatible bucket credentials for volume snapshots & database backups
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-2">
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

					<div className="flex items-center justify-between gap-4 pt-4 border-t border-border/50 mt-2">
						<Button
							type="button"
							variant="outline"
							onClick={handleTestConnection}
							disabled={testing}
							className="h-10 text-xs font-semibold flex items-center gap-2 px-4 rounded-lg border-border"
						>
							{testing ? <RefreshCw className="w-4 h-4 animate-spin text-primary" /> : <ShieldCheck className="w-4 h-4 text-primary" />} Test Connection
						</Button>
						<Button type="submit" disabled={submitting} className="h-10 text-xs font-bold px-6 rounded-lg shadow-sm">
							{submitting ? 'Saving...' : editingDestination ? 'Save Changes' : 'Create Destination'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
