import {Pencil, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TagBadge} from '#/components/shared/tag-badge';
import type {TagItem} from '#/components/tags/tag-edit-dialog';

/** A single row in the tags list. */
export function TagListItem({
	tag,
	onEdit,
	onDelete,
}: {
	tag: TagItem;
	onEdit: (tag: TagItem) => void;
	onDelete: (tag: TagItem) => void;
}) {
	return (
		<div className="group flex items-center justify-between px-4 py-3 transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-center gap-3.5">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
					<span
						className="size-3.5 shrink-0 rounded-full"
						style={{backgroundColor: tag.color || '#3b82f6'}}
					/>
				</div>
				<div className="min-w-0 flex-1">
					<span className="block truncate text-sm font-medium text-foreground">
						{tag.name}
					</span>
					<span className="font-mono text-[11px] text-muted-foreground">
						{tag.color}
					</span>
				</div>
			</div>

			<div className="flex items-center gap-2">
				<TagBadge name={tag.name} color={tag.color} />
				<div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onEdit(tag)}
						className="size-7 text-muted-foreground hover:text-foreground">
						<Pencil className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onDelete(tag)}
						className="size-7 text-muted-foreground hover:text-destructive">
						<Trash2 className="size-3.5" />
					</Button>
				</div>
			</div>
		</div>
	);
}
