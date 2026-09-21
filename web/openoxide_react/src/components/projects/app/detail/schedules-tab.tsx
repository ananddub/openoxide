import {useState} from 'react';
import {Calendar, Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useScheduleDelete,
	useScheduleRun,
	useScheduleUpdate,
} from '#/api/live';
import {DeleteConfirmDialog} from '#/components/shared/delete-confirm-dialog';
import {CreateCronDialog} from './schedules-tab/create-cron-dialog';
import {ScheduleJobRow} from './schedules-tab/schedule-job-row';

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
	const [deleteId, setDeleteId] = useState<number | null>(null);

	const schedules = Array.isArray(passedSchedules) ? passedSchedules : [];

	const runMutation = useScheduleRun();
	const patchMutation = useScheduleUpdate();
	const deleteMutation = useScheduleDelete();

	const handleRunNow = async (id: number) => {
		try {
			await runMutation.mutateAsync(id);
			toast.success('Schedule triggered manually');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleToggleEnable = async (job: any) => {
		const newStatus = job.enabled ? 0 : 1;
		try {
			await patchMutation.mutateAsync({id: job.id, enabled: newStatus});
			toast.success(`Schedule ${newStatus ? 'enabled' : 'paused'}`);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const confirmDelete = async () => {
		if (!deleteId) return;
		try {
			await deleteMutation.mutateAsync(deleteId);
			toast.success('Schedule deleted successfully');
			setDeleteId(null);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Action Header */}
			<section className="flex items-center justify-between rounded-xl bg-card p-5">
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
						className="flex h-8 items-center gap-1.5  text-xs font-semibold text-foreground hover:bg-muted">
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
			<section className="overflow-hidden rounded-xl bg-card">
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
							<ScheduleJobRow
								key={job.id}
								job={job}
								onRun={handleRunNow}
								onToggle={handleToggleEnable}
								onDelete={setDeleteId}
							/>
						))}
					</div>
				)}
			</section>

			<CreateCronDialog
				app={app}
				open={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				onCreated={onRefresh}
			/>

			<DeleteConfirmDialog
				open={deleteId !== null}
				onOpenChange={open => !open && setDeleteId(null)}
				title="Delete Schedule"
				description="Are you sure you want to delete this schedule?"
				onConfirm={confirmDelete}
			/>
		</div>
	);
}
