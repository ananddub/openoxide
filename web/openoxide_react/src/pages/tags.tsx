import {useState, useMemo} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {Plus, Search, Trash2, Pencil, RefreshCw, Tag as TagIcon} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {TagBadge} from '#/components/shared/tag-badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {useTagListAll} from 'virtual:openoxide-live';
import {formatApiError} from '#/api/utils';
import { useAppStore } from '#/stores/app-store';

export const Route = createFileRoute('/_app/tags')({
	component: MinimalTagsPage,
});

const PRESET_COLORS = [
	'#3b82f6', // Blue
	'#10b981', // Green
	'#f59e0b', // Amber
	'#ef4444', // Red
	'#8b5cf6', // Purple
	'#ec4899', // Pink
	'#06b6d4', // Cyan
	'#64748b', // Slate
];

interface TagItem {
	id: number;
	name: string;
	color: string;
}

function MinimalTagsPage() {
	const [searchQuery, setSearchQuery] = useState('');
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [editingTag, setEditingTag] = useState<TagItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<TagItem | null>(null);

	const [tagName, setTagName] = useState('');
	const [tagColor, setTagColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [isDeleting, setIsDeleting] = useState(false);

	const storeTags = useAppStore((state) => state.tags);

	const { data: rawTags, loading: isQueryLoading } = useTagListAll();

	const tagsList: TagItem[] = useMemo(() => {
		const source = (rawTags && Array.isArray(rawTags) && rawTags.length > 0)
			? rawTags
			: (storeTags || []);
		return Array.isArray(source) ? (source as unknown as TagItem[]) : [];
	}, [rawTags, storeTags]);

	const isLoading = tagsList.length === 0 && isQueryLoading;

	const createMutation = $api.useMutation('post', '/tags' as any);
	const patchMutation = $api.useMutation('patch', '/tags/{id}' as any);
	const deleteMutation = $api.useMutation('delete', '/tags/{id}' as any);

	const filteredTags = useMemo(() => {
		if (!searchQuery.trim()) return tagsList;
		const q = searchQuery.toLowerCase();
		return tagsList.filter(t => t.name.toLowerCase().includes(q));
	}, [tagsList, searchQuery]);

	const handleOpenCreate = () => {
		setEditingTag(null);
		setTagName('');
		setTagColor('#3b82f6');
		setIsCreateOpen(true);
	};

	const handleOpenEdit = (tag: TagItem) => {
		setEditingTag(tag);
		setTagName(tag.name);
		setTagColor(tag.color || '#3b82f6');
		setIsCreateOpen(true);
	};

	const handleSaveTag = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!tagName.trim()) return;

		setIsSubmitting(true);
		try {
			if (editingTag) {
				await patchMutation.mutateAsync({
					params: {path: {id: editingTag.id}},
					body: {name: tagName.trim(), color: tagColor},
				});
				toast.success('Tag updated');
			} else {
				await createMutation.mutateAsync({
					body: {name: tagName.trim(), color: tagColor},
				});
				toast.success('Tag created');
			}
			setIsCreateOpen(false);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const confirmDeleteTag = async () => {
		if (!deleteTarget) return;
		setIsDeleting(true);
		try {
			await deleteMutation.mutateAsync({params: {path: {id: deleteTarget.id}}});
			toast.success('Tag removed');
			setDeleteTarget(null);
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsDeleting(false);
		}
	};

	return (
		<div className="flex flex-col gap-5 p-6 max-w-4xl mx-auto w-full pb-16 animate-in fade-in duration-150">
			{/* Minimal Page Header */}
			<div className="flex items-center justify-between gap-4 pb-2 border-b border-border/40">
				<div>
					<h1 className="text-base font-semibold text-foreground tracking-tight flex items-center gap-2">
						<TagIcon className="w-4 h-4 text-muted-foreground" />
						<span>Tags</span>
						<span className="text-xs text-muted-foreground font-normal font-mono">({tagsList.length})</span>
					</h1>
					<p className="text-xs text-muted-foreground mt-0.5">
						Color-coded labels for organizing projects and resources
					</p>
				</div>

				<div className="flex items-center gap-2">

					<Button
						size="sm"
						onClick={handleOpenCreate}
						className="h-8 text-xs font-medium px-3 bg-primary text-primary-foreground hover:bg-primary/90"
					>
						<Plus className="w-3.5 h-3.5 mr-1" />
						New Tag
					</Button>
				</div>
			</div>

			{/* Minimal Search Row */}
			{tagsList.length > 0 && (
				<div className="relative max-w-xs">
					<Search className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground/60" />
					<Input
						placeholder="Filter tags..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-8 h-8 text-xs bg-muted/20 border-border/40 focus:bg-background"
					/>
				</div>
			)}

			{/* Minimal List Container */}
			{isLoading ? (
				<div className="flex items-center justify-center min-h-[180px] text-muted-foreground text-xs">
					<RefreshCw className="w-3.5 h-3.5 animate-spin mr-2" />
					Loading...
				</div>
			) : filteredTags.length === 0 ? (
				<div className="border border-dashed border-border/50 rounded-xl p-8 text-center flex flex-col items-center justify-center gap-2 bg-muted/10">
					<span className="text-xs text-muted-foreground">
						{searchQuery ? `No tags match "${searchQuery}"` : 'No tags created yet'}
					</span>
					{!searchQuery && (
						<Button size="sm" variant="outline" onClick={handleOpenCreate} className="h-7 text-xs mt-1">
							<Plus className="w-3 h-3 mr-1" /> Create Tag
						</Button>
					)}
				</div>
			) : (
				<div className="border border-border/50 rounded-xl overflow-hidden divide-y divide-border/40 bg-card/50">
					{filteredTags.map(tag => (
						<div
							key={tag.id}
							className="flex items-center justify-between px-4 py-2.5 hover:bg-muted/30 transition-colors group"
						>
							<div className="flex items-center gap-3 min-w-0">
								<span
									className="size-2.5 rounded-full shrink-0"
									style={{backgroundColor: tag.color || '#3b82f6'}}
								/>
								<span className="text-xs font-medium text-foreground truncate">
									{tag.name}
								</span>
								<span className="text-[11px] font-mono text-muted-foreground/50">
									{tag.color}
								</span>
							</div>

							<div className="flex items-center gap-2">
								<TagBadge name={tag.name} color={tag.color} />
								<div className="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
									<Button
										variant="ghost"
										size="icon"
										onClick={() => handleOpenEdit(tag)}
										className="h-7 w-7 text-muted-foreground hover:text-foreground"
									>
										<Pencil className="w-3.5 h-3.5" />
									</Button>
									<Button
										variant="ghost"
										size="icon"
										onClick={() => setDeleteTarget(tag)}
										className="h-7 w-7 text-muted-foreground hover:text-destructive"
									>
										<Trash2 className="w-3.5 h-3.5" />
									</Button>
								</div>
							</div>
						</div>
					))}
				</div>
			)}

			{/* Minimal Create/Edit Modal */}
			<Dialog open={isCreateOpen} onOpenChange={open => !open && setIsCreateOpen(false)}>
				<DialogContent className="sm:max-w-sm rounded-xl p-5 border-border/60">
					<DialogHeader className="pb-1 border-b border-border/40">
						<DialogTitle className="text-sm font-semibold">
							{editingTag ? 'Edit Tag' : 'New Tag'}
						</DialogTitle>
					</DialogHeader>

					<form onSubmit={handleSaveTag} className="flex flex-col gap-3.5 pt-2">
						<div className="flex flex-col gap-1">
							<label className="text-[11px] font-medium text-muted-foreground">Name</label>
							<Input
								placeholder="Tag name"
								value={tagName}
								onChange={e => setTagName(e.target.value)}
								className="h-8 text-xs bg-muted/20 border-border/50"
								autoFocus
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-medium text-muted-foreground">Color</label>
							<div className="flex items-center gap-1.5 flex-wrap">
								{PRESET_COLORS.map(c => (
									<button
										key={c}
										type="button"
										onClick={() => setTagColor(c)}
										className={`size-5 rounded-full transition-transform ${
											tagColor === c ? 'scale-125 ring-2 ring-primary/60 ring-offset-1' : 'opacity-75 hover:opacity-100'
										}`}
										style={{backgroundColor: c}}
									/>
								))}
							</div>
							<div className="flex items-center gap-2 mt-1">
								<span className="text-[11px] text-muted-foreground font-mono">Hex:</span>
								<Input
									type="text"
									value={tagColor}
									onChange={e => setTagColor(e.target.value)}
									className="w-24 h-7 text-[11px] font-mono bg-muted/20 border-border/50"
								/>
							</div>
						</div>

						{/* Minimal Preview */}
						<div className="flex items-center justify-between pt-2 border-t border-border/40">
							<span className="text-[11px] text-muted-foreground">Preview:</span>
							<TagBadge name={tagName.trim() || 'Preview'} color={tagColor} />
						</div>

						<div className="flex items-center justify-end gap-2 pt-3 border-t border-border/40">
							<Button
								type="button"
								variant="ghost"
								onClick={() => setIsCreateOpen(false)}
								className="h-8 text-xs"
							>
								Cancel
							</Button>
							<Button
								type="submit"
								disabled={isSubmitting}
								className="h-8 text-xs font-medium px-4"
							>
								{isSubmitting ? 'Saving...' : 'Save'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Minimal Delete Alert */}
			<AlertDialog open={deleteTarget !== null} onOpenChange={open => !open && setDeleteTarget(null)}>
				<AlertDialogContent className="sm:max-w-sm rounded-xl p-5 border-border/60">
					<AlertDialogHeader>
						<AlertDialogTitle className="text-sm font-semibold">Delete Tag</AlertDialogTitle>
						<AlertDialogDescription className="text-xs">
							Remove tag "{deleteTarget?.name}"?
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter className="pt-2">
						<AlertDialogCancel disabled={isDeleting} className="h-8 text-xs">Cancel</AlertDialogCancel>
						<AlertDialogAction
							onClick={confirmDeleteTag}
							disabled={isDeleting}
							className="h-8 text-xs bg-destructive text-destructive-foreground hover:bg-destructive/90"
						>
							{isDeleting ? 'Deleting...' : 'Delete'}
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
