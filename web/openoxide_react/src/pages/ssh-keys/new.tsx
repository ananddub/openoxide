import {useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {KeyRound, RefreshCw, Sparkles} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {Titlebar} from '#/components/shared/titlebar';
import {toast} from 'sonner';
import {useSshKeyCreate, useSshKeyGeneratePair} from '#/api/live/ssh_keys';
import {AuthorizeCommandBlock} from '#/components/ssh-keys/authorize-command-block';
import {SshKeyDownloadButtons} from '#/components/ssh-keys/ssh-key-download-buttons';

export const Route = createFileRoute('/_app/ssh-keys/new')({
	component: SshKeyCreatePage,
});

function SshKeyCreatePage() {
	const navigate = useNavigate();
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [generating, setGenerating] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const generatePairMutation = useSshKeyGeneratePair();
	const createMutation = useSshKeyCreate();

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGenerating(true);
		try {
			const res: any = await generatePairMutation.mutateAsync({
				key_type: type,
			});
			// ResourceResponse wraps the pair as a JSON string payload.
			const payload =
				typeof res?.payload === 'string'
					? JSON.parse(res.payload)
					: (res?.payload ?? {});
			if (payload.private_key && payload.public_key) {
				setPrivateKey(payload.private_key);
				setPublicKey(payload.public_key);
				if (!name) {
					setName(
						`Auto-${type.toUpperCase()}-${Date.now().toString().slice(-4)}`,
					);
				}
				toast.success(
					`Generated ${type.toUpperCase()} SSH key pair! Click "Save SSH Key" when ready.`,
				);
			}
		} catch (err: any) {
			toast.error(err?.message || 'Failed to generate SSH key pair');
		} finally {
			setGenerating(false);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !privateKey) {
			toast.error('Name and Private Key are required');
			return;
		}
		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name,
				description: description || undefined,
				private_key: privateKey,
				public_key: publicKey || '',
			});
			toast.success('SSH Key created successfully');
			void navigate({to: '/ssh-keys'});
		} catch (err: any) {
			toast.error(err?.message || 'Failed to create SSH Key');
		} finally {
			setSubmitting(false);
		}
	};

	const formattedFileName =
		name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Vercel-style titlebar ── */}
			<Titlebar
				breadcrumbs={[
					{label: 'SSH Keys', to: '/ssh-keys', icon: KeyRound},
					{label: 'Add SSH Key'},
				]}
				icon={<KeyRound className="text-foreground/70" />}
				title="Add SSH Key"
				description="Add an existing SSH key pair or generate a new one."
			/>

			{/* ── Form card ── */}
			<div className="mx-auto w-full max-w-3xl min-w-0 rounded-xl bg-card p-6 ring-1 ring-border/50">
				<form onSubmit={handleSubmit} className="min-w-0 space-y-4">
					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">Key Name *</label>
						<Input
							placeholder="e.g. Production Server Key"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							autoFocus
							className="h-9 w-full min-w-0 text-xs"
						/>
					</div>

					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">
							Description (Optional)
						</label>
						<Input
							placeholder="e.g. Key for deployment worker server"
							value={description}
							onChange={e => setDescription(e.target.value)}
							className="h-9 w-full min-w-0 text-xs"
						/>
					</div>

					<div className="grid w-full min-w-0 grid-cols-2 gap-2.5">
						{(['ed25519', 'rsa'] as const).map(type => (
							<Button
								key={type}
								type="button"
								variant="outline"
								size="sm"
								onClick={() => handleGeneratePair(type)}
								disabled={generating}
								className="h-9 w-full text-xs font-medium transition-colors hover:bg-accent/60">
								{generating ? (
									<RefreshCw className="mr-1.5 h-3.5 w-3.5 animate-spin" />
								) : (
									<Sparkles className="mr-1.5 h-3.5 w-3.5" />
								)}
								{type === 'ed25519'
									? 'Generate ED25519'
									: 'Generate RSA 4096'}
							</Button>
						))}
					</div>

					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">
							Private Key (PEM / OpenSSH) *
						</label>
						<Textarea
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							rows={5}
							required
							className="max-h-40 w-full min-w-0 overflow-y-auto font-mono text-xs break-all"
						/>
					</div>

					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">
							Public Key (Optional)
						</label>
						<Textarea
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							rows={3}
							className="max-h-32 w-full min-w-0 overflow-y-auto font-mono text-xs break-all"
						/>
					</div>

					<AuthorizeCommandBlock publicKey={publicKey} />

					<div className="flex flex-col items-center justify-between gap-2 border-t border-border/50 pt-4 sm:flex-row">
						<SshKeyDownloadButtons
							fileName={formattedFileName}
							publicKey={publicKey}
							privateKey={privateKey}
						/>

						<Button
							type="submit"
							disabled={submitting}
							className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
							{submitting ? 'Saving Key...' : 'Save SSH Key'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
