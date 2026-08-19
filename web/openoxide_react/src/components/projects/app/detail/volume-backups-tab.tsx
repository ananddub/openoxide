import {useState} from 'react';
import {
	HardDrive,
	Plus,
	Play,
	RotateCcw,
	Trash2,
	Clock,
	ShieldCheck,
	RefreshCw,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogDescription,
	DialogFooter,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {CreateVolumeBackupModal} from './create-volume-backup-modal';

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

	// Filter backups for current application
	const appBackups = backups.filter(
		b => b.application_id === app?.id || b.app_name === app?.app_name,
	);

	// Mutations
	const runMutation = $api.useMutation('post', '/backups/volume/{id}/run');
	const restoreMutation = $api.useMutation(
		'post',
		'/backups/volume/{id}/restore',
	);
	const deleteMutation = $api.useMutation(
		'delete',
		'/backups/volume/{id}',
	);

	const handleRunBackup = async (id: number) => {
		setActiveActionId({id, action: 'run'});
		try {
			await runMutation.mutateAsync({params: {path: {id}}});
			toast.success('Volume backup execution started successfully');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setActiveActionId(null);
		}
	};

	const handleRestoreBackup = async (id: number) => {
		setActiveActionId({id, action: 'restore'});
		try {
			await restoreMutation.mutateAsync({
				params: {path: {id}},
				body: {backup_file: ''},
			});
			toast.success('Volume restore process initiated successfully');
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
			await deleteMutation.mutateAsync({
				params: {path: {id: deleteTargetId}},
			});
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
			<section className="flex items-center justify-between rounded-xl border border-border bg-card p-5 shadow-sm">
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
			<section className="overflow-hidden rounded-xl border border-border bg-card shadow-sm">
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
							<div
								key={b.id}
								className="flex flex-col justify-between gap-4 p-4 transition-colors hover:bg-muted/10 sm:flex-row sm:items-center">
								<div className="flex min-w-0 items-start gap-3">
									<div className="mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-primary/20 bg-primary/10 text-primary">
										<HardDrive className="h-4 w-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<span className="truncate text-xs font-bold text-foreground">
												{b.name}
											</span>
											{b.turn_off === 1 && (
												<span className="rounded border border-amber-500/20 bg-amber-500/10 px-1.5 py-0.5 font-mono text-[10px] text-amber-600 dark:text-amber-400">
													Pause container on backup
												</span>
											)}
										</div>
										<div className="mt-1.5 flex flex-wrap items-center gap-x-3 gap-y-1 text-[11px] font-medium text-muted-foreground">
											<span className="rounded bg-muted px-1.5 py-0.5 font-mono text-foreground/80">
												Volume: {b.volume_name}
											</span>
											<span className="flex items-center gap-1">
												<Clock className="h-3 w-3 text-muted-foreground" />{' '}
												Schedule: {b.cron_expression}
											</span>
											<span className="flex items-center gap-1">
												<ShieldCheck className="h-3 w-3 text-emerald-500" />{' '}
												S3 Prefix: {b.prefix}
											</span>
										</div>
									</div>
								</div>

								<div className="flex shrink-0 items-center gap-2 self-end sm:self-auto">
									<Button
										onClick={() => handleRunBackup(b.id)}
										disabled={activeActionId?.id === b.id}
										variant="outline"
										size="sm"
										className="flex h-8 items-center gap-1.5 rounded-lg border-border text-xs font-semibold text-foreground hover:bg-muted">
										{activeActionId?.id === b.id &&
										activeActionId?.action === 'run' ? (
											<RefreshCw className="h-3.5 w-3.5 animate-spin" />
										) : (
											<Play className="h-3.5 w-3.5" />
										)}
										Run Now
									</Button>
									<Button
										onClick={() => handleRestoreBackup(b.id)}
										disabled={activeActionId?.id === b.id}
										variant="outline"
										size="sm"
										className="flex h-8 items-center gap-1.5 rounded-lg border-border text-xs font-semibold text-foreground hover:bg-muted">
										{activeActionId?.id === b.id &&
										activeActionId?.action === 'restore' ? (
											<RefreshCw className="h-3.5 w-3.5 animate-spin" />
										) : (
											<RotateCcw className="h-3.5 w-3.5" />
										)}
										Restore
									</Button>
									<Button
										onClick={() => setDeleteTargetId(b.id)}
										variant="ghost"
										size="icon"
										className="h-8 w-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
										<Trash2 className="h-4 w-4" />
									</Button>
								</div>
							</div>
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

			{/* Custom Shadcn Delete Confirmation Modal */}
			<Dialog
				open={deleteTargetId !== null}
				onOpenChange={open => !open && setDeleteTargetId(null)}>
				<DialogContent className="sm:max-w-[420px]">
					<DialogHeader>
						<DialogTitle>Delete Volume Backup</DialogTitle>
						<DialogDescription>
							Are you sure you want to delete this volume backup
							configuration? Scheduled S3 snapshot backups will stop
							running.
						</DialogDescription>
					</DialogHeader>
					<DialogFooter className="pt-2">
						<Button
							variant="destructive"
							onClick={handleDelete}
							className="h-9 w-full px-6 text-xs font-bold sm:w-auto">
							Delete Backup Rule
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
