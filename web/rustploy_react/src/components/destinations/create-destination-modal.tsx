import {useState, useEffect} from 'react';
import {HardDrive, RefreshCw, ShieldCheck, Plug} from 'lucide-react';
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
	{id: 'aws', name: 'Amazon Web Services (AWS S3)', region: 'us-east-1', endpoint: 'https://s3.us-east-1.amazonaws.com'},
	{id: 'cloudflare_r2', name: 'Cloudflare R2', region: 'auto', endpoint: 'https://<account-id>.r2.cloudflarestorage.com'},
	{id: 'minio', name: 'MinIO Storage (Self-Hosted)', region: 'us-east-1', endpoint: 'http://localhost:9000'},
	{id: 'digitalocean', name: 'DigitalOcean Spaces', region: 'nyc3', endpoint: 'https://nyc3.digitaloceanspaces.com'},
	{id: 'wasabi', name: 'Wasabi Hot Cloud Storage', region: 'us-east-1', endpoint: 'https://s3.wasabisys.com'},
	{id: 'backblaze', name: 'Backblaze B2 Cloud Storage', region: 'us-west-004', endpoint: 'https://s3.us-west-004.backblazeb2.com'},
	{id: 'scaleway', name: 'Scaleway Elements Object Storage', region: 'fr-par', endpoint: 'https://s3.fr-par.scw.cloud'},
	{id: 'linode', name: 'Akamai / Linode Object Storage', region: 'us-east-1', endpoint: 'https://us-east-1.linodeobjects.com'},
	{id: 'hetzner', name: 'Hetzner Object Storage', region: 'fsn1', endpoint: 'https://fsn1.your-objectstorage.com'},
	{id: 'gcp', name: 'Google Cloud Storage (S3 Interoperable)', region: 'auto', endpoint: 'https://storage.googleapis.com'},
	{id: 'vultr', name: 'Vultr Object Storage', region: 'ewr1', endpoint: 'https://ewr1.vultrobjects.com'},
	{id: 'ovh', name: 'OVHcloud Object Storage', region: 'gra', endpoint: 'https://s3.gra.io.cloud.ovh.net'},
	{id: 'oracle', name: 'Oracle Cloud Infrastructure (OCI S3)', region: 'us-ashburn-1', endpoint: 'https://<namespace>.compat.objectstorage.us-ashburn-1.oraclecloud.com'},
	{id: 'azure', name: 'Microsoft Azure Blob Storage (S3 Gateway)', region: 'auto', endpoint: 'https://<account>.blob.core.windows.net'},
	{id: 'upcloud', name: 'UpCloud Object Storage', region: 'fi-hel2', endpoint: 'https://fi-hel2.objectstorage.upcloud.com'},
	{id: 'storj', name: 'Storj Distributed Cloud Storage', region: 'us-east-1', endpoint: 'https://gateway.storjshare.io'},
	{id: 'ceph', name: 'Ceph RADOS Gateway (RGW)', region: 'us-east-1', endpoint: 'https://rgw.yourdomain.com'},
	{id: 'garage', name: 'Garage S3 Storage', region: 'garage', endpoint: 'http://localhost:3900'},
	{id: 'seaweedfs', name: 'SeaweedFS S3 Gateway', region: 'us-east-1', endpoint: 'http://localhost:8333'},
	{id: 'custom', name: 'Custom S3 Compatible Provider', region: 'us-east-1', endpoint: ''},
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
				body: {name: name || 'Test', provider, bucket, region, endpoint, access_key: accessKey, secret_access_key: secretKey, organization_id: 1} as any,
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
					body: {name, provider, bucket, region, endpoint, access_key: accessKey, secret_access_key: secretKey} as any,
				});
				toast.success('S3 Storage Destination updated');
			} else {
				await createMutation.mutateAsync({
					body: {name, provider, bucket, region, endpoint, access_key: accessKey, secret_access_key: secretKey, organization_id: 1} as any,
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
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground">
						{editingDestination ? 'Edit Destination' : 'Create Destination'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Configure your S3 storage bucket credentials for backups and volume snapshots
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

					<div className="flex items-center justify-between gap-3 pt-4 border-t border-border/40 mt-1">
						<Button
							type="button"
							variant="outline"
							size="icon"
							onClick={handleTestConnection}
							disabled={testing}
							title="Test Connection"
							className="h-9 w-9 shrink-0"
						>
							{testing ? <RefreshCw className="w-4 h-4 animate-spin text-primary" /> : <Plug className="w-4 h-4 text-muted-foreground hover:text-foreground" />}
						</Button>
						<Button type="submit" disabled={submitting} className="h-9 text-xs font-semibold px-5">
							{submitting ? 'Saving...' : editingDestination ? 'Save' : 'Create'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
