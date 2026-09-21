import {useState} from 'react';
import {HardDrive, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeleteConfirmDialog} from '#/components/shared/delete-confirm-dialog';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {CreateVolumeBackupModal} from './create-volume-backup-modal';
import {
	useVolumeBackupDelete,
	useVolumeBackupRestore,
	useVolumeBackupRun,
} from '#/api/live';
import {VolumeBackupRow} from './volume-backups-tab/volume-backup-row';

interface VolumeBackupsTabProps {
	app: any;
	backups?: any[];
	onRefresh?: () => void;
}

export function VolumeBackupsTab({
	app,
	backups: passedBackups,
	onRefresh,
}: VolumeBackupsTabProps) {
	const [showCreateModal, setShowCreateModal] = useState(false);
	const [deleteTargetId, setDeleteTargetId] = useState<number | null>(
		null,
	);
	const [activeActionId, setActiveActionId] = useState<{
		id: number;
		action: 'run' | 'restore';
	} | null>(null);

	const backups = Array.isArray(passedBackups) ? passedBackups : [];
	const appBackups = backups.filter(
		b => b.application_id === app?.id || b.app_name === app?.app_name,
	);

	const runMutation = useVolumeBackupRun();
	const restoreMutation = useVolumeBackupRestore();
	const deleteMutation = useVolumeBackupDelete();

	const runAction = async (
		id: number,
		action: 'run' | 'restore',
		fn: () => Promise<unknown>,
		success: string,
	) => {
		setActiveActionId({id, action});
		try {
			await fn();
			toast.success(success);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setActiveActionId(null);
		}
	};

	const handleDelete = async () => {
		if (!deleteTargetId) return;
		try {
			await deleteMutation.mutateAsync(deleteTargetId);
			toast.success('Volume backup configuration deleted');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleteTargetId(null);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="flex items-center justify-between rounded-xl bg-card p-5 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Volume Storage Snapshots
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Manage container volumes, S3 snapshot streaming, and
						point-in-time restores
					</p>
				</div>
				<Button
					onClick={() => setShowCreateModal(true)}
					size="sm"
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Plus className="h-4 w-4" /> Add Volume Backup
				</Button>
			</section>

			{/* Backups List */}
			<section className="overflow-hidden rounded-xl border bg-card">
				{appBackups.length === 0 ? (
					<div className="p-12 text-center text-muted-foreground">
						<HardDrive className="mx-auto mb-2 h-8 w-8 text-muted-foreground/50" />
						<p className="text-sm font-semibold text-foreground">
							No volume backups configured
						</p>
						<p className="mt-1 text-xs text-muted-foreground">
							Create a volume backup rule to stream S3 snapshots of your
							container data
						</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{appBackups.map(b => (
							<VolumeBackupRow
								key={b.id}
								backup={b}
								activeAction={
									activeActionId && activeActionId.id === b.id
										? activeActionId.action
										: null
								}
								onRun={() =>
									void runAction(
										b.id,
										'run',
										() => runMutation.mutateAsync(b.id),
										'Volume backup execution started successfully',
									)
								}
								onRestore={() =>
									void runAction(
										b.id,
										'restore',
										() =>
											restoreMutation.mutateAsync({
												id: b.id,
												backup_file: '',
											}),
										'Volume restore process initiated successfully',
									)
								}
								onDelete={() => setDeleteTargetId(b.id)}
							/>
						))}
					</div>
				)}
			</section>

			{/* Create Modal */}
			<CreateVolumeBackupModal
				open={showCreateModal}
				onOpenChange={setShowCreateModal}
				app={app}
				onSuccess={() => onRefresh?.()}
			/>

			<DeleteConfirmDialog
				open={deleteTargetId !== null}
				onOpenChange={open => !open && setDeleteTargetId(null)}
				title="Delete Volume Backup"
				description="Are you sure you want to delete this volume backup configuration? Scheduled S3 snapshot backups will stop running."
				onConfirm={handleDelete}
			/>
		</div>
	);
}
