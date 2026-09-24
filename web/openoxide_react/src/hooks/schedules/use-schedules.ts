import * as React from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useOrganizations} from '#/stores/session-context';
import type {ScheduleResponse} from '#/types/api-helpers';
import {
	useScheduleListByOrganization,
	useScheduleCreate,
	useScheduleDelete,
	useScheduleUpdate,
	useScheduleRun,
	useRemoteServerList,
} from '#/api/live';

export type Schedule = ScheduleResponse;

export function useSchedules() {
	const {activeOrg} = useOrganizations();
	const [searchQuery, setSearchQuery] = React.useState('');
	const [isDialogOpen, setIsDialogOpen] = React.useState(false);
	const [editingSchedule, setEditingSchedule] =
		React.useState<Schedule | null>(null);

	// Live schedules query; the table-watch stream (schedules tables) keeps it fresh, so no zustand slice or manual refetch is involved.
	const schedulesQuery = useScheduleListByOrganization(
		BigInt(activeOrg?.id ?? 1),
	);
	const schedules = schedulesQuery.data ?? [];

	// Live servers query; the table-watch stream keeps it fresh.
	const {data: servers = []} = useRemoteServerList();

	const createMutation = useScheduleCreate();
	const deleteMutation = useScheduleDelete();
	const patchMutation = useScheduleUpdate();
	const runMutation = useScheduleRun();

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Schedule deleted successfully');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleToggleEnabled = async (schedule: Schedule) => {
		if (schedule.id === undefined) return;
		try {
			await patchMutation.mutateAsync({
				id: schedule.id,
				enabled: schedule.enabled ? 0 : 1,
			});
			toast.success(
				`Schedule ${schedule.enabled ? 'paused' : 'resumed'} successfully`,
			);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleRunManual = async (id: number) => {
		try {
			toast.loading('Triggering manual run...', {id: `run-${id}`});
			const res: any = await runMutation.mutateAsync(id);
			toast.success(res?.message || 'Manual run completed successfully', {
				id: `run-${id}`,
			});
		} catch (err: unknown) {
			toast.error(formatApiError(err), {id: `run-${id}`});
		}
	};

	const handleOpenCreate = () => {
		setEditingSchedule(null);
		setIsDialogOpen(true);
	};

	const handleOpenEdit = (schedule: Schedule) => {
		setEditingSchedule(schedule);
		setIsDialogOpen(true);
	};

	const filteredSchedules = React.useMemo(() => {
		const list = schedules ?? [];
		return list.filter(s => {
			const query = searchQuery.toLowerCase();
			return (
				s.name.toLowerCase().includes(query) ||
				(s.description || '').toLowerCase().includes(query) ||
				s.command.toLowerCase().includes(query)
			);
		});
	}, [schedules, searchQuery]);

	return {
		activeOrg,
		schedules: filteredSchedules as unknown as Schedule[],
		isLoading: schedulesQuery.isLoading,
		servers: (servers ?? []) as unknown as any[],
		refetch: () => {},
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
	};
}
