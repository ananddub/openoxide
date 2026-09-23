import {useEffect, useMemo, useRef, useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {
	LayoutGrid,
	List,
	Plus,
	Search,
	Tag as TagIcon,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {SectionHeader} from '#/components/settings/list-card';
import {
	TagEditDialog,
	type TagItem,
} from '#/components/tags/tag-edit-dialog';
import {TagDeleteDialog} from '#/components/tags/tag-delete-dialog';
import {TagListItem} from '#/components/tags/tag-list-item';
import {TagCard} from '#/components/tags/tag-card';
import {TagCreateSheet} from '#/components/tags/tag-create-sheet';
import {useTagListAll} from '#/api/live';
import {cn} from '#/api/utils';

export const Route = createFileRoute('/_app/tags')({
	component: TagsPage,
});

type TagsView = 'grid' | 'list';

const VIEW_STORAGE_KEY = 'tags-view';

/**
 * Tags — the SSH Keys / Projects-page grammar: a section title, then a
 * toolbar (search + grid/list toggle + New) and cards or one bordered list
 * card below; create opens a Geist sheet. No side rail.
 */
function TagsPage() {
	const [search, setSearch] = useState('');
	const [view, setView] = useState<TagsView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingTag, setEditingTag] = useState<TagItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<TagItem | null>(null);

	const handleViewChange = (next: TagsView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	// Live tags query keyed ['tags','list'] — the table-watch stream
	// invalidates it on tag changes while this page is mounted.
	const tagsListQuery = useTagListAll();
	const isLoading = tagsListQuery.isLoading;

	const tagsList: TagItem[] = useMemo(() => {
		return Array.isArray(tagsListQuery.data)
			? (tagsListQuery.data as unknown as TagItem[])
			: [];
	}, [tagsListQuery.data]);

	const filteredTags = useMemo(() => {
		if (!search.trim()) return tagsList;
		const q = search.toLowerCase();
		return tagsList.filter(t => t.name.toLowerCase().includes(q));
	}, [tagsList, search]);

	const hasFilters = search.trim() !== '';

	const searchRef = useRef<HTMLInputElement>(null);

	// Same `/` shortcut as the Projects toolbar.
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

	const handleOpenEdit = (tag: TagItem) => {
		setEditingTag(tag);
		setIsEditOpen(true);
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Tags"
				description="Color-coded labels to group projects, apps and databases by environment or team."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Tag
					</Button>
				}
			/>

			{/* Search + grid/list toggle */}
			<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
				<div className="relative w-full sm:grow">
					<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						ref={searchRef}
						placeholder="Search Tags"
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
					{/* Grid / list toggle */}
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

			{/* Content: grid cards or compact list */}
			{!isLoading && tagsList.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
					<TagIcon className="mb-4 size-10 text-muted-foreground/40" />
					<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
						No tags yet
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
						Create color-coded labels to group projects, apps and databases
						by environment or team.
					</p>
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="mt-4 h-9 gap-1.5 text-xs font-semibold">
						<Plus className="size-3.5" />
						New Tag
					</Button>
				</div>
			) : isLoading ? (
				<div className="min-h-[200px] animate-pulse rounded-xl border bg-card" />
			) : filteredTags.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
					<Search className="size-10 text-muted-foreground/40" />
					<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
						No matching tags found
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
						No tags match &ldquo;{search}&rdquo;
					</p>
					{hasFilters && (
						<Button
							variant="ghost"
							onClick={() => setSearch('')}
							className="mt-4 text-xs font-semibold text-primary">
							Clear All Filters
						</Button>
					)}
				</div>
			) : view === 'grid' ? (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
					{filteredTags.map(tag => (
						<TagCard
							key={String(tag.id)}
							tag={tag}
							onEdit={handleOpenEdit}
							onDelete={setDeleteTarget}
						/>
					))}
				</div>
			) : (
				<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
					{filteredTags.map(tag => (
						<TagListItem
							key={String(tag.id)}
							tag={tag}
							onEdit={handleOpenEdit}
							onDelete={setDeleteTarget}
						/>
					))}
				</div>
			)}

			{/* Create — Geist sheet */}
			<TagCreateSheet
				isOpen={isCreateOpen}
				onOpenChange={setIsCreateOpen}
			/>

			{/* Edit — Geist sheet, same grammar as New Tag */}
			<TagEditDialog
				tag={editingTag}
				open={isEditOpen}
				onOpenChange={setIsEditOpen}
			/>
			<TagDeleteDialog
				tag={deleteTarget}
				open={deleteTarget !== null}
				onOpenChange={() => setDeleteTarget(null)}
			/>
		</div>
	);
}
