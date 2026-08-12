import {useState, useMemo} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';

export function useAppDetail(appId: number) {
	const queryClient = useQueryClient();
	const [activeTab, setActiveTab] = useState('General');

	const appQueryKey = ['get', '/applications/{id}', {params: {path: {id: appId}}}] as const;

	// 1. App Query
	const {
		data: app,
		isLoading: isLoadingApp,
		refetch: refetchApp,
	} = $api.useQuery(
		'get',
		'/applications/{id}',
		{params: {path: {id: appId}}},
		{
			retry: false,
			refetchInterval: query => {
				const data = query.state.data as ApplicationResponse | undefined;
				const st = (data?.app_status || '').toUpperCase();
				return ['QUEUED', 'STARTING', 'BUILDING', 'DEPLOYING', 'STOPPING'].includes(st) ? 1500 : false;
			},
		},
	);

	// 2. Domains Query
	const {
		data: rawDomains = [],
		isLoading: isLoadingDomains,
		refetch: refetchDomains,
	} = $api.useQuery(
		'get',
		'/domains/application/{application_id}',
		{
			params: {path: {application_id: appId}},
			enabled: !!appId,
		} as any
	);

	// 3. Schedules Query
	const {
		data: rawSchedules = [],
		isLoading: isLoadingSchedules,
		refetch: refetchSchedules,
	} = $api.useQuery(
		'get',
		'/schedules/application/{application_id}',
		{
			params: {path: {application_id: appId}},
			enabled: !!appId,
		} as any
	);

	// 4. Backups Query
	const {
		data: rawBackups = [],
		isLoading: isLoadingBackups,
		refetch: refetchBackups,
	} = $api.useQuery(
		'get',
		'/backups/volume',
		{
			params: {query: {application_id: appId}},
			enabled: !!appId,
		} as any
	);

	// 5. Deployments Query
	const {
		data: rawDeployments = [],
		isLoading: isLoadingDeployments,
		refetch: refetchDeployments,
	} = $api.useQuery(
		'get',
		'/deployments',
		{
			params: {query: {application_id: appId}},
			enabled: !!appId,
		} as any
	);

	const domains = useMemo(() => (Array.isArray(rawDomains) ? rawDomains : []), [rawDomains]);
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackups) ? rawBackups : [];
		return all.filter((b: any) => b.application_id === appId);
	}, [rawBackups, appId]);
	const deployments = useMemo(() => (Array.isArray(rawDeployments) ? rawDeployments : []), [rawDeployments]);

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(appId, 'application');

	const refetchAll = () => {
		refetchApp();
		refetchDomains();
		refetchSchedules();
		refetchBackups();
		refetchDeployments();
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
			await queryClient.cancelQueries({queryKey: appQueryKey});
			let res: {application?: ApplicationResponse; data?: {application?: ApplicationResponse}} | undefined;
			if (action === 'deploy') {
				res = (await deployMutation.mutateAsync({params: {path: {id: appId}}})) as unknown as {application?: ApplicationResponse; data?: {application?: ApplicationResponse}};
				toast.success('Deployment triggered successfully');
			} else if (action === 'reload') {
				res = (await reloadMutation.mutateAsync({params: {path: {id: appId}}})) as unknown as {application?: ApplicationResponse; data?: {application?: ApplicationResponse}};
				toast.success('Reload triggered successfully');
			} else if (action === 'rebuild') {
				res = (await rebuildMutation.mutateAsync({params: {path: {id: appId}}})) as unknown as {application?: ApplicationResponse; data?: {application?: ApplicationResponse}};
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				res = (await startMutation.mutateAsync({params: {path: {id: appId}}})) as unknown as {application?: ApplicationResponse; data?: {application?: ApplicationResponse}};
				toast.success('Start triggered successfully');
			} else if (action === 'stop' || action === 'cancel') {
				res = (await cancelMutation.mutateAsync({params: {path: {id: appId}}})) as unknown as {application?: ApplicationResponse; data?: {application?: ApplicationResponse}};
				toast.success(action === 'stop' ? 'Application stopped successfully' : 'Cancellation triggered successfully');
			}

			const updatedApp = res?.data?.application || res?.application;
			if (updatedApp) {
				queryClient.setQueryData(appQueryKey, updatedApp);
			}

			refetchAll();
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
			refetchAll();
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
		refetch: refetchApp,
		refetchDomains,
		refetchSchedules,
		refetchBackups,
		refetchDeployments,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
		deleteMutation,
	};
}
