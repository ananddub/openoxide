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
import {formatApiError} from '#/api/utils';
import {Key, RefreshCw, Terminal, Copy, Check} from 'lucide-react';

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
	const [submitting, setSubmitting] = useState(false);
	const [generatingType, setGeneratingType] = useState<'ed25519' | 'rsa' | null>(null);
	const [copiedCmd, setCopiedCmd] = useState(false);

	const createMutation = $api.useMutation('post', '/ssh-keys');
	const generatePairMutation = $api.useMutation('post', '/ssh-keys/generate-pair');

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGeneratingType(type);
		try {
			const res = await generatePairMutation.mutateAsync({
				body: {key_type: type} as any,
			});
			const keyPair = res as any;
			setPublicKey(keyPair.public_key || '');
			setPrivateKey(keyPair.private_key || '');
			if (!name) {
				setName(`Generated-${type.toUpperCase()}-Key`);
			}
			toast.success(`Auto-generated ${type.toUpperCase()} SSH key pair!`);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setGeneratingType(null);
		}
	};

	const setupCommand = publicKey.trim()
		? `mkdir -p ~/.ssh && echo "${publicKey.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`
		: '';

	const handleCopyCommand = () => {
		if (setupCommand) {
			navigator.clipboard.writeText(setupCommand);
			setCopiedCmd(true);
			toast.success('Server authorization command copied!');
			setTimeout(() => setCopiedCmd(false), 2000);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !privateKey || !publicKey) {
			toast.error('Name, Private Key, and Public Key are required');
			return;
		}

		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					private_key: privateKey,
					public_key: publicKey,
				},
			});
			toast.success(`SSH Key "${name}" saved successfully`);
			handleCloseModal();
			onSuccess();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	const handleCloseModal = () => {
		setName('');
		setDescription('');
		setPrivateKey('');
		setPublicKey('');
		setGeneratingType(null);
		onClose();
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleCloseModal()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-sm font-bold text-foreground flex items-center gap-2">
						<Key className="w-4 h-4 text-primary shrink-0" />
						<span>Add SSH Key</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Paste an existing SSH key pair or click an auto-generate option below
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-3.5 mt-2">
					{/* Centered Taller Auto Generate Buttons */}
					<div className="flex items-center justify-center gap-3 py-1.5">
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('ed25519')}
							disabled={!!generatingType || submitting}
							className="h-9 text-xs font-semibold px-5 rounded-lg border-border hover:bg-muted/80 shadow-sm"
						>
							{generatingType === 'ed25519' && <RefreshCw className="w-3.5 h-3.5 animate-spin mr-2 text-primary" />}
							Auto Generate ED25519
						</Button>
						<Button
							type="button"
							variant="outline"
							size="sm"
							onClick={() => handleGeneratePair('rsa')}
							disabled={!!generatingType || submitting}
							className="h-9 text-xs font-semibold px-5 rounded-lg border-border hover:bg-muted/80 shadow-sm"
						>
							{generatingType === 'rsa' && <RefreshCw className="w-3.5 h-3.5 animate-spin mr-2 text-primary" />}
							Auto Generate RSA 4096
						</Button>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Key Name *</label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. Production Key"
							className="h-9 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
						<Input
							value={description}
							onChange={e => setDescription(e.target.value)}
							placeholder="Optional description for this key pair"
							className="h-9 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Public Key *</label>
						<Textarea
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... user@server"
							className="h-16 text-xs font-mono bg-background border-border rounded-md p-3 resize-none break-all"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Private Key *</label>
						<Textarea
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							className="h-24 text-xs font-mono bg-background border-border rounded-md p-3 resize-none break-all"
						/>
					</div>

					{setupCommand && (
						<div className="flex flex-col gap-1.5 pt-2 border-t border-border/40">
							<div className="flex items-center justify-between">
								<label className="text-xs font-semibold text-foreground flex items-center gap-1.5">
									<Terminal className="w-3.5 h-3.5 text-primary shrink-0" />
									<span>Authorize Key on Remote Server</span>
								</label>
								<Button type="button" variant="outline" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-medium gap-1.5 px-2.5">
									{copiedCmd ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
									{copiedCmd ? 'Copied' : 'Copy Command'}
								</Button>
							</div>
							<div className="p-2.5 bg-muted/40 border border-border/50 rounded-md text-[11px] font-mono text-muted-foreground break-all select-all leading-relaxed max-h-20 overflow-y-auto">
								{setupCommand}
							</div>
						</div>
					)}

					<div className="flex items-center justify-end pt-3 border-t border-border/50">
						<Button type="submit" disabled={submitting || !!generatingType} className="h-9 text-xs font-bold px-6 shadow-md w-full sm:w-auto">
							{submitting ? 'Saving SSH Key...' : 'Save SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
