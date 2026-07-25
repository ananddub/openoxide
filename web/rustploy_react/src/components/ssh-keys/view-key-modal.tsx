import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Textarea} from '#/components/ui/textarea';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {Copy, Check, Eye, EyeOff, Key} from 'lucide-react';

interface ViewKeyModalProps {
	isOpen: boolean;
	sshKey: any | null;
	onClose: () => void;
}

export function ViewKeyModal({
	isOpen,
	sshKey,
	onClose,
}: ViewKeyModalProps) {
	const [showPrivate, setShowPrivate] = useState(false);
	const [copiedPublic, setCopiedPublic] = useState(false);
	const [copiedPrivate, setCopiedPrivate] = useState(false);

	// Query full key details if available
	const {data: fullKeyDetails} = $api.useQuery(
		'get',
		'/ssh-keys/{id}',
		{
			params: {
				path: {
					id: sshKey?.id || 0,
				},
			},
		},
		{
			enabled: !!sshKey?.id,
		}
	);

	const activeKey = fullKeyDetails || sshKey;

	const handleCopyPub = () => {
		if (activeKey?.public_key) {
			navigator.clipboard.writeText(activeKey.public_key);
			setCopiedPublic(true);
			toast.success('Public SSH Key copied');
			setTimeout(() => setCopiedPublic(false), 2000);
		}
	};

	const handleCopyPriv = () => {
		if (activeKey?.private_key) {
			navigator.clipboard.writeText(activeKey.private_key);
			setCopiedPrivate(true);
			toast.success('Private SSH Key copied');
			setTimeout(() => setCopiedPrivate(false), 2000);
		}
	};

	if (!activeKey) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Key className="w-5 h-5 text-primary" />
						{activeKey.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{activeKey.description || 'SSH Key Pair Details'}
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-5 py-2">
					<div className="flex flex-col gap-2">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Public Key (Copy to remote server ~/.ssh/authorized_keys)</label>
							<Button variant="ghost" size="sm" onClick={handleCopyPub} className="h-7 text-xs gap-1 text-primary">
								{copiedPublic ? <Check className="w-3.5 h-3.5" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedPublic ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<Textarea
							readOnly
							value={activeKey.public_key || ''}
							className="h-24 text-xs font-mono bg-muted/20 border-border rounded-md p-3 resize-none break-all overflow-x-hidden"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Private Key</label>
							<div className="flex items-center gap-2">
								<Button
									variant="ghost"
									size="sm"
									onClick={() => setShowPrivate(!showPrivate)}
									className="h-7 text-xs gap-1"
								>
									{showPrivate ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
									{showPrivate ? 'Hide' : 'Reveal'}
								</Button>
								{showPrivate && (
									<Button variant="ghost" size="sm" onClick={handleCopyPriv} className="h-7 text-xs gap-1 text-primary">
										{copiedPrivate ? <Check className="w-3.5 h-3.5" /> : <Copy className="w-3.5 h-3.5" />}
										{copiedPrivate ? 'Copied' : 'Copy'}
									</Button>
								)}
							</div>
						</div>

						{showPrivate ? (
							<Textarea
								readOnly
								value={activeKey.private_key || 'Private key restricted or missing'}
								className="h-36 text-xs font-mono bg-muted/20 border-border rounded-md p-3 resize-none break-all overflow-x-hidden"
							/>
						) : (
							<div className="h-20 flex items-center justify-center bg-muted/20 border border-border rounded-md text-xs text-muted-foreground font-mono">
								•••••••••••••••• (Click "Reveal" to inspect private key)
							</div>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
