import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ApplicationResponse} from '#/types/api-helpers';
import {
	useApplicationCancel,
	useApplicationDeploy,
	useApplicationRebuild,
	useApplicationReload,
	useApplicationStart,
	useApplicationStop,
	useApplicationUpdate,
} from '#/api/live';

export type AppAction =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'stop'
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
 * Lifecycle action + settings-patch handlers. Guards against overlapping
 * in-flight actions (cancel excepted), layers an optimistic intermediate
 * status and relies on table-watch to refetch the settled state.
 */
export function useAppActions(args: {
	appId: number;
	app: any;
	setLocalStatusOverride: (s: string | null) => void;
	actionInFlight: {current: boolean};
	statusBeforeAction: {current: string | null};
	clearOverride: () => void;
	refetchAll: () => void;
}) {
	const {
		appId,
		app,
		setLocalStatusOverride,
		actionInFlight,
		statusBeforeAction,
		clearOverride,
		refetchAll,
	} = args;

	const deployMutation = useApplicationDeploy();
	const reloadMutation = useApplicationReload();
	const rebuildMutation = useApplicationRebuild();
	const startMutation = useApplicationStart();
	const stopMutation = useApplicationStop();
	const cancelMutation = useApplicationCancel();
	const patchMutation = useApplicationUpdate();

	const actions: Record<
		AppAction,
		{run: (id: number) => Promise<unknown>; message: string}
	> = {
		deploy: {
			run: id => deployMutation.mutateAsync(id),
			message: 'Deployment triggered successfully',
		},
		reload: {
			run: id => reloadMutation.mutateAsync(id),
			message: 'Application reloaded successfully',
		},
		rebuild: {
			run: id => rebuildMutation.mutateAsync(id),
			message: 'Rebuild triggered successfully',
		},
		start: {
			run: id => startMutation.mutateAsync(id),
			message: 'Application started successfully',
		},
		stop: {
			run: id => stopMutation.mutateAsync(id),
			message: 'Application stopped successfully',
		},
		cancel: {
			run: id => cancelMutation.mutateAsync(id),
			message: 'Action cancelled successfully',
		},
	};

	const handleAction = async (action: AppAction) => {
		if (actionInFlight.current && action !== 'cancel') return;
		actionInFlight.current = true;
		const currentSt = (app?.app_status || app?.status || '').toUpperCase();
		statusBeforeAction.current = currentSt;
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
		// invalidates ['applications'] when the action lands, refetching
		// fresh status through the live query.

		try {
			await actions[action].run(appId);
			toast.success(actions[action].message);
			refetchAll();
		} catch (err) {
			clearOverride();
			// Surface the RPC's own message — a generic "Action failed"
			// hid the builder's real error (e.g. a broken deployment
			// config query) and made failed deploys look like no-ops.
			toast.error(formatApiError(err));
		}
	};

	const handleUpdate = async (patchData: Partial<ApplicationResponse>) => {
		try {
			await patchMutation.mutateAsync({id: appId, ...(patchData as any)});
			toast.success('Application settings updated');
			refetchAll();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return {handleAction, handleUpdate};
}
