import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {useSshKeyItems} from '#/api/live';
import {toast} from 'sonner';
import {Copy, Check, Key, Download} from 'lucide-react';
import type {SshKeyResponse} from '#/types/api-helpers';
import {downloadKeyFile} from '#/utils/ssh-key-utils';
import {ViewKeyPrivateBlock} from '#/components/ssh-keys/view-key-private-block';
import {AuthorizeCommandBlock} from '#/components/ssh-keys/authorize-command-block';

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
	const [copiedPublic, setCopiedPublic] = useState(false);

	// Live SSH keys query; the table-watch stream (ssh_keys table) keeps it
	// fresh, so no zustand slice involved.
	const {data: sshKeyItems = []} = useSshKeyItems();
	const fullKeyDetails = sshKeyItems.find(
		(k: any) => String(k.id) === String(sshKey?.id),
	);
	const activeKey = fullKeyDetails || sshKey;

	const handleCopyPub = () => {
		if (!activeKey?.public_key) return;
		navigator.clipboard.writeText(activeKey.public_key);
		setCopiedPublic(true);
		toast.success('Public SSH Key copied to clipboard');
		setTimeout(() => setCopiedPublic(false), 2000);
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
				<span className="font-bold text-amber-400">{keyType}</span>{' '}
				<span className="text-emerald-300">{keyBody}</span>
				{keyComment && (
					<span className="font-semibold text-sky-400"> {keyComment}</span>
				)}
			</span>
		);
	};

	if (!activeKey) return null;

	const formattedFileName =
		activeKey.name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="border- max-h-[90vh] w-full min-w-0 overflow-y-auto rounded-xl bg-card p-6 shadow-md sm:max-w-lg">
				<DialogHeader className="border-b  pb-3">
					<DialogTitle className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Key className="h-4 w-4 shrink-0 text-primary" />
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
							<label className="text-xs font-semibold text-foreground">
								Public Key
							</label>
							<Button
								variant="ghost"
								size="sm"
								onClick={handleCopyPub}
								className="h-7 gap-1.5 px-2 text-xs">
								{copiedPublic ? (
									<Check className="h-3.5 w-3.5 text-emerald-500" />
								) : (
									<Copy className="h-3.5 w-3.5" />
								)}
								{copiedPublic ? 'Copied' : 'Copy'}
							</Button>
						</div>
						<div className="max-h-28 overflow-y-auto rounded-md bg-zinc-950/90 p-3 font-mono text-xs leading-relaxed break-all text-zinc-100">
							{renderHighlightedPublicKey(activeKey.public_key || '')}
						</div>
					</div>

					{/* Private Key Block */}
					<ViewKeyPrivateBlock privateKey={activeKey.private_key} />

					{/* Server Authorization Command (reuses shared block) */}
					<AuthorizeCommandBlock publicKey={activeKey.public_key || ''} />

					{/* Bottom Footer Download Row */}
					<div className="flex items-center gap-2 border-t  pt-3">
						{activeKey.public_key && (
							<Button
								type="button"
								variant="outline"
								size="sm"
								onClick={() =>
									downloadKeyFile(
										`${formattedFileName}.pub`,
										activeKey.public_key || '',
									)
								}
								className="h-8 gap-1.5  text-xs font-medium">
								<Download className="h-3.5 w-3.5" />
								Download Pub
							</Button>
						)}
						{activeKey.private_key && (
							<Button
								type="button"
								variant="outline"
								size="sm"
								onClick={() =>
									downloadKeyFile(
										`${formattedFileName}.pem`,
										activeKey.private_key || '',
									)
								}
								className="h-8 gap-1.5  text-xs font-medium">
								<Download className="h-3.5 w-3.5 text-amber-500" />
								Download Private
							</Button>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
