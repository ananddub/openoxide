import {useMemo, useState} from 'react';
import {
	ComposeVisualizer,
	type ComposeService,
} from '#/components/projects/compose/visualizer/compose-visualizer';
import type {DomainFormData} from '#/components/projects/common/domain-dialog';
import type {ComposeArchitectureTabProps} from './compose-architecture-tab/types';
import {useRemoteComposeFile} from './compose-architecture-tab/use-remote-compose-file';
import type {
	ActiveModal,
	ArchitectureOwner,
} from '#/components/projects/detail/architecture/types';
import {useDomainActions} from '#/components/projects/detail/architecture/use-domain-actions';
import {
	useScheduleActions,
	type ScheduleFormData,
} from '#/components/projects/detail/architecture/use-schedule-actions';
import {
	useBackupActions,
	type BackupFormData,
} from '#/components/projects/detail/architecture/use-backup-actions';
import {TabModals} from '#/components/projects/detail/architecture/tab-modals';
import {useComposeServiceStatus} from './containers/use-compose-service-status';
import {SectionHeader} from '#/components/settings/list-card';

export function ComposeArchitectureTab({
	compose,
	domains: passedDomains,
	schedules: passedSchedules,
	backups: passedBackups,
	onRefresh,
}: ComposeArchitectureTabProps) {
	const [activeModal, setActiveModal] = useState<ActiveModal>(null);
	const [selectedService, setSelectedService] =
		useState<ComposeService | null>(null);
	const fetchedComposeFile = useRemoteComposeFile(compose);
	const serviceStatus = useComposeServiceStatus(
		compose,
		fetchedComposeFile || compose?.compose_file,
	);

	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const schedules = Array.isArray(passedSchedules) ? passedSchedules : [];

	const owner = useMemo<ArchitectureOwner>(
		() => ({
			kind: 'compose',
			id: compose?.id,
			appName: compose?.app_name,
			name: compose?.name,
		}),
		[compose],
	);

	const domainActions = useDomainActions(owner, onRefresh);
	const scheduleActions = useScheduleActions(owner, onRefresh);
	const backupActions = useBackupActions(owner, onRefresh);

	const composeBackups = useMemo(() => {
		const list = Array.isArray(passedBackups) ? passedBackups : [];
		return list.filter(
			(b: any) =>
				b.compose_id === compose?.id || b.app_name === compose?.app_name,
		);
	}, [passedBackups, compose]);

	const isGit = compose?.source_type && compose?.source_type !== 'RAW';
	const gitBuildPath =
		compose?.build_path ||
		compose?.custom_git_build_path ||
		'docker-compose.yml';

	const servicesList = useMemo(() => {
		if (selectedService) return [selectedService.name];
		return [compose?.app_name || 'app'];
	}, [selectedService, compose]);

	// Context-menu handlers: attach a resource to a service or edit an item.
	const attach = (
		modal: Exclude<ActiveModal, null>,
		service: ComposeService,
	) => {
		setSelectedService(service);
		setActiveModal(modal);
	};
	const openAdd =
		(modal: Exclude<ActiveModal, null>) => (service: ComposeService) => {
			if (modal === 'domain') domainActions.setEditing(null);
			if (modal === 'schedule') scheduleActions.setEditing(null);
			if (modal === 'backup') backupActions.setEditing(null);
			attach(modal, service);
		};
	const openEdit =
		(modal: 'domain' | 'schedule' | 'backup') => (item: any) => {
			if (modal === 'domain') domainActions.setEditing(item);
			if (modal === 'schedule') scheduleActions.setEditing(item);
			if (modal === 'backup') backupActions.setEditing(item);
			setActiveModal(modal);
		};

	const closeModals = () => setActiveModal(null);
	const closeModal = (clearEditing: () => void) => () => {
		clearEditing();
		closeModals();
	};

	const handleSaveDomain = async (data: DomainFormData) => {
		if (await domainActions.save(data)) setActiveModal(null);
	};
	const handleSaveSchedule = async (data: ScheduleFormData) => {
		if (await scheduleActions.save(data)) setActiveModal(null);
	};
	const handleSaveBackup = async (data: BackupFormData) => {
		if (await backupActions.save(data)) setActiveModal(null);
	};

	const terminalAppData = useMemo(
		() => ({
			...compose,
			name: `${compose?.name || 'compose'}:${selectedService?.name || 'app'}`,
			compose_id: compose?.id,
			app_name: compose?.app_name,
			service_name: selectedService?.name,
		}),
		[compose, selectedService],
	);

	return (
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Architecture"
				description="Live map of this stack's services, domains, volume backups and cron jobs. Click any node for context actions."
			/>

			<ComposeVisualizer
				composeFile={fetchedComposeFile || compose?.compose_file || ''}
				serviceStatus={serviceStatus}
				stackName={compose?.name || compose?.app_name}
				gitBuildPath={gitBuildPath}
				isGitSource={isGit}
				backups={composeBackups as any}
				schedules={schedules as any}
				domains={domains as any}
				onAddDomain={openAdd('domain')}
				onAddSchedule={openAdd('schedule')}
				onAddBackup={openAdd('backup')}
				onOpenTerminal={service => attach('terminal', service)}
				onViewLogs={service => attach('logs', service)}
				onViewDeployLogs={service => attach('deployLogs', service)}
				onEditDomain={openEdit('domain')}
				onDeleteDomain={domainActions.remove}
				onEditSchedule={openEdit('schedule')}
				onDeleteSchedule={scheduleActions.remove}
				onEditBackup={openEdit('backup')}
				onDeleteBackup={backupActions.remove}
			/>

			<TabModals
				resource={compose}
				activeModal={activeModal}
				terminalResource={terminalAppData}
				serviceName={selectedService?.name}
				domainServicesList={servicesList}
				servicesList={servicesList}
				defaultServiceName={selectedService?.name}
				domainActions={domainActions}
				scheduleActions={scheduleActions}
				backupActions={backupActions}
				closeModals={closeModals}
				closeModal={closeModal}
				onSaveDomain={handleSaveDomain}
				onSaveSchedule={handleSaveSchedule}
				onSaveBackup={handleSaveBackup}
			/>
		</div>
	);
}
