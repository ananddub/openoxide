import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {AlertTriangle, Trash2, RefreshCw} from 'lucide-react';
import {useDeleteServer} from '#/hooks/remote-servers/use-remote-servers';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface DeleteServerModalProps {
	isOpen: boolean;
	server: RemoteServerResponse | null;
	onClose: () => void;
	onSuccess: () => void;
}

export function DeleteServerModal({
	isOpen,
	server,
	onClose,
	onSuccess,
}: DeleteServerModalProps) {
	const {deleting, handleDelete} = useDeleteServer(Number(server?.id || 0), () => {
		onSuccess();
		onClose();
	});

	if (!server) return null;

	return (
		<Dialog open={isOpen} onOpenChange={onClose}>
			<DialogContent className="max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-destructive">
						<AlertTriangle className="w-5 h-5" />
						Delete Remote Server
					</DialogTitle>
					<DialogDescription>
						Are you sure you want to delete the remote server <strong className="text-foreground">{server.name}</strong> ({server.ip_address})?
					</DialogDescription>
				</DialogHeader>

				<div className="space-y-3 py-2">
					<div className="p-3 rounded-lg border border-destructive/20 bg-destructive/5 text-xs text-muted-foreground space-y-1">
						<p className="font-semibold text-destructive">Warning</p>
						<p>Any applications, compose stacks, or databases deployed on this remote server will lose automated management until reassigned.</p>
					</div>

					<div className="flex justify-end pt-2">
						<Button
							type="button"
							variant="destructive"
							onClick={handleDelete}
							disabled={deleting}
							className="w-full sm:w-auto h-9 px-6 font-bold text-xs gap-1.5"
						>
							{deleting ? (
								<>
									<RefreshCw className="w-3.5 h-3.5 animate-spin" />
									Deleting...
								</>
							) : (
								<>
									<Trash2 className="w-3.5 h-3.5" />
									Delete Server
								</>
							)}
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
