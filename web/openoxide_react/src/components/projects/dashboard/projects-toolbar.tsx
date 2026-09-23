import {useEffect, useRef} from 'react';
import {
	ArrowUpDown,
	LayoutGrid,
	List,
	Search,
} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {TagSelector} from '#/components/shared/tag-selector';
import {cn} from '#/api/utils';

export type ProjectsView = 'grid' | 'list';

type ProjectsToolbarProps = {
	searchQuery: string;
	onSearchChange: (val: string) => void;
	sortBy: string;
	onSortChange: (val: string) => void;
	selectedTags: string[];
	onTagsChange: (tags: string[]) => void;
	view: ProjectsView;
	onViewChange: (view: ProjectsView) => void;
};

const SORT_LABELS: Record<string, string> = {
	newest: 'Newest First',
	oldest: 'Oldest First',
	'alphabetical-asc': 'Name (A-Z)',
	'alphabetical-desc': 'Name (Z-A)',
};

export function ProjectsToolbar({
	searchQuery,
	onSearchChange,
	sortBy,
	onSortChange,
	selectedTags,
	onTagsChange,
	view,
	onViewChange,
}: ProjectsToolbarProps) {
	const searchRef = useRef<HTMLInputElement>(null);

	// Vercel's `/` shortcut: focus search from anywhere unless already
	// typing in a field.
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
			{/* Prominent search — Geist command-style input: flat
			    background-100 surface, hairline border that lifts on hover,
			    muted prefix glyph, kbd suffix */}
			<div className="relative w-full sm:grow">
				<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
				<Input
					ref={searchRef}
					placeholder="Search Projects"
					value={searchQuery}
					onChange={e => onSearchChange(e.target.value)}
					className="h-9 rounded-lg pr-11 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
				/>
				<kbd
					aria-hidden
					className="pointer-events-none absolute top-1/2 right-3 flex size-5 -translate-y-1/2 items-center justify-center rounded-md border border-border bg-foreground/[0.04] font-mono text-[11px] text-muted-foreground select-none">
					/
				</kbd>
			</div>

			<div className="flex shrink-0 items-center gap-2">
				{/* Tag filter */}
				<TagSelector
					selectedTags={selectedTags}
					onTagsChange={onTagsChange}
					placeholder="Tags"
					variant="filter"
					className="h-9 min-h-9 w-auto rounded-lg"
				/>

				{/* Sort */}
				<Select value={sortBy} onValueChange={val => val && onSortChange(val)}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<div className="flex min-w-0 items-center gap-2">
							<ArrowUpDown className="size-3.5 shrink-0 text-muted-foreground" />
							<SelectValue>{SORT_LABELS[sortBy] ?? 'Newest'}</SelectValue>
						</div>
					</SelectTrigger>
					<SelectContent className="bg-card">
						{Object.entries(SORT_LABELS).map(([value, label]) => (
							<SelectItem key={value} value={value}>
								Sort: {label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>

				{/* Grid / list toggle */}
				<div className="flex items-center rounded-lg border bg-card p-1">
					<button
						type="button"
						aria-label="Grid view"
						title="Grid view"
						onClick={() => onViewChange('grid')}
						className={cn(
							'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
							view === 'grid' && 'bg-accent text-foreground',
						)}>
						<LayoutGrid className="size-3.5" />
					</button>
					<button
						type="button"
						aria-label="List view"
						title="List view"
						onClick={() => onViewChange('list')}
						className={cn(
							'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
							view === 'list' && 'bg-accent text-foreground',
						)}>
						<List className="size-3.5" />
					</button>
					</div>
			</div>
		</div>
	);
}
