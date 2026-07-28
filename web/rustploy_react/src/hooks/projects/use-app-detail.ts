import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ApplicationResponse} from '#/types/api-helpers';

export function useAppDetail(appId: number) {
	const queryClient = useQueryClient();
	const [activeTab, setActiveTab] = useState('General');

	const appQueryKey = ['get', '/applications/{id}', {params: {path: {id: appId}}}] as const;

	const {
		data: app,
		isLoading,
		refetch,
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

			await refetch();
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
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return {
		app: (app as ApplicationResponse | undefined) || null,
		isLoading,
		refetch,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
		deleteMutation,
	};
}
