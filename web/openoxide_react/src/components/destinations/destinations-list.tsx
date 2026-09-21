import {useState, useMemo} from 'react';
import {HardDrive, Search, X} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import type {DestinationResponse} from '#/types/api-helpers';
import {DestinationsSkeleton} from '#/components/destinations/destinations-skeleton';
import {
	PROVIDER_FILTERS,
	PROVIDER_LABELS,
	providerLabel,
	type ProviderFilter,
} from '#/components/destinations/destinations-shared';
import {
	DestinationRow,
	type TestStatus,
} from '#/components/destinations/destination-row';

interface DestinationsListProps {
	destinations: DestinationResponse[];
	isLoading: boolean;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (id: string | number) => void;
	onTest: (id: string | number) => Promise<void>;
}

export function DestinationsList({
	destinations,
	isLoading,
	onEdit,
	onDelete,
	onTest,
}: DestinationsListProps) {
	const [testStatusMap, setTestStatusMap] = useState<
		Record<string, TestStatus>
	>({});
	const [deletingId, setDeletingId] = useState<string | number | null>(
		null,
	);
	const [search, setSearch] = useState('');
	const [providerFilter, setProviderFilter] =
		useState<ProviderFilter>('All');

	const handleTest = async (id: string | number) => {
		const key = String(id);
		setTestStatusMap(prev => ({...prev, [key]: 'testing'}));
		try {
			await onTest(id);
			setTestStatusMap(prev => ({...prev, [key]: 'success'}));
		} catch {
			setTestStatusMap(prev => ({...prev, [key]: 'failed'}));
		}
	};

	const handleDelete = async (id: string | number) => {
		setDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setDeletingId(null);
		}
	};

	const filtered = useMemo(() => {
		return destinations.filter(d => {
			const matchName =
				d.name?.toLowerCase().includes(search.toLowerCase()) ||
				d.bucket?.toLowerCase().includes(search.toLowerCase());
			const label = providerLabel(d.provider);
			const matchProvider =
				providerFilter === 'All' ||
				label === providerFilter ||
				(providerFilter === 'S3 Compatible' &&
					!Object.values(PROVIDER_LABELS).includes(label));
			return matchName && matchProvider;
		});
	}, [destinations, search, providerFilter]);

	const hasFilters = search !== '' || providerFilter !== 'All';
	const clearFilters = () => {
		setSearch('');
		setProviderFilter('All');
	};

	/* ── Loading ── */
	if (isLoading && destinations.length === 0) {
		return <DestinationsSkeleton />;
	}

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + filter bar ── */}
			<div className="flex items-center gap-2">
				<div className="relative flex-1">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						placeholder="Search by name or bucket…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-8 pl-8 text-xs"
					/>
					{search && (
						<button
							onClick={() => setSearch('')}
							className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground">
							<X className="h-3 w-3" />
						</button>
					)}
				</div>

				{/* Provider filter dropdown */}
				<Select
					value={providerFilter}
					onValueChange={v => setProviderFilter(v as ProviderFilter)}>
					<SelectTrigger size="sm" className="h-8 w-44 text-xs">
						<SelectValue placeholder="Provider" />
					</SelectTrigger>
					<SelectContent>
						{PROVIDER_FILTERS.map(f => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Providers' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty: no destinations ── */}
			{destinations.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg  py-20 text-center">
					<div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<HardDrive className="h-5 w-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">
							No S3 destinations yet
						</p>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Add an S3-compatible bucket to enable backups.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg  py-14 text-center">
					<Search className="h-5 w-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No destinations match your filter
					</p>
					<Button
						variant="ghost"
						size="sm"
						onClick={clearFilters}
						className="h-7 text-xs">
						Clear filters
					</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border overflow-hidden rounded-lg ">
					{filtered.map((d: DestinationResponse) => (
						<DestinationRow
							key={d.id}
							destination={d}
							status={testStatusMap[String(d.id)]}
							isDeleting={deletingId === d.id}
							onTest={handleTest}
							onEdit={onEdit}
							onDelete={handleDelete}
						/>
					))}
				</div>
			)}

			{hasFilters && filtered.length > 0 && (
				<p className="px-1 text-xs text-muted-foreground">
					Showing {filtered.length} of {destinations.length} destinations
				</p>
			)}
		</div>
	);
}
