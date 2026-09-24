import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useRemoteServerDelete} from '#/api/live';

export function useDeleteServer(serverId: number, onClose: () => void) {
	const queryClient = useQueryClient();
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = useRemoteServerDelete();

	const handleDelete = async () => {
		if (!serverId) return;
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync(serverId);
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
