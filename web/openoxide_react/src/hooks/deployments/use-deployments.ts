import * as React from 'react';
import {
	useDeploymentCancel,
	useDeploymentDelete,
	useDeploymentDeleteByApplication,
	useDeploymentItems,
} from '#/api/live';
import {useQueryClient} from '@tanstack/react-query';

import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {DeploymentResponse} from '#/types/api-helpers';
import {useDeploymentLogs} from './use-deployment-logs';
import {filterAndSortDeployments} from '#/utils/deployment-filter-sort';
import {FINAL_STATES} from '#/components/projects/detail/deployments/build-status';

export type Deployment = DeploymentResponse;
export type SortKey = 'created_at' | 'title' | 'status';

export function useDeployments() {
	const queryClient = useQueryClient();
	const [refreshing, setRefreshing] = React.useState(false);

	// Filters and sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [statusFilter, setStatusFilter] = React.useState('all');
	const [typeFilter, setTypeFilter] = React.useState('all');
	const [sortBy, setSortBy] = React.useState<SortKey>('created_at');
	const [sortDir, setSortDir] = React.useState<'asc' | 'desc'>('desc');

	// Logs Dialog state
	const [selectedDeployment, setSelectedDeployment] =
		React.useState<Deployment | null>(null);

	// Delegate logs streaming state to the dedicated hook
	const {logs, isLogsLoading, copied, handleCopyLogs} =
		useDeploymentLogs(selectedDeployment);

	// Error Dialog state
	const [errorDetailDeployment, setErrorDetailDeployment] =
		React.useState<Deployment | null>(null);

	const cancelMutation = useDeploymentCancel();
	const deleteMutation = useDeploymentDelete();
	const clearAllMutation = useDeploymentDeleteByApplication();

	const {
		data: apiDeployments,
		isLoading: isApiLoading,
		refetch,
	} = useDeploymentItems();

	// Live query is the single source of truth; the table-watch stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const deployments = React.useMemo(() => {
		if (Array.isArray(apiDeployments)) {
			return apiDeployments as unknown as Deployment[];
		}
		return [];
	}, [apiDeployments]);

	const isLoading =
		isApiLoading && (!deployments || deployments.length === 0);

	const activeQueue = React.useMemo(() => {
		if (!deployments) return [];
		return deployments.filter(d => {
			const s = (d.status || '').toUpperCase();
			return !FINAL_STATES.includes(s);
		});
	}, [deployments]);

	const handleRefresh = async () => {
		setRefreshing(true);
		try {
			await refetch();
			await queryClient.invalidateQueries({
				queryKey: ['get', '/deployments'],
			});
			toast.success('Deployments refreshed');
		} finally {
			setRefreshing(false);
		}
	};

	const handleCancelDeployment = async (id: number) => {
		try {
			await cancelMutation.mutateAsync(id);
			toast.success('Deployment cancellation requested');
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteDeployment = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success(`Deployment #${id} deleted`);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleClearAllDeployments = async () => {
		try {
			const res = await clearAllMutation.mutateAsync(0);
			const data = res as any;
			toast.success(
				`Cleared ${data?.cleared_count || 0} deployment logs & history`,
			);
			queryClient.invalidateQueries({queryKey: ['get', '/deployments']});
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	// Filter & Sort list
	const filteredAndSorted = React.useMemo(
		() =>
			filterAndSortDeployments(deployments, {
				searchQuery,
				statusFilter,
				typeFilter,
				sortBy,
				sortDir,
			}),
		[deployments, searchQuery, statusFilter, typeFilter, sortBy, sortDir],
	);

	const clearFilters = () => {
		setSearchQuery('');
		setStatusFilter('all');
		setTypeFilter('all');
	};

	return {
		isLoading,
		refreshing,
		handleRefresh,
		searchQuery,
		setSearchQuery,
		statusFilter,
		setStatusFilter,
		typeFilter,
		setTypeFilter,
		sortBy,
		sortDir,
		setSortBy,
		setSortDir,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		isLogsLoading,
		copied,
		handleCopyLogs,
		errorDetailDeployment,
		setErrorDetailDeployment,
		handleCancelDeployment,
		handleDeleteDeployment,
		handleClearAllDeployments,
		filteredAndSorted,
		deployments,
		activeQueue,
		clearFilters,
	};
}
