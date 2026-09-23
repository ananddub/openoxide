import {useState} from 'react';
import {toast} from 'sonner';
import {Rocket} from 'lucide-react';
import {
	useComposeDeploy,
	useComposeRedeploy,
	useComposeReload,
	useComposeStart,
	useComposeStop,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {ComposeConfirmDialog} from './cards/compose-confirm-dialog';
import {ComposeDeployActions} from './cards/compose-deploy-actions';
import {
	useComposeDeployState,
	type ComposeActionType,
} from './cards/use-compose-deploy-state';

interface ComposeDeployCardProps {
	compose: any;
	onUpdated?: () => void;
	onOpenTerminal?: () => void;
	/** Render inside another card (no own panel background/shadow). */
	embedded?: boolean;
}

export function ComposeDeployCard({
	compose,
	onUpdated,
	onOpenTerminal,
	embedded = false,
}: ComposeDeployCardProps) {
	const [confirmAction, setConfirmAction] =
		useState<ComposeActionType | null>(null);
	const [activeLoading, setActiveLoading] =
		useState<ComposeActionType | null>(null);

	const {setActionInFlight, pendingMutationAction, isBuilding, isRunning} =
		useComposeDeployState(compose);

	// Mutations
	const deployMutation = useComposeDeploy();
	const reloadMutation = useComposeReload();
	const rebuildMutation = useComposeRedeploy();
	const startMutation = useComposeStart();
	const stopMutation = useComposeStop();

	const executeAction = async (action: ComposeActionType) => {
		setConfirmAction(null);
		setActiveLoading(action);
		if (['start', 'deploy', 'rebuild', 'reload'].includes(action)) {
			setActionInFlight(action);
		} else {
			setActionInFlight(null);
		}
		try {
			const id = compose?.id;
			if (action === 'deploy') {
				await deployMutation.mutateAsync(id);
				toast.success('Compose stack deployment initiated');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync(id);
				toast.success('Compose stack reloaded');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync(id);
				toast.success('Compose stack rebuild initiated');
			} else if (action === 'start') {
				await startMutation.mutateAsync(id);
				toast.success('Compose stack started');
			} else if (action === 'stop' || action === 'cancel') {
				await stopMutation.mutateAsync(id);
				toast.success(
					action === 'stop'
						? 'Compose stack stopped'
						: 'Compose deployment cancelled',
				);
			}

			// Live queries refresh via the table-watch stream; notify the parent.
			onUpdated?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
			setActionInFlight(null);
		} finally {
			setActiveLoading(null);
		}
	};

	const isProcessing = activeLoading !== null || !!pendingMutationAction;

	return (
		<section
			className={
				embedded
					? 'flex flex-col gap-3'
					: 'flex flex-col gap-4 rounded-xl bg-card p-5 shadow-sm'
			}>
			<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
				<Rocket className="size-3.5 text-primary" />
				Deploy Settings
			</h3>

			<ComposeDeployActions
				isProcessing={isProcessing}
				isBuilding={isBuilding}
				isRunning={isRunning}
				composeStatus={compose?.compose_status}
				activeLoading={activeLoading || pendingMutationAction}
				onAction={executeAction}
				onOpenTerminal={onOpenTerminal}
			/>

			<ComposeConfirmDialog
				confirmAction={confirmAction}
				onClose={() => setConfirmAction(null)}
				onConfirm={executeAction}
			/>
		</section>
	);
}
