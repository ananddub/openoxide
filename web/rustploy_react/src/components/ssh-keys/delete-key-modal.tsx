import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {AlertTriangle, Trash2, RefreshCw} from 'lucide-react';
import {useDeleteSshKey} from '#/hooks/ssh-keys/use-ssh-keys';
import type {SshKeyResponse} from '#/types/api-helpers';

interface DeleteKeyModalProps {
	isOpen: boolean;
	sshKey: SshKeyResponse | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function DeleteKeyModal({
	isOpen,
	sshKey,
	onClose,
	onSuccess,
}: DeleteKeyModalProps) {
	const {deleting, handleDelete} = useDeleteSshKey(Number(sshKey?.id || 0), () => {
		onSuccess();
		onClose();
	});

	if (!sshKey) return null;

	return (
		<Dialog open={isOpen} onOpenChange={onClose}>
			<DialogContent className="max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-destructive">
						<AlertTriangle className="w-5 h-5" />
						Delete SSH Key
					</DialogTitle>
					<DialogDescription>
						Are you sure you want to delete the SSH key <strong className="text-foreground">{sshKey.name}</strong>?
					</DialogDescription>
				</DialogHeader>

				<div className="space-y-3 py-2">
					<div className="p-3 rounded-lg border border-destructive/20 bg-destructive/5 text-xs text-muted-foreground space-y-1">
						<p className="font-semibold text-destructive">Warning</p>
						<p>Any remote servers or deployments relying on this SSH key will lose automated deployment access until updated.</p>
					</div>

					<div className="flex justify-end pt-2">
						<Button
							type="button"
							variant="destructive"
							onClick={handleDelete}
							disabled={deleting}
							className="w-full sm:w-auto h-9 px-6 font-bold text-xs"
						>
							{deleting ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : <Trash2 className="w-3.5 h-3.5 mr-1" />}
							{deleting ? 'Deleting...' : 'Delete Key'}
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
