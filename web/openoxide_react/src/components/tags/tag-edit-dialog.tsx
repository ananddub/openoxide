import {useEffect, useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {TagBadge} from '#/components/shared/tag-badge';
import {TagColorPicker} from '#/components/tags/tag-color-picker';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagUpdate} from '#/api/live';

export interface TagItem {
	id: number;
	name: string;
	color?: string;
}

/** Inline editor for renaming / recoloring an existing tag. */
export function TagEditDialog({
	tag,
	open,
	onOpenChange,
}: {
	tag: TagItem | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}) {
	const [tagName, setTagName] = useState('');
	const [tagColor, setTagColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const patchMutation = useTagUpdate();

	useEffect(() => {
		if (tag) {
			setTagName(tag.name);
			setTagColor(tag.color || '#3b82f6');
		}
	}, [tag]);

	const handleSaveTag = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!tag || !tagName.trim()) return;
		setIsSubmitting(true);
		try {
			await patchMutation.mutateAsync({
				id: tag.id,
				name: tagName.trim(),
				color: tagColor,
			});
			toast.success('Tag updated');
			onOpenChange(false);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="rounded-xl p-5 sm:max-w-sm">
				<DialogHeader className="border-b pb-1">
					<DialogTitle className="text-sm font-semibold">
						Edit Tag
					</DialogTitle>
				</DialogHeader>

				<form
					onSubmit={handleSaveTag}
					className="flex flex-col gap-3.5 pt-2">
					<div className="flex flex-col gap-1">
						<label className="text-[11px] font-medium text-muted-foreground">
							Name
						</label>
						<Input
							placeholder="Tag name"
							value={tagName}
							onChange={e => setTagName(e.target.value)}
							className="h-8 bg-muted/20 text-xs"
							autoFocus
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-[11px] font-medium text-muted-foreground">
							Color
						</label>
						<TagColorPicker color={tagColor} onChange={setTagColor} />
					</div>

					{/* Preview */}
					<div className="flex items-center justify-between border-t pt-2">
						<span className="text-[11px] text-muted-foreground">
							Preview:
						</span>
						<TagBadge
							name={tagName.trim() || 'Preview'}
							color={tagColor}
						/>
					</div>

					<div className="flex items-center justify-end gap-2 border-t pt-3">
						<Button
							type="button"
							variant="ghost"
							onClick={() => onOpenChange(false)}
							className="h-8 text-xs">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting}
							className="h-8 px-4 text-xs font-medium">
							{isSubmitting ? 'Saving...' : 'Save'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
