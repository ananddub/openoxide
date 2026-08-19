import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {Key, RefreshCw, Download} from 'lucide-react';
import {downloadKeyFile} from '#/utils/ssh-key-utils';
import {AuthorizeCommandBlock} from './authorize-command-block';

interface CreateKeyModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

export function CreateKeyModal({
	isOpen,
	onClose,
	onSuccess,
}: CreateKeyModalProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [generating, setGenerating] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const generatePairMutation = $api.useMutation(
		'post',
		'/ssh-keys/generate-pair',
	);
	const createMutation = $api.useMutation('post', '/ssh-keys');

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGenerating(true);
		try {
			const res = await generatePairMutation.mutateAsync({
				body: {key_type: type},
			});

			if (res && res.private_key && res.public_key) {
				setPrivateKey(res.private_key);
				setPublicKey(res.public_key);
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
				body: {
					name,
					description: description || undefined,
					private_key: privateKey,
					public_key: publicKey || '',
				},
			});
			toast.success('SSH Key created successfully');
			onSuccess();
			handleClose();
		} catch (err: any) {
			toast.error(err?.message || 'Failed to create SSH Key');
		} finally {
			setSubmitting(false);
		}
	};

	const handleClose = () => {
		setName('');
		setDescription('');
		setPrivateKey('');
		setPublicKey('');
		onClose();
	};

	const formattedFileName =
		name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
			<DialogContent className="max-h-[90vh] w-full min-w-0 overflow-y-auto rounded-xl border-border bg-card p-6 shadow-xl sm:max-w-lg">
				<DialogHeader className="border-b border-border/50 pb-3">
					<DialogTitle className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Key className="h-4 w-4 shrink-0 text-primary" />
						<span>Add SSH Key</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Add an existing SSH key pair or generate a new key pair.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="min-w-0 space-y-4 py-2">
					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">Key Name *</label>
						<Input
							placeholder="e.g. Production Server Key"
							value={name}
							onChange={e => setName(e.target.value)}
							required
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
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('ed25519')}
							disabled={generating}
							className="h-8.5 w-full border-border text-xs font-medium transition-colors hover:bg-accent/60">
							{generating ? (
								<RefreshCw className="mr-1.5 h-3.5 w-3.5 animate-spin" />
							) : null}
							Generate ED25519
						</Button>
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('rsa')}
							disabled={generating}
							className="h-8.5 w-full border-border text-xs font-medium transition-colors hover:bg-accent/60">
							{generating ? (
								<RefreshCw className="mr-1.5 h-3.5 w-3.5 animate-spin" />
							) : null}
							Generate RSA 4096
						</Button>
					</div>

					<div className="min-w-0 space-y-1.5">
						<label className="text-xs font-semibold">
							Private Key (PEM / OpenSSH) *
						</label>
						<Textarea
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							rows={4}
							required
							className="max-h-36 w-full min-w-0 overflow-y-auto font-mono text-xs break-all"
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
							rows={2}
							className="max-h-28 w-full min-w-0 overflow-y-auto font-mono text-xs break-all"
						/>
					</div>

					<AuthorizeCommandBlock publicKey={publicKey} />

					<div className="flex flex-col items-center justify-between gap-2 border-t border-border/50 pt-3 sm:flex-row">
						<div className="flex w-full items-center gap-2 sm:w-auto">
							{publicKey && (
								<Button
									type="button"
									variant="outline"
									size="sm"
									onClick={() =>
										downloadKeyFile(`${formattedFileName}.pub`, publicKey)
									}
									className="h-8 gap-1.5 border-border text-xs font-medium">
									<Download className="h-3.5 w-3.5" />
									Download Pub
								</Button>
							)}
							{privateKey && (
								<Button
									type="button"
									variant="outline"
									size="sm"
									onClick={() =>
										downloadKeyFile(`${formattedFileName}.pem`, privateKey)
									}
									className="h-8 gap-1.5 border-border text-xs font-medium">
									<Download className="h-3.5 w-3.5 text-amber-500" />
									Download Private
								</Button>
							)}
						</div>

						<Button
							type="submit"
							disabled={submitting}
							className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
							{submitting ? 'Saving Key...' : 'Save SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
