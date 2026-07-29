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

	const generatePairMutation = $api.useMutation('post', '/ssh-keys/generate-pair');
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
					setName(`Auto-${type.toUpperCase()}-${Date.now().toString().slice(-4)}`);
				}
				toast.success(`Generated ${type.toUpperCase()} SSH key pair! Click "Save SSH Key" when ready.`);
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

	const formattedFileName = name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full max-h-[90vh] overflow-y-auto bg-card border-border p-6 shadow-xl rounded-xl min-w-0">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-sm font-bold text-foreground flex items-center gap-2">
						<Key className="w-4 h-4 text-primary shrink-0" />
						<span>Add SSH Key</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Add an existing SSH key pair or generate a new key pair.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 py-2 min-w-0">
					<div className="space-y-1.5 min-w-0">
						<label className="text-xs font-semibold">Key Name *</label>
						<Input
							placeholder="e.g. Production Server Key"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							className="h-9 text-xs min-w-0 w-full"
						/>
					</div>

					<div className="space-y-1.5 min-w-0">
						<label className="text-xs font-semibold">Description (Optional)</label>
						<Input
							placeholder="e.g. Key for deployment worker server"
							value={description}
							onChange={e => setDescription(e.target.value)}
							className="h-9 text-xs min-w-0 w-full"
						/>
					</div>

					<div className="grid grid-cols-2 gap-2.5 min-w-0 w-full">
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('ed25519')}
							disabled={generating}
							className="h-8.5 text-xs font-medium border-border hover:bg-accent/60 transition-colors w-full"
						>
							{generating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1.5" /> : null}
							Generate ED25519
						</Button>
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('rsa')}
							disabled={generating}
							className="h-8.5 text-xs font-medium border-border hover:bg-accent/60 transition-colors w-full"
						>
							{generating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1.5" /> : null}
							Generate RSA 4096
						</Button>
					</div>

					<div className="space-y-1.5 min-w-0">
						<label className="text-xs font-semibold">Private Key (PEM / OpenSSH) *</label>
						<Textarea
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							rows={4}
							required
							className="font-mono text-xs break-all max-h-36 overflow-y-auto min-w-0 w-full"
						/>
					</div>

					<div className="space-y-1.5 min-w-0">
						<label className="text-xs font-semibold">Public Key (Optional)</label>
						<Textarea
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							rows={2}
							className="font-mono text-xs break-all max-h-28 overflow-y-auto min-w-0 w-full"
						/>
					</div>

					<AuthorizeCommandBlock publicKey={publicKey} />

					<div className="flex flex-col sm:flex-row items-center justify-between gap-2 pt-3 border-t border-border/50">
						<div className="flex items-center gap-2 w-full sm:w-auto">
							{publicKey && (
								<Button
									type="button"
									variant="outline"
									size="sm"
									onClick={() => downloadKeyFile(`${formattedFileName}.pub`, publicKey)}
									className="h-8 text-xs font-medium gap-1.5 border-border"
								>
									<Download className="w-3.5 h-3.5" />
									Download Pub
								</Button>
							)}
							{privateKey && (
								<Button
									type="button"
									variant="outline"
									size="sm"
									onClick={() => downloadKeyFile(`${formattedFileName}.pem`, privateKey)}
									className="h-8 text-xs font-medium gap-1.5 border-border"
								>
									<Download className="w-3.5 h-3.5 text-amber-500" />
									Download Private
								</Button>
							)}
						</div>

						<Button type="submit" disabled={submitting} className="w-full sm:w-auto h-9 px-6 font-bold text-xs bg-primary hover:bg-primary/90 text-primary-foreground shadow-md">
							{submitting ? 'Saving Key...' : 'Save SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
