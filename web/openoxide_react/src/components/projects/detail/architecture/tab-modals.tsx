import {ComposeDomainModal} from '#/components/projects/compose/detail/domains/compose-domain-modal';
import {CreateScheduleModal} from '#/components/projects/compose/detail/schedules/create-schedule-modal';
import {CreateBackupModal} from '#/components/projects/compose/detail/backups/create-backup-modal';
import {ComposeDirectContainerLogsModal} from '#/components/projects/compose/detail/logs/compose-direct-container-logs-modal';
import {ComposeDirectDeployLogsModal} from '#/components/projects/compose/detail/deployments/compose-direct-deploy-logs-modal';
import {TerminalCard} from '#/components/shared/terminal-card';
import type {DomainFormData} from '#/components/projects/common/domain-dialog';
import type {ActiveModal} from './types';
import type {useDomainActions} from './use-domain-actions';
import type {
	ScheduleFormData,
	useScheduleActions,
} from './use-schedule-actions';
import type {BackupFormData, useBackupActions} from './use-backup-actions';

export interface TabModalsProps {
	resource: any;
	activeModal: ActiveModal;
	/** Object handed to TerminalCard (compose tabs pass a service-scoped view). */
	terminalResource: any;
	/** Service shown in the container/deploy log modals; undefined = app-wide. */
	serviceName?: string | null;
	domainServicesList: string[];
	servicesList: string[];
	defaultServiceName?: string;
	domainActions: ReturnType<typeof useDomainActions>;
	scheduleActions: ReturnType<typeof useScheduleActions>;
	backupActions: ReturnType<typeof useBackupActions>;
	closeModals: () => void;
	closeModal: (clearEditing: () => void) => () => void;
	onSaveDomain: (data: DomainFormData) => Promise<void>;
	onSaveSchedule: (data: ScheduleFormData) => Promise<void>;
	onSaveBackup: (data: BackupFormData) => Promise<void>;
}

/** Domain/schedule/backup/terminal/log modals shared by app & compose tabs. */
export function TabModals({
	resource,
	activeModal,
	terminalResource,
	serviceName,
	domainServicesList,
	servicesList,
	defaultServiceName,
	domainActions,
	scheduleActions,
	backupActions,
	closeModals,
	closeModal,
	onSaveDomain,
	onSaveSchedule,
	onSaveBackup,
}: TabModalsProps) {
	return (
		<>
			{activeModal === 'domain' && (
				<ComposeDomainModal
					isOpen={true}
					onClose={closeModal(() => domainActions.setEditing(null))}
					editingDomain={domainActions.editing}
					servicesList={domainServicesList}
					onSave={onSaveDomain}
				/>
			)}

			{activeModal === 'schedule' && (
				<CreateScheduleModal
					isOpen={true}
					onClose={closeModal(() => scheduleActions.setEditing(null))}
					editingSchedule={scheduleActions.editing}
					servicesList={servicesList}
					onCreate={onSaveSchedule}
				/>
			)}

			{activeModal === 'backup' && (
				<CreateBackupModal
					isOpen={true}
					onClose={closeModal(() => backupActions.setEditing(null))}
					editingBackup={backupActions.editing}
					servicesList={servicesList}
					defaultServiceName={defaultServiceName}
					onCreate={onSaveBackup}
				/>
			)}

			{activeModal === 'terminal' && (
				<TerminalCard
					app={terminalResource}
					enabled={true}
					onClose={closeModals}
				/>
			)}

			{activeModal === 'logs' && (
				<ComposeDirectContainerLogsModal
					isOpen={true}
					onClose={closeModals}
					compose={resource}
					serviceName={serviceName ?? undefined}
				/>
			)}

			{activeModal === 'deployLogs' && (
				<ComposeDirectDeployLogsModal
					isOpen={true}
					onClose={closeModals}
					composeId={resource?.id}
					serviceName={serviceName ?? undefined}
				/>
			)}
		</>
	);
}
