import {useState} from 'react';
import {Calendar, Plus, Play, Trash2, Clock, RefreshCw, Power, X} from 'lucide-react';
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

export function SchedulesTab({app, schedules: passedSchedules, onRefresh}: SchedulesTabProps) {
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
			const appNameVal = app.app_name || app.appName || app.name || 'application';
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
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground">Schedules & Workers</h3>
					<p className="text-xs text-muted-foreground mt-1">Configure automated cron jobs and recurring trigger workers for this service</p>
				</div>
				<div className="flex items-center gap-2">
					<Button variant="outline" size="sm" onClick={() => onRefresh?.()} className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5">
						<RefreshCw className="w-3.5 h-3.5" /> Refresh
					</Button>
					<Button size="sm" onClick={() => setIsCreateOpen(true)} className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold flex items-center gap-1.5 h-8 text-xs">
						<Plus className="w-3.5 h-3.5" /> Create Cron Job
					</Button>
				</div>
			</section>

			{/* Jobs Table List */}
			<section className="bg-card border border-border rounded-xl overflow-hidden">
				{schedules.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground">
						<Calendar className="w-10 h-10 opacity-30 mb-3" />
						<p className="text-xs font-semibold">No cron jobs configured yet</p>
						<p className="text-[11px] mt-1 text-muted-foreground/70">Create a scheduled cron job to execute automated background tasks</p>
					</div>
				) : (
					<div className="divide-y divide-border/60">
						{schedules.map((job: any) => (
							<div key={job.id} className="flex flex-col sm:flex-row sm:items-center justify-between p-4 gap-4 hover:bg-muted/10 transition-colors">
								<div className="flex items-start gap-3 min-w-0">
									<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-muted-foreground border border-border/40 shrink-0 mt-0.5">
										<Calendar className="w-4 h-4" />
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2">
											<span className="text-xs font-semibold text-foreground">{job.name}</span>
											<span className={`text-[9px] font-bold px-1.5 py-0.5 rounded border uppercase tracking-wider ${
												job.enabled
													? 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20'
													: 'text-muted-foreground bg-muted/40 border-border'
											}`}>
												{job.enabled ? 'Active' : 'Paused'}
											</span>
										</div>
										<div className="flex flex-wrap items-center gap-x-3 gap-y-1 mt-1 text-[10px] text-muted-foreground font-medium">
											<span className="font-mono bg-muted/30 px-1.5 py-0.5 rounded text-foreground">{job.cron_expression}</span>
											<span className="font-mono text-muted-foreground truncate max-w-xs">{job.command}</span>
											<span className="flex items-center gap-1"><Clock className="w-3 h-3" /> {new Date(job.created_at * 1000).toLocaleDateString()}</span>
										</div>
									</div>
								</div>

								<div className="flex items-center gap-2 self-end sm:self-auto shrink-0">
									<Button variant="outline" size="sm" onClick={() => handleRunNow(job.id)} className="border-border text-foreground hover:bg-muted font-semibold h-7 text-[10px] flex items-center gap-1 rounded-md">
										<Play className="w-3 h-3" /> Run Now
									</Button>
									<Button variant="ghost" size="icon" onClick={() => handleToggleEnable(job)} title={job.enabled ? 'Pause Job' : 'Activate Job'} className="w-7 h-7 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md">
										<Power className="w-3.5 h-3.5" />
									</Button>
									<Button variant="ghost" size="icon" onClick={() => setDeleteId(job.id)} title="Delete Job" className="w-7 h-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-md">
										<Trash2 className="w-3.5 h-3.5" />
									</Button>
								</div>
							</div>
						))}
					</div>
				)}
			</section>

			{/* Create Cron Job Modal */}
			{isCreateOpen && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<form onSubmit={handleCreate} className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md p-5 flex flex-col gap-4">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">Create Cron Job</h3>
							<Button type="button" variant="ghost" size="sm" onClick={() => setIsCreateOpen(false)} className="h-7 w-7 p-0 text-muted-foreground hover:text-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>

						<div className="flex flex-col gap-3">
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">Job Name</span>
								<Input placeholder="Nightly Database Backup" value={name} onChange={e => setName(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">Cron Expression</span>
								<Input placeholder="0 2 * * *" value={cronExpression} onChange={e => setCronExpression(e.target.value)} className="bg-card border-border font-mono text-xs h-9" />
								<span className="text-[10px] text-muted-foreground">Standard 5-part cron syntax (e.g. 0 2 * * * for 2 AM daily)</span>
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">Command to Execute</span>
								<Input placeholder="npm run backup" value={command} onChange={e => setCommand(e.target.value)} className="bg-card border-border font-mono text-xs h-9" />
							</div>
						</div>

						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button type="submit" size="sm" disabled={creating} className="w-full sm:w-auto bg-primary hover:bg-primary/90 text-primary-foreground h-9 px-6 text-xs font-bold shadow-md">
								{creating ? 'Creating...' : 'Create Job'}
							</Button>
						</div>
					</form>
				</div>
			)}

			{/* Delete Schedule Confirmation Modal */}
			{deleteId !== null && (
				<div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm">
					<div className="bg-card border border-border rounded-xl w-full max-w-sm shadow-2xl p-5 flex flex-col gap-4 animate-in fade-in duration-150">
						<div className="flex items-center justify-between border-b border-border/60 pb-3">
							<h3 className="text-sm font-bold text-foreground">Delete Schedule</h3>
							<Button variant="ghost" size="icon" onClick={() => setDeleteId(null)} className="h-7 w-7 p-0 text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<p className="text-xs text-muted-foreground leading-relaxed">
							Are you sure you want to delete this schedule?
						</p>
						<div className="flex justify-end border-t border-border/60 pt-3">
							<Button
								size="sm"
								variant="destructive"
								disabled={deleting}
								onClick={confirmDelete}
								className="w-full sm:w-auto h-9 px-6 text-xs font-bold shadow-md"
							>
								{deleting ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
								{deleting ? 'Deleting...' : 'Delete Schedule'}
							</Button>
						</div>
					</div>
				</div>
			)}
		</div>
	);
}
