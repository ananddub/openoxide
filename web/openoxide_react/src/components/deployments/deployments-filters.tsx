import {useEffect, useRef} from 'react';
import {Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {SortKey} from '#/hooks/deployments/use-deployments';

interface DeploymentsFiltersProps {
	searchQuery: string;
	setSearchQuery: (val: string) => void;
	statusFilter: string;
	setStatusFilter: (val: string) => void;
	typeFilter: string;
	setTypeFilter: (val: string) => void;
	sortBy: SortKey;
	sortDir: 'asc' | 'desc';
	setSortBy: (val: SortKey) => void;
	setSortDir: (val: 'asc' | 'desc') => void;
}

const STATUS_FILTERS = [
	{value: 'all', label: 'All statuses'},
	{value: 'done', label: 'Success'},
	{value: 'error', label: 'Failed'},
	{value: 'running', label: 'Building'},
	{value: 'queued', label: 'Queued'},
	{value: 'cancelled', label: 'Canceled'},
];

const TYPES = [
	{value: 'all', label: 'All types'},
	{value: 'application', label: 'Application'},
	{value: 'compose', label: 'Compose'},
	{value: 'database', label: 'Database'},
];

const SORTS = [
	{value: 'created_at-desc', label: 'Newest First'},
	{value: 'created_at-asc', label: 'Oldest First'},
	{value: 'title-asc', label: 'Title (A-Z)'},
	{value: 'title-desc', label: 'Title (Z-A)'},
];

/**
 * History toolbar in the house grammar (SSH Keys / Tags): search with the
 * `/` focus shortcut on the left, quiet selects on the right.
 */
export function DeploymentsFilters({
	searchQuery,
	setSearchQuery,
	statusFilter,
	setStatusFilter,
	typeFilter,
	setTypeFilter,
	sortBy,
	sortDir,
	setSortBy,
	setSortDir,
}: DeploymentsFiltersProps) {
	const searchRef = useRef<HTMLInputElement>(null);

	// Same `/` shortcut as the SSH Keys / Tags toolbars.
	useEffect(() => {
		const onKey = (e: KeyboardEvent) => {
			if (e.key !== '/' || e.metaKey || e.ctrlKey || e.altKey) return;
			const el = document.activeElement as HTMLElement | null;
			if (
				el &&
				(el.tagName === 'INPUT' ||
					el.tagName === 'TEXTAREA' ||
					el.tagName === 'SELECT' ||
					el.isContentEditable)
			) {
				return;
			}
			e.preventDefault();
			searchRef.current?.focus();
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, []);

	return (
		<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
			<div className="relative w-full sm:grow">
				<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
				<Input
					ref={searchRef}
					placeholder="Search by title, description or logs..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-9 rounded-lg pr-11 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
				/>
				<kbd
					aria-hidden
					className="pointer-events-none absolute top-1/2 right-3 flex size-5 -translate-y-1/2 items-center justify-center rounded-md border border-border bg-foreground/[0.04] font-mono text-[11px] text-muted-foreground select-none">
					/
				</kbd>
			</div>

			<div className="flex shrink-0 flex-wrap items-center gap-2">
				<Select value={statusFilter} onValueChange={setStatusFilter}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<SelectValue>
							{STATUS_FILTERS.find(s => s.value === statusFilter)
								?.label ?? 'All statuses'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card">
						{STATUS_FILTERS.map(s => (
							<SelectItem key={s.value} value={s.value}>
								{s.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>

				<Select value={typeFilter} onValueChange={setTypeFilter}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<SelectValue>
							{TYPES.find(t => t.value === typeFilter)?.label ??
								'All types'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card">
						{TYPES.map(t => (
							<SelectItem key={t.value} value={t.value}>
								{t.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>

				<Select
					value={`${sortBy}-${sortDir}`}
					onValueChange={val => {
						if (val) {
							const [key, dir] = val.split('-');
							setSortBy(key as SortKey);
							setSortDir(dir as 'asc' | 'desc');
						}
					}}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<SelectValue>
							{SORTS.find(s => s.value === `${sortBy}-${sortDir}`)
								?.label ?? 'Newest First'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card">
						{SORTS.map(s => (
							<SelectItem key={s.value} value={s.value}>
								{s.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
