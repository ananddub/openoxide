import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {
	ArrowLeft,
	Check,
	HardDrive,
	Plug,
	RefreshCw,
	X,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useDestinationCreate,
	useDestinationTest,
} from '#/api/live';
import {DestinationFormFields} from '#/components/destinations/destination-form-fields';

export const Route = createFileRoute('/_app/destinations/new')({
	component: DestinationCreatePage,
});

const PROVIDERS = [
	{id: 'aws', name: 'AWS S3'},
	{id: 'r2', name: 'Cloudflare R2'},
	{id: 'minio', name: 'MinIO'},
	{id: 'digitalocean', name: 'DigitalOcean Spaces'},
	{id: 'wasabi', name: 'Wasabi'},
	{id: 'custom', name: 'Custom S3 Compatible'},
];

function DestinationCreatePage() {
	const navigate = useNavigate();
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

	const createMutation = useDestinationCreate();
	const testMutation = useDestinationTest();

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
		if (!secretKey) {
			toast.error('Secret Access Key is required to test connection');
			return;
		}
		setTestStatus('testing');
		try {
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
					setTimeout(() => reject(new Error('S3 connection test timed out')), 15_000),
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
		if (!secretKey) {
			toast.error('Secret Access Key is required for new destination');
			return;
		}
		setSubmitting(true);
		try {
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
			void navigate({to: '/destinations'});
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Page header ── */}
			<div className="flex items-center gap-3">
				<button
					type="button"
					onClick={() => void navigate({to: '/destinations'})}
					title="Back to Destinations"
					className="flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
					<ArrowLeft className="size-4" />
				</button>
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted">
					<HardDrive className="size-4 text-foreground/70" />
				</div>
				<div className="min-w-0">
					<h1 className="truncate text-2xl font-medium tracking-tight text-foreground/80">
						Create Destination
					</h1>
					<p className="text-sm text-muted-foreground/70">
						S3 storage bucket credentials for backups and volume snapshots.
					</p>
				</div>
			</div>

			{/* ── Form card ── */}
			<div className="mx-auto w-full min-w-0 max-w-2xl rounded-xl bg-card p-6 ring-1 ring-border/50">
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

					<div className="mt-1 flex items-center justify-between gap-3 border-t border-border/50 pt-4">
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
							{submitting ? 'Saving...' : 'Create'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
