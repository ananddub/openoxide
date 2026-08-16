import * as React from 'react';
import {$api} from '#/api/query';
import {authFetch} from '#/api/client';

import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {components} from '#/types/api';
import {useDeploymentLogs} from './use-deployment-logs';
import {useAppStore} from '#/stores/app-store';

export type Deployment = components['schemas']['DeploymentResponseDto'];
export type SortKey = 'created_at' | 'title' | 'status';

const FINAL_STATES = ['DONE', 'DEPLOYED', 'SUCCESS', 'FAILED', 'ERROR', 'CANCELLED', 'STOPPEDBYUSER', 'CRASHED'];

export function useDeployments() {
	const [refreshing] = React.useState(false);

	// Filters and sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [statusFilter, setStatusFilter] = React.useState('all');
	const [typeFilter, setTypeFilter] = React.useState('all');
	const [sortBy, setSortBy] = React.useState<SortKey>('created_at');
	const [sortDir, setSortDir] = React.useState<'asc' | 'desc'>('desc');

	// Logs Dialog state
	const [selectedDeployment, setSelectedDeployment] = React.useState<Deployment | null>(null);
	
	// Delegate logs streaming state to the dedicated hook
	const {logs, isLogsLoading, copied, handleCopyLogs} = useDeploymentLogs(selectedDeployment);

	// Error Dialog state
	const [errorDetailDeployment, setErrorDetailDeployment] = React.useState<Deployment | null>(null);

	const cancelMutation = $api.useMutation('post', '/deployments/{id}/cancel');

	const storeDeployments = useAppStore((state) => state.deployments);

	const deployments = (storeDeployments || []) as unknown as Deployment[];
	const isLoading = false;

	const activeQueue = React.useMemo(() => {
		if (!deployments) return [];
		return deployments.filter(d => {
			const s = (d.status || '').toUpperCase();
			return !FINAL_STATES.includes(s);
		});
	}, [deployments]);

	const handleRefresh = () => {
		toast.success('Deployments list is live and auto-updating');
	};

	const handleCancelDeployment = async (id: number) => {
		try {
			await cancelMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
			toast.success('Deployment cancellation requested');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDeleteDeployment = async (id: number) => {
		try {
			const res = await authFetch(`/deployments/${id}`, {
				method: 'DELETE',
			});
			if (!res.ok) throw new Error('Failed to delete deployment');
			toast.success(`Deployment #${id} deleted`);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleClearAllDeployments = async () => {
		try {
			const res = await authFetch('/deployments', {
				method: 'DELETE',
			});
			if (!res.ok) throw new Error('Failed to clear deployments');
			const data = await res.json();
			toast.success(`Cleared ${data.cleared_count || 0} deployment logs & history`);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	// Filter & Sort list
	const filteredAndSorted = React.useMemo(() => {
		if (!deployments) return [];

		// 1. Filtering
		let result = deployments.filter(d => {
			const q = searchQuery.toLowerCase();
			const matchesSearch =
				d.title.toLowerCase().includes(q) ||
				(d.description || '').toLowerCase().includes(q) ||
				(d.error_message || '').toLowerCase().includes(q);

			const status = d.status.toUpperCase();
			const matchesStatus =
				statusFilter === 'all' ||
				(statusFilter === 'running' && (status === 'RUNNING' || status === 'CANCELLING')) ||
				(statusFilter === 'queued' && status === 'QUEUED') ||
				(statusFilter === 'done' && status === 'DONE') ||
				(statusFilter === 'error' && status === 'ERROR') ||
				(statusFilter === 'cancelled' && (status === 'CANCELLED' || status === 'CANCELLING'));

			const hasApp = d.application_id !== null && d.application_id !== undefined;
			const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
			const hasDatabase = d.database_id !== null && d.database_id !== undefined;

			const matchesType =
				typeFilter === 'all' ||
				(typeFilter === 'application' && hasApp) ||
				(typeFilter === 'compose' && hasCompose) ||
				(typeFilter === 'database' && hasDatabase);

			return matchesSearch && matchesStatus && matchesType;
		});

		// 2. Sorting
		return [...result].sort((a, b) => {
			if (sortBy === 'created_at') {
				return sortDir === 'desc'
					? Number(b.created_at || 0) - Number(a.created_at || 0)
					: Number(a.created_at || 0) - Number(b.created_at || 0);
			}
			if (sortBy === 'title') {
				return sortDir === 'desc' ? b.title.localeCompare(a.title) : a.title.localeCompare(b.title);
			}
			if (sortBy === 'status') {
				return sortDir === 'desc' ? b.status.localeCompare(a.status) : a.status.localeCompare(b.status);
			}
			return 0;
		});
	}, [deployments, searchQuery, statusFilter, typeFilter, sortBy, sortDir]);

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
		activeQueue,
		clearFilters,
	};
}
