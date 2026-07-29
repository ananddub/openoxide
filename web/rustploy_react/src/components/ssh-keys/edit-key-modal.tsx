import {useState, useEffect} from 'react';
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
import {Label} from '#/components/ui/label';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {Key, RefreshCw, Download} from 'lucide-react';
import type {SshKeyResponse} from '#/types/api-helpers';
import {downloadKeyFile} from '#/utils/ssh-key-utils';

interface EditKeyModalProps {
	isOpen: boolean;
	sshKey: SshKeyResponse | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function EditKeyModal({
	isOpen,
	sshKey,
	onClose,
	onSuccess,
}: EditKeyModalProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [privateKey, setPrivateKey] = useState('');

	// Fetch full details when opened
	const {data: fullDetails, isLoading: loadingDetails} = $api.useQuery(
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
			enabled: !!sshKey?.id && isOpen,
		}
	);

	const active = fullDetails || sshKey;
	const activeId = active?.id;

	useEffect(() => {
		if (isOpen && active) {
			setName(active.name || '');
			setDescription(active.description || '');
			setPublicKey(active.public_key || '');
			setPrivateKey(active.private_key || '');
		}
	}, [activeId, isOpen, fullDetails]);

	const patchMutation = $api.useMutation('patch', '/ssh-keys/{id}', {
		onSuccess: () => {
			toast.success('SSH Key updated successfully');
			onSuccess();
			onClose();
		},
		onError: (err: any) => {
			toast.error(err?.message || 'Failed to update SSH key');
		},
	});

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!sshKey?.id) return;
		if (!name.trim()) {
			toast.error('Key name is required');
			return;
		}

		patchMutation.mutate({
			params: {
				path: {
					id: sshKey.id,
				},
			},
			body: {
				name: name.trim(),
				description: description.trim() || undefined,
				public_key: publicKey.trim() || undefined,
				private_key: privateKey.trim() || undefined,
			},
		});
	};

	if (!sshKey) return null;

	const formattedFileName = name.trim().toLowerCase().replace(/\s+/g, '_') || 'ssh_key';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full max-h-[90vh] overflow-y-auto bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-sm font-bold text-foreground flex items-center gap-2">
						<Key className="w-4 h-4 text-primary shrink-0" />
						<span>Edit SSH Key</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Update the name, description, or key material for "{sshKey.name}"
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 py-2">
					<div className="flex flex-col gap-1.5 w-full">
						<Label className="text-xs font-semibold text-foreground">Key Name *</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. Production Server Key"
							className="h-9 text-xs w-full bg-background border-border"
							required
						/>
					</div>

					<div className="flex flex-col gap-1.5 w-full">
						<Label className="text-xs font-semibold text-foreground">Description</Label>
						<Input
							value={description}
							onChange={e => setDescription(e.target.value)}
							placeholder="e.g. Used for main server deployment"
							className="h-9 text-xs w-full bg-background border-border"
						/>
					</div>

					<div className="flex flex-col gap-1.5 w-full">
						<Label className="text-xs font-semibold text-foreground">Public Key</Label>
						<Textarea
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
							rows={3}
							className="text-xs font-mono w-full max-h-28 overflow-y-auto bg-background border-border break-all leading-relaxed"
						/>
					</div>

					<div className="flex flex-col gap-1.5 w-full">
						<Label className="text-xs font-semibold text-foreground">Private Key</Label>
						<Textarea
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							rows={4}
							className="text-xs font-mono w-full max-h-36 overflow-y-auto bg-background border-border break-all leading-relaxed"
						/>
					</div>

					<div className="pt-3 flex flex-col sm:flex-row items-center justify-between gap-2 border-t border-border/50">
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

						<Button
							type="submit"
							disabled={patchMutation.isPending || loadingDetails}
							className="h-9 px-6 text-xs font-bold bg-primary hover:bg-primary/90 text-primary-foreground shadow-md w-full sm:w-auto"
						>
							{patchMutation.isPending ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1.5" /> : null}
							Save Changes
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
