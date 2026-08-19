import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {useSchedules} from '#/hooks/use-schedules';
import {SchedulesHeader} from '#/components/schedules/schedules-header';
import {SchedulesList} from '#/components/schedules/schedules-list';
import {ScheduleDialog} from '#/components/schedules/schedule-dialog';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';

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
		<div className="mx-auto flex w-full max-w-7xl flex-col gap-6 p-6">
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

			{/* Delete Confirmation Alert Dialog */}
			<AlertDialog
				open={deletingId !== null}
				onOpenChange={open => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Automated Schedule</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this schedule? This action
							will stop all future automated cron executions.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>
							Cancel
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground cursor-pointer bg-destructive hover:bg-destructive/90"
							onClick={async () => {
								if (deletingId) {
									await handleDelete(deletingId);
									setDeletingId(null);
								}
							}}>
							Delete Schedule
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
