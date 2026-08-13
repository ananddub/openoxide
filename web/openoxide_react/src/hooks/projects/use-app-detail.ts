import {useState, useMemo} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useApplicationGet,
	useDomainListByApplication,
	useScheduleListByApplication,
	useBackupListVolumeBackups,
	useDeploymentList,
} from 'virtual:openoxide-live';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');

	// 1. App Query — live push replaces refetchInterval
	const {data: app, loading: isLoadingApp} = useApplicationGet(BigInt(appId));

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
				toast.success('Reload triggered successfully');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Start triggered successfully');
			} else if (action === 'stop' || action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success(action === 'stop' ? 'Application stopped successfully' : 'Cancellation triggered successfully');
			}
			// Live hooks auto-push updated data — no manual refetch needed
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleUpdate = async (body: Record<string, unknown>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: appId}},
				body,
			});
			toast.success('Application updated successfully');
			// Live hooks auto-push updated data — no manual refetch needed
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return {
		app: (app as ApplicationResponse | undefined) || null,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: isLoadingApp,
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
		handleUpdate,
		deleteMutation,
	};
}
