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
import {Copy, Check, Eye, EyeOff, Key, Terminal} from 'lucide-react';

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
	const [copiedCmd, setCopiedCmd] = useState(false);

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

	const setupCommand = activeKey?.public_key
		? `mkdir -p ~/.ssh && echo "${activeKey.public_key.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`
		: '';

	const handleCopyCommand = () => {
		if (setupCommand) {
			navigator.clipboard.writeText(setupCommand);
			setCopiedCmd(true);
			toast.success('Server setup command copied!');
			setTimeout(() => setCopiedCmd(false), 2000);
		}
	};

	if (!activeKey) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card border-border p-6 shadow-xl rounded-xl min-w-0 overflow-hidden">
				<DialogHeader className="pb-3 border-b border-border/40 min-w-0 w-full">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2 truncate min-w-0">
						<Key className="w-5 h-5 text-primary shrink-0" />
						<span className="truncate">{activeKey.name}</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground truncate">
						{activeKey.description || 'SSH Key Pair Details'}
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 py-2 min-w-0 w-full overflow-hidden">
					<div className="flex flex-col gap-2 min-w-0 w-full">
						<div className="flex items-center justify-between gap-2 min-w-0">
							<label className="text-xs font-semibold text-foreground truncate">Public Key</label>
							<Button variant="ghost" size="sm" onClick={handleCopyPub} className="h-7 text-xs gap-1 text-primary shrink-0">
								{copiedPublic ? <Check className="w-3.5 h-3.5" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedPublic ? 'Copied' : 'Copy Key'}
							</Button>
						</div>
						<Textarea
							readOnly
							value={activeKey.public_key || ''}
							className="h-24 text-xs font-mono bg-muted/20 border-border rounded-md p-3 resize-none break-all [word-break:break-all] whitespace-pre-wrap w-full max-w-full overflow-y-auto leading-relaxed"
						/>
					</div>

					{setupCommand && (
						<div className="bg-primary/5 p-3 rounded-xl border border-primary/20 flex flex-col gap-2 min-w-0 w-full">
							<div className="flex items-center justify-between gap-2 min-w-0">
								<span className="text-xs font-bold text-foreground flex items-center gap-1.5 truncate">
									<Terminal className="w-4 h-4 text-primary shrink-0" />
									Run on Remote Server to Authorize Key:
								</span>
								<Button variant="secondary" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-semibold gap-1 shrink-0">
									{copiedCmd ? <Check className="w-3.5 h-3.5 text-primary" /> : <Copy className="w-3.5 h-3.5" />}
									{copiedCmd ? 'Copied' : 'Copy Command'}
								</Button>
							</div>
							<div className="p-2.5 bg-background/80 border border-border/60 rounded-lg text-[11px] font-mono text-foreground break-all [word-break:break-all] select-all max-h-24 overflow-y-auto leading-relaxed">
								{setupCommand}
							</div>
						</div>
					)}

					<div className="flex flex-col gap-2 min-w-0 w-full">
						<div className="flex items-center justify-between gap-2 min-w-0">
							<label className="text-xs font-semibold text-foreground truncate">Private Key</label>
							<div className="flex items-center gap-2 shrink-0">
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
										{copiedPrivate ? 'Copied' : 'Copy Key'}
									</Button>
								)}
							</div>
						</div>

						{showPrivate ? (
							<Textarea
								readOnly
								value={activeKey.private_key || 'Private key restricted or missing'}
								className="h-32 text-xs font-mono bg-muted/20 border-border rounded-md p-3 resize-none break-all [word-break:break-all] whitespace-pre-wrap w-full max-w-full overflow-y-auto leading-relaxed"
							/>
						) : (
							<div className="h-16 flex items-center justify-center bg-muted/20 border border-border rounded-md text-xs text-muted-foreground font-mono w-full">
								•••••••••••••••• (Click "Reveal" to inspect private key)
							</div>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
