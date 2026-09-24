import {runResourceAction} from '#/utils/run-resource-action';
import type {ComposeResponse} from '#/types/api-helpers';
import {
	useComposeCancel,
	useComposeDeploy,
	useComposeReload,
	useComposeStart,
	useComposeStop,
	useComposeUpdate,
} from '#/api/live';
import {intermediateStatusFor} from '../shared/resource-status';

export type ComposeAction =
	| 'deploy'
	| 'redeploy'
	| 'rebuild'
	| 'reload'
	| 'start'
	| 'stop'
	| 'cancel';

/** Lifecycle action + settings-patch handlers for a compose stack. Compose has no dedicated rebuild/redeploy RPC, so both fall back to a fresh deploy. */
export function useComposeActions(args: {
	composeId: number;
	compose: any;
	setLocalStatusOverride: (s: string | null) => void;
	refetchAll: () => void;
}) {
	const {composeId, compose, setLocalStatusOverride, refetchAll} = args;

	const deployMutation = useComposeDeploy();
	const reloadMutation = useComposeReload();
	const startMutation = useComposeStart();
	const stopMutation = useComposeStop();
	const cancelMutation = useComposeCancel();
	const patchMutation = useComposeUpdate();

	const actions: Record<
		ComposeAction,
		{run: () => Promise<unknown>; message: string}
	> = {
		deploy: {
			run: () => deployMutation.mutateAsync(composeId),
			message: 'Compose stack deployment triggered',
		},
		redeploy: {
			// No redeploy RPC — a fresh deploy covers the same semantics.
			run: () => deployMutation.mutateAsync(composeId),
			message: 'Compose stack redeploy triggered',
		},
		rebuild: {
			run: () => deployMutation.mutateAsync(composeId),
			message: 'Compose stack rebuild triggered',
		},
		reload: {
			run: () => reloadMutation.mutateAsync(composeId),
			message: 'Compose stack reloaded',
		},
		start: {
			run: () => startMutation.mutateAsync(composeId),
			message: 'Compose stack started',
		},
		stop: {
			run: () => stopMutation.mutateAsync(composeId),
			message: 'Compose stack stopped',
		},
		cancel: {
			run: () => cancelMutation.mutateAsync(composeId),
			message: 'Compose stack cancellation requested',
		},
	};

	const handleAction = async (action: ComposeAction) => {
		const currentStatus = String(
			compose?.compose_status || compose?.status || '',
		).toUpperCase();
		setLocalStatusOverride(intermediateStatusFor(action, currentStatus));
		// Legacy updateServiceStatus store write removed — table-watch invalidates ['composes'] when the action lands, refetching fresh status through the live query.

		await runResourceAction(actions[action].run, {
			success: actions[action].message,
			onDone: refetchAll,
		});
	};

	const handleUpdate = async (patchData: Partial<ComposeResponse>) => {
		await runResourceAction(
			() =>
				patchMutation.mutateAsync({id: composeId, ...(patchData as any)}),
			{success: 'Compose settings updated', onDone: refetchAll},
		);
	};

	return {handleAction, handleUpdate};
}
