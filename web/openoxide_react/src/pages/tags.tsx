import {useMemo, useState} from 'react';
import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {
	Hash,
	Palette,
	Plus,
	RefreshCw,
	Search,
	Tag as TagIcon,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Titlebar} from '#/components/shared/titlebar';
import {
	ResourceGuideCard,
	ResourceInfoCard,
} from '#/components/shared/resource-sidebar';
import {useTagListAll} from '#/api/live';
import {
	TagEditDialog,
	type TagItem,
} from '#/components/tags/tag-edit-dialog';
import {TagDeleteDialog} from '#/components/tags/tag-delete-dialog';
import {TagListItem} from '#/components/tags/tag-list-item';

export const Route = createFileRoute('/_app/tags')({
	component: MinimalTagsPage,
});

function MinimalTagsPage() {
	const navigate = useNavigate();
	const [searchQuery, setSearchQuery] = useState('');
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingTag, setEditingTag] = useState<TagItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<TagItem | null>(null);

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
		if (!searchQuery.trim()) return tagsList;
		const q = searchQuery.toLowerCase();
		return tagsList.filter(t => t.name.toLowerCase().includes(q));
	}, [tagsList, searchQuery]);

	const distinctColors = useMemo(
		() => new Set(tagsList.map(t => t.color || '#3b82f6')).size,
		[tagsList],
	);

	const handleOpenCreate = () => void navigate({to: '/tags/new'});

	const handleOpenEdit = (tag: TagItem) => {
		setEditingTag(tag);
		setIsEditOpen(true);
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Page Header */}
			<Titlebar
				breadcrumbs={[{label: 'Tags'}]}
				icon={<TagIcon className="text-foreground/70" />}
				title="Tags"
				titleSuffix={
					<span className="font-mono text-sm font-normal text-muted-foreground/70">
						({tagsList.length})
					</span>
				}
				description="Color-coded labels for organizing projects and resources"
				actions={
					<Button size="sm" onClick={handleOpenCreate}>
						<Plus className="size-3.5" />
						New Tag
					</Button>
				}
			/>

			{/* Search Row */}
			{tagsList.length > 0 && (
				<div className="relative max-w-xs">
					<Search className="absolute top-1/2 left-3 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground/60" />
					<Input
						placeholder="Filter tags..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="h-8 bg-muted/20 pl-8 text-xs focus:bg-background"
					/>
				</div>
			)}
			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					{/* List Container */}
					{isLoading ? (
						<div className="flex min-h-[180px] items-center justify-center text-xs text-muted-foreground">
							<RefreshCw className="mr-2 h-3.5 w-3.5 animate-spin" />
							Loading...
						</div>
					) : filteredTags.length === 0 ? (
						<div className="flex flex-col items-center justify-center gap-2 rounded-xl bg-muted/10 p-8 text-center">
							<span className="text-xs text-muted-foreground">
								{searchQuery
									? `No tags match "${searchQuery}"`
									: 'No tags created yet'}
							</span>
							{!searchQuery && (
								<Button
									size="sm"
									variant="outline"
									onClick={handleOpenCreate}
									className="mt-1 h-7 text-xs">
									<Plus className="mr-1 h-3 w-3" /> Create Tag
								</Button>
							)}
						</div>
					) : (
						<div className="divide-y divide-border/40 overflow-hidden rounded-xl bg-card/50">
							{filteredTags.map(tag => (
								<TagListItem
									key={tag.id}
									tag={tag}
									onEdit={handleOpenEdit}
									onDelete={setDeleteTarget}
								/>
							))}
						</div>
					)}
				</div>

				{/* ── Right rail ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<ResourceInfoCard
						title="Overview"
						icon={TagIcon}
						rows={[
							{icon: Hash, label: 'Total tags', value: tagsList.length},
							{icon: Palette, label: 'Colors used', value: distinctColors},
						]}
					/>
					<ResourceGuideCard
						title="Where tags help"
						icon={Hash}
						steps={[
							{
								title: 'Group projects',
								text: 'Attach tags to projects, apps and databases to filter dashboards by environment or team.',
							},
							{
								title: 'Color code',
								text: 'The tag color shows everywhere it is attached — quick visual status at a glance.',
							},
						]}
					/>
				</div>
			</div>

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
