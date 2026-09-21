import {useMemo, useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {Database, Globe, Lock} from 'lucide-react';
import {useRegistryDelete, useRegistryList} from '#/api/live';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {RegistryHeader} from '#/components/registry/registry-header';
import {RegistriesList} from '#/components/registry/registries-list';
import {CreateRegistryModal} from '#/components/registry/create-registry-modal';
import {
	ResourceGuideCard,
	ResourceInfoCard,
} from '#/components/shared/resource-sidebar';
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
	const navigate = useNavigate();
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingRegistry, setEditingRegistry] =
		useState<RegistryResponse | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const {data: rawRegistries, isLoading, refetch} = useRegistryList();

	const registries = Array.isArray(rawRegistries ?? [])
		? ((rawRegistries ?? []) as unknown as RegistryResponse[])
		: [];
	const deleteMutation = useRegistryDelete();

	const stats = useMemo(() => {
		const hosts = new Set(
			registries
				.map(r => String(r.registry_url || '').split('/')[0])
				.filter(Boolean),
		);
		return {total: registries.length, hosts: hosts.size};
	}, [registries]);

	const handleOpenAdd = () => void navigate({to: '/registry/new'});

	const handleOpenEdit = (item: RegistryResponse) => {
		setEditingRegistry(item);
		setIsEditOpen(true);
	};

	const confirmDelete = async () => {
		if (deletingId === null) return;
		try {
			await deleteMutation.mutateAsync(deletingId);
			toast.success('Registry deleted');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<RegistryHeader
				onAddRegistry={handleOpenAdd}
				onRefresh={() => void refetch()}
				isRefreshing={isLoading}
			/>

			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<RegistriesList
					registries={registries}
					isLoading={isLoading}
					onEdit={handleOpenEdit}
					onDelete={id => setDeletingId(id)}
				/>

				{/* ── Right rail ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<ResourceInfoCard
						title="Overview"
						icon={Database}
						rows={[
							{icon: Database, label: 'Registries', value: stats.total},
							{icon: Globe, label: 'Distinct hosts', value: stats.hosts},
						]}
					/>
					<ResourceGuideCard
						title="Registry basics"
						icon={Lock}
						steps={[
							{
								title: 'Hostname only',
								text: 'Store just the registry host (docker.io, ghcr.io, ECR URL) — no path, no scheme.',
							},
							{
								title: 'Least-privilege token',
								text: 'Use a read/write token scoped to the namespaces you push to, not an account password.',
							},
							{
								title: 'Auth host',
								text: 'Choose which server authenticates — pick a remote one when only it can reach a private registry.',
							},
						]}
					/>
				</div>
			</div>

			{/* Edit Modal (create lives on /registry/new) */}
			<CreateRegistryModal
				isOpen={isEditOpen}
				onClose={() => {
					setIsEditOpen(false);
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
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={confirmDelete}>
							Delete
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
