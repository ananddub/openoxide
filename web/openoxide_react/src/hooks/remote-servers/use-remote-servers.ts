import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

export function useDeleteServer(serverId: number, onClose: () => void) {
	const queryClient = useQueryClient();
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = $api.useMutation(
		'delete',
		'/remote-servers/{id}',
	);

	const handleDelete = async () => {
		if (!serverId) return;
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync({
				params: {path: {id: serverId}},
			});
			toast.success('Remote server deleted successfully');
			queryClient.invalidateQueries({
				queryKey: ['get', '/remote-servers'],
			});
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return {deleting, handleDelete};
}

export function useTestServerConnection(serverId?: number) {
	const [testing, setTesting] = useState(false);
	const testConnMutation = $api.useMutation(
		'post',
		'/servers/{id}/test-connection',
	);

	const handleTestConnection = async (targetId?: number) => {
		const idToTest = targetId || serverId;
		if (!idToTest) return;

		setTesting(true);
		try {
			const res = await testConnMutation.mutateAsync({
				params: {path: {id: idToTest}},
				body: {host_key_fingerprint: ''},
			});
			const data = res as Record<string, unknown>;
			if (data.status === 'SUCCESS' || data.success) {
				toast.success('Server connection test successful!');
			} else {
				toast.error(String(data.message || 'Server connection failed'));
			}
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setTesting(false);
		}
	};

	return {testing, handleTestConnection};
}
