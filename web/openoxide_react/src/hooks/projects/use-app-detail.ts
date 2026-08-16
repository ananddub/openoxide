import {useState, useMemo, useEffect} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import { useAppStore, selectApplicationById } from '#/stores/app-store';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');
	const [localStatusOverride, setLocalStatusOverride] = useState<string | null>(null);

	// 100% Pure Centralized Zustand Store Resolution
	const rawApp = useAppStore((state) => selectApplicationById(state, appId));

	const app = rawApp ? {
		...rawApp,
		status: localStatusOverride || (rawApp as any).app_status || (rawApp as any).status || 'STOPPED',
		app_status: localStatusOverride || (rawApp as any).app_status || (rawApp as any).status || 'STOPPED',
	} : null;

	// Auto-clear localStatusOverride when Zustand status updates
	useEffect(() => {
		if (app && localStatusOverride) {
			const fetchedStatus = (app.status || app.app_status || '').toUpperCase();
			const overrideUpper = localStatusOverride.toUpperCase();
			if (
				fetchedStatus === overrideUpper ||
				(overrideUpper === 'STARTING' && (fetchedStatus === 'RUNNING' || fetchedStatus === 'HEALTHY')) ||
				(overrideUpper === 'STOPPING' && (fetchedStatus === 'STOPPED' || fetchedStatus === 'IDLE' || fetchedStatus === 'CANCELLED')) ||
				(overrideUpper === 'CANCELLING' && (fetchedStatus === 'CANCELLED' || fetchedStatus === 'STOPPED' || fetchedStatus === 'IDLE'))
			) {
				setLocalStatusOverride(null);
			}
		}
	}, [app, localStatusOverride]);

	// Read domains, schedules, backups, and deployments 100% directly from Zustand RAM store (0ms Instant & 0 extra queries)
	const domains = useAppStore((state) =>
		(state.domains || []).filter((d: any) => Number(d.application_id) === Number(appId))
	);
	const schedules = useAppStore((state) =>
		(state.schedules || []).filter((s: any) => Number(s.application_id) === Number(appId))
	);
	const backups = useAppStore((state) =>
		(state.backups || []).filter((b: any) => Number(b.application_id) === Number(appId))
	);
	const deployments = useAppStore((state) =>
		(state.deployments || []).filter((d: any) => Number(d.application_id) === Number(appId))
	);

	const isLoadingDomains = false;
	const isLoadingSchedules = false;
	const isLoadingBackups = false;
	const isLoadingDeployments = false;

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(appId, 'application');

	// Live hooks auto-push updates — only trigger monitoring refresh
	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const deployMutation = $api.useMutation('post', '/applications/{id}/deploy');
	const reloadMutation = $api.useMutation('post', '/applications/{id}/reload');
	const rebuildMutation = $api.useMutation('post', '/applications/{id}/rebuild');
	const startMutation = $api.useMutation('post', '/applications/{id}/start');
	const stopMutation = $api.useMutation('post', '/applications/{id}/stop');
	const cancelMutation = $api.useMutation('post', '/applications/{id}/cancel');
	const patchMutation = $api.useMutation('patch', '/applications/{id}');
	const deleteMutation = $api.useMutation('delete', '/applications/{id}');

	const handleAction = async (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel') => {
		const currentSt = (app?.app_status || app?.status || '').toUpperCase();
		const isCurrentlyBuilding = ['QUEUED', 'BUILDING', 'STARTING', 'PREPARING', 'PENDING', 'DEPLOYING'].includes(currentSt);
		const intermediateStatus = (action === 'stop' || action === 'cancel')
			? (isCurrentlyBuilding ? 'CANCELLING' : 'STOPPING')
			: action === 'start' ? 'STARTING'
			: 'DEPLOYING';
		setLocalStatusOverride(intermediateStatus);
		(useAppStore.getState() as any).updateServiceStatus?.(appId, intermediateStatus);

		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Deployment triggered successfully');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application reloaded successfully');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application started successfully');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application stopped successfully');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Action cancelled successfully');
			}
			refetchAll();
		} catch (err) {
			toast.error(`Action failed: ${action}`);
		}
	};

	const handleUpdate = async (patchData: Partial<ApplicationResponse>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: appId}},
				body: patchData as any,
			});
			toast.success('Application settings updated');
			refetchAll();
		} catch (err) {
			toast.error('Failed to update application settings');
		}
	};

	return {
		app,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: !app,
		isLoadingDomains,
		isLoadingSchedules,
		isLoadingBackups,
		isLoadingDeployments,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
	};
}
