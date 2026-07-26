import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {RegistryHeader} from '#/components/registry/registry-header';
import {RegistriesList} from '#/components/registry/registries-list';
import {CreateRegistryModal} from '#/components/registry/create-registry-modal';

export const Route = createFileRoute('/_app/registry')({
	component: RegistrySettingsPage,
});

function RegistrySettingsPage() {
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [editingRegistry, setEditingRegistry] = useState<any | null>(null);

	const {
		data: registries = [],
		isLoading,
		refetch,
		isRefetching,
	} = $api.useQuery('get', '/registries');

	const deleteMutation = $api.useMutation('delete', '/registries/{id}');

	const handleOpenAdd = () => {
		setEditingRegistry(null);
		setIsModalOpen(true);
	};

	const handleOpenEdit = (item: any) => {
		setEditingRegistry(item);
		setIsModalOpen(true);
	};

	const handleDelete = async (id: number) => {
		if (!confirm('Are you sure you want to delete this container registry?')) return;
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			toast.success('Registry deleted successfully');
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-3 p-4 max-w-7xl mx-auto w-full">
			<RegistryHeader
				onAddRegistry={handleOpenAdd}
				onRefresh={refetch}
				isRefreshing={isLoading || isRefetching}
			/>

			<RegistriesList
				registries={Array.isArray(registries) ? registries : []}
				isLoading={isLoading}
				onEdit={handleOpenEdit}
				onDelete={handleDelete}
				onRefresh={refetch}
			/>

			<CreateRegistryModal
				isOpen={isModalOpen}
				initialData={editingRegistry}
				onClose={() => setIsModalOpen(false)}
				onSuccess={refetch}
			/>
		</div>
	);
}
