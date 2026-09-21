import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {
	useScheduleCreate,
	useScheduleDelete,
	useScheduleUpdate,
} from '#/api/live';
import {useOrganizations} from '#/stores/session-context';
import type {ArchitectureOwner} from './types';

export interface ScheduleFormData {
	name: string;
	serviceName: string;
	command: string;
	cronExpr: string;
}

export function useScheduleActions(
	owner: ArchitectureOwner,
	onRefresh?: () => void,
) {
	const queryClient = useQueryClient();
	const {activeOrg} = useOrganizations();
	const create = useScheduleCreate();
	const patch = useScheduleUpdate();
	const removeMutation = useScheduleDelete();
	const [editing, setEditing] = useState<any | null>(null);

	const isCompose = owner.kind === 'compose';
	const scopeField = isCompose ? 'compose_id' : 'application_id';

	/** Returns true when the schedule was saved successfully. */
	const save = async (data: ScheduleFormData): Promise<boolean> => {
		try {
			if (editing?.id) {
				await patch.mutateAsync({
					id: editing.id,
					name: data.name,
					cron_expression: data.cronExpr,
					command: data.command,
					service_name: data.serviceName,
				});
				toast.success('Schedule updated successfully');
			} else {
				await create.mutateAsync({
					name: data.name,
					cron_expression: data.cronExpr,
					command: data.command,
					[scopeField]: owner.id,
					organization_id: activeOrg?.id || 1,
					app_name: owner.appName || owner.name || data.serviceName,
					service_name: data.serviceName,
					schedule_type: isCompose ? 'compose' : 'APPLICATION',
					schedule_action: isCompose ? 'exec' : 'EXEC',
					shell_type: isCompose ? 'sh' : 'BASH',
					enabled: 1,
				});
				toast.success(
					isCompose
						? `Schedule attached to ${data.serviceName}`
						: 'Schedule created successfully',
				);
			}
			queryClient.invalidateQueries();
			setEditing(null);
			return true;
		} catch (err: any) {
			toast.error(err?.message || 'Failed to save schedule');
			return false;
		}
	};

	const remove = async (scheduleData: any) => {
		if (!scheduleData?.id) return;
		try {
			await removeMutation.mutateAsync(scheduleData.id);
			toast.success('Schedule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete schedule');
		}
	};

	return {editing, setEditing, save, remove};
}
