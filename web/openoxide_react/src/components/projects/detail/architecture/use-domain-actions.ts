import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {
	useDomainCreate,
	useDomainDelete,
	useDomainUpdate,
} from '#/api/live';
import type {DomainFormData} from '#/components/projects/common/domain-dialog';
import type {ArchitectureOwner} from './types';

export function useDomainActions(
	owner: ArchitectureOwner,
	onRefresh?: () => void,
) {
	const queryClient = useQueryClient();
	const create = useDomainCreate();
	const patch = useDomainUpdate();
	const removeMutation = useDomainDelete();
	const [editing, setEditing] = useState<any | null>(null);

	const scopeField =
		owner.kind === 'compose' ? 'compose_id' : 'application_id';

	/** Returns true when the domain was saved successfully. */
	const save = async (data: DomainFormData): Promise<boolean> => {
		try {
			if (editing?.id) {
				await patch.mutateAsync({id: editing.id, ...data});
				toast.success('Domain updated successfully');
			} else {
				await create.mutateAsync({...data, [scopeField]: owner.id});
				toast.success(
					owner.kind === 'compose'
						? `Domain attached to ${data.service_name}`
						: 'Domain attached successfully',
				);
			}
			queryClient.invalidateQueries();
			setEditing(null);
			return true;
		} catch (e: any) {
			toast.error(e?.message || 'Failed to save domain');
			return false;
		}
	};

	const remove = async (domainData: any) => {
		if (!domainData?.id) return;
		try {
			await removeMutation.mutateAsync(domainData.id);
			toast.success('Domain removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete domain');
		}
	};

	return {editing, setEditing, save, remove};
}
