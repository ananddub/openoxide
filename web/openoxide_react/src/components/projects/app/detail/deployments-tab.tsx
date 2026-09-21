import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {
	useApplicationDeploy,
	useApplicationRedeploy,
	useDeploymentCancel,
	useDeploymentDelete,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {useDeploymentLogStream} from '#/components/projects/detail/deployments/use-deployment-log-stream';
import {DeploymentsHeader} from '#/components/projects/detail/deployments/deployments-header';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
import {StreamLogsModal} from '#/components/projects/detail/deployments/stream-logs-modal';
import {CancelDeploymentDialog} from '#/components/projects/detail/deployments/cancel-deployment-dialog';

interface DeploymentsTabProps {
	appId: number;
	deployments?: any[];
	onRefresh?: () => void;
}

export function DeploymentsTab({
	appId,
	deployments: passedDeployments,
	onRefresh,
}: DeploymentsTabProps) {
	const queryClient = useQueryClient();
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const liveLogs = useDeploymentLogStream(activeLogId);
	const [isTriggering, setIsTriggering] = useState(false);
	const [cancelingId, setCancelingId] = useState<number | null>(null);

	const events = Array.isArray(passedDeployments) ? passedDeployments : [];
	const activeDeployment = events.find(isBuildActive);

	const deployMutation = useApplicationDeploy();
	const redeployMutation = useApplicationRedeploy();
	const cancelMutation = useDeploymentCancel();
	const deleteMutation = useDeploymentDelete();
	const clearAppMutation = useDeploymentDelete();

	/** Run a deployment trigger/cancel with shared busy state + toasts. */
	const run = async (action: () => Promise<unknown>, success: string) => {
		setIsTriggering(true);
		try {
			await action();
			toast.success(success);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleDeploy = () =>
		run(
			() => deployMutation.mutateAsync(appId),
			'Deployment triggered successfully',
		);
	const handleRedeploy = () =>
		run(
			() => redeployMutation.mutateAsync(appId),
			'Redeploy triggered successfully',
		);
	const confirmCancel = async () => {
		if (!cancelingId) return;
		await run(
			() => cancelMutation.mutateAsync(cancelingId),
			'Deployment cancellation requested',
		);
		setCancelingId(null);
	};

	const handleClearAppDeployments = async () => {
		try {
			const res = await clearAppMutation.mutateAsync(appId);
			const data = res as any;
			toast.success(
				`Cleared ${data?.cleared_count || 0} deployment logs & history`,
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

	const isActionPending =
		isTriggering ||
		deployMutation.isPending ||
		redeployMutation.isPending ||
		cancelMutation.isPending;

	return (
		<div className="flex flex-col gap-6">
			<DeploymentsHeader
				title="Deployments History"
				description="Audit log of building, deployment operations and lifecycle events"
				pillLabel={
					activeDeployment || isActionPending
						? isActionPending
							? 'Triggering...'
							: 'Active Build'
						: null
				}
				onClear={handleClearAppDeployments}
				clearDisabled={isActionPending}
				onRefresh={() => onRefresh?.()}
				refreshSpinning={isActionPending}
				onRedeploy={handleRedeploy}
				redeployDisabled={!!activeDeployment || isActionPending}
				redeploySpinning={isActionPending}
				isProcessing={isActionPending}
				showCancel={!!activeDeployment}
				onCancel={() =>
					activeDeployment?.id && setCancelingId(activeDeployment.id)
				}
				onDeploy={handleDeploy}
			/>

			<DeploymentsList
				deployments={events}
				onOpenStream={setActiveLogId}
				onCancel={setCancelingId}
				onDelete={handleDeleteSingleDeployment}
			/>

			<CancelDeploymentDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}
				onConfirm={confirmCancel}
			/>

			<StreamLogsModal
				deploymentId={activeLogId}
				logs={liveLogs}
				onClose={() => setActiveLogId(null)}
				loadingText="Connecting to realtime SSE deployment stream..."
			/>
		</div>
	);
}
