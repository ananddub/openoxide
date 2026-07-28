import {createFileRoute} from '@tanstack/react-router';
import {Plus, CalendarDays, Search, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {useSchedules} from '#/hooks/use-schedules';
import {ScheduleCard} from '#/components/schedules/schedule-card';
import {ScheduleDialog} from '#/components/schedules/schedule-dialog';

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
		searchQuery,
		setSearchQuery,
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

	return (
		<div className="flex flex-col gap-6 w-full pb-10 animate-in fade-in duration-200">
			{/* Page Header */}
			<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/30 pb-5">
				<div>
					<h1 className="text-3xl font-extrabold tracking-tight text-foreground">
						Schedules
					</h1>
					<p className="text-muted-foreground mt-1 text-xs font-medium">
						Manage automated tasks, cron jobs, database backups, and server maintenance scripts
					</p>
				</div>

				<div className="flex items-center gap-3">
					<Button
						onClick={handleOpenCreate}
						disabled={!activeOrg}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold h-10 px-4 rounded-lg flex items-center gap-2 shadow-2xs">
						<Plus className="size-4.5" />
						Create Schedule
					</Button>
				</div>
			</div>

			{/* Filters Bar */}
			<div className="flex flex-col sm:flex-row items-center gap-3">
				<div className="relative w-full sm:grow">
					<Search className="absolute left-3.5 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
					<Input
						placeholder="Search by schedule name, description, or command..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-9.5 bg-card border-border h-10 w-full text-xs rounded-lg shadow-2xs"
					/>
				</div>
				<Button
					variant="outline"
					onClick={() => refetch()}
					className="border-border bg-card hover:bg-muted/50 font-semibold h-10 px-4 text-xs rounded-lg shrink-0 flex items-center gap-2 shadow-2xs">
					<RefreshCw className="size-4" />
					Refresh
				</Button>
			</div>

			{/* Content Body */}
			{isLoading ? (
				<div className="flex flex-col gap-3 py-24 items-center justify-center">
					<RefreshCw className="size-8 animate-spin text-primary" />
					<p className="text-sm text-muted-foreground font-medium animate-pulse">
						Loading automated schedules...
					</p>
				</div>
			) : !activeOrg ? (
				<div className="flex flex-col items-center justify-center border border-dashed border-border/40 rounded-2xl py-24 text-center bg-muted/10">
					<CalendarDays className="size-12 opacity-20 text-muted-foreground" />
					<h3 className="text-md font-bold text-foreground mt-3">No Organization Selected</h3>
					<p className="text-muted-foreground mt-1 text-xs max-w-sm">
						Please choose or create an organization first to manage schedules.
					</p>
				</div>
			) : schedules.length > 0 ? (
				<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4">
					{schedules.map(s => (
						<ScheduleCard
							key={s.id}
							schedule={s}
							onEdit={handleOpenEdit}
							onDelete={handleDelete}
							onToggle={handleToggleEnabled}
							onRun={handleRunManual}
							servers={servers}
						/>
					))}
				</div>
			) : (
				<div className="flex flex-col items-center justify-center border border-dashed border-border/40 rounded-2xl py-24 text-center bg-muted/10">
					<CalendarDays className="size-12 opacity-20 text-muted-foreground" />
					<h3 className="text-md font-bold text-foreground mt-3">No Schedules Found</h3>
					<p className="text-muted-foreground mt-1 text-xs max-w-sm font-medium">
						{searchQuery
							? 'No schedules match your search. Try adjusting filters.'
							: 'You have not registered any cron jobs or automated tasks yet.'}
					</p>
					{!searchQuery && (
						<Button
							onClick={handleOpenCreate}
							className="mt-4 text-xs font-semibold bg-primary hover:bg-primary/95 text-primary-foreground h-9 px-4">
							Create First Schedule
						</Button>
					)}
				</div>
			)}

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
		</div>
	);
}
