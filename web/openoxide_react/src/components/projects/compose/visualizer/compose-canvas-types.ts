import type {ComposeService, ServiceStatus} from './compose-parse';
import type {VolumeBackupData} from './compose-backup-node';
import type {CronJobData} from './compose-cron-node';
import type {ComposeDomainData} from './compose-domain-node';
import type {ServiceActionHandlers} from './compose-graph';

/** Handlers for satellite nodes (domains / schedules / backups). */
export type ComposeSatelliteHandlers = {
	onViewDeployLogs?: (service: ComposeService) => void;
	onEditDomain?: (domainData: any) => void;
	onDeleteDomain?: (domainData: any) => void;
	onEditSchedule?: (scheduleData: any) => void;
	onDeleteSchedule?: (scheduleData: any) => void;
	onEditBackup?: (backupData: any) => void;
	onDeleteBackup?: (backupData: any) => void;
};

/** Common graph data + action props shared by the canvas and visualizer. */
export type ComposeGraphProps = {
	backups?: VolumeBackupData[];
	schedules?: CronJobData[];
	domains?: ComposeDomainData[];
	gitBuildPath?: string;
	isGitSource?: boolean;
	/** Static topology preview: no interactions, controls or menus. */
	readOnly?: boolean;
	/** Live per-service runtime state (service name → status). */
	serviceStatus?: Record<string, ServiceStatus>;
};

export type CanvasInnerProps = ComposeGraphProps &
	ServiceActionHandlers &
	ComposeSatelliteHandlers & {
		services: ComposeService[];
	};

export type ComposeVisualizerProps = ComposeGraphProps &
	ServiceActionHandlers &
	ComposeSatelliteHandlers & {
		composeFile?: string | null;
		stackName?: string;
		customServices?: ComposeService[];
		wrapperClassName?: string;
	};
