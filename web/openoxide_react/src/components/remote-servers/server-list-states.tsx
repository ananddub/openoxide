import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Plus, Search, Server, X} from 'lucide-react';

const STATUS_FILTERS = ['All', 'Connected', 'Not Connected'] as const;
export type StatusFilter = (typeof STATUS_FILTERS)[number];

/** Loading placeholder rows. */
export function ServerListSkeleton() {
	return (
		<div className="flex flex-col gap-2">
			{[1, 2, 3].map(i => (
				<div
					key={i}
					className="flex items-center gap-4 rounded-lg px-4 py-4">
					<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-40" />
						<Skeleton className="h-3 w-56" />
					</div>
					<Skeleton className="h-5 w-14 rounded-full" />
				</div>
			))}
		</div>
	);
}

/** First-run empty state with the add CTA. */
export function ServerListEmpty({onAdd}: {onAdd: () => void}) {
	return (
		<div className="flex flex-col items-center justify-center gap-4 rounded-xl bg-muted/10 px-6 py-20 text-center">
			<div className="flex size-12 items-center justify-center rounded-full bg-muted">
				<Server className="size-5 text-muted-foreground" />
			</div>
			<div>
				<p className="text-sm font-medium text-foreground">
					No remote servers yet
				</p>
				<p className="mx-auto mt-1 max-w-sm text-xs text-muted-foreground">
					Connect a Linux node over SSH to deploy applications, containers
					and databases onto it from this panel.
				</p>
			</div>
			<Button
				size="sm"
				onClick={onAdd}
				className="h-9 cursor-pointer gap-1.5 px-4 text-xs font-semibold">
				<Plus className="size-3.5" />
				Add Server
			</Button>
		</div>
	);
}

/** Search input + status filter select. */
export function ServersToolbar({
	search,
	status,
	onSearch,
	onStatus,
}: {
	search: string;
	status: StatusFilter;
	onSearch: (v: string) => void;
	onStatus: (v: StatusFilter) => void;
}) {
	return (
		<div className="flex items-center gap-2">
			<div className="relative flex-1">
				<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
				<Input
					placeholder="Search by name or IP…"
					value={search}
					onChange={e => onSearch(e.target.value)}
					className="h-8 pl-8 text-xs"
				/>
				{search && (
					<button
						onClick={() => onSearch('')}
						className="absolute top-1/2 right-2 -translate-y-1/2 cursor-pointer text-muted-foreground hover:text-foreground">
						<X className="h-3 w-3" />
					</button>
				)}
			</div>
			<Select
				value={status}
				onValueChange={v => onStatus(v as StatusFilter)}>
				<SelectTrigger size="sm" className="h-8 w-40 text-xs">
					<SelectValue placeholder="Status" />
				</SelectTrigger>
				<SelectContent>
					{STATUS_FILTERS.map(f => (
						<SelectItem key={f} value={f}>
							{f === 'All' ? 'All Servers' : f}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</div>
	);
}

/** No-results state shown when filters exclude everything. */
export function ServerFilterEmpty({onClear}: {onClear: () => void}) {
	return (
		<div className="flex flex-col items-center justify-center gap-2 rounded-xl bg-card py-14 text-center ring-1 ring-border/50">
			<Search className="size-5 text-muted-foreground" />
			<p className="text-sm text-muted-foreground">
				No servers match your filter
			</p>
			<Button
				variant="ghost"
				size="sm"
				onClick={onClear}
				className="h-7 cursor-pointer text-xs">
				Clear filters
			</Button>
		</div>
	);
}
