import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');
	const queryClient = useQueryClient();

	const appQueryKey = ['get', '/applications/{id}', {params: {path: {id: appId}}}];

	// Queries
	const {data: app, isLoading, refetch} = $api.useQuery(
		'get',
		'/applications/{id}',
		{
			params: {
				path: {id: appId},
			},
		},
		{
			refetchInterval: 5000,
		}
	);

	// Action Mutations
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
			let res: any;
			if (action === 'deploy') {
				res = await deployMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Deployment triggered successfully');
			} else if (action === 'reload') {
				res = await reloadMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Reload triggered successfully');
			} else if (action === 'rebuild') {
				res = await rebuildMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				res = await startMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Start triggered successfully');
			} else if (action === 'stop' || action === 'cancel') {
				res = await cancelMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success(action === 'stop' ? 'Application stopped successfully' : 'Cancellation triggered successfully');
			}

			const updatedApp = res?.data?.application || res?.application;
			if (updatedApp) {
				queryClient.setQueryData(appQueryKey, updatedApp);
			}

			await refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleUpdate = async (body: any) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: appId}},
				body,
			});
			toast.success('Application updated successfully');
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {
		app,
		isLoading,
		refetch,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
		deleteMutation,
	};
}
