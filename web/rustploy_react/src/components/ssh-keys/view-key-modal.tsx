import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {Copy, Check, Eye, EyeOff, Key, Terminal, Download} from 'lucide-react';
import type {SshKeyResponse} from '#/types/api-helpers';
import {downloadKeyFile} from '#/utils/ssh-key-utils';

interface ViewKeyModalProps {
	isOpen: boolean;
	sshKey: SshKeyResponse | null;
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

	// Format public key parts with syntax highlighting colors
	const renderHighlightedPublicKey = (pubKey: string) => {
		if (!pubKey) return null;
		const parts = pubKey.trim().split(/\s+/);
		const keyType = parts[0] || '';
		const keyBody = parts[1] || '';
		const keyComment = parts.slice(2).join(' ') || '';

		return (
			<span>
				<span className="text-amber-400 font-bold">{keyType}</span>{' '}
				<span className="text-emerald-300">{keyBody}</span>
				{keyComment && <span className="text-sky-400 font-semibold"> {keyComment}</span>}
			</span>
		);
	};

	if (!activeKey) return null;

	const formattedFileName = activeKey.name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full max-h-[90vh] overflow-y-auto bg-card border-border p-6 shadow-xl rounded-xl">
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
					{/* Public Key Block */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Public Key</label>
							<Button variant="ghost" size="sm" onClick={handleCopyPub} className="h-7 text-xs gap-1.5 px-2">
								{copiedPublic ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedPublic ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<div className="max-h-28 text-xs font-mono bg-zinc-950/90 border border-zinc-800 rounded-md p-3 break-all overflow-y-auto leading-relaxed text-zinc-100">
							{renderHighlightedPublicKey(activeKey.public_key || '')}
						</div>
					</div>

					{/* Private Key Block */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-foreground">Private Key</label>
							<div className="flex items-center gap-1.5">
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
							<div className="max-h-36 text-xs font-mono bg-zinc-950/90 border border-zinc-800 rounded-md p-3 break-all overflow-y-auto leading-relaxed text-emerald-400 whitespace-pre-wrap">
								{activeKey.private_key || 'No private key stored for this SSH key.'}
							</div>
						) : (
							<div className="h-10 bg-zinc-950/60 border border-zinc-800 rounded-md flex items-center px-3 text-xs text-zinc-500 italic font-mono">
								•••••••••••••••••••••••• (Hidden for security)
							</div>
						)}
					</div>

					{/* Server Authorization Command Block (Highlighted Shell Code) */}
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
							<div className="p-3 bg-zinc-950/90 border border-zinc-800 rounded-md text-[11px] font-mono break-all select-all leading-relaxed max-h-24 overflow-y-auto text-zinc-200">
								<span className="text-emerald-400 font-bold">mkdir</span> <span className="text-amber-400">-p</span> <span className="text-cyan-300">~/.ssh</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">echo</span> <span className="text-sky-300">"{activeKey.public_key?.trim()}"</span> <span className="text-amber-400">&gt;&gt;</span> <span className="text-cyan-300">~/.ssh/authorized_keys</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">chmod</span> <span className="text-amber-400">700</span> <span className="text-cyan-300">~/.ssh</span> <span className="text-zinc-500">&amp;&amp;</span> <span className="text-emerald-400 font-bold">chmod</span> <span className="text-amber-400">600</span> <span className="text-cyan-300">~/.ssh/authorized_keys</span>
							</div>
						</div>
					)}

					{/* Bottom Footer Download Row */}
					<div className="flex items-center gap-2 pt-3 border-t border-border/50">
						{activeKey.public_key && (
							<Button
								type="button"
								variant="outline"
								size="sm"
								onClick={() => downloadKeyFile(`${formattedFileName}.pub`, activeKey.public_key || '')}
								className="h-8 text-xs font-medium gap-1.5 border-border"
							>
								<Download className="w-3.5 h-3.5" />
								Download Pub
							</Button>
						)}
						{activeKey.private_key && (
							<Button
								type="button"
								variant="outline"
								size="sm"
								onClick={() => downloadKeyFile(`${formattedFileName}.pem`, activeKey.private_key || '')}
								className="h-8 text-xs font-medium gap-1.5 border-border"
							>
								<Download className="w-3.5 h-3.5 text-amber-500" />
								Download Private
							</Button>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
