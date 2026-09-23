import {Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Skeleton} from '#/components/ui/skeleton';
import {ListCard, ListEmpty} from '#/components/settings/list-card';
import {VaultProviderCard, VaultProviderRow} from './vault-provider-item';
import type {VaultProviderItem} from './vault-provider-catalog';

export type VaultProvidersView = 'grid' | 'list';

interface VaultProvidersListProps {
	/** Already filtered by the page toolbar. */
	providers: VaultProviderItem[];
	isLoading: boolean;
	hasFilters: boolean;
	view: VaultProvidersView;
	copiedId: number | null;
	testingId: number | null;
	onClearFilters: () => void;
	onSecrets: (provider: VaultProviderItem) => void;
	onTest: (provider: VaultProviderItem) => void;
	onCopy: (provider: VaultProviderItem) => void;
	onEdit: (provider: VaultProviderItem) => void;
	onDelete: (provider: VaultProviderItem) => void;
}

/** Providers surface — Projects grammar: grid cards or one bordered list card. */
export function VaultProvidersList({
	providers,
	isLoading,
	hasFilters,
	view,
	copiedId,
	testingId,
	onClearFilters,
	onSecrets,
	onTest,
	onCopy,
	onEdit,
	onDelete,
}: VaultProvidersListProps) {
	if (isLoading) {
		if (view === 'grid') {
			return (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{[1, 2, 3, 4].map(i => (
						<div
							key={i}
							className="min-h-[124px] animate-pulse rounded-lg border bg-card"
						/>
					))}
				</div>
			);
		}
		return (
			<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-3.5 px-4 py-3">
						<Skeleton className="size-9 shrink-0 rounded-lg" />
						<div className="flex-1 space-y-2">
							<Skeleton className="h-3.5 w-40" />
							<Skeleton className="h-3 w-64" />
						</div>
						<Skeleton className="h-5 w-16 rounded-full" />
					</div>
				))}
			</div>
		);
	}

	if (providers.length === 0) {
		return (
			<ListEmpty
				icon={Search}
				title="No providers match your filters"
				description="Try a different name, endpoint or provider type."
				action={
					hasFilters && (
						<Button
							variant="ghost"
							onClick={onClearFilters}
							className="text-xs font-semibold text-primary">
							Clear All Filters
						</Button>
					)
				}
			/>
		);
	}

	const shared = (provider: VaultProviderItem) => ({
		provider,
		copied: copiedId === provider.id,
		testing: testingId === provider.id,
		onSecrets: () => onSecrets(provider),
		onTest: () => onTest(provider),
		onCopy: () => onCopy(provider),
		onEdit: () => onEdit(provider),
		onDelete: () => onDelete(provider),
	});

	if (view === 'grid') {
		return (
			<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{providers.map(provider => (
					<VaultProviderCard key={provider.id} {...shared(provider)} />
				))}
			</div>
		);
	}

	return (
		<ListCard>
			{providers.map(provider => (
				<VaultProviderRow key={provider.id} {...shared(provider)} />
			))}
		</ListCard>
	);
}
