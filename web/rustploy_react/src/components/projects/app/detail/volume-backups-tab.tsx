import {useState} from 'react';
import {HardDrive, Plus, Play, RotateCcw, Trash2, Clock, ShieldCheck, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {CreateVolumeBackupModal} from './create-volume-backup-modal';

interface VolumeBackupsTabProps {
	app: any;
	backups?: any[];
	onRefresh?: () => void;
}

export function VolumeBackupsTab({app, backups: passedBackups, onRefresh}: VolumeBackupsTabProps) {
	const [showCreateModal, setShowCreateModal] = useState(false);
	const [deleteTargetId, setDeleteTargetId] = useState<number | null>(null);
	const [activeActionId, setActiveActionId] = useState<{id: number; action: 'run' | 'restore'} | null>(null);

	const backups = Array.isArray(passedBackups) ? passedBackups : [];

	// Filter backups for current application
	const appBackups = backups.filter(b => b.application_id === app?.id || b.app_name === app?.app_name);

	// Mutations
	const runMutation = $api.useMutation('post', '/backups/volume/{id}/run');
	const restoreMutation = $api.useMutation('post', '/backups/volume/{id}/restore');
	const deleteMutation = $api.useMutation('delete', '/backups/volume/{id}');

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
			await restoreMutation.mutateAsync({params: {path: {id}}, body: {backup_file: ''}});
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
			await deleteMutation.mutateAsync({params: {path: {id: deleteTargetId}}});
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
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">Volume Storage Snapshots</h3>
					<p className="text-xs text-muted-foreground mt-1">Manage container volumes, S3 snapshot streaming, and point-in-time restores</p>
				</div>
				<Button
					onClick={() => setShowCreateModal(true)}
					size="sm"
					className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 text-xs rounded-lg"
				>
					<Plus className="w-4 h-4" /> Add Volume Backup
				</Button>
			</section>

			{/* Backups List */}
			<section className="bg-card border border-border rounded-xl overflow-hidden shadow-sm">
				{appBackups.length === 0 ? (
					<div className="p-12 text-center text-muted-foreground">
						<HardDrive className="w-8 h-8 mx-auto mb-2 text-muted-foreground/50" />
						<p className="text-sm font-semibold text-foreground">No volume backups configured</p>
						<p className="text-xs text-muted-foreground mt-1">Create a volume backup rule to stream S3 snapshots of your container data</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{appBackups.map(b => (
							<div key={b.id} className="flex flex-col sm:flex-row sm:items-center justify-between p-4 gap-4 hover:bg-muted/10 transition-colors">
								<div className="flex items-start gap-3 min-w-0">
									<div className="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary border border-primary/20 shrink-0 mt-0.5">
										<HardDrive className="w-4 h-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<span className="text-xs font-bold text-foreground truncate">{b.name}</span>
											{b.turn_off === 1 && (
												<span className="text-[10px] bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20 px-1.5 py-0.5 rounded font-mono">
													Pause container on backup
												</span>
											)}
										</div>
										<div className="flex flex-wrap items-center gap-x-3 gap-y-1 mt-1.5 text-[11px] text-muted-foreground font-medium">
											<span className="font-mono bg-muted px-1.5 py-0.5 rounded text-foreground/80">Volume: {b.volume_name}</span>
											<span className="flex items-center gap-1"><Clock className="w-3 h-3 text-muted-foreground" /> Schedule: {b.cron_expression}</span>
											<span className="flex items-center gap-1"><ShieldCheck className="w-3 h-3 text-emerald-500" /> S3 Prefix: {b.prefix}</span>
										</div>
									</div>
								</div>

								<div className="flex items-center gap-2 self-end sm:self-auto shrink-0">
									<Button
										onClick={() => handleRunBackup(b.id)}
										disabled={activeActionId?.id === b.id}
										variant="outline"
										size="sm"
										className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5 rounded-lg"
									>
										{activeActionId?.id === b.id && activeActionId?.action === 'run' ? (
											<RefreshCw className="w-3.5 h-3.5 animate-spin" />
										) : (
											<Play className="w-3.5 h-3.5" />
										)}
										Run Now
									</Button>
									<Button
										onClick={() => handleRestoreBackup(b.id)}
										disabled={activeActionId?.id === b.id}
										variant="outline"
										size="sm"
										className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5 rounded-lg"
									>
										{activeActionId?.id === b.id && activeActionId?.action === 'restore' ? (
											<RefreshCw className="w-3.5 h-3.5 animate-spin" />
										) : (
											<RotateCcw className="w-3.5 h-3.5" />
										)}
										Restore
									</Button>
									<Button
										onClick={() => setDeleteTargetId(b.id)}
										variant="ghost"
										size="icon"
										className="w-8 h-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
									>
										<Trash2 className="w-4 h-4" />
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
			<Dialog open={deleteTargetId !== null} onOpenChange={open => !open && setDeleteTargetId(null)}>
				<DialogContent className="sm:max-w-[420px]">
					<DialogHeader>
						<DialogTitle>Delete Volume Backup</DialogTitle>
						<DialogDescription>
							Are you sure you want to delete this volume backup configuration? Scheduled S3 snapshot backups will stop running.
						</DialogDescription>
					</DialogHeader>
					<DialogFooter className="pt-2">
						<Button variant="destructive" onClick={handleDelete} className="w-full sm:w-auto h-9 px-6 font-bold text-xs">
							Delete Backup Rule
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
