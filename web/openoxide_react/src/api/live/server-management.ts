import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	ServerCleanupStatus,
	ServerManagementService,
	type ServerCleanupExecution,
	type ServerManagement,
} from '#/gen/connect/openoxide/api/v1/server_management_pb';

const managementClient = createServiceClient(ServerManagementService);

/** Cleanup scopes stored in a server's policy; order matches the checkboxes in the UI. */
export const CLEANUP_SCOPES = [
	'containers',
	'images',
	'networks',
	'volumes',
	'packages',
] as const;

export type CleanupScope = (typeof CLEANUP_SCOPES)[number];

export const CLEANUP_SCOPE_LABELS: Record<CleanupScope, string> = {
	containers: 'Stopped containers',
	images: 'Unused images',
	networks: 'Unused networks',
	volumes: 'Unused volumes',
	packages: 'Host packages',
};

export type ServerCleanupPolicyPayload = Record<CleanupScope, boolean>;

/** Editable management settings of one server; labels are pairs so the form can render them. */
export type ServerManagementPayload = {
	maintenanceMode: boolean;
	maintenanceMessage: string;
	labels: Array<[string, string]>;
	cleanupPolicy: ServerCleanupPolicyPayload;
	gpuEnabled: boolean;
	updatedAt: bigint;
};

/** One host-side management command: cleanup, upgrade, backup, diagnostics, audit or GPU setup. */
export const MANAGEMENT_ACTIONS = [
	'runCleanup',
	'upgrade',
	'backup',
	'diagnostics',
	'repairAudit',
	'configureGpu',
] as const;

export type ManagementAction = (typeof MANAGEMENT_ACTIONS)[number];

export const MANAGEMENT_ACTION_LABELS: Record<ManagementAction, string> = {
	runCleanup: 'Run Cleanup',
	upgrade: 'Upgrade Host',
	backup: 'Backup',
	diagnostics: 'Diagnostics',
	repairAudit: 'Repair Audit',
	configureGpu: 'Configure GPU',
};

export const MANAGEMENT_ACTION_HINTS: Record<ManagementAction, string> = {
	runCleanup: 'Prune what the cleanup policy selects.',
	upgrade: 'Update host packages and dependencies.',
	backup: 'Snapshot this server to its backup destination.',
	diagnostics: 'Collect a host diagnostic report.',
	repairAudit: 'Re-run audit remediation for stored findings.',
	configureGpu: 'Install NVIDIA container toolkit support.',
};

/** Long-running host commands; the rest finish quickly enough to block on. */
export const isSlowAction = (action: ManagementAction): boolean =>
	action === 'upgrade' || action === 'backup' || action === 'configureGpu';

export type ServerActionPayload = {
	success: boolean;
	stdout: string;
	stderr: string;
};

export type CleanupExecutionPayload = {
	id: string;
	status: string;
	succeeded: boolean;
	policy: string;
	stdout: string;
	stderr: string;
	startedAt: bigint;
	finishedAt: bigint | null;
};

const EMPTY_POLICY: ServerCleanupPolicyPayload = {
	containers: false,
	images: false,
	networks: false,
	volumes: false,
	packages: false,
};

function toPolicy(policy: ServerCleanupPolicyPayload | undefined | null) {
	return {
		containers: policy?.containers ?? false,
		images: policy?.images ?? false,
		networks: policy?.networks ?? false,
		volumes: policy?.volumes ?? false,
		packages: policy?.packages ?? false,
	};
}

export function toManagementPayload(
	message: ServerManagement,
): ServerManagementPayload {
	return {
		maintenanceMode: message.maintenanceMode,
		maintenanceMessage: message.maintenanceMessage ?? '',
		labels: Object.entries(message.labels ?? {}),
		cleanupPolicy: toPolicy(message.cleanupPolicy ?? EMPTY_POLICY),
		gpuEnabled: message.gpuEnabled,
		updatedAt: message.updatedAt,
	};
}

