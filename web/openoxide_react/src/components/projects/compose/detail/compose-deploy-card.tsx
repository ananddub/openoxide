import {useState, useEffect, useMemo} from 'react';
import {useMutationState} from '@tanstack/react-query';
import {toast} from 'sonner';
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
import {useDeploymentItems} from '#/api/live';

interface ComposeDeployCardProps {
	compose: any;
	onUpdated?: () => void;
	onOpenTerminal?: () => void;
}

type ActionType =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'stop'
	| 'cancel';

export function ComposeDeployCard({
	compose,
	onUpdated,
	onOpenTerminal,
}: ComposeDeployCardProps) {
	const [confirmAction, setConfirmAction] = useState<ActionType | null>(
		null,
	);
	const [activeLoading, setActiveLoading] = useState<ActionType | null>(
		null,
	);

	// Live deployments query filtered to this compose; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: storeDeployments = []} = useDeploymentItems();
	const rawDeployments = useMemo(() => {
		return storeDeployments.filter(
			(d: any) => String(d.compose_id) === String(compose?.id),
		);
	}, [storeDeployments, compose?.id]);

	const events = useMemo(
		() => (Array.isArray(rawDeployments) ? rawDeployments : []),
		[rawDeployments],
	);
	const deploymentRevision = useMemo(
		() =>
			events
				.map(
					(event: any) =>
						`${event.id}:${event.status}:${event.state}:${event.last_state_at ?? ''}`,
				)
				.join('|'),
		[events],
	);

	// 2. Track any pending compose mutation automatically via TanStack Query useMutationState
	const pendingMutationAction = useMutationState({
		filters: {status: 'pending'},
		select: mutation => {
			const meta = mutation.state.variables as any;
			return meta?.actionName as ActionType | undefined;
		},
	})[0];

	const hasActiveDeployment = (events || []).some((e: any) => {
		if (!e || (e.finished_at && Number(e.finished_at) > 0)) return false;
		const s = (e.status || e.state || '').toUpperCase();
		return [
			'QUEUED',
			'RUNNING',
			'BUILDING',
			'STARTING',
			'DEPLOYING',
			'PREPARING',
			'PENDING',
		].includes(s);
	});

	const [actionInFlight, setActionInFlight] = useState<ActionType | null>(
		null,
	);

	const rawStatus = (
		compose?.compose_status ||
		compose?.status ||
		''
	).toLowerCase();
	const isRunning = [
		'running',
		'deployed',
		'done',
		'success',
		'active',
		'ok',
	].includes(rawStatus);
	const isBuilding =
		!!actionInFlight ||
		!!pendingMutationAction ||
		hasActiveDeployment ||
		['starting', 'building', 'queued', 'preparing'].includes(rawStatus);

	// Smoothly handoff actionInFlight to queries once backend returns active deployment or updated status
	useEffect(() => {
		if (
			hasActiveDeployment ||
			rawStatus === 'running' ||
			rawStatus === 'starting' ||
			rawStatus === 'error'
		) {
			setActionInFlight(null);
		}
	}, [hasActiveDeployment, rawStatus]);


	useEffect(() => {
		console.debug('[Compose Deployments Live]', {
			composeId: compose?.id,
			count: events.length,
			deployments: events.map((event: any) => ({
				id: event.id,
				status: event.status,
				state: event.state,
				lastStateAt: event.last_state_at,
			})),
		});
	}, [compose?.id, deploymentRevision]);

	// Mutations
	const deployMutation = useComposeDeploy();
	const reloadMutation = useComposeReload();
	const rebuildMutation = useComposeRedeploy();
	const startMutation = useComposeStart();
	const stopMutation = useComposeStop();

	const executeAction = async (action: ActionType) => {
		setConfirmAction(null);
		setActiveLoading(action);
		if (
			action === 'start' ||
			action === 'deploy' ||
			action === 'rebuild' ||
			action === 'reload'
		) {
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
		<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-sm">
			<h3 className="text-sm font-bold text-foreground">
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
