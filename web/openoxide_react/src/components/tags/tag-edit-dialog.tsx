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
			<SheetContent className="p-0 sm:data-[side=right]:w-[430px]">
				<SheetHeader>
					<SheetTitle>Edit Tag</SheetTitle>
					<SheetDescription>
						Rename this label or change its badge color.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSaveTag}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
						{/* Live preview — exactly the badge shown on resources. */}
						<div className="flex justify-center rounded-xl bg-secondary/40 px-4 py-6">
							<TagBadge
								name={tagName.trim() || 'New tag'}
								color={tagColor}
								className="px-4 py-2 text-[15px] font-semibold [&>span:first-child]:size-3"
							/>
						</div>

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
					</div>

					<SheetFooter>
						<Button
							type="submit"
							disabled={isSubmitting}
							className="h-10 gap-1.5 px-5">
							{isSubmitting ? 'Saving…' : 'Save'}
						</Button>
					</SheetFooter>
				</form>
			</SheetContent>
		</Sheet>
	);
}
