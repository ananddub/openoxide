import {useEffect, useMemo, useRef, useState} from 'react';
import {
	LayoutGrid,
	List,
	Search,
	LayoutTemplate as TemplateIcon,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {ListRegion, SectionHeader} from '#/components/settings/list-card';
import {TemplateCard} from './template-card';
import {TemplateListItem} from './template-list-item';
import {TemplateDetailDialog} from './template-detail-dialog';
import {useTemplateList, type TemplateMeta} from '#/api/live';
import {cn} from '#/api/utils';

const VIEW_STORAGE_KEY = 'templates-view';
const PAGE_SIZE = 60;

/** Templates — the tags-page grammar (title, search + grid/list toolbar, responsive card grid) over the remote template registry; clicking a card opens the detail dialog with a one-click Deploy. */
export function TemplatesPage() {
	const [search, setSearch] = useState('');
	const [view, setView] = useState<'grid' | 'list'>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const [visible, setVisible] = useState(PAGE_SIZE);
	const [selected, setSelected] = useState<TemplateMeta | null>(null);

	const handleViewChange = (next: 'grid' | 'list') => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	const templatesQuery = useTemplateList();
	const isLoading = templatesQuery.isLoading;

	const templates: TemplateMeta[] = useMemo(
		() => (Array.isArray(templatesQuery.data) ? templatesQuery.data : []),
		[templatesQuery.data],
	);

	const filtered = useMemo(() => {
		const q = search.trim().toLowerCase();
		if (!q) return templates;
		return templates.filter(
			t =>
				t.name.toLowerCase().includes(q) ||
				t.description?.toLowerCase().includes(q) ||
				t.tags?.some(tag => tag.toLowerCase().includes(q)),
		);
	}, [templates, search]);

	useEffect(() => setVisible(PAGE_SIZE), [search]);

	const searchRef = useRef<HTMLInputElement>(null);

	// Same `/` shortcut as the Tags toolbar.
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

	const shown = filtered.slice(0, visible);

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Templates"
				description="Ready-made application blueprints — browse the registry and deploy any of them as a compose stack in one click."
			/>

			{/* Search + grid/list toggle */}
			<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
				<div className="relative w-full sm:grow">
					<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						ref={searchRef}
						placeholder="Search templates"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-9 rounded-lg pr-11 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
					/>
					<kbd
						aria-hidden
						className="pointer-events-none absolute top-1/2 right-3 flex size-5 -translate-y-1/2 items-center justify-center rounded-md border border-border bg-foreground/[0.04] font-mono text-[11px] text-muted-foreground select-none">
						/
					</kbd>
				</div>

				<div className="flex shrink-0 items-center gap-2">
					<div className="flex items-center rounded-lg border bg-card p-1">
						<button
							type="button"
							aria-label="Grid view"
							title="Grid view"
							onClick={() => handleViewChange('grid')}
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
							onClick={() => handleViewChange('list')}
							className={cn(
								'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
								view === 'list' && 'bg-accent text-foreground',
							)}>
							<List className="size-3.5" />
						</button>
					</div>
				</div>
			</div>

			<ListRegion>
				{isLoading ? (
					<div className="min-h-[200px] animate-pulse rounded-xl border bg-card" />
				) : templatesQuery.isError ? (
					<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
						<TemplateIcon className="size-10 text-muted-foreground/40" />
						<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
							Template registry unreachable
						</h3>
						<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
							Could not load the template catalog. Check your network
							connection and try again.
						</p>
						<Button
							variant="outline"
							onClick={() => templatesQuery.refetch()}
							className="mt-2 text-xs font-semibold">
							Retry
						</Button>
					</div>
				) : filtered.length === 0 ? (
					<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
						<Search className="size-10 text-muted-foreground/40" />
						<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
							No matching templates found
						</h3>
						<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
							No templates match &ldquo;{search}&rdquo;
						</p>
						{search.trim() && (
							<Button
								variant="ghost"
								onClick={() => setSearch('')}
								className="mt-4 text-xs font-semibold text-primary">
								Clear Search
							</Button>
						)}
					</div>
				) : view === 'grid' ? (
					<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
						{shown.map(template => (
							<TemplateCard
								key={template.id}
								template={template}
								onOpen={setSelected}
							/>
						))}
					</div>
				) : (
					<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
						{shown.map(template => (
							<TemplateListItem
								key={template.id}
								template={template}
								onOpen={setSelected}
							/>
						))}
					</div>
				)}

				{shown.length < filtered.length && (
					<div className="flex justify-center">
						<Button
							variant="outline"
							onClick={() => setVisible(v => v + PAGE_SIZE)}
							className="h-9 gap-1.5 text-xs font-semibold">
							Show more ({filtered.length - shown.length} left)
						</Button>
					</div>
				)}
			</ListRegion>

			<TemplateDetailDialog
				template={selected}
				onOpenChange={open => !open && setSelected(null)}
			/>
		</div>
	);
}
