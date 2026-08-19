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
	const {deleting, handleDelete} = useDeleteServer(
		Number(server?.id || 0),
		() => {
			onSuccess();
			onClose();
		},
	);

	if (!server) return null;

	return (
		<Dialog open={isOpen} onOpenChange={onClose}>
			<DialogContent className="max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-destructive">
						<AlertTriangle className="h-5 w-5" />
						Delete Remote Server
					</DialogTitle>
					<DialogDescription>
						Are you sure you want to delete the remote server{' '}
						<strong className="text-foreground">{server.name}</strong> (
						{server.ip_address})?
					</DialogDescription>
				</DialogHeader>

				<div className="space-y-3 py-2">
					<div className="space-y-1 rounded-lg border border-destructive/20 bg-destructive/5 p-3 text-xs text-muted-foreground">
						<p className="font-semibold text-destructive">Warning</p>
						<p>
							Any applications, compose stacks, or databases deployed on
							this remote server will lose automated management until
							reassigned.
						</p>
					</div>

					<div className="flex justify-end pt-2">
						<Button
							type="button"
							variant="destructive"
							onClick={handleDelete}
							disabled={deleting}
							className="h-9 w-full gap-1.5 px-6 text-xs font-bold sm:w-auto">
							{deleting ? (
								<>
									<RefreshCw className="h-3.5 w-3.5 animate-spin" />
									Deleting...
								</>
							) : (
								<>
									<Trash2 className="h-3.5 w-3.5" />
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
