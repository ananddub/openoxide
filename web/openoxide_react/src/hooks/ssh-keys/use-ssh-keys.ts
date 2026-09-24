import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useSshKeyDelete} from '#/api/live';

export function useDeleteSshKey(keyId: number, onClose: () => void) {
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = useSshKeyDelete();

	const handleDelete = async () => {
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync(keyId);
			toast.success('SSH key deleted');
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return {deleting, handleDelete};
}
