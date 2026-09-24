import {useState} from 'react';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {formatApiError} from '#/api/utils';
import {
	useDeploymentDeleteByDatabase,
	useDeploymentItems,
} from '#/api/live';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {useAutoExpandBuildLogs} from '#/components/projects/detail/deployments/use-auto-expand-build-logs';
import {DeploymentsHeader} from '#/components/projects/detail/deployments/deployments-header';
import {DeploymentsList} from '#/components/projects/detail/deployments/deployments-list';
import {CancelDeploymentDialog} from '#/components/projects/detail/deployments/cancel-deployment-dialog';

interface DatabaseDeploymentsTabProps {
	dbId: number;
	kind?: string;
	database?: any;
	deployments?: any[];
	onRefresh?: () => void;
	onAction?: (
		action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel',
	) => Promise<void>;
}

/** Read-only provisioning/deployment history for a database. Deploy / redeploy / start / stop live on the General tab, so this view only carries the audit log plus its Cancel/Clear controls. */
export function DatabaseDeploymentsTab({
	dbId,
	kind = 'postgres',
	deployments: passedDeployments,
	onRefresh,
	onAction,
}: DatabaseDeploymentsTabProps) {
	const queryClient = useQueryClient();
	const [expandedId, setExpandedId] = useState<number | null>(null);
	const [cancelingId, setCancelingId] = useState<number | null>(null);

	// Live deployments query filtered to this database; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: allDeployments = []} = useDeploymentItems();
	const dbDeployments = allDeployments.filter(
		(d: any) => String(d.database_id) === String(dbId),
	);
	const events =
		dbDeployments.length > 0 ? dbDeployments : (passedDeployments ?? []);
	const activeDeployment = events.find(isBuildActive);
	useAutoExpandBuildLogs(events, setExpandedId);

	const clearDbMutation = useDeploymentDeleteByDatabase();

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

	return (
		<div className="flex flex-col gap-4">
			<DeploymentsHeader
				title="Deployment logs"
				description={`Provisioning, container deployment and lifecycle events for this ${kind} database.`}
				pillLabel={activeDeployment ? 'Provisioning Container...' : null}
				onClear={handleClearDatabaseDeployments}
				onRefresh={() => onRefresh?.()}
				showCancel={!!activeDeployment}
				onCancel={() =>
					activeDeployment?.id && setCancelingId(activeDeployment.id)
				}
			/>

			<DeploymentsList
				deployments={events}
				emptyLabel="No database deployment history recorded yet"
				emptyDescription="Deploy the database from the General tab to start the history."
				titlePrefix="Database Deployment"
				defaultStatus="DEPLOYED"
				expandedId={expandedId}
				onToggleExpand={id =>
					setExpandedId(prev => (prev === id ? null : id))
				}
			/>

			<CancelDeploymentDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}
				onConfirm={confirmCancel}
				title="Cancel Database Deployment"
				description="Are you sure you want to cancel this database container deployment? This action will stop the running container setup."
			/>
		</div>
	);
}
