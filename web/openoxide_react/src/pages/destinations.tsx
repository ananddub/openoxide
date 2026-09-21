import {useMemo, useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {
	HardDrive as BucketIcon,
	Cloud,
	KeyRound,
} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {DestinationsHeader} from '#/components/destinations/destinations-header';
import {DestinationsList} from '#/components/destinations/destinations-list';
import {CreateDestinationModal} from '#/components/destinations/create-destination-modal';
import {
	ResourceGuideCard,
	ResourceInfoCard,
} from '#/components/shared/resource-sidebar';

import {
	useDestinationList,
	useDestinationDelete,
	useDestinationTest,
} from '#/api/live';

import type {DestinationResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/destinations')({
	component: DestinationsPage,
});

function DestinationsPage() {
	const navigate = useNavigate();
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingDestination, setEditingDestination] =
		useState<DestinationResponse | null>(null);

	// Live destinations query; the table-watch stream keeps it fresh.
	const destinationsQuery = useDestinationList();
	const destinations = (destinationsQuery.data ??
		[]) as unknown as DestinationResponse[];

	// Mutations
	const deleteMutation = useDestinationDelete();
	const testMutation = useDestinationTest();

	const providerCount = useMemo(() => {
		const counts: Record<string, number> = {};
		for (const item of destinations) {
			const key = String(item.provider || 'custom');
			counts[key] = (counts[key] ?? 0) + 1;
		}
		const top = Object.entries(counts).sort((a, b) => b[1] - a[1])[0];
		return top ? `${top[0]} (${top[1]})` : '—';
	}, [destinations]);

	const handleDelete = async (id: string | number) => {
		try {
			await deleteMutation.mutateAsync(String(id));
			toast.success('S3 Storage Destination deleted');
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
		setIsEditOpen(true);
	};

	const handleModalClose = () => {
		setIsEditOpen(false);
		setEditingDestination(null);
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<DestinationsHeader
				totalCount={destinations.length}
				onAdd={() => void navigate({to: '/destinations/new'})}
				onRefresh={() => {}}
				isRefreshing={destinationsQuery.isLoading}
			/>

			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<DestinationsList
					destinations={destinations}
					isLoading={destinationsQuery.isLoading}
					onEdit={handleEdit}
					onDelete={handleDelete}
					onTest={handleTest}
				/>

				{/* ── Right rail ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<ResourceInfoCard
						title="Overview"
						icon={BucketIcon}
						rows={[
							{icon: BucketIcon, label: 'Destinations', value: destinations.length},
							{icon: Cloud, label: 'Top provider', value: providerCount},
						]}
					/>
					<ResourceGuideCard
						title="Backup destinations"
						icon={KeyRound}
						steps={[
							{
								title: 'Any S3-compatible store',
								text: 'AWS, R2, MinIO, Spaces, Wasabi — or a custom endpoint for self-hosted buckets.',
							},
							{
								title: 'Scoped keys',
								text: 'Give the access key only bucket read/write — backups never need account-wide rights.',
							},
							{
								title: 'Test before trusting',
								text: 'The per-card test button verifies credentials and reachability without uploading data.',
							},
						]}
					/>
				</div>
			</div>

			{/* Edit Modal (create lives on /destinations/new) */}
			<CreateDestinationModal
				isOpen={isEditOpen}
				onClose={handleModalClose}
				onSuccess={() => {}}
				editingDestination={editingDestination}
			/>
		</div>
	);
}
