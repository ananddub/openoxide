import {useState, useMemo} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {ComposeDomainModal} from '#/components/projects/compose/detail/domains/compose-domain-modal';
import {CreateScheduleModal} from '#/components/projects/compose/detail/schedules/create-schedule-modal';
import {CreateBackupModal} from '#/components/projects/compose/detail/backups/create-backup-modal';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import {ComposeDirectContainerLogsModal} from '#/components/projects/compose/detail/logs/compose-direct-container-logs-modal';
import {ComposeDirectDeployLogsModal} from '#/components/projects/compose/detail/deployments/compose-direct-deploy-logs-modal';

interface AppArchitectureTabProps {
	app: any;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onRefresh?: () => void;
}

export function AppArchitectureTab({app, domains: passedDomains, schedules: passedSchedules, backups: passedBackups, onRefresh}: AppArchitectureTabProps) {
	const queryClient = useQueryClient();
	const appId = app?.id;

	const createDomainMutation = $api.useMutation('post', '/domains');
	const patchDomainMutation = $api.useMutation('patch', '/domains/{id}');
	const deleteDomainMutation = $api.useMutation('delete', '/domains/{id}');

	const createScheduleMutation = $api.useMutation('post', '/schedules');
	const patchScheduleMutation = $api.useMutation('patch', '/schedules/{id}');
	const deleteScheduleMutation = $api.useMutation('delete', '/schedules/{id}');

	const createBackupMutation = $api.useMutation('post', '/backups/volume');
	const patchBackupMutation = $api.useMutation('patch', '/backups/volume/{id}');
	const deleteBackupMutation = $api.useMutation('delete', '/backups/volume/{id}');

	const [activeModal, setActiveModal] = useState<'domain' | 'schedule' | 'backup' | 'terminal' | 'logs' | 'deployLogs' | null>(null);
	const [editingDomainData, setEditingDomainData] = useState<any | null>(null);
	const [editingScheduleData, setEditingScheduleData] = useState<any | null>(null);
	const [editingBackupData, setEditingBackupData] = useState<any | null>(null);

	const appService = useMemo(() => [{
		name: app?.name || app?.app_name || 'app',
		image: app?.repository || app?.dockerfile || app?.build_type || 'app',
		dependsOn: [],
		envVars: {},
		volumes: app?.port ? [`port:${app.port}`] : [],
		ports: app?.port ? [String(app.port)] : [],
	}], [app]);

	const domains = Array.isArray(passedDomains) ? passedDomains : [];

	const appBackups = useMemo(() => {
		const list = Array.isArray(passedBackups) ? passedBackups : [];
		return list.filter((b: any) => b.application_id === appId || b.app_name === app?.app_name);
	}, [passedBackups, appId, app]);

	const appSchedules = useMemo(() => {
		return Array.isArray(passedSchedules) ? passedSchedules : [];
	}, [passedSchedules]);

	const servicesList = useMemo(() => [app?.name || app?.app_name || 'app'], [app]);

	// Context Menu Handlers
	const handleAddDomain = () => {
		setEditingDomainData(null);
		setActiveModal('domain');
	};

	const handleEditDomain = (domainData: any) => {
		setEditingDomainData(domainData);
		setActiveModal('domain');
	};

	const handleAddSchedule = () => {
		setEditingScheduleData(null);
		setActiveModal('schedule');
	};

	const handleEditSchedule = (scheduleData: any) => {
		setEditingScheduleData(scheduleData);
		setActiveModal('schedule');
	};

	const handleAddBackup = () => {
		setEditingBackupData(null);
		setActiveModal('backup');
	};

	const handleEditBackup = (backupData: any) => {
		setEditingBackupData(backupData);
		setActiveModal('backup');
	};

	const handleOpenTerminal = () => {
		setActiveModal('terminal');
	};

	const handleViewLogs = () => {
		setActiveModal('logs');
	};

	const handleViewDeployLogs = () => {
		setActiveModal('deployLogs');
	};

	const handleDeleteDomain = async (domainData: any) => {
		const domainId = domainData?.id;
		if (!domainId) return;
		try {
			await deleteDomainMutation.mutateAsync({params: {path: {id: domainId}}});
			toast.success('Domain removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete domain');
		}
	};

	const handleDeleteSchedule = async (scheduleData: any) => {
		const scheduleId = scheduleData?.id;
		if (!scheduleId) return;
		try {
			await deleteScheduleMutation.mutateAsync({params: {path: {id: scheduleId}}});
			toast.success('Schedule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete schedule');
		}
	};

	const handleDeleteBackup = async (backupData: any) => {
		const backupId = backupData?.id;
		if (!backupId) return;
		try {
			await deleteBackupMutation.mutateAsync({params: {path: {id: backupId}}});
			toast.success('Backup rule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete backup rule');
		}
	};

	const handleSaveDomain = async (data: {
		domain: string;
		serviceName: string;
		containerPort: number;
		https: boolean;
		path: string;
	}) => {
		try {
			if (editingDomainData?.id) {
				await patchDomainMutation.mutateAsync({
					params: {path: {id: editingDomainData.id}},
					body: {
						host: data.domain,
						service_name: data.serviceName,
						port: data.containerPort,
						https: data.https,
						path: data.path,
					} as any,
				});
				toast.success('Domain updated successfully');
			} else {
				await createDomainMutation.mutateAsync({
					body: {
						application_id: appId,
						host: data.domain,
						service_name: data.serviceName,
						port: data.containerPort,
						https: data.https,
						path: data.path,
					} as any,
				});
				toast.success('Domain attached successfully');
			}
			queryClient.invalidateQueries();
			setActiveModal(null);
			setEditingDomainData(null);
		} catch (e: any) {
			toast.error(e?.message || 'Failed to save domain');
		}
	};

	const handleSaveSchedule = async (data: {
		name: string;
		serviceName: string;
		command: string;
		cronExpr: string;
	}) => {
		try {
			if (editingScheduleData?.id) {
				await patchScheduleMutation.mutateAsync({
					params: {path: {id: editingScheduleData.id}},
					body: {
						name: data.name,
						cron_expression: data.cronExpr,
						command: data.command,
						service_name: data.serviceName,
					} as any,
				});
				toast.success('Schedule updated successfully');
			} else {
				await createScheduleMutation.mutateAsync({
					body: {
						name: data.name,
						cron_expression: data.cronExpr,
						command: data.command,
						application_id: appId,
						app_name: app?.app_name || app?.name || data.serviceName,
						service_name: data.serviceName,
						schedule_type: 'APPLICATION',
						schedule_action: 'EXEC',
						shell_type: 'BASH',
						enabled: 1,
					} as any,
				});
				toast.success('Schedule created successfully');
			}
			queryClient.invalidateQueries();
			setActiveModal(null);
			setEditingScheduleData(null);
		} catch (e: any) {
			toast.error(e?.message || 'Failed to save schedule');
		}
	};

	const handleSaveBackup = async (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
	}) => {
		try {
			if (editingBackupData?.id) {
				await patchBackupMutation.mutateAsync({
					params: {path: {id: editingBackupData.id}},
					body: {
						name: data.name,
						volume_name: data.volumeName,
						prefix: data.prefix || '',
						service_name: data.serviceName,
						turn_off: data.turnOff ? 1 : 0,
						cron_expression: data.cronExpr,
					} as any,
				});
				toast.success('Volume backup updated successfully');
			} else {
				await createBackupMutation.mutateAsync({
					body: {
						name: data.name,
						volume_name: data.volumeName,
						prefix: data.prefix || '',
						service_type: 'standalone',
						app_name: app?.app_name || app?.name || data.serviceName || 'app',
						service_name: data.serviceName,
						turn_off: data.turnOff ? 1 : 0,
						cron_expression: data.cronExpr,
						application_id: appId,
					} as any,
				});
				toast.success('Volume backup created successfully');
			}
			queryClient.invalidateQueries({queryKey: ['get', '/backups/volume']});
			setActiveModal(null);
			setEditingBackupData(null);
		} catch (e: any) {
			toast.error(e?.message || 'Failed to save volume backup');
		}
	};

	return (
		<div className="flex flex-col gap-4 w-full animate-in fade-in duration-200">
			<div>
				<h3 className="text-sm font-bold text-foreground">Application Topology & Connections</h3>
				<p className="text-xs text-muted-foreground">
					Interactive real-time map of application service, domains, volume backups, and cron jobs. Click any node to add or edit resources.
				</p>
			</div>

			<ComposeVisualizer
				customServices={appService}
				domains={domains as any}
				backups={appBackups as any}
				schedules={appSchedules as any}
				onAddDomain={handleAddDomain}
				onAddSchedule={handleAddSchedule}
				onAddBackup={handleAddBackup}
				onOpenTerminal={handleOpenTerminal}
				onViewLogs={handleViewLogs}
				onViewDeployLogs={handleViewDeployLogs}
				onEditDomain={handleEditDomain}
				onDeleteDomain={handleDeleteDomain}
				onEditSchedule={handleEditSchedule}
				onDeleteSchedule={handleDeleteSchedule}
				onEditBackup={handleEditBackup}
				onDeleteBackup={handleDeleteBackup}
			/>

			{/* Domain Modal */}
			{activeModal === 'domain' && (
				<ComposeDomainModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingDomainData(null);
					}}
					editingDomain={editingDomainData}
					servicesList={servicesList}
					onSave={handleSaveDomain}
				/>
			)}

			{/* Schedule Modal */}
			{activeModal === 'schedule' && (
				<CreateScheduleModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingScheduleData(null);
					}}
					editingSchedule={editingScheduleData}
					servicesList={servicesList}
					onCreate={handleSaveSchedule}
				/>
			)}

			{/* Backup Modal */}
			{activeModal === 'backup' && (
				<CreateBackupModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingBackupData(null);
					}}
					editingBackup={editingBackupData}
					servicesList={servicesList}
					defaultServiceName={app?.name || app?.app_name}
					onCreate={handleSaveBackup}
				/>
			)}

			{/* Terminal Shell Modal */}
			{activeModal === 'terminal' && (
				<TerminalModal
					app={app}
					open={true}
					onClose={() => setActiveModal(null)}
				/>
			)}

			{/* Live Container Logs Modal */}
			{activeModal === 'logs' && (
				<ComposeDirectContainerLogsModal
					isOpen={true}
					onClose={() => setActiveModal(null)}
					compose={app}
					serviceName={app?.name || app?.app_name}
				/>
			)}

			{/* Direct Deployment Logs Stream Modal */}
			{activeModal === 'deployLogs' && (
				<ComposeDirectDeployLogsModal
					isOpen={true}
					onClose={() => setActiveModal(null)}
					composeId={appId}
					serviceName={app?.name || app?.app_name}
				/>
			)}
		</div>
	);
}
