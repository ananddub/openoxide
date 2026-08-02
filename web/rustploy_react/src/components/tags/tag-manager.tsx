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
import {HandleTag} from './handle-tag';

interface TagItem {
	id: number;
	name: string;
	color: string;
}

export const TagManager = () => {
	const [searchQuery, setSearchQuery] = useState('');
	const {data: rawTags = [], isLoading, refetch} = $api.useQuery('get', '/tags' as any, {});
	const deleteMutation = $api.useMutation('delete', '/tags/{id}' as any);

	const tags: TagItem[] = useMemo(() => {
		return Array.isArray(rawTags) ? (rawTags as TagItem[]) : [];
	}, [rawTags]);

	const filteredTags = useMemo(() => {
		if (!searchQuery.trim()) return tags;
		const q = searchQuery.toLowerCase();
		return tags.filter(t => t.name.toLowerCase().includes(q));
	}, [tags, searchQuery]);

	return (
		<div className="w-full max-w-5xl mx-auto">
			<Card className="bg-card border border-border rounded-2xl shadow-sm overflow-hidden">
				<CardHeader className="flex flex-row items-center justify-between flex-wrap gap-4 pb-4">
					<div className="flex flex-col gap-1">
						<div className="flex items-center gap-2.5">
							<CardTitle className="text-xl font-bold flex items-center gap-2">
								<TagIcon className="size-5 text-primary" />
								<span>Tags &amp; Labels</span>
							</CardTitle>
							<Badge variant="secondary" className="text-xs font-mono px-2 py-0.5">
								{tags.length} Total
							</Badge>
						</div>
						<CardDescription className="text-xs text-muted-foreground">
							Create and manage color-coded tags to organize your projects and deployments
						</CardDescription>
					</div>

					<div className="flex items-center gap-3">
						<HandleTag onSuccess={refetch} />
					</div>
				</CardHeader>

				<CardContent className="space-y-4 pt-4 border-t border-border/60">
					{/* Search & Filter Bar */}
					{tags.length > 0 && (
						<div className="relative max-w-sm w-full">
							<Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search tags by name..."
								value={searchQuery}
								onChange={e => setSearchQuery(e.target.value)}
								className="pl-9 h-9 text-xs bg-muted/20"
							/>
						</div>
					)}

					{isLoading ? (
						<div className="flex flex-row gap-2 items-center justify-center text-sm text-muted-foreground min-h-[25vh]">
							<Loader2 className="animate-spin size-4 text-primary" />
							<span>Loading tags...</span>
						</div>
					) : !tags || tags.length === 0 ? (
						<div className="flex flex-col items-center gap-3 min-h-[28vh] justify-center text-center p-6 border border-dashed border-border rounded-xl bg-muted/10">
							<TagIcon className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">No tags created yet</span>
							<span className="text-xs text-muted-foreground max-w-sm">
								Create color-coded tags to easily search, filter, and organize your applications.
							</span>
							<HandleTag onSuccess={refetch} />
						</div>
					) : filteredTags.length === 0 ? (
						<div className="flex flex-col items-center gap-2 min-h-[20vh] justify-center text-center text-muted-foreground">
							<span className="text-sm font-semibold text-foreground">No tags match "{searchQuery}"</span>
							<span className="text-xs text-muted-foreground">Try searching for a different tag name</span>
						</div>
					) : (
						<div className="flex flex-col gap-2.5 min-h-[25vh]">
							{filteredTags.map(tag => (
								<div
									key={tag.id}
									className="flex items-center justify-between p-3 rounded-xl bg-card border border-border/70 hover:border-border transition-all shadow-2xs group"
								>
									<div className="flex items-center gap-3">
										<TagBadge name={tag.name} color={tag.color} />
										{tag.color && (
											<span className="text-xs text-muted-foreground font-mono bg-muted/30 px-2 py-0.5 rounded border border-border/40">
												{tag.color}
											</span>
										)}
									</div>
									<div className="flex items-center gap-1">
										<HandleTag tagId={tag.id} tagData={tag} onSuccess={refetch} />
										<DialogAction
											title="Delete Tag"
											description={`Are you sure you want to delete the tag "${tag.name}"? This will remove the tag from all projects. This action cannot be undone.`}
											type="destructive"
											onClick={async () => {
												await deleteMutation.mutateAsync({
													params: {path: {id: tag.id}},
												})
													.then(async () => {
														refetch();
														toast.success('Tag deleted successfully');
													})
													.catch(() => {
														toast.error('Error deleting tag');
													});
											}}
										>
											<Button
												variant="ghost"
												size="icon"
												className="h-8 w-8 group hover:bg-rose-500/10 text-muted-foreground hover:text-rose-500 rounded-lg"
											>
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