function cleanupStatusLabel(status: ServerCleanupStatus): string {
	if (status === ServerCleanupStatus.SUCCEEDED) return 'Succeeded';
	if (status === ServerCleanupStatus.FAILED) return 'Failed';
	return 'Unknown';
}

function toExecution(
	row: ServerCleanupExecution,
): CleanupExecutionPayload {
	return {
		id: row.id.toString(),
		status: cleanupStatusLabel(row.status),
		succeeded: row.status === ServerCleanupStatus.SUCCEEDED,
		policy: row.policy,
		stdout: row.stdout ?? '',
		stderr: row.stderr ?? '',
		startedAt: row.startedAt,
		finishedAt: row.finishedAt ?? null,
	};
}

/** Management settings for one server (maintenance mode, labels, cleanup policy, GPU). */
export function useServerManagement(serverId: bigint) {
	return useConnectQuery<ServerManagementPayload>(
		['server-management', 'settings', serverId.toString()],
		async () => {
			const response = await managementClient.getManagement({
				id: serverId,
			});
			if (!response.management) {
				throw new Error('this server has no management settings yet');
			}
			return toManagementPayload(response.management);
		},
	);
}

/** Past cleanup executions for one server, newest first. */
export function useCleanupHistory(serverId: bigint) {
	return useConnectQuery<CleanupExecutionPayload[]>(
		['server-management', 'cleanup-history', serverId.toString()],
		async () => {
			const response = await managementClient.listCleanupHistory({
				serverId,
			});
			return response.executions.map(toExecution);
		},
	);
}

/** PUT the whole management record: the backend requires every field, so the form sends its current state. */
export function useServerManagementSave(serverId: bigint) {
	return useConnectMutation(
		(input: ServerManagementPayload) =>
			managementClient
				.updateManagement({
					id: serverId,
					maintenanceMode: input.maintenanceMode,
					maintenanceMessage: input.maintenanceMessage || undefined,
					labels: Object.fromEntries(
						input.labels.filter(([key]) => key.trim().length > 0),
					),
					cleanupPolicy: toPolicy(input.cleanupPolicy),
					gpuEnabled: input.gpuEnabled,
				})
				.then(response =>
					response.management
						? toManagementPayload(response.management)
						: input,
				),
		{invalidate: [['server-management', 'settings', serverId.toString()]]},
	);
}

/** Run one management action on this server and return its raw command outcome. */
export function useServerManagementAction(serverId: bigint) {
	return useConnectMutation(
		async (action: ManagementAction): Promise<ServerActionPayload> => {
			if (action === 'runCleanup') {
				const response = await managementClient.runCleanup({id: serverId});
				return toActionPayload(response.result);
			}
			if (action === 'upgrade') {
				const response = await managementClient.upgrade({id: serverId});
				return toActionPayload(response.result);
			}
			if (action === 'diagnostics') {
				const response = await managementClient.diagnostics({
					id: serverId,
				});
				return toActionPayload(response.result);
			}
			if (action === 'repairAudit') {
				const response = await managementClient.repairAudit({
					id: serverId,
				});
				return toActionPayload(response.result);
			}
			if (action === 'configureGpu') {
				const response = await managementClient.configureGpu({
					id: serverId,
				});
				return toActionPayload(response.result);
			}
			const response = await managementClient.backup({id: serverId});
			return {
				success: response.backup != null,
				stdout: response.backup
					? `Backup stored at ${response.backup.remotePath}`
					: '',
				stderr: response.backup
					? ''
					: 'the server returned no backup record',
			};
		},
		{
			invalidate: [
				['server-management', 'cleanup-history', serverId.toString()],
				['global', 'disk-usage', serverId.toString()],
			],
		},
	);
}

function toActionPayload(
	result:
		| {success: boolean; stdout: string; stderr: string}
		| null
		| undefined,
): ServerActionPayload {
	return {
		success: result?.success ?? false,
		stdout: result?.stdout ?? '',
		stderr: result?.stderr ?? '',
	};
}
