import {useState} from 'react';
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
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagCreate} from '#/api/live';
import {TagColorPicker} from '#/components/tags/tag-color-picker';
import {TagBadge} from '#/components/shared/tag-badge';
import {WizardField} from '#/components/shared/sheet-wizard';

interface TagCreateSheetProps {
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
}

/** Create Tag — Geist side sheet, same grammar as New SSH Key. */
export function TagCreateSheet({
	isOpen,
	onOpenChange,
}: TagCreateSheetProps) {
	const [tagName, setTagName] = useState('');
	const [tagColor, setTagColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = useTagCreate();

	const reset = () => {
		setTagName('');
		setTagColor('#3b82f6');
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!tagName.trim()) return;
		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name: tagName.trim(),
				color: tagColor,
			});
			toast.success('Tag created');
			reset();
			onOpenChange(false);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Sheet
			open={isOpen}
			onOpenChange={open => {
				if (!open) reset();
				onOpenChange(open);
			}}>
			<SheetContent className="p-0 sm:data-[side=right]:w-[430px]">
				<SheetHeader>
					<SheetTitle>New Tag</SheetTitle>
					<SheetDescription>
						Color-coded label for organizing projects and resources.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSubmit}
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
