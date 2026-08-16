import {useState, useMemo, useEffect} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useDomainListByApplication,
	useScheduleListByApplication,
} from 'virtual:openoxide-live';

import { useAppStore } from '#/stores/app-store';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');

	// 100% Pure Realtime Zustand RAM Store Read (0ms Instant)
	const applications = useAppStore((state) => state.applications);
	const overviewServices = useAppStore((state) => state.overviewServices);

	const [localStatusOverride, setLocalStatusOverride] = useState<string | null>(null);

	// Pure Zustand Store Application Resolution (React Compiler Auto-Memoized)
	const storeApp = applications.find((a) => String(a.id) === String(appId));
	const serviceApp = overviewServices.find(
		(s) => String(s.id) === String(appId) && (s.type === 'application' || s.kind === 'application' || !s.type)
	);
	const rawApp = storeApp || (serviceApp ? {
		id: serviceApp.id,
		name: serviceApp.name,
		app_name: serviceApp.name,
		project_id: serviceApp.project_id,
		status: serviceApp.status,
		created_at: serviceApp.created_at,
	} as any : undefined);

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

	// 2. Domains Query
	const {data: rawDomains, loading: isLoadingDomains} = useDomainListByApplication(BigInt(appId));

	// 3. Schedules Query
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByApplication(BigInt(appId));

	// Read backups and deployments directly from Zustand RAM store
	const storeBackups = useAppStore((state) => state.backups || []);
	const storeDeployments = useAppStore((state) => state.deployments || []);

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		return storeBackups.filter((b: any) => Number(b.application_id) === Number(appId));
	}, [storeBackups, appId]);
	const deployments = useMemo(() => {
		return storeDeployments.filter((d: any) => Number(d.application_id) === Number(appId));
	}, [storeDeployments, appId]);
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
