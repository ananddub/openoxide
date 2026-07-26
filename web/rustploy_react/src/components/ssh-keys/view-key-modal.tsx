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
			toast.success('Public SSH Key copied to clipboard');
			setTimeout(() => setCopiedPublic(false), 2000);
		}
	};

	const handleCopyPriv = () => {
		if (activeKey?.private_key) {
			navigator.clipboard.writeText(activeKey.private_key);
			setCopiedPrivate(true);
			toast.success('Private SSH Key copied to clipboard');
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
			toast.success('Server authorization command copied');
			setTimeout(() => setCopiedCmd(false), 2000);
		}
	};

	if (!activeKey) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-sm font-bold text-foreground flex items-center gap-2">
						<Key className="w-4 h-4 text-primary shrink-0" />
						<span>{activeKey.name}</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{activeKey.description || 'SSH Key Details'}
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 py-2">
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Public Key</label>
							<Button variant="ghost" size="sm" onClick={handleCopyPub} className="h-7 text-xs gap-1.5 px-2">
								{copiedPublic ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedPublic ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<Textarea
							readOnly
							value={activeKey.public_key || ''}
							className="h-20 text-xs font-mono bg-muted/30 border-border rounded-md p-3 resize-none break-all"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Private Key</label>
							<div className="flex items-center gap-2">
								<Button
									variant="ghost"
									size="sm"
									onClick={() => setShowPrivate(!showPrivate)}
									className="h-7 text-xs gap-1.5 px-2"
								>
									{showPrivate ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
									{showPrivate ? 'Hide' : 'Reveal'}
								</Button>
								{showPrivate && (
									<Button variant="ghost" size="sm" onClick={handleCopyPriv} className="h-7 text-xs gap-1.5 px-2">
										{copiedPrivate ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
										{copiedPrivate ? 'Copied' : 'Copy'}
									</Button>
								)}
							</div>
						</div>

						{showPrivate ? (
							<Textarea
								readOnly
								value={activeKey.private_key || ''}
								className="h-28 text-xs font-mono bg-muted/30 border-border rounded-md p-3 resize-none break-all"
							/>
						) : (
							<div className="h-10 bg-muted/20 border border-border/50 rounded-md flex items-center px-3 text-xs text-muted-foreground italic font-mono">
								•••••••••••••••••••••••• (Hidden for security)
							</div>
						)}
					</div>

					{/* Server Authorization Command Section (Placed at the very bottom) */}
					{setupCommand && (
						<div className="flex flex-col gap-1.5 pt-2 border-t border-border/40">
							<div className="flex items-center justify-between">
								<label className="text-xs font-semibold text-foreground flex items-center gap-1.5">
									<Terminal className="w-3.5 h-3.5 text-primary shrink-0" />
									<span>Authorize Key on Remote Server</span>
								</label>
								<Button variant="outline" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-medium gap-1.5 px-2.5">
									{copiedCmd ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
									{copiedCmd ? 'Copied' : 'Copy Command'}
								</Button>
							</div>
							<div className="p-2.5 bg-muted/40 border border-border/50 rounded-md text-[11px] font-mono text-muted-foreground break-all select-all leading-relaxed max-h-20 overflow-y-auto">
								{setupCommand}
							</div>
						</div>
					)}
				</div>

				<div className="flex justify-end pt-2 border-t border-border/50">
					<Button variant="outline" size="sm" onClick={onClose} className="h-8 text-xs px-4">
						Close
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
