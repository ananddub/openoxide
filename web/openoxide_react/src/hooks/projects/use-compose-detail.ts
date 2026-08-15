import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
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

	// 0ms Instant Zustand Store Read with fallback to overviewServices
	const composes = useAppStore((state) => state.composes);
	const overviewServices = useAppStore((state) => state.overviewServices);

	const storeCompose = useMemo(() => {
		const direct = composes.find((c) => String(c.id) === String(composeId));
		if (direct) return direct;
		const service = overviewServices.find(
			(s) => String(s.id) === String(composeId) && (s.type === 'compose' || s.kind === 'compose')
		);
		if (service) {
			return {
				id: service.id,
				name: service.name,
				app_name: service.name,
				project_id: service.project_id,
				status: service.status,
				created_at: service.created_at,
			} as any;
		}
		return undefined;
	}, [composes, overviewServices, composeId]);

	// 1. Central Compose Query — live push replaces refetchInterval
	const {data: liveCompose} = useComposeGet(BigInt(composeId));

	const compose = liveCompose || storeCompose;

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

	const deployMutation = $api.useMutation('post', '/composes/{id}/deploy');
	const reloadMutation = $api.useMutation('post', '/composes/{id}/reload');
	const startMutation = $api.useMutation('post', '/composes/{id}/start');
	const stopMutation = $api.useMutation('post', '/composes/{id}/stop');
	const patchMutation = $api.useMutation('patch', '/composes/{id}');

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop') => {
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
