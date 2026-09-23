import {useState, useMemo} from 'react';
import {Calendar, Plus, RefreshCw, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListEmpty,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewGrid,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useScheduleDelete,
	useScheduleRun,
	useScheduleUpdate,
} from '#/api/live';
import {CreateCronDialog} from './schedules-tab/create-cron-dialog';
import {
	ScheduleJobCard,
	ScheduleJobRow,
} from './schedules-tab/schedule-job-row';

interface SchedulesTabProps {
	app: any;
	schedules?: any[];
	onRefresh?: () => void;
}

/**
 * Service → Schedules — the SSH Keys grammar: section title with the one
 * primary action, toolbar (search + grid/list) and cron jobs as grid cards
 * or one bordered list card.
 */
export function SchedulesTab({
	app,
	schedules: passedSchedules,
	onRefresh,
}: SchedulesTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [deleteId, setDeleteId] = useState<number | null>(null);
	const [query, setQuery] = useState('');
	const [view, setView] = useListView('service-schedules-view');

	const allSchedules = Array.isArray(passedSchedules)
		? passedSchedules
		: [];
	const q = query.trim().toLowerCase();
	const schedules = useMemo(
		() =>
			q
				? allSchedules.filter((s: any) =>
						[s?.name, s?.command, s?.cron_expression]
							.filter(Boolean)
							.join(' ')
							.toLowerCase()
							.includes(q),
					)
				: allSchedules,
		[allSchedules, q],
	);
	const hasAny = allSchedules.length > 0;

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

	const createButton = (
		<Button onClick={() => setIsCreateOpen(true)} className="h-9 gap-1.5">
			<Plus className="size-3.5" />
			New Cron Job
		</Button>
	);

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Schedules"
				description="Cron jobs that run background tasks inside this service."
				action={
					<>
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onRefresh?.()}
							title="Refresh"
							className="size-9 text-muted-foreground hover:text-foreground">
							<RefreshCw className="size-4" />
						</Button>
						{createButton}
					</>
				}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={query}
					onChange={setQuery}
					placeholder="Search schedules…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{!hasAny ? (
				<ListEmpty
					icon={Calendar}
					title="No cron jobs yet"
					description="Create a scheduled cron job to run automated background tasks for this service."
					action={createButton}
				/>
			) : schedules.length === 0 ? (
				<ListEmpty
					icon={Search}
					title="No schedules match your search"
					description="Try a different job name, command or cron expression."
				/>
			) : view === 'grid' ? (
				<ViewGrid>
					{schedules.map((job: any) => (
						<ScheduleJobCard
							key={job.id}
							job={job}
							onRun={handleRunNow}
							onToggle={handleToggleEnable}
							onDelete={setDeleteId}
						/>
					))}
				</ViewGrid>
			) : (
				<ListCard>
					{schedules.map((job: any) => (
						<ScheduleJobRow
							key={job.id}
							job={job}
							onRun={handleRunNow}
							onToggle={handleToggleEnable}
							onDelete={setDeleteId}
						/>
					))}
				</ListCard>
			)}

			<CreateCronDialog
				app={app}
				open={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				onCreated={onRefresh}
			/>

			<ConfirmDeleteDialog
				open={deleteId !== null}
				onOpenChange={open => !open && setDeleteId(null)}
				title="Delete schedule"
				description={
					<span>
						This cron job stops running immediately. Deployments that
						depend on it running lose that automation. This cannot be
						undone.
					</span>
				}
				confirmLabel="Delete schedule"
				busy={deleteMutation.isPending}
				onConfirm={() => void confirmDelete()}
			/>
		</div>
	);
}
