import {useEffect, useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {TagBadge} from '#/components/shared/tag-badge';
import {TagColorPicker} from '#/components/tags/tag-color-picker';
import {WizardField} from '#/components/shared/sheet-wizard';
import {Tag as TagIcon} from 'lucide-react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagUpdate} from '#/api/live';

export interface TagItem {
	id: number;
	name: string;
	color?: string;
}

/** Edit Tag — Geist side sheet, same grammar as New Tag / New SSH Key. */
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
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent className="p-0">
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<TagIcon className="size-4 text-primary" />
						Edit Tag
					</SheetTitle>
					<SheetDescription>
						Rename this label or change its badge color.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSaveTag}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
						<WizardField label="Name" required>
							<Input
								placeholder="e.g. Production"
								value={tagName}
								onChange={e => setTagName(e.target.value)}
								required
								autoFocus
								className="bg-background"
							/>
						</WizardField>

						<WizardField
							label="Color"
							hint="Used for the badge across projects and resources.">
							<TagColorPicker color={tagColor} onChange={setTagColor} />
						</WizardField>

						<div className="flex items-center justify-between rounded-xl border border-border/60 px-4 py-3.5">
							<span className="text-[13px] text-muted-foreground">
								Preview
							</span>
							<TagBadge
								name={tagName.trim() || 'Preview'}
								color={tagColor}
							/>
						</div>
					</div>

					<SheetFooter>
						<Button
							type="button"
							variant="outline"
							onClick={() => onOpenChange(false)}>
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting}
							className="h-10 gap-1.5 px-5">
							{isSubmitting ? 'Saving…' : 'Save Changes'}
						</Button>
					</SheetFooter>
				</form>
			</SheetContent>
		</Sheet>
	);
}
