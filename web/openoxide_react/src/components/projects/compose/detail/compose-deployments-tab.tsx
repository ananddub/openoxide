import {useState} from 'react';
import {toast} from 'sonner';
import {
	useComposeCancel,
	useComposeDeploy,
	useComposeRedeploy,
	useDeploymentDelete,
	useDeploymentDeleteByCompose,
	useDeploymentItems,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {useDeploymentLogStream} from '#/components/projects/detail/deployments/use-deployment-log-stream';
import {DeploymentsHeader} from '#/components/projects/detail/deployments/deployments-header';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
import {StreamLogsModal} from '#/components/projects/detail/deployments/stream-logs-modal';
import {CancelDeploymentDialog} from '#/components/projects/detail/deployments/cancel-deployment-dialog';

interface ComposeDeploymentsTabProps {
	composeId: number;
	deployments?: any[];
}

export function ComposeDeploymentsTab({
	composeId,
	deployments: passedDeployments,
}: ComposeDeploymentsTabProps) {
	const [activeLogId, setActiveLogId] = useState<number | null>(null);
	const [isTriggering, setIsTriggering] = useState(false);
	const [isConfirmCancelOpen, setIsConfirmCancelOpen] = useState(false);

	// Live deployments query filtered to this compose; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: allDeployments = []} = useDeploymentItems();
	const rawDeployments = allDeployments.filter(
		(d: any) => String(d.compose_id) === String(composeId),
	);
	const deployments =
		rawDeployments.length > 0 ? rawDeployments : (passedDeployments ?? []);
	const activeDeployment = deployments.find(isBuildActive);
	const selectedEvent = deployments.find(d => d.id === activeLogId);

	// Historical rows fall back to their stored log text while streaming.
	const fallback =
		(selectedEvent as any)?.log_content ||
		(selectedEvent as any)?.message ||
		selectedEvent?.description;
	const liveLogs = useDeploymentLogStream(activeLogId, fallback);

	const deployMutation = useComposeDeploy();
	const redeployMutation = useComposeRedeploy();
	const cancelMutation = useComposeCancel();
	const deleteMutation = useDeploymentDelete();
	const clearComposeMutation = useDeploymentDeleteByCompose();

	const run = async (action: () => Promise<unknown>, success: string) => {
		setIsTriggering(true);
		try {
			await action();
			toast.success(success);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsTriggering(false);
		}
	};

	const handleDeploy = () =>
		run(
			() => deployMutation.mutateAsync(composeId),
			'Compose deployment triggered successfully',
		);
	const handleRedeploy = () =>
		run(
			() => redeployMutation.mutateAsync(composeId),
			'Compose redeploy triggered successfully',
		);
	const handleCancel = async () => {
		await run(
			() => cancelMutation.mutateAsync(composeId),
			'Deployment cancellation requested',
		);
		setIsConfirmCancelOpen(false);
	};

	const handleClearComposeDeployments = async () => {
		try {
			const res = await clearComposeMutation.mutateAsync(composeId);
			const data = res as any;
			toast.success(
				`Cleared ${data?.cleared_count || 0} compose deployment logs & history`,
			);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteSingleDeployment = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success(`Deployment #${id} deleted`);
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
				onClear={handleClearComposeDeployments}
				clearDisabled={isActionPending}
				onRedeploy={handleRedeploy}
				redeployDisabled={!!activeDeployment || isActionPending}
				redeploySpinning={isActionPending}
				isProcessing={isActionPending}
				showCancel={!!activeDeployment}
				onCancel={() => setIsConfirmCancelOpen(true)}
				onDeploy={handleDeploy}
			/>

			<DeploymentsList
				deployments={deployments}
				isLoading={false}
				onOpenStream={setActiveLogId}
				onCancel={() => setIsConfirmCancelOpen(true)}
				onDelete={handleDeleteSingleDeployment}
			/>

			<CancelDeploymentDialog
				open={isConfirmCancelOpen}
				onOpenChange={setIsConfirmCancelOpen}
				onConfirm={handleCancel}
				title="Cancel Compose Deployment"
				description="Are you sure you want to cancel this compose deployment? This action will stop the running build process."
				cancelLabel="No, Keep Running"
			/>

			<StreamLogsModal
				deploymentId={activeLogId}
				logs={liveLogs}
				onClose={() => setActiveLogId(null)}
			/>
		</div>
	);
}
