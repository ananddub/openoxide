import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import type {ComposeResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useComposeGet,
	useDomainListByCompose,
	useScheduleListByCompose,
	useBackupListVolumeBackups,
	useDeploymentList,
} from 'virtual:openoxide-live';

import { useAppStore } from '#/stores/app-store';

export function useComposeDetail(composeId: number) {
	const [activeTab, setActiveTab] = useState<string>('General');

	// 0ms Instant Zustand Store Read
	const storeCompose = useAppStore((state) =>
		state.composes.find((c) => String(c.id) === String(composeId))
	);

	// 1. Central Compose Query — live push replaces refetchInterval
	const {data: liveCompose} = useComposeGet(BigInt(composeId));

	const compose = liveCompose || storeCompose;
	const isLoadingCompose = !compose;

	// 2. Central Domains Query
	const {data: rawDomains, loading: isLoadingDomains} = useDomainListByCompose(BigInt(composeId));

	// 3. Central Schedules Query
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByCompose(BigInt(composeId));

	// 4. Central Backups Query — filter locally by compose_id
	const {data: rawBackupsAll, loading: isLoadingBackups} = useBackupListVolumeBackups();

	// 5. Central Deployments Query
	const {data: rawDeployments, loading: isLoadingDeployments} = useDeploymentList({
		status: null,
		state: null,
		application_id: null,
		compose_id: BigInt(composeId),
		database_id: null,
		server_id: null,
		limit: null,
		offset: null,
	});

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackupsAll) ? rawBackupsAll : [];
		return all.filter((b: any) => b.compose_id === composeId);
	}, [rawBackupsAll, composeId]);
	const deployments = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(composeId, 'compose');

	// Live hooks auto-push updates — only trigger monitoring refresh
	const refetchAll = () => {
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
			// Live hooks auto-push updated data — no manual refetch needed
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
		refetch: () => {},
		refetchDomains: () => {},
		refetchSchedules: () => {},
		refetchBackups: () => {},
		refetchDeployments: () => {},
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	};
}
