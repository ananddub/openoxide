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
import {useAutoExpandBuildLogs} from '#/components/projects/detail/deployments/use-auto-expand-build-logs';
import {DeploymentsHeader} from '#/components/projects/detail/deployments/deployments-header';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
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
	const [expandedId, setExpandedId] = useState<number | null>(null);
	const [isTriggering, setIsTriggering] = useState(false);
	const [cancelingId, setCancelingId] = useState<number | null>(null);

	const events = Array.isArray(passedDeployments) ? passedDeployments : [];
	const activeDeployment = events.find(isBuildActive);
	useAutoExpandBuildLogs(events, setExpandedId);

	const deployMutation = useApplicationDeploy();
	const redeployMutation = useApplicationRedeploy();
	const cancelMutation = useDeploymentCancel();
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

	const isActionPending =
		isTriggering ||
		deployMutation.isPending ||
		redeployMutation.isPending ||
		cancelMutation.isPending;

	return (
		<div className="flex flex-col gap-4">
			<DeploymentsHeader
				title="Deployment logs"
				description="Build, deploy and lifecycle events for this application."
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
				expandedId={expandedId}
				onToggleExpand={id =>
					setExpandedId(prev => (prev === id ? null : id))
				}
			/>

			<CancelDeploymentDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}
				onConfirm={confirmCancel}
			/>
		</div>
	);
}
