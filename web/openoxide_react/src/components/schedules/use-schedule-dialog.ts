import {useEffect, useMemo, useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useProjectListByOrganization} from '#/api/live';
import type {Schedule} from '#/hooks/use-schedules';
import {collectProjectApplications} from './schedule-applications';

export interface ScheduleDialogProps {
	isOpen: boolean;
	onClose: () => void;
	editingSchedule: Schedule | null;
	servers: any[];
	refetch: () => void;
	activeOrgId?: number;
	createMutation: any;
	patchMutation: any;
}

/** State, derived lists and submit logic for {@link ScheduleDialog}. */
export function useScheduleDialog(props: ScheduleDialogProps) {
	const {
		isOpen,
		onClose,
		editingSchedule,
		servers,
		refetch,
		activeOrgId,
		createMutation,
		patchMutation,
	} = props;

	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [cronExpression, setCronExpression] = useState('');
	const [command, setCommand] = useState('');
	const [shellType, setShellType] = useState('bash');
	const [targetType, setTargetType] = useState<'SERVER' | 'APPLICATION'>(
		'SERVER',
	);
	const [targetId, setTargetId] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	// Live projects query keyed ['projects','list',org] — table-watch
	// invalidates it while the schedules page is mounted.
	const projectsQuery = useProjectListByOrganization(
		BigInt(activeOrgId ?? 1),
	);
	const projectsList = (projectsQuery.data ?? []) as any[];

	const allApplications = useMemo(
		() => collectProjectApplications(projectsList),
		[projectsList],
	);

	const selectedAppName = useMemo(() => {
		if (!targetId) return '';
		const found = allApplications.find(
			a => String(a.id) === String(targetId),
		);
		return found?.name || (targetId ? `Application #${targetId}` : '');
	}, [targetId, allApplications]);

	const selectedServerName = useMemo(() => {
		if (!targetId) return '';
		const found = servers.find(s => String(s.id) === String(targetId));
		return found?.name || (targetId ? `Server #${targetId}` : '');
	}, [targetId, servers]);

	useEffect(() => {
		if (editingSchedule) {
			setName(editingSchedule.name);
			setDescription(editingSchedule.description || '');
			setCronExpression(editingSchedule.cron_expression);
			setCommand(editingSchedule.command);
			setShellType(editingSchedule.shell_type || 'bash');
			if (editingSchedule.application_id) {
				setTargetType('APPLICATION');
				setTargetId(String(editingSchedule.application_id));
			} else {
				setTargetType('SERVER');
				setTargetId(
					editingSchedule.server_id
						? String(editingSchedule.server_id)
						: '',
				);
			}
		} else {
			setName('');
			setDescription('');
			setCronExpression('*/5 * * * *');
			setCommand('');
			setShellType('bash');
			setTargetType('SERVER');
			setTargetId(servers[0]?.id ? String(servers[0].id) : '');
		}
	}, [editingSchedule, isOpen, servers]);

	const handleTargetTypeChange = (val: string | null) => {
		const newType = (val ?? 'SERVER') as 'SERVER' | 'APPLICATION';
		setTargetType(newType);
		if (newType === 'SERVER') {
			setTargetId(servers[0]?.id ? String(servers[0].id) : '');
		} else {
			setTargetId(
				allApplications[0]?.id ? String(allApplications[0].id) : '',
			);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (
			!name.trim() ||
			!cronExpression.trim() ||
			!command.trim() ||
			!targetId
		) {
			toast.error(
				'Please fill in all required fields including Target Item',
			);
			return;
		}

		setIsSubmitting(true);
		try {
			const body: any = {
				name,
				description,
				cron_expression: cronExpression,
				command,
				shell_type: shellType.toUpperCase(),
				schedule_type: targetType,
				schedule_action: 'EXEC',
				organization_id: activeOrgId,
				enabled: editingSchedule ? editingSchedule.enabled : 1,
			};

			if (targetType === 'SERVER') {
				body.server_id = Number(targetId);
			} else {
				body.application_id = Number(targetId);
			}

			if (editingSchedule?.id !== undefined) {
				await patchMutation.mutateAsync({
					id: editingSchedule.id,
					...body,
				});
				toast.success('Schedule updated');
			} else {
				await createMutation.mutateAsync(body);
				toast.success('Schedule created');
			}
			refetch();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return {
		name,
		setName,
		description,
		setDescription,
		cronExpression,
		setCronExpression,
		command,
		setCommand,
		shellType,
		setShellType,
		targetType,
		targetId,
		setTargetId,
		handleTargetTypeChange,
		allApplications,
		selectedAppName,
		selectedServerName,
		servers,
		editingSchedule,
		isSubmitting,
		handleSubmit,
	};
}

export type ScheduleDialogController = ReturnType<
	typeof useScheduleDialog
>;
