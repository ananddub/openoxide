import {useState, useMemo} from 'react';
import {
	HardDrive,
	Trash2,
	Pencil,
	RefreshCw,
	Check,
	X,
	Plug,
	MoreVertical,
	Search,
} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';

import type {DestinationResponse} from '#/types/api-helpers';

interface DestinationsListProps {
	destinations: DestinationResponse[];
	isLoading: boolean;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (id: string | number) => void;
	onTest: (id: string | number) => Promise<void>;
}

const PROVIDER_LABELS: Record<string, string> = {
	aws: 'AWS S3',
	s3: 'AWS S3',
	r2: 'Cloudflare R2',
	cloudflare: 'Cloudflare R2',
	minio: 'MinIO',
	backblaze: 'Backblaze B2',
};

function providerLabel(provider?: string): string {
	if (!provider) return 'S3 Compatible';
	return PROVIDER_LABELS[provider.toLowerCase()] ?? provider.toUpperCase();
}

const PROVIDER_FILTERS = [
	'All',
	'AWS S3',
	'Cloudflare R2',
	'MinIO',
	'S3 Compatible',
] as const;
type ProviderFilter = (typeof PROVIDER_FILTERS)[number];

export function DestinationsList({
	destinations,
	isLoading,
	onEdit,
	onDelete,
	onTest,
}: DestinationsListProps) {
	const [testStatusMap, setTestStatusMap] = useState<
		Record<string, 'testing' | 'success' | 'failed' | undefined>
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
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div
						key={i}
						className="flex items-center gap-4 rounded-lg border border-border px-4 py-4">
						<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
						<div className="flex-1 space-y-2">
							<Skeleton className="h-3.5 w-40" />
							<Skeleton className="h-3 w-56" />
						</div>
						<Skeleton className="h-5 w-20 rounded-full" />
					</div>
				))}
			</div>
		);
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
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-border py-20 text-center">
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
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-border py-14 text-center">
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
				<div className="divide-y divide-border overflow-hidden rounded-lg border border-border">
					{filtered.map((d: DestinationResponse) => {
						const status = testStatusMap[String(d.id)];
						const isTesting = status === 'testing';
						const label = providerLabel(d.provider);

						const dotCls = isTesting
							? 'bg-amber-400 animate-pulse'
							: status === 'success'
								? 'bg-emerald-500'
								: status === 'failed'
									? 'bg-rose-500'
									: 'bg-zinc-500/60';

						return (
							<div
								key={d.id}
								className="group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30">
								{/* Icon + test status dot */}
								<div className="relative shrink-0">
									<div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
										<HardDrive className="h-4 w-4 text-foreground/70" />
									</div>
									<span
										className={`absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background ${dotCls}`}
									/>
								</div>

								{/* Info */}
								<div className="min-w-0 flex-1">
									<div className="mb-0.5 flex items-center gap-2">
										<span className="truncate text-sm font-medium text-foreground">
											{d.name}
										</span>
										<Badge
											variant="secondary"
											className="shrink-0 py-0 font-mono text-[10px]">
											{label}
										</Badge>
									</div>
									<p className="truncate font-mono text-[11px] text-muted-foreground">
										{d.bucket}
										{d.region ? ` · ${d.region}` : ''}
										{d.endpoint ? ` · ${d.endpoint}` : ''}
									</p>
								</div>

								{/* Hover: test button */}
								<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
									<button
										onClick={() => handleTest(d.id)}
										disabled={isTesting}
										title={isTesting ? 'Testing…' : 'Test connection'}
										className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
										{isTesting ? (
											<RefreshCw className="h-4 w-4 animate-spin text-amber-500" />
										) : status === 'success' ? (
											<Check className="h-4 w-4 text-emerald-500" />
										) : status === 'failed' ? (
											<X className="h-4 w-4 text-rose-500" />
										) : (
											<Plug className="h-4 w-4" />
										)}
									</button>
								</div>

								<Separator
									orientation="vertical"
									className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
								/>

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
										}>
										<MoreVertical className="h-4 w-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-40">
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => handleTest(d.id)}>
											<Plug className="h-3.5 w-3.5" /> Test Connection
										</DropdownMenuItem>
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => onEdit(d)}>
											<Pencil className="h-3.5 w-3.5" /> Edit
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="cursor-pointer gap-2 text-destructive focus:text-destructive"
											onClick={() => handleDelete(d.id)}
											disabled={deletingId === d.id}>
											<Trash2 className="h-3.5 w-3.5" />
											{deletingId === d.id ? 'Deleting…' : 'Delete'}
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						);
					})}
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
