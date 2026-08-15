import {useState, useMemo} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useApplicationGet,
	useDomainListByApplication,
	useScheduleListByApplication,
	useBackupListVolumeBackups,
	useDeploymentList,
} from 'virtual:openoxide-live';

import { useAppStore } from '#/stores/app-store';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');

	// 0ms Instant Zustand Store Read with fallback to overviewServices
	const applications = useAppStore((state) => state.applications);
	const overviewServices = useAppStore((state) => state.overviewServices);

	const storeApp = useMemo(() => {
		const direct = applications.find((a) => String(a.id) === String(appId));
		if (direct) return direct;
		const service = overviewServices.find(
			(s) => String(s.id) === String(appId) && (s.type === 'application' || s.kind === 'application' || !s.type)
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
	}, [applications, overviewServices, appId]);

	// 1. App Query — live push replaces refetchInterval
	const {data: liveApp} = useApplicationGet(BigInt(appId));

	const app = liveApp || storeApp;

	// 2. Domains Query
	const {data: rawDomains, loading: isLoadingDomains} = useDomainListByApplication(BigInt(appId));

	// 3. Schedules Query
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByApplication(BigInt(appId));

	// 4. Backups Query — filter locally by application_id
	const {data: rawBackupsAll, loading: isLoadingBackups} = useBackupListVolumeBackups();

	// 5. Deployments Query
	const {data: rawDeployments, loading: isLoadingDeployments} = useDeploymentList({
		status: null,
		state: null,
		application_id: BigInt(appId),
		compose_id: null,
		database_id: null,
		server_id: null,
		limit: null,
		offset: null,
	});

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackupsAll) ? rawBackupsAll : [];
		return all.filter((b: any) => b.application_id === appId);
	}, [rawBackupsAll, appId]);
	const deployments = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

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
	const cancelMutation = $api.useMutation('post', '/applications/{id}/cancel');
	const patchMutation = $api.useMutation('patch', '/applications/{id}');
	const deleteMutation = $api.useMutation('delete', '/applications/{id}');

	const handleAction = async (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel') => {
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
				await patchMutation.mutateAsync({
					params: {path: {id: appId}},
					body: {status: 'STOPPED'},
				});
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
