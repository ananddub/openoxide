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
import {Key, Terminal, Copy, Check} from 'lucide-react';

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
	const [copiedCmd, setCopiedCmd] = useState(false);

	const createMutation = $api.useMutation('post', '/ssh-keys');

	const setupCommand = publicKey.trim()
		? `mkdir -p ~/.ssh && echo "${publicKey.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`
		: '';

	const handleCopyCommand = () => {
		if (setupCommand) {
			navigator.clipboard.writeText(setupCommand);
			setCopiedCmd(true);
			toast.success('Server setup command copied!');
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
			toast.success('SSH Key added successfully');
			setName('');
			setDescription('');
			setPrivateKey('');
			setPublicKey('');
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
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Key className="w-5 h-5 text-primary" />
						Add Existing SSH Key
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Paste your existing SSH private and public key pair credentials
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-2">
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Key Name *</label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. Production Server Key"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
						<Input
							value={description}
							onChange={e => setDescription(e.target.value)}
							placeholder="Deploy key for web cluster"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Public Key *</label>
						<Textarea
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... user@server"
							className="h-20 text-xs font-mono bg-background border-border rounded-md p-3 resize-none"
						/>
					</div>

					{setupCommand && (
						<div className="bg-primary/5 p-3 rounded-xl border border-primary/20 flex flex-col gap-2 min-w-0 w-full">
							<div className="flex items-center justify-between gap-2 min-w-0">
								<span className="text-xs font-bold text-foreground flex items-center gap-1.5 truncate">
									<Terminal className="w-4 h-4 text-primary shrink-0" />
									Run on Remote Server to Authorize Key:
								</span>
								<Button type="button" variant="secondary" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-semibold gap-1 shrink-0">
									{copiedCmd ? <Check className="w-3.5 h-3.5 text-primary" /> : <Copy className="w-3.5 h-3.5" />}
									{copiedCmd ? 'Copied' : 'Copy Command'}
								</Button>
							</div>
							<div className="p-2.5 bg-background/80 border border-border/60 rounded-lg text-[11px] font-mono text-foreground break-all [word-break:break-all] select-all max-h-24 overflow-y-auto leading-relaxed">
								{setupCommand}
							</div>
						</div>
					)}

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Private Key *</label>
						<Textarea
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----&#10;...&#10;-----END OPENSSH PRIVATE KEY-----"
							className="h-32 text-xs font-mono bg-background border-border rounded-md p-3 resize-none"
						/>
					</div>

					<div className="flex items-center justify-end gap-3 pt-4 border-t border-border/40 mt-1">
						<Button type="submit" disabled={submitting} className="h-9 text-xs font-semibold px-6">
							{submitting ? 'Saving...' : 'Add SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
