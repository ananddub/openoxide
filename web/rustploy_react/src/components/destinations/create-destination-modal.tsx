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
			<DialogContent className="max-w-md bg-card border-border p-6 shadow-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<HardDrive className="w-5 h-5 text-primary" />
						{editingDestination ? 'Edit S3 Storage Destination' : 'Add S3 Storage Destination'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure S3 compatible bucket credentials for volume snapshots & database backups
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4">
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

					<div className="flex items-center justify-between gap-3 pt-3 border-t border-border/40 mt-2">
						<Button type="button" variant="outline" onClick={handleTestConnection} disabled={testing} className="h-9 text-xs font-semibold flex items-center gap-1.5">
							{testing ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <ShieldCheck className="w-3.5 h-3.5 text-primary" />} Test Connection
						</Button>
						<Button type="submit" disabled={submitting} className="h-9 text-xs font-semibold px-5">
							{submitting ? 'Saving...' : editingDestination ? 'Update' : 'Create'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
