import {useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {AlertTriangle, Trash2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	useApplicationClearHistory,
	useApplicationCleanupQueue,
	useApplicationDelete,
	useApplicationForceKill,
} from '#/api/live';
import {DangerAction} from '#/components/projects/detail/advanced/shared';

import {formatApiError} from '#/api/utils';

export function DangerSection({app}: {app: any}) {
	const navigate = useNavigate();
	const deleteApplication = useApplicationDelete();
	const clearHistory = useApplicationClearHistory();
	const cleanupQueue = useApplicationCleanupQueue();
	const forceKill = useApplicationForceKill();
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

	const handleDeleteApp = async () => {
		setDeleting(true);
		try {
			await deleteApplication.mutateAsync(app.id);
			toast.success('Application deleted');
			navigate({to: `/projects/${app.project_id}`});
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
							() => clearHistory.mutateAsync(app.id),
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
							() => cleanupQueue.mutateAsync(app.id),
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
							() => forceKill.mutateAsync(app.id),
							'Deployment killed',
						)
					}
				/>
			</div>
			<p className="text-xs text-muted-foreground">
				Permanently remove this application and all associated data,
				settings, domains, and routes.
			</p>
			{confirmDelete ? (
				<div className="flex flex-col gap-3 rounded-lg bg-destructive/5 p-4">
					<p className="text-xs font-semibold text-foreground">
						Are you sure? This action is absolutely irreversible.
					</p>
					<div className="flex items-center gap-2">
						<Button
							onClick={handleDeleteApp}
							disabled={deleting}
							className="text-destructive-foreground h-9 bg-destructive px-6 text-xs font-semibold shadow-md hover:bg-destructive/90">
							{deleting ? 'Deleting…' : 'Yes, Delete application'}
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
						<Trash2 className="h-4 w-4" /> Delete Application
					</Button>
				</div>
			)}
		</section>
	);
}
