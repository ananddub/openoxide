import {Pencil, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TagBadge} from '#/components/shared/tag-badge';
import type {TagItem} from '#/components/tags/tag-edit-dialog';

/** A tag tile: hairline card holding the color, an outlined square
 *  carrying the swatch. Same height grammar as the SSH key cards. */
export function TagCard({
	tag,
	onEdit,
	onDelete,
}: {
	tag: TagItem;
	onEdit: (tag: TagItem) => void;
	onDelete: (tag: TagItem) => void;
}) {
	const color = tag.color || '#3b82f6';

	return (
		<div className="group flex min-h-[88px] flex-col justify-between gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-start gap-3">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
					<span
						aria-hidden
						className="size-3.5 rounded-full"
						style={{backgroundColor: color}}
					/>
				</div>
				<div className="min-w-0 flex-1">
					<p className="truncate text-sm font-medium text-foreground">
						{tag.name}
					</p>
					<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
						{color}
					</p>
				</div>
			</div>
			<div className="flex items-center justify-between gap-2">
				<TagBadge name={tag.name} color={tag.color} />
				<div className="flex shrink-0 items-center gap-1">
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onEdit(tag)}
						title="Edit tag"
						className="size-7 rounded-md text-muted-foreground hover:text-foreground">
						<Pencil className="size-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onDelete(tag)}
						title="Delete tag"
						className="size-7 rounded-md text-muted-foreground hover:text-destructive">
						<Trash2 className="size-3.5" />
					</Button>
				</div>
			</div>
		</div>
	);
}
