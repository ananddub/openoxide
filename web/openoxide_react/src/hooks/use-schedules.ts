import * as React from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useOrganizationStore} from '#/stores/organization-store';
import type {components} from '#/types/api';
import {useScheduleListByOrganization, useRemoteServerList} from 'virtual:openoxide-live';

import { useAppStore } from '#/stores/app-store';

export type Schedule = components['schemas']['ScheduleResponseDto'];

export function useSchedules() {
	const activeOrg = useOrganizationStore(state => state.activeOrg);
	const [searchQuery, setSearchQuery] = React.useState('');
	const [isDialogOpen, setIsDialogOpen] = React.useState(false);
	const [editingSchedule, setEditingSchedule] = React.useState<Schedule | null>(null);

	const storeSchedules = useAppStore((state) => state.schedules);

	// Fetch schedules — live hook returns undefined which falls back to Zustand store
	const {data: rawSchedules, loading: isQueryLoading} = useScheduleListByOrganization(BigInt(activeOrg?.id ?? 1));

	const schedules = (rawSchedules && Array.isArray(rawSchedules) && rawSchedules.length > 0)
		? rawSchedules
		: (storeSchedules || []);

	const loading = schedules.length === 0 && isQueryLoading;

	// Fetch remote servers to let user link a schedule to a server
	const {data: servers} = useRemoteServerList();

	// Mutations
	const createMutation = $api.useMutation('post', '/schedules');
	const deleteMutation = $api.useMutation('delete', '/schedules/{id}');
	const patchMutation = $api.useMutation('patch', '/schedules/{id}');
	const runMutation = $api.useMutation('post', '/schedules/{id}/run');

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
			toast.success('Schedule deleted successfully');
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
		isLoading: loading,
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
