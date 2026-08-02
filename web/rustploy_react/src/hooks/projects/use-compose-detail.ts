import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import type {ComposeResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';

export function useComposeDetail(composeId: number) {
	const [activeTab, setActiveTab] = useState<string>('General');

	// 1. Central Compose Query
	const {
		data: compose,
		isLoading: isLoadingCompose,
		refetch: refetchCompose,
	} = $api.useQuery(
		'get',
		'/compose/{id}',
		{
			params: {path: {id: composeId}},
		},
		{
			refetchInterval: 3000,
		}
	);

	// 2. Central Domains Query
	const {
		data: rawDomains = [],
		isLoading: isLoadingDomains,
		refetch: refetchDomains,
	} = $api.useQuery(
		'get',
		'/domains/compose/{compose_id}',
		{
			params: {path: {compose_id: composeId}},
			enabled: !!composeId,
		} as any
	);

	// 3. Central Schedules Query
	const {
		data: rawSchedules = [],
		isLoading: isLoadingSchedules,
		refetch: refetchSchedules,
	} = $api.useQuery(
		'get',
		'/schedules/compose/{compose_id}',
		{
			params: {path: {compose_id: composeId}},
			enabled: !!composeId,
		} as any
	);

	// 4. Central Backups Query
	const {
		data: rawBackups = [],
		isLoading: isLoadingBackups,
		refetch: refetchBackups,
	} = $api.useQuery(
		'get',
		'/backups/volume',
		{
			params: {query: {compose_id: composeId}},
			enabled: !!composeId,
		} as any
	);

	// 5. Central Deployments Query
	const {
		data: rawDeployments = [],
		isLoading: isLoadingDeployments,
		refetch: refetchDeployments,
	} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {query: {compose_id: composeId}},
			enabled: !!composeId,
		} as any
	);

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackups) ? rawBackups : [];
		return all.filter((b: any) => b.compose_id === composeId);
	}, [rawBackups, composeId]);
	const deployments = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(composeId, 'compose');

	const refetchAll = () => {
		refetchCompose();
		refetchDomains();
		refetchSchedules();
		refetchBackups();
		refetchDeployments();
		monitoring.triggerRefresh();
	};

	// Actions & Mutations
	const deployMutation = $api.useMutation('post', '/compose/{id}/deploy');
	const redeployMutation = $api.useMutation('post', '/compose/{id}/redeploy');
	const startMutation = $api.useMutation('post', '/compose/{id}/start');
	const stopMutation = $api.useMutation('post', '/compose/{id}/stop');
	const reloadMutation = $api.useMutation('post', '/compose/{id}/reload');
	const cancelMutation = $api.useMutation('post', '/compose/{id}/cancel');

	const handleAction = async (
		action: 'deploy' | 'redeploy' | 'rebuild' | 'start' | 'stop' | 'reload' | 'cancel',
	) => {
		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose deployment started');
			} else if (action === 'redeploy' || action === 'rebuild') {
				await redeployMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose redeploy triggered');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose services starting');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose services stopping');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose configuration reloaded');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: composeId}}});
				toast.success('Compose action cancelled');
			}
			refetchAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return {
		compose: (compose as ComposeResponse | undefined) || null,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: isLoadingCompose,
		isLoadingDomains,
		isLoadingSchedules,
		isLoadingBackups,
		isLoadingDeployments,
		refetch: refetchCompose,
		refetchDomains,
		refetchSchedules,
		refetchBackups,
		refetchDeployments,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	};
}
