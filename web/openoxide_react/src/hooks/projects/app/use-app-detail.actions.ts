import {runResourceAction} from '#/utils/run-resource-action';
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
import {intermediateStatusFor} from '../shared/resource-status';

export type AppAction =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'stop'
	| 'cancel';

/** Lifecycle action + settings-patch handlers. Guards against overlapping in-flight actions (cancel excepted), layers an optimistic intermediate status and relies on table-watch to refetch the settled state. */
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
		const currentStatus = String(
			app?.app_status || app?.status || '',
		).toUpperCase();
		statusBeforeAction.current = currentStatus;
		setLocalStatusOverride(intermediateStatusFor(action, currentStatus));
		// Legacy updateServiceStatus store write removed — table-watch invalidates ['applications'] when the action lands, refetching fresh status through the live query.

		await runResourceAction(() => actions[action].run(appId), {
			success: actions[action].message,
			onDone: refetchAll,
			// Surface the RPC's own message — a generic "Action failed" hid the builder's real error and made failed deploys look like no-ops.
			onFail: clearOverride,
		});
	};

	const handleUpdate = async (patchData: Partial<ApplicationResponse>) => {
		await runResourceAction(
			() => patchMutation.mutateAsync({id: appId, ...(patchData as any)}),
			{success: 'Application settings updated', onDone: refetchAll},
		);
	};

	return {handleAction, handleUpdate};
}
