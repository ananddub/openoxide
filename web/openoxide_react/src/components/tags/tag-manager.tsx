import {useState, useMemo} from 'react';
import {Loader2, TagIcon, Trash2, Search} from 'lucide-react';
import {toast} from 'sonner';
import {DialogAction} from '#/components/shared/dialog-action';
import {TagBadge} from '#/components/shared/tag-badge';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {$api} from '#/api/query';
import {useAppStore} from '#/stores/app-store';
import {HandleTag} from './handle-tag';

interface TagItem {
	id: number;
	name: string;
	color: string;
}

export const TagManager = () => {
	const [searchQuery, setSearchQuery] = useState('');
	const rawTags = useAppStore(state => state.tags);
	const isLoading = false;
	const deleteMutation = $api.useMutation('delete', '/tags/{id}' as any);

	const tags: TagItem[] = useMemo(() => {
		return Array.isArray(rawTags) ? (rawTags as unknown as TagItem[]) : [];
	}, [rawTags]);

	const filteredTags = useMemo(() => {
		if (!searchQuery.trim()) return tags;
		const q = searchQuery.toLowerCase();
		return tags.filter(t => t.name.toLowerCase().includes(q));
	}, [tags, searchQuery]);

	return (
		<div className="mx-auto w-full max-w-5xl">
			<Card className="overflow-hidden rounded-2xl border border-border bg-card shadow-sm">
				<CardHeader className="flex flex-row flex-wrap items-center justify-between gap-4 pb-4">
					<div className="flex flex-col gap-1">
						<div className="flex items-center gap-2.5">
							<CardTitle className="flex items-center gap-2 text-xl font-bold">
								<TagIcon className="size-5 text-primary" />
								<span>Tags &amp; Labels</span>
							</CardTitle>
							<Badge
								variant="secondary"
								className="px-2 py-0.5 font-mono text-xs">
								{tags.length} Total
							</Badge>
						</div>
						<CardDescription className="text-xs text-muted-foreground">
							Create and manage color-coded tags to organize your projects
							and deployments
						</CardDescription>
					</div>

					<div className="flex items-center gap-3">
						<HandleTag onSuccess={() => {}} />
					</div>
				</CardHeader>

				<CardContent className="space-y-4 border-t border-border/60 pt-4">
					{/* Search & Filter Bar */}
					{tags.length > 0 && (
						<div className="relative w-full max-w-sm">
							<Search className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search tags by name..."
								value={searchQuery}
								onChange={e => setSearchQuery(e.target.value)}
								className="h-9 bg-muted/20 pl-9 text-xs"
							/>
						</div>
					)}

					{isLoading ? (
						<div className="flex min-h-[25vh] flex-row items-center justify-center gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin text-primary" />
							<span>Loading tags...</span>
						</div>
					) : !tags || tags.length === 0 ? (
						<div className="flex min-h-[28vh] flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border bg-muted/10 p-6 text-center">
							<TagIcon className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">
								No tags created yet
							</span>
							<span className="max-w-sm text-xs text-muted-foreground">
								Create color-coded tags to easily search, filter, and
								organize your applications.
							</span>
							<HandleTag onSuccess={() => {}} />
						</div>
					) : filteredTags.length === 0 ? (
						<div className="flex min-h-[20vh] flex-col items-center justify-center gap-2 text-center text-muted-foreground">
							<span className="text-sm font-semibold text-foreground">
								No tags match "{searchQuery}"
							</span>
							<span className="text-xs text-muted-foreground">
								Try searching for a different tag name
							</span>
						</div>
					) : (
						<div className="flex min-h-[25vh] flex-col gap-2.5">
							{filteredTags.map(tag => (
								<div
									key={tag.id}
									className="group flex items-center justify-between rounded-xl border border-border/70 bg-card p-3 shadow-2xs transition-all hover:border-border">
									<div className="flex items-center gap-3">
										<TagBadge name={tag.name} color={tag.color} />
										{tag.color && (
											<span className="rounded border border-border/40 bg-muted/30 px-2 py-0.5 font-mono text-xs text-muted-foreground">
												{tag.color}
											</span>
										)}
									</div>
									<div className="flex items-center gap-1">
										<HandleTag
											tagId={tag.id}
											tagData={tag}
											onSuccess={() => {}}
										/>
										<DialogAction
											title="Delete Tag"
											description={`Are you sure you want to delete the tag "${tag.name}"? This will remove the tag from all projects. This action cannot be undone.`}
											type="destructive"
											onClick={async () => {
												await deleteMutation
													.mutateAsync({
														params: {path: {id: tag.id}},
													})
													.then(async () => {
														toast.success('Tag deleted successfully');
													})
													.catch(() => {
														toast.error('Error deleting tag');
													});
											}}>
											<Button
												variant="ghost"
												size="icon"
												className="group h-8 w-8 rounded-lg text-muted-foreground hover:bg-rose-500/10 hover:text-rose-500">
												<Trash2 className="size-4" />
											</Button>
										</DialogAction>
									</div>
								</div>
							))}
						</div>
					)}
				</CardContent>
			</Card>
		</div>
	);
};
