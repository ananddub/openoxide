import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {DestinationsHeader} from '#/components/destinations/destinations-header';
import {DestinationsList} from '#/components/destinations/destinations-list';
import {CreateDestinationModal} from '#/components/destinations/create-destination-modal';

import {useDestinationList, useDestinationDelete, useDestinationTest} from '#/api/live';

import type {DestinationResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/destinations')({
	component: DestinationsPage,
});

function DestinationsPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingDestination, setEditingDestination] =
		useState<DestinationResponse | null>(null);

	// Live destinations query; the table-watch stream keeps it fresh.
	const destinationsQuery = useDestinationList();
	const destinations = (destinationsQuery.data ??
		[]) as unknown as DestinationResponse[];

	// Mutations
	const deleteMutation = useDestinationDelete();
	const testMutation = useDestinationTest();

	const handleDelete = async (id: string | number) => {
		try {
			await deleteMutation.mutateAsync(String(id));
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
					provider: destination.provider,
					bucket: destination.bucket,
					region: destination.region,
					endpoint: destination.endpoint,
					access_key: destination.access_key,
					secret_access_key: destination.secret_access_key,
				}),
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
				isRefreshing={destinationsQuery.isLoading}
			/>

			{/* Destinations Grid List Component (< 200 lines) */}
			<DestinationsList
				destinations={destinations}
				isLoading={destinationsQuery.isLoading}
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
