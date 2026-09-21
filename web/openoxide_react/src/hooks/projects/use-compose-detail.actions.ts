import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ComposeResponse} from '#/types/api-helpers';
import {
	useComposeCancel,
	useComposeDeploy,
	useComposeReload,
	useComposeStart,
	useComposeStop,
	useComposeUpdate,
} from '#/api/live';

export type ComposeAction =
	| 'deploy'
	| 'redeploy'
	| 'rebuild'
	| 'reload'
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
 * Lifecycle action + settings-patch handlers for a compose stack. Compose has
 * no dedicated rebuild/redeploy RPC, so both fall back to a fresh deploy.
 */
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

	const handleAction = async (action: ComposeAction) => {
		const currentSt = (
			compose?.compose_status ||
			compose?.status ||
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
		// invalidates ['composes'] when the action lands, refetching
		// fresh status through the live query.

		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync(composeId);
				toast.success('Compose stack deployment triggered');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync(composeId);
				toast.success('Compose stack reloaded');
			} else if (action === 'start') {
				await startMutation.mutateAsync(composeId);
				toast.success('Compose stack started');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync(composeId);
				toast.success('Compose stack stopped');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync(composeId);
				toast.success('Compose stack cancellation requested');
			} else if (action === 'rebuild' || action === 'redeploy') {
				// Compose has no dedicated rebuild/redeploy RPC — a fresh
				// deploy covers the same semantics.
				await deployMutation.mutateAsync(composeId);
				toast.success(`Compose stack ${action} triggered`);
			}
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleUpdate = async (patchData: Partial<ComposeResponse>) => {
		try {
			await patchMutation.mutateAsync({
				id: composeId,
				...(patchData as any),
			});
			toast.success('Compose settings updated');
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {handleAction, handleUpdate};
}
