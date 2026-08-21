import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DestinationsHeader} from '#/components/destinations/destinations-header';
import {DestinationsList} from '#/components/destinations/destinations-list';
import {CreateDestinationModal} from '#/components/destinations/create-destination-modal';

import {useAppStore} from '#/stores/app-store';

import type {DestinationResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/destinations')({
	component: DestinationsPage,
});

function DestinationsPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingDestination, setEditingDestination] =
		useState<DestinationResponse | null>(null);

	const storeDestinations = useAppStore(state => state.destinations);

	const destinations = (storeDestinations ??
		[]) as unknown as DestinationResponse[];
	const isLoading = false;

	// Mutations
	const deleteMutation = $api.useMutation('delete', '/destinations/{id}');
	const testMutation = $api.useMutation('post', '/destinations/test');

	const handleDelete = async (id: string | number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id: String(id)}}});
			toast.success('S3 Storage Destination deleted successfully');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleTest = async (id: string | number) => {
		const destination = destinations.find(item => String(item.id) === String(id));
		if (!destination) {
			toast.error('S3 destination not found');
			return;
		}
		try {
			await Promise.race([
				testMutation.mutateAsync({
				body: {
					provider: destination.provider,
					bucket: destination.bucket,
					region: destination.region,
					endpoint: destination.endpoint,
					access_key: destination.access_key,
					secret_access_key: destination.secret_access_key,
				},
				parseAs: 'text',
				} as any),
				new Promise<never>((_, reject) =>
					setTimeout(() => reject(new Error('S3 connection test timed out')), 15_000),
				),
			]);
			toast.success('S3 Storage Destination connection test passed!');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
			throw err;
		}
	};

	const handleEdit = (item: DestinationResponse) => {
		setEditingDestination(item);
		setIsCreateOpen(true);
	};

	const handleModalClose = () => {
		setIsCreateOpen(false);
		setEditingDestination(null);
	};

	return (
		<div className="mx-auto flex w-full max-w-5xl flex-col gap-6 pb-12">
			{/* Header Component (< 200 lines) */}
			<DestinationsHeader
				totalCount={destinations.length}
				onAdd={() => {
					setEditingDestination(null);
					setIsCreateOpen(true);
				}}
				onRefresh={() => {}}
				isRefreshing={isLoading}
			/>

			{/* Destinations Grid List Component (< 200 lines) */}
			<DestinationsList
				destinations={destinations}
				isLoading={isLoading}
				onEdit={handleEdit}
				onDelete={handleDelete}
				onTest={handleTest}
			/>

			{/* Create/Edit Modal Component (< 200 lines) */}
			<CreateDestinationModal
				isOpen={isCreateOpen}
				onClose={handleModalClose}
				onSuccess={() => {}}
				editingDestination={editingDestination}
			/>
		</div>
	);
}
