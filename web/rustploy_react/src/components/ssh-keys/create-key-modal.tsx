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
import {toast} from 'sonner';
import {Key, RefreshCw, Terminal, Copy, Check} from 'lucide-react';
import {useCreateSshKey} from '#/hooks/ssh-keys/use-ssh-keys';

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
	const [copiedCmd, setCopiedCmd] = useState(false);

	const handleCloseModal = () => {
		setName('');
		setDescription('');
		setPrivateKey('');
		setPublicKey('');
		onClose();
	};

	const {
		name,
		setName,
		description,
		setDescription,
		privateKey,
		setPrivateKey,
		publicKey,
		setPublicKey,
		submitting,
		generatingType,
		handleGeneratePair,
		handleSubmit,
	} = useCreateSshKey(handleCloseModal, onSuccess);

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

	return (
		<Dialog open={isOpen} onOpenChange={handleCloseModal}>
			<DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2">
						<Key className="w-5 h-5 text-primary" />
						Add SSH Key
					</DialogTitle>
					<DialogDescription>
						Auto-generate a new secure SSH key pair or paste an existing private key to access remote servers.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 py-2">
					<div className="space-y-2">
						<label className="text-xs font-semibold">Key Name *</label>
						<Input
							placeholder="e.g., Production Deployer Key"
							value={name}
							onChange={e => setName(e.target.value)}
							required
						/>
					</div>

					<div className="space-y-2">
						<label className="text-xs font-semibold">Description</label>
						<Input
							placeholder="e.g., Key for DigitalOcean Droplet #1"
							value={description}
							onChange={e => setDescription(e.target.value)}
						/>
					</div>

					{/* Auto-Generation Helper Bar */}
					<div className="rounded-xl border border-primary/20 bg-primary/5 p-3.5 space-y-2">
						<div className="flex items-center justify-between">
							<span className="text-xs font-bold text-foreground flex items-center gap-1.5">
								<RefreshCw className="w-3.5 h-3.5 text-primary" />
								Auto-Generate SSH Key Pair
							</span>
							<div className="flex gap-2">
								<Button
									type="button"
									variant="outline"
									size="sm"
									disabled={generatingType !== null}
									onClick={() => handleGeneratePair('ed25519')}
									className="h-7 text-xs font-semibold"
								>
									{generatingType === 'ed25519' ? 'Generating...' : 'Generate ED25519'}
								</Button>
								<Button
									type="button"
									variant="outline"
									size="sm"
									disabled={generatingType !== null}
									onClick={() => handleGeneratePair('rsa')}
									className="h-7 text-xs font-semibold"
								>
									{generatingType === 'rsa' ? 'Generating...' : 'Generate RSA 4096'}
								</Button>
							</div>
						</div>
						<p className="text-[11px] text-muted-foreground">
							Clicking generate creates a fresh private/public key pair locally in your browser.
						</p>
					</div>

					<div className="space-y-2">
						<label className="text-xs font-semibold">Private Key (PEM / OpenSSH) *</label>
						<Textarea
							rows={5}
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							className="font-mono text-xs"
							required
						/>
					</div>

					<div className="space-y-2">
						<label className="text-xs font-semibold">Public Key</label>
						<Textarea
							rows={2}
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA..."
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							className="font-mono text-xs"
						/>
					</div>

					{/* One-Click Remote Server Setup Helper */}
					{setupCommand && (
						<div className="rounded-xl border border-border bg-muted/30 p-3 space-y-2">
							<div className="flex items-center justify-between">
								<span className="text-xs font-bold text-foreground flex items-center gap-1.5">
									<Terminal className="w-3.5 h-3.5 text-amber-500" />
									Remote Server Quick-Setup Command
								</span>
								<Button
									type="button"
									variant="ghost"
									size="sm"
									onClick={handleCopyCommand}
									className="h-7 text-xs gap-1 font-semibold text-primary hover:text-primary"
								>
									{copiedCmd ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
									{copiedCmd ? 'Copied!' : 'Copy Command'}
								</Button>
							</div>
							<p className="text-[11px] text-muted-foreground">
								Run this one-liner command on your remote VPS terminal to append this public key to your server's authorized_keys:
							</p>
							<div className="p-2 rounded bg-black/40 font-mono text-[11px] text-emerald-400 break-all select-all border border-border/50">
								{setupCommand}
							</div>
						</div>
					)}

					<div className="flex justify-end gap-2 pt-2">
						<Button type="button" variant="outline" onClick={handleCloseModal}>
							Cancel
						</Button>
						<Button type="submit" disabled={submitting}>
							{submitting ? 'Saving Key...' : 'Save SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
