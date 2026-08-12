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
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
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

const PROVIDER_FILTERS = ['All', 'AWS S3', 'Cloudflare R2', 'MinIO', 'S3 Compatible'] as const;
type ProviderFilter = (typeof PROVIDER_FILTERS)[number];

export function DestinationsList({destinations, isLoading, onEdit, onDelete, onTest}: DestinationsListProps) {
	const [testStatusMap, setTestStatusMap] = useState<Record<string, 'testing' | 'success' | 'failed' | undefined>>({});
	const [deletingId, setDeletingId] = useState<string | number | null>(null);
	const [search, setSearch] = useState('');
	const [providerFilter, setProviderFilter] = useState<ProviderFilter>('All');

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
		try { await onDelete(id); } finally { setDeletingId(null); }
	};

	const filtered = useMemo(() => {
		return destinations.filter(d => {
			const matchName = d.name?.toLowerCase().includes(search.toLowerCase()) ||
				d.bucket?.toLowerCase().includes(search.toLowerCase());
			const label = providerLabel(d.provider);
			const matchProvider = providerFilter === 'All' || label === providerFilter ||
				(providerFilter === 'S3 Compatible' && !Object.values(PROVIDER_LABELS).includes(label));
			return matchName && matchProvider;
		});
	}, [destinations, search, providerFilter]);

	const hasFilters = search !== '' || providerFilter !== 'All';
	const clearFilters = () => { setSearch(''); setProviderFilter('All'); };

	/* ── Loading ── */
	if (isLoading && destinations.length === 0) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-4 px-4 py-4 border border-border rounded-lg">
						<Skeleton className="w-9 h-9 rounded-lg shrink-0" />
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
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search by name or bucket…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="pl-8 h-8 text-xs"
					/>
					{search && (
						<button onClick={() => setSearch('')} className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground">
							<X className="w-3 h-3" />
						</button>
					)}
				</div>

				{/* Provider filter dropdown */}
				<Select value={providerFilter} onValueChange={(v) => setProviderFilter(v as ProviderFilter)}>
					<SelectTrigger size="sm" className="h-8 w-44 text-xs">
						<SelectValue placeholder="Provider" />
					</SelectTrigger>
					<SelectContent>
						{PROVIDER_FILTERS.map(f => (
							<SelectItem key={f} value={f}>{f === 'All' ? 'All Providers' : f}</SelectItem>
						))}
					</SelectContent>
				</Select>


			</div>

			{/* ── Empty: no destinations ── */}
			{destinations.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 py-20 text-center border border-dashed border-border rounded-lg">
					<div className="w-12 h-12 rounded-full bg-muted flex items-center justify-center">
						<HardDrive className="w-5 h-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">No S3 destinations yet</p>
						<p className="text-xs text-muted-foreground mt-0.5">Add an S3-compatible bucket to enable backups.</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 py-14 text-center border border-dashed border-border rounded-lg">
					<Search className="w-5 h-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">No destinations match your filter</p>
					<Button variant="ghost" size="sm" onClick={clearFilters} className="text-xs h-7">Clear filters</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="border border-border rounded-lg overflow-hidden divide-y divide-border">
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
								className="group flex items-center gap-4 px-4 py-3.5 bg-card hover:bg-accent/30 transition-colors"
							>
								{/* Icon + test status dot */}
								<div className="relative shrink-0">
									<div className="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
										<HardDrive className="w-4 h-4 text-foreground/70" />
									</div>
									<span className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-background ${dotCls}`} />
								</div>

								{/* Info */}
								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2 mb-0.5">
										<span className="text-sm font-medium text-foreground truncate">{d.name}</span>
										<Badge variant="secondary" className="shrink-0 text-[10px] py-0 font-mono">
											{label}
										</Badge>
									</div>
									<p className="text-[11px] font-mono text-muted-foreground truncate">
										{d.bucket}{d.region ? ` · ${d.region}` : ''}{d.endpoint ? ` · ${d.endpoint}` : ''}
									</p>
								</div>

								{/* Hover: test button */}
								<div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
									<button
										onClick={() => handleTest(d.id)}
										disabled={isTesting}
										title={isTesting ? 'Testing…' : 'Test connection'}
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
									>
										{isTesting ? (
											<RefreshCw className="w-4 h-4 animate-spin text-amber-500" />
										) : status === 'success' ? (
											<Check className="w-4 h-4 text-emerald-500" />
										) : status === 'failed' ? (
											<X className="w-4 h-4 text-rose-500" />
										) : (
											<Plug className="w-4 h-4" />
										)}
									</button>
								</div>

								<Separator orientation="vertical" className="h-5 opacity-0 group-hover:opacity-100 transition-opacity" />

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger render={<button className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer" />}>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-40">
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => handleTest(d.id)}>
											<Plug className="w-3.5 h-3.5" /> Test Connection
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onEdit(d)}>
											<Pencil className="w-3.5 h-3.5" /> Edit
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="gap-2 cursor-pointer text-destructive focus:text-destructive"
											onClick={() => handleDelete(d.id)}
											disabled={deletingId === d.id}
										>
											<Trash2 className="w-3.5 h-3.5" />
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
				<p className="text-xs text-muted-foreground px-1">
					Showing {filtered.length} of {destinations.length} destinations
				</p>
			)}
		</div>
	);
}
