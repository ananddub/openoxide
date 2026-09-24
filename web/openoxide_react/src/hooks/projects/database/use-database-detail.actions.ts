import {useState} from 'react';
import {runResourceAction} from '#/utils/run-resource-action';
import {
	useDatabaseDeploy,
	useDatabaseRedeploy,
	useDatabaseReload,
	useDatabaseStart,
	useDatabaseStop,
	useDatabaseCancel,
	useDatabaseUpdate,
	parseDatabaseKind,
} from '#/api/live';
import {intermediateStatusFor} from '../shared/resource-status';

type DatabaseAction =
	| 'deploy'
	| 'reload'
	| 'start'
	| 'stop'
	| 'redeploy'
	| 'cancel';

/** Lifecycle action + settings-update handlers. The engine kind is resolved per call, an intermediate status is layered optimistically, and the table-watch stream refetches the fresh state once the action lands. */
export function useDatabaseActions(args: {
	dbId: number;
	currentKind: string;
	targetKind?: string;
	database: any;
	setLocalStatusOverride: (s: string | null) => void;
	refetchAll: () => void;
}) {
	const {
		dbId,
		currentKind,
		targetKind,
		database,
		setLocalStatusOverride,
		refetchAll,
	} = args;

	const deployMutation = useDatabaseDeploy();
	const redeployMutation = useDatabaseRedeploy();
	const reloadMutation = useDatabaseReload();
	const startMutation = useDatabaseStart();
	const stopMutation = useDatabaseStop();
	const cancelMutation = useDatabaseCancel();
	const updateMutation = useDatabaseUpdate();
	const databaseActionMutations: Record<
		DatabaseAction,
		(i: any) => Promise<any>
	> = {
		deploy: deployMutation.mutateAsync,
		redeploy: redeployMutation.mutateAsync,
		reload: reloadMutation.mutateAsync,
		start: startMutation.mutateAsync,
		stop: stopMutation.mutateAsync,
		cancel: cancelMutation.mutateAsync,
	};

	const [actionLoading, setActionLoading] =
		useState<DatabaseAction | null>(null);

	const engine = currentKind || targetKind || 'postgres';

	const handleAction = async (action: DatabaseAction) => {
		setActionLoading(action);
		const currentStatus = String(
			database?.app_status || database?.status || '',
		).toUpperCase();
		setLocalStatusOverride(intermediateStatusFor(action, currentStatus));
		// Legacy updateServiceStatus store write removed — table-watch invalidates ['databases'] when the action lands, refetching fresh status through the live query.

		await runResourceAction(
			() =>
				databaseActionMutations[action]({
					kind: parseDatabaseKind(engine),
					id: dbId,
				}),
			{
				success: `${engine.toUpperCase()} ${action} triggered`,
				onDone: refetchAll,
				onFail: () => setLocalStatusOverride(null),
			},
		);
		setActionLoading(null);
	};

	const handleUpdateEnv = async (patchData: Record<string, unknown>) => {
		await runResourceAction(
			() =>
				updateMutation.mutateAsync({
					kind: parseDatabaseKind(engine),
					id: dbId,
					...patchData,
				} as any),
			{success: 'Database settings updated', onDone: refetchAll},
		);
	};

	return {actionLoading, handleAction, handleUpdateEnv};
}
