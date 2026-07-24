import {useState} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');

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

	const handleAction = async (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel') => {
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
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Cancellation triggered successfully');
			}
			refetch();
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
