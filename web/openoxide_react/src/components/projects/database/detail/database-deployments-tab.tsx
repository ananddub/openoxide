import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {formatApiError} from '#/api/utils';
import {
	useDeploymentDelete,
	useDeploymentDeleteByDatabase,
	useDeploymentItems,
} from '#/api/live';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {useDeploymentLogStream} from '#/components/projects/detail/deployments/use-deployment-log-stream';
import {DeploymentsHeader} from '#/components/projects/detail/deployments/deployments-header';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
import {StreamLogsModal} from '#/components/projects/detail/deployments/stream-logs-modal';
import {CancelDeploymentDialog} from '#/components/projects/detail/deployments/cancel-deployment-dialog';

interface DatabaseDeploymentsTabProps {
	dbId: number;
	kind?: string;
	database?: any;
	deployments?: any[];
	actionLoading?: string | null;
	onRefresh?: () => void;
	onAction?: (
		action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel',
	) => Promise<void>;
}

export function DatabaseDeploymentsTab({
	dbId,
	kind = 'postgres',
	deployments: passedDeployments,
	actionLoading,
	onRefresh,
	onAction,
}: DatabaseDeploymentsTabProps) {
	const queryClient = useQueryClient();
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const liveLogs = useDeploymentLogStream(activeLogId);
	const [cancelingId, setCancelingId] = useState<number | null>(null);
	// Optimistic hold: flips the header toggle to "Deploying..." the moment
	// Deploy/Redeploy is clicked, without waiting for the QUEUED deployment
	// row to round-trip through the table-watch invalidation. Released when
	// the row lands (activeDeployment) or by the safety timeout if the
	// trigger failed server-side and no row will arrive.
	const [pendingTrigger, setPendingTrigger] = useState<
		'deploy' | 'redeploy' | null
	>(null);

	// Live deployments query filtered to this database; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: allDeployments = []} = useDeploymentItems();
	const dbDeployments = allDeployments.filter(
		(d: any) => String(d.database_id) === String(dbId),
	);
	const events =
		dbDeployments.length > 0 ? dbDeployments : (passedDeployments ?? []);
	const activeDeployment = events.find(isBuildActive);

	const deleteMutation = useDeploymentDelete();
	const clearDbMutation = useDeploymentDeleteByDatabase();

	const isTriggerPending =
		pendingTrigger !== null ||
		actionLoading === 'deploy' ||
		actionLoading === 'redeploy';

	// The optimistic QUEUED row always lands before the mutation resolves;
	// if it never arrives (trigger failed), the timeout stops the button
	// from being stuck in "Deploying..." forever.
	useEffect(() => {
		if (!pendingTrigger) return;
		const timer = setTimeout(() => setPendingTrigger(null), 6000);
		return () => clearTimeout(timer);
	}, [pendingTrigger]);

	useEffect(() => {
		if (activeDeployment && pendingTrigger) setPendingTrigger(null);
	}, [activeDeployment, pendingTrigger]);

	const handleClearDatabaseDeployments = async () => {
		try {
			const res = await clearDbMutation.mutateAsync(dbId);
			const data = res as any;
			toast.success(
				`Cleared ${data?.cleared_count || 0} database deployment logs & history`,
			);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteSingleDeployment = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success(`Deployment #${id} deleted`);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			onRefresh?.();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const confirmCancel = async () => {
		if (!cancelingId) return;
		try {
			await onAction?.('cancel');
			toast.success('Database deployment cancellation requested');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setCancelingId(null);
		}
	};

	const deploying = pendingTrigger === 'deploy';

	return (
		<div className="flex flex-col gap-6">
			<DeploymentsHeader
				title="Database Container Deployments History"
				description={`Audit log of provisioning, database container deployment operations and container lifecycle events (${kind.toUpperCase()})`}
				pillLabel={activeDeployment ? 'Provisioning Container...' : null}
				onClear={handleClearDatabaseDeployments}
				onRefresh={() => onRefresh?.()}
				redeployLabel={
					pendingTrigger === 'redeploy'
						? 'Redeploying...'
						: 'Redeploy Container'
				}
				onRedeploy={() => {
					setPendingTrigger('redeploy');
					void onAction?.('redeploy');
				}}
				redeployDisabled={!!activeDeployment || isTriggerPending}
				redeploySpinning={pendingTrigger === 'redeploy'}
				showCancel={!!activeDeployment || deploying}
				cancelDisabled={!activeDeployment}
				cancelLabel={
					deploying && !activeDeployment ? 'Deploying...' : 'Cancel Build'
				}
				onCancel={() =>
					activeDeployment?.id && setCancelingId(activeDeployment.id)
				}
				onDeploy={() => {
					setPendingTrigger('deploy');
					void onAction?.('deploy');
				}}
				deployLabel="Deploy Database"
			/>

			<DeploymentsList
				deployments={events}
				emptyLabel="No database deployment history recorded yet"
				titlePrefix="Database Deployment"
				defaultStatus="DEPLOYED"
				onOpenStream={setActiveLogId}
				onCancel={setCancelingId}
				onDelete={handleDeleteSingleDeployment}
			/>

			<CancelDeploymentDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}
				onConfirm={confirmCancel}
				title="Cancel Database Deployment"
				description="Are you sure you want to cancel this database container deployment? This action will stop the running container setup."
			/>

			<StreamLogsModal
				deploymentId={activeLogId}
				logs={liveLogs}
				onClose={() => setActiveLogId(null)}
				streamLabel="Live Database Container Deployment Stream"
				loadingText="Connecting to realtime SSE database deployment stream..."
				emptyText="No container deployment build log entries available."
			/>
		</div>
	);
}
