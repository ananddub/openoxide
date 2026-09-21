import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
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

type DatabaseAction =
	| 'deploy'
	| 'reload'
	| 'start'
	| 'stop'
	| 'redeploy'
	| 'cancel';

const BUILDING_STATUSES = [
	'QUEUED',
	'BUILDING',
	'STARTING',
	'PREPARING',
	'PENDING',
	'DEPLOYING',
];

/**
 * Lifecycle action + settings-update handlers. The engine kind is resolved
 * per call, an intermediate status is layered optimistically, and the
 * table-watch stream refetches the fresh state once the action lands.
 */
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

	const handleAction = async (action: DatabaseAction) => {
		setActionLoading(action);
		const currentSt = (
			database?.app_status ||
			database?.status ||
			''
		).toUpperCase();
		const isCurrentlyBuilding = BUILDING_STATUSES.includes(currentSt);
		const intermediateStatus =
			action === 'stop' || action === 'cancel'
				? isCurrentlyBuilding
					? 'CANCELLING'
					: 'STOPPING'
				: action === 'start'
					? 'STARTING'
					: 'DEPLOYING';

		setLocalStatusOverride(intermediateStatus);
		// Legacy updateServiceStatus store write removed — table-watch
		// invalidates ['databases'] when the action lands, refetching
		// fresh status through the live query.

		try {
			const kind = parseDatabaseKind(
				currentKind || targetKind || 'postgres',
			);
			await databaseActionMutations[action]({kind, id: dbId});
			toast.success(
				`${(currentKind || targetKind || 'postgres').toUpperCase()} ${action} triggered`,
			);
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
			setLocalStatusOverride(null);
		} finally {
			setActionLoading(null);
		}
	};

	const handleUpdateEnv = async (patchData: Record<string, unknown>) => {
		try {
			const kind = parseDatabaseKind(
				currentKind || targetKind || 'postgres',
			);
			await updateMutation.mutateAsync({
				kind,
				id: dbId,
				...patchData,
			} as any);
			toast.success('Database settings updated');
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {actionLoading, handleAction, handleUpdateEnv};
}
