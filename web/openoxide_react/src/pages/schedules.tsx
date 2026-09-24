import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {useSchedules} from '#/hooks/schedules/use-schedules';
import {SchedulesHeader} from '#/components/schedules/schedules-header';
import {SchedulesList} from '#/components/schedules/schedules-list';
import {ScheduleDialog} from '#/components/schedules/schedule-dialog';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';

export const Route = createFileRoute('/_app/schedules')({
	component: SchedulesPage,
});

function SchedulesPage() {
	const {
		activeOrg,
		schedules,
		isLoading,
		servers,
		refetch,
		isDialogOpen,
		setIsDialogOpen,
		editingSchedule,
		handleDelete,
		handleToggleEnabled,
		handleRunManual,
		handleOpenCreate,
		handleOpenEdit,
		createMutation,
		patchMutation,
	} = useSchedules();

	const [deletingId, setDeletingId] = useState<number | null>(null);

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			{/* Header */}
			<SchedulesHeader
				onOpenCreate={handleOpenCreate}
				onRefresh={refetch}
				isRefreshing={isLoading}
				disabled={!activeOrg}
			/>

			{/* List */}
			<SchedulesList
				schedules={schedules}
				isLoading={isLoading}
				servers={servers}
				onEdit={handleOpenEdit}
				onDelete={id => setDeletingId(id)}
				onToggle={handleToggleEnabled}
				onRun={handleRunManual}
			/>

			{/* Dialog Modal */}
			<ScheduleDialog
				isOpen={isDialogOpen}
				onClose={() => setIsDialogOpen(false)}
				editingSchedule={editingSchedule}
				servers={servers}
				refetch={refetch}
				activeOrgId={activeOrg?.id}
				createMutation={createMutation}
				patchMutation={patchMutation}
			/>

			{/* Delete Confirmation */}
			<ConfirmDeleteDialog
				open={deletingId !== null}
				onOpenChange={open => !open && setDeletingId(null)}
				title="Delete Schedule"
				description="Are you sure you want to delete this schedule? Future automated cron executions will stop."
				confirmLabel="Delete Schedule"
				onConfirm={async () => {
					if (deletingId) {
						await handleDelete(deletingId);
						setDeletingId(null);
					}
				}}
			/>
		</div>
	);
}
