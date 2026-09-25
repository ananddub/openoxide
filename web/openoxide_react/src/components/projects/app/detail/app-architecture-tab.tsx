import {useMemo, useState} from 'react';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {
	buildAppArchService,
	filterAppArchBackups,
} from './arch-visual-data';
import type {DomainFormData} from '#/components/projects/common/domain-dialog';
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
import {SectionHeader} from '#/components/settings/list-card';

interface AppArchitectureTabProps {
	app: any;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onRefresh?: () => void;
}

export function AppArchitectureTab({
	app,
	domains: passedDomains,
	schedules: passedSchedules,
	backups: passedBackups,
	onRefresh,
}: AppArchitectureTabProps) {
	const [activeModal, setActiveModal] = useState<ActiveModal>(null);

	const owner = useMemo<ArchitectureOwner>(
		() => ({
			kind: 'app',
			id: app?.id,
			appName: app?.app_name,
			name: app?.name,
		}),
		[app],
	);

	const domainActions = useDomainActions(owner, onRefresh);
	const scheduleActions = useScheduleActions(owner, onRefresh);
	const backupActions = useBackupActions(owner, onRefresh);

	const appService = useMemo(() => buildAppArchService(app), [app]);

	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const schedules = Array.isArray(passedSchedules) ? passedSchedules : [];

	const appBackups = useMemo(
		() => filterAppArchBackups(passedBackups, app),
		[passedBackups, app],
	);

	const servicesList = useMemo(
		() => [app?.name || app?.app_name || 'app'],
		[app],
	);

	const openModal =
		(modal: ActiveModal, clearEditing?: () => void) => () => {
			clearEditing?.();
			setActiveModal(modal);
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

	return (
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Architecture"
				description="Live map of this application's service, domains, volume backups and cron jobs. Click any node to add or edit a resource."
			/>

			<ComposeVisualizer
				customServices={appService}
				monitorId={Number(app?.application_id ?? app?.id ?? 0)}
				monitorType="application"
				domains={domains as any}
				backups={appBackups as any}
				schedules={schedules as any}
				onAddDomain={openModal('domain', () =>
					domainActions.setEditing(null),
				)}
				onAddSchedule={openModal('schedule', () =>
					scheduleActions.setEditing(null),
				)}
				onAddBackup={openModal('backup', () =>
					backupActions.setEditing(null),
				)}
				onOpenTerminal={openModal('terminal')}
				onViewLogs={openModal('logs')}
				onViewDeployLogs={openModal('deployLogs')}
				onEditDomain={openEdit('domain')}
				onDeleteDomain={domainActions.remove}
				onEditSchedule={openEdit('schedule')}
				onDeleteSchedule={scheduleActions.remove}
				onEditBackup={openEdit('backup')}
				onDeleteBackup={backupActions.remove}
			/>

			<TabModals
				resource={app}
				activeModal={activeModal}
				terminalResource={app}
				serviceName={app?.name || app?.app_name}
				domainServicesList={[]}
				servicesList={servicesList}
				defaultServiceName={app?.name || app?.app_name}
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
