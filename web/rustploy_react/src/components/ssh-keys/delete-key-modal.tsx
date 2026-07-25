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
import {formatApiError} from '#/api/utils';
import {AlertTriangle, Trash2, RefreshCw} from 'lucide-react';

interface DeleteKeyModalProps {
	isOpen: boolean;
	sshKey: any | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function DeleteKeyModal({
	isOpen,
	sshKey,
	onClose,
	onSuccess,
}: DeleteKeyModalProps) {
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = $api.useMutation('delete', '/ssh-keys/{id}');

	const handleDelete = async () => {
		if (!sshKey?.id) return;

		setDeleting(true);
		try {
			await deleteMutation.mutateAsync({
				params: {
					path: {
						id: Number(sshKey.id),
					},
				},
			});
			toast.success(`SSH Key "${sshKey.name}" deleted successfully`);
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	if (!sshKey) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md w-full bg-card border-border p-6 shadow-2xl rounded-2xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-destructive flex items-center gap-2">
						<AlertTriangle className="w-5 h-5 text-destructive" />
						Delete SSH Key
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						This action cannot be undone. Are you sure you want to permanently delete this SSH Key pair?
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-3 py-3">
					<div className="p-3 bg-destructive/10 border border-destructive/20 rounded-xl flex flex-col gap-1">
						<span className="text-xs font-bold text-foreground">{sshKey.name}</span>
						<span className="text-[11px] text-muted-foreground font-mono truncate overflow-hidden max-w-full block">
							{sshKey.public_key || 'No public key preview'}
						</span>
					</div>
					<p className="text-xs text-muted-foreground">
						Deleting this SSH key may break automated deployments if it is attached to active server nodes or private Git repos.
					</p>
				</div>

				<div className="flex items-center justify-end gap-3 pt-3 border-t border-border/40">
					<Button variant="ghost" size="sm" onClick={onClose} className="h-9 text-xs">
						Cancel
					</Button>
					<Button
						variant="destructive"
						size="sm"
						onClick={handleDelete}
						disabled={deleting}
						className="h-9 text-xs font-bold gap-1.5 px-4"
					>
						{deleting ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <Trash2 className="w-3.5 h-3.5" />}
						{deleting ? 'Deleting...' : 'Delete Key'}
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
