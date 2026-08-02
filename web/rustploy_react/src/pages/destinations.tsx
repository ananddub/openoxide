import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DestinationsHeader} from '#/components/destinations/destinations-header';
import {DestinationsList} from '#/components/destinations/destinations-list';
import {CreateDestinationModal} from '#/components/destinations/create-destination-modal';

import type {DestinationResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/destinations')({
	component: DestinationsPage,
});

function DestinationsPage() {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingDestination, setEditingDestination] = useState<DestinationResponse | null>(null);

	// Query destinations with safe array fallback
	const {data: rawDestinations = [], isLoading, refetch, isRefetching} = $api.useQuery('get', '/destinations');
	const destinations = Array.isArray(rawDestinations) ? rawDestinations : [];

	// Mutations
	const deleteMutation = $api.useMutation('delete', '/destinations/{id}');
	const testMutation = $api.useMutation('post', '/destinations/{id}/test');

	const handleDelete = async (id: string | number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id: String(id)}}});
			toast.success('S3 Storage Destination deleted successfully');
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleTest = async (id: string | number) => {
		try {
			await testMutation.mutateAsync({params: {path: {id: String(id)}}, parseAs: 'text'} as any);
			toast.success('S3 Storage Destination connection test passed!');
		} catch (err: unknown) {
			const msg = String((err as any)?.message || err || '');
			if (msg.toLowerCase().includes('json') || msg.toLowerCase().includes('unexpected end')) {
				toast.success('S3 Storage Destination connection test passed!');
				return;
			}
			toast.error(formatApiError(err));
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
		<div className="flex flex-col gap-6 w-full max-w-5xl mx-auto pb-12">
			{/* Header Component (< 200 lines) */}
			<DestinationsHeader
				totalCount={destinations.length}
				onAdd={() => {
					setEditingDestination(null);
					setIsCreateOpen(true);
				}}
				onRefresh={refetch}
				isRefreshing={isLoading || isRefetching}
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
				onSuccess={refetch}
				editingDestination={editingDestination}
			/>
		</div>
	);
}
