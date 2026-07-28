import * as React from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useOrganizationStore} from '#/stores/organization-store';
import type {components} from '#/types/api';

export type Schedule = components['schemas']['ScheduleResponseDto'];

export function useSchedules() {
	const activeOrg = useOrganizationStore(state => state.activeOrg);
	const [searchQuery, setSearchQuery] = React.useState('');
	const [isDialogOpen, setIsDialogOpen] = React.useState(false);
	const [editingSchedule, setEditingSchedule] = React.useState<Schedule | null>(null);

	// Fetch schedules
	const {
		data: schedules,
		isLoading,
		refetch,
	} = $api.useQuery(
		'get',
		'/schedules/organization/{organization_id}',
		{
			params: {
				path: {
					organization_id: activeOrg?.id ?? 0,
				},
			},
		},
		{
			enabled: !!activeOrg?.id,
		},
	);

	// Fetch remote servers to let user link a schedule to a server
	const {data: servers} = $api.useQuery('get', '/remote-servers', {}, {enabled: !!activeOrg?.id});

	// Mutations
	const createMutation = $api.useMutation('post', '/schedules');
	const deleteMutation = $api.useMutation('delete', '/schedules/{id}');
	const patchMutation = $api.useMutation('patch', '/schedules/{id}');
	const runMutation = $api.useMutation('post', '/schedules/{id}/run');

	const handleDelete = async (id: number) => {
		if (!confirm('Are you sure you want to delete this schedule?')) return;
		try {
			await deleteMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
			toast.success('Schedule deleted successfully');
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleToggleEnabled = async (schedule: Schedule) => {
		if (schedule.id === undefined) return;
		try {
			await patchMutation.mutateAsync({
				params: {
					path: {
						id: schedule.id,
					},
				},
				body: {
					enabled: schedule.enabled ? 0 : 1,
				},
			});
			toast.success(`Schedule ${schedule.enabled ? 'paused' : 'resumed'} successfully`);
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleRunManual = async (id: number) => {
		try {
			toast.loading('Triggering manual run...', {id: `run-${id}`});
			const res = await runMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
			toast.success(res?.message || 'Manual run completed successfully', {id: `run-${id}`});
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
		if (!schedules) return [];
		return schedules.filter(s => {
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
		schedules: filteredSchedules,
		isLoading,
		servers: servers || [],
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
	};
}
