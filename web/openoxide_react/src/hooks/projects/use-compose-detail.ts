import {useState, useMemo, useEffect} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import type {ComposeResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useDomainListByCompose,
	useScheduleListByCompose,
} from 'virtual:openoxide-live';

import { useAppStore, selectComposeById } from '#/stores/app-store';

export function useComposeDetail(composeId: number) {
	const [activeTab, setActiveTab] = useState<string>('General');
	const [localStatusOverride, setLocalStatusOverride] = useState<string | null>(null);

	// 100% Pure Centralized Zustand Store Resolution
	const rawCompose = useAppStore((state) => selectComposeById(state, composeId));

	const compose = rawCompose ? {
		...rawCompose,
		status: localStatusOverride || (rawCompose as any).compose_status || (rawCompose as any).status || 'STOPPED',
		compose_status: localStatusOverride || (rawCompose as any).compose_status || (rawCompose as any).status || 'STOPPED',
	} : null;

	// Auto-clear localStatusOverride when Zustand status updates
	useEffect(() => {
		if (compose && localStatusOverride) {
			const fetchedStatus = (compose.status || compose.compose_status || '').toUpperCase();
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
	}, [compose, localStatusOverride]);

	// 2. Central Domains Query
	const {data: rawDomains, loading: isLoadingDomains} = useDomainListByCompose(BigInt(composeId));

	// 3. Central Schedules Query
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByCompose(BigInt(composeId));

	// Read backups and deployments directly from Zustand RAM store
	const storeBackups = useAppStore((state) => state.backups || []);
	const storeDeployments = useAppStore((state) => state.deployments || []);

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		return storeBackups.filter((b: any) => Number(b.compose_id) === Number(composeId));
	}, [storeBackups, composeId]);
	const deployments = useMemo(() => {
		return storeDeployments.filter((d: any) => Number(d.compose_id) === Number(composeId));
	}, [storeDeployments, composeId]);
	const isLoadingBackups = false;
	const isLoadingDeployments = false;

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(composeId, 'compose');

	// Live hooks auto-push updates — only trigger monitoring refresh
	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const deployMutation = $api.useMutation('post', '/composes/{id}/deploy');
	const reloadMutation = $api.useMutation('post', '/composes/{id}/reload');
	const startMutation = $api.useMutation('post', '/composes/{id}/start');
	const stopMutation = $api.useMutation('post', '/composes/{id}/stop');
	const cancelMutation = $api.useMutation('post', '/composes/{id}/cancel');
	const patchMutation = $api.useMutation('patch', '/composes/{id}');

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel') => {
		const currentSt = (compose?.compose_status || compose?.status || '').toUpperCase();
		const isCurrentlyBuilding = ['QUEUED', 'BUILDING', 'STARTING', 'PREPARING', 'PENDING', 'DEPLOYING'].includes(currentSt);
		const intermediateStatus = (action === 'stop' || action === 'cancel')
			? (isCurrentlyBuilding ? 'CANCELLING' : 'STOPPING')
			: action === 'start' ? 'STARTING'
			: 'DEPLOYING';
		setLocalStatusOverride(intermediateStatus);
		(useAppStore.getState() as any).updateServiceStatus?.(composeId, intermediateStatus, 'compose');

		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose stack deployment triggered');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose stack reloaded');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose stack started');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose stack stopped');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose stack cancellation requested');
			}
			refetchAll();
		} catch (err) {
			toast.error(`Action failed: ${action}`);
		}
	};

	const handleUpdate = async (patchData: Partial<ComposeResponse>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: composeId}},
				body: patchData as any,
			});
			toast.success('Compose settings updated');
			refetchAll();
		} catch (err) {
			toast.error('Failed to update compose settings');
		}
	};

	return {
		compose,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: !compose,
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
