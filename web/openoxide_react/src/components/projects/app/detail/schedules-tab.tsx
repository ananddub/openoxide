import {useState} from 'react';
import {
	Calendar,
	Plus,
	Play,
	Trash2,
	Clock,
	RefreshCw,
	Power,
	X,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface SchedulesTabProps {
	app: any;
	schedules?: any[];
	onRefresh?: () => void;
}

export function SchedulesTab({
	app,
	schedules: passedSchedules,
	onRefresh,
}: SchedulesTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [name, setName] = useState('');
	const [cronExpression, setCronExpression] = useState('0 2 * * *');
	const [command, setCommand] = useState('echo "running scheduled task"');
	const [creating, setCreating] = useState(false);

	const [deleteId, setDeleteId] = useState<number | null>(null);
	const [deleting, setDeleting] = useState(false);

	const schedules = Array.isArray(passedSchedules) ? passedSchedules : [];

	// Mutations
	const createMutation = $api.useMutation('post', '/schedules');
	const runMutation = $api.useMutation('post', '/schedules/{id}/run');
	const patchMutation = $api.useMutation('patch', '/schedules/{id}');
	const deleteMutation = $api.useMutation('delete', '/schedules/{id}');

	const handleCreate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !cronExpression || !command) {
			toast.error('Please fill in all required fields');
			return;
		}
		setCreating(true);
		try {
			const appNameVal =
				app.app_name || app.appName || app.name || 'application';
			await createMutation.mutateAsync({
				body: {
					name,
					cron_expression: cronExpression,
					command,
					application_id: app.id,
					app_name: appNameVal,
					enabled: 1,
					schedule_type: 'APPLICATION',
					schedule_action: 'EXEC',
					shell_type: 'bash',
				} as any,
			});
			toast.success('Schedule created successfully');
			setIsCreateOpen(false);
			setName('');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setCreating(false);
		}
	};

	const handleRunNow = async (id: number) => {
		try {
			await runMutation.mutateAsync({params: {path: {id}}});
			toast.success('Schedule triggered manually');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleToggleEnable = async (job: any) => {
		const newStatus = job.enabled ? 0 : 1;
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: job.id}},
				body: {enabled: newStatus} as any,
			});
			toast.success(`Schedule ${newStatus ? 'enabled' : 'paused'}`);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const confirmDelete = async () => {
		if (!deleteId) return;
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync({params: {path: {id: deleteId}}});
			toast.success('Schedule deleted successfully');
			setDeleteId(null);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header */}
			<section className="flex items-center justify-between rounded-xl border border-border bg-card p-5">
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Schedules & Workers
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure automated cron jobs and recurring trigger workers for
						this service
					</p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={() => onRefresh?.()}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw className="h-3.5 w-3.5" /> Refresh
					</Button>
					<Button
						size="sm"
						onClick={() => setIsCreateOpen(true)}
						className="flex h-8 items-center gap-1.5 bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/90">
						<Plus className="h-3.5 w-3.5" /> Create Cron Job
					</Button>
				</div>
			</section>

			{/* Jobs Table List */}
			<section className="overflow-hidden rounded-xl border border-border bg-card">
				{schedules.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
						<Calendar className="mb-3 h-10 w-10 opacity-30" />
						<p className="text-xs font-semibold">
							No cron jobs configured yet
						</p>
						<p className="mt-1 text-[11px] text-muted-foreground/70">
							Create a scheduled cron job to execute automated background
							tasks
						</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{schedules.map((job: any) => (
							<div
								key={job.id}
								className="flex flex-col justify-between gap-4 p-4 transition-colors hover:bg-muted/10 sm:flex-row sm:items-center">
								<div className="flex min-w-0 items-start gap-3">
									<div className="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-border/40 bg-secondary text-muted-foreground">
										<Calendar className="h-4 w-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<span className="text-xs font-semibold text-foreground">
												{job.name}
											</span>
											<span
												className={`rounded border px-1.5 py-0.5 text-[9px] font-bold tracking-wider uppercase ${
													job.enabled
														? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-500'
														: 'border-border bg-muted/40 text-muted-foreground'
												}`}>
												{job.enabled ? 'Active' : 'Paused'}
											</span>
										</div>
										<div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-[10px] font-medium text-muted-foreground">
											<span className="rounded bg-muted/30 px-1.5 py-0.5 font-mono text-foreground">
												{job.cron_expression}
											</span>
											<span className="max-w-xs truncate font-mono text-muted-foreground">
												{job.command}
											</span>
											<span className="flex items-center gap-1">
												<Clock className="h-3 w-3" />{' '}
												{new Date(
													job.created_at * 1000,
												).toLocaleDateString()}
											</span>
										</div>
									</div>
								</div>

								<div className="flex shrink-0 items-center gap-2 self-end sm:self-auto">
									<Button
										variant="outline"
										size="sm"
										onClick={() => handleRunNow(job.id)}
										className="flex h-7 items-center gap-1 rounded-md border-border text-[10px] font-semibold text-foreground hover:bg-muted">
										<Play className="h-3 w-3" /> Run Now
									</Button>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => handleToggleEnable(job)}
										title={job.enabled ? 'Pause Job' : 'Activate Job'}
										className="h-7 w-7 rounded-md text-muted-foreground hover:bg-muted hover:text-foreground">
										<Power className="h-3.5 w-3.5" />
									</Button>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => setDeleteId(job.id)}
										title="Delete Job"
										className="h-7 w-7 rounded-md text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
										<Trash2 className="h-3.5 w-3.5" />
									</Button>
								</div>
							</div>
						))}
					</div>
				)}
			</section>

			{/* Create Cron Job Modal */}
			{isCreateOpen && (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
					<form
						onSubmit={handleCreate}
						className="flex w-full max-w-md flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-2xl">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">
								Create Cron Job
							</h3>
							<Button
								type="button"
								variant="ghost"
								size="sm"
								onClick={() => setIsCreateOpen(false)}
								className="h-7 w-7 p-0 text-muted-foreground hover:text-foreground">
								<X className="h-4 w-4" />
							</Button>
						</div>

						<div className="flex flex-col gap-3">
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">
									Job Name
								</span>
								<Input
									placeholder="Nightly Database Backup"
									value={name}
									onChange={e => setName(e.target.value)}
									className="h-9 border-border bg-card text-xs"
								/>
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">
									Cron Expression
								</span>
								<Input
									placeholder="0 2 * * *"
									value={cronExpression}
									onChange={e => setCronExpression(e.target.value)}
									className="h-9 border-border bg-card font-mono text-xs"
								/>
								<span className="text-[10px] text-muted-foreground">
									Standard 5-part cron syntax (e.g. 0 2 * * * for 2 AM
									daily)
								</span>
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">
									Command to Execute
								</span>
								<Input
									placeholder="npm run backup"
									value={command}
									onChange={e => setCommand(e.target.value)}
									className="h-9 border-border bg-card font-mono text-xs"
								/>
							</div>
						</div>

						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button
								type="submit"
								size="sm"
								disabled={creating}
								className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
								{creating ? 'Creating...' : 'Create Job'}
							</Button>
						</div>
					</form>
				</div>
			)}

			{/* Delete Schedule Confirmation Modal */}
			{deleteId !== null && (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
					<div className="flex w-full max-w-sm animate-in flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-2xl duration-150 fade-in">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">
								Delete Schedule
							</h3>
							<Button
								variant="ghost"
								size="icon"
								onClick={() => setDeleteId(null)}
								className="h-7 w-7 p-0 text-muted-foreground">
								<X className="h-4 w-4" />
							</Button>
						</div>
						<p className="text-xs leading-relaxed text-muted-foreground">
							Are you sure you want to delete this schedule?
						</p>
						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button
								size="sm"
								variant="destructive"
								disabled={deleting}
								onClick={confirmDelete}
								className="h-9 w-full px-6 text-xs font-bold shadow-md sm:w-auto">
								{deleting ? (
									<RefreshCw className="mr-1 h-3.5 w-3.5 animate-spin" />
								) : null}
								{deleting ? 'Deleting...' : 'Delete Schedule'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
