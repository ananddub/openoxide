import {useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {AlertTriangle, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {formatApiError} from '#/api/utils';
import {
	useComposeClearHistory,
	useComposeCleanupQueue,
	useComposeDelete,
	useComposeForceKill,
} from '#/api/live';
import {DangerAction} from '#/components/projects/detail/advanced/shared';

export function DangerSection({
	compose,
	projectId,
}: {
	compose: any;
	projectId?: number;
}) {
	const navigate = useNavigate();
	const deleteCompose = useComposeDelete();
	const clearHistory = useComposeClearHistory();
	const cleanupQueue = useComposeCleanupQueue();
	const forceKill = useComposeForceKill();
	const [confirmDelete, setConfirmDelete] = useState(false);
	const [busy, setBusy] = useState<string | null>(null);
	const [deleting, setDeleting] = useState(false);

	const run = async (
		key: string,
		fn: () => Promise<unknown>,
		message: string,
	) => {
		setBusy(key);
		try {
			await fn();
			toast.success(message);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setBusy(null);
		}
	};

	const handleDelete = async () => {
		setDeleting(true);
		try {
			await deleteCompose.mutateAsync(compose.id);
			toast.success('Compose stack deleted');
			navigate({
				to: `/projects/${projectId ?? compose.project_id ?? ''}`,
			});
		} catch (err) {
			toast.error(formatApiError(err));
			setDeleting(false);
		}
	};

	return (
		<section className="flex flex-col gap-4 rounded-xl bg-card p-6 shadow-sm">
			<div className="flex items-center gap-2 text-destructive">
				<AlertTriangle className="h-4 w-4" />
				<h3 className="text-sm font-bold">Danger Zone</h3>
			</div>
			<div className="flex flex-wrap gap-2">
				<DangerAction
					label="Clear Deployment History"
					busy={busy === 'history'}
					onConfirm={() =>
						run(
							'history',
							() => clearHistory.mutateAsync(compose.id),
							'Deployment history cleared',
						)
					}
				/>
				<DangerAction
					label="Cleanup Deployment Queue"
					busy={busy === 'queue'}
					onConfirm={() =>
						run(
							'queue',
							() => cleanupQueue.mutateAsync(compose.id),
							'Deployment queue cleaned',
						)
					}
				/>
				<DangerAction
					label="Force Kill Running Deployment"
					busy={busy === 'kill'}
					onConfirm={() =>
						run(
							'kill',
							() => forceKill.mutateAsync(compose.id),
							'Deployment killed',
						)
					}
				/>
			</div>
			<p className="text-xs text-muted-foreground">
				Permanently remove this compose stack and all associated
				containers, networks, and volumes.
			</p>
			{confirmDelete ? (
				<div className="flex flex-col gap-3 rounded-lg bg-destructive/5 p-4">
					<p className="text-xs font-semibold text-foreground">
						Are you sure? This action is absolutely irreversible.
					</p>
					<div className="flex items-center gap-2">
						<Button
							onClick={handleDelete}
							disabled={deleting}
							className="text-destructive-foreground h-9 bg-destructive px-6 text-xs font-semibold shadow-md hover:bg-destructive/90">
							{deleting ? 'Deleting…' : 'Yes, Delete stack'}
						</Button>
						<Button
							variant="outline"
							onClick={() => setConfirmDelete(false)}
							className="h-9 text-xs">
							Cancel
						</Button>
					</div>
				</div>
			) : (
				<div>
					<Button
						variant="outline"
						onClick={() => setConfirmDelete(true)}
						className="flex h-9 items-center gap-1.5 border-destructive/40 text-xs font-semibold text-destructive/80 hover:bg-destructive/5 hover:text-destructive">
						<Trash2 className="h-4 w-4" /> Delete Compose Stack
					</Button>
				</div>
			)}
		</section>
	);
}
