import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useRegistryList} from 'virtual:openoxide-live';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {RegistryHeader} from '#/components/registry/registry-header';
import {RegistriesList} from '#/components/registry/registries-list';
import {CreateRegistryModal} from '#/components/registry/create-registry-modal';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';

import type {RegistryResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/registry')({
	component: RegistryPage,
});

function RegistryPage() {
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [editingRegistry, setEditingRegistry] =
		useState<RegistryResponse | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const {data: rawRegistries, loading: isLoading} = useRegistryList();

	const registries = Array.isArray(rawRegistries ?? [])
		? ((rawRegistries ?? []) as unknown as RegistryResponse[])
		: [];
	const deleteMutation = $api.useMutation('delete', '/registries/{id}');

	const handleOpenAdd = () => {
		setEditingRegistry(null);
		setIsModalOpen(true);
	};

	const handleOpenEdit = (item: RegistryResponse) => {
		setEditingRegistry(item);
		setIsModalOpen(true);
	};

	const confirmDelete = async () => {
		if (!deletingId) return;
		try {
			await deleteMutation.mutateAsync({params: {path: {id: deletingId}}});
			toast.success('Registry deleted successfully');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-7xl flex-col gap-3 p-4">
			<RegistryHeader
				onAddRegistry={handleOpenAdd}
				onRefresh={() => {}}
				isRefreshing={isLoading}
			/>

			<RegistriesList
				registries={Array.isArray(registries) ? registries : []}
				isLoading={isLoading}
				onEdit={handleOpenEdit}
				onDelete={id => setDeletingId(id)}
				onRefresh={() => {}}
			/>

			<CreateRegistryModal
				isOpen={isModalOpen}
				onClose={() => {
					setIsModalOpen(false);
					setEditingRegistry(null);
				}}
				onSuccess={() => {}}
				initialData={editingRegistry}
			/>

			<AlertDialog
				open={deletingId !== null}
				onOpenChange={open => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Container Registry</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this container registry? This
							action cannot be undone.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>
							Cancel
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
							onClick={confirmDelete}>
							Delete
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
