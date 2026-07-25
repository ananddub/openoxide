import {useState} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

export function useComposeDetail(composeId: number) {
	const [activeTab, setActiveTab] = useState<string>('General');

	const {
		data: compose,
		isLoading,
		refetch,
	} = $api.useQuery('get', '/compose/{id}', {
		params: {path: {id: composeId}},
	}, {
		refetchInterval: 3000,
	});

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
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {
		compose,
		isLoading,
		refetch,
		activeTab,
		setActiveTab,
		handleAction,
	};
}
