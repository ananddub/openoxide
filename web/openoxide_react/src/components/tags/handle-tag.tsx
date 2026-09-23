import {PenBoxIcon, PlusIcon} from 'lucide-react';
import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {AlertBlock} from '#/components/shared/alert-block';
import {TagBadge} from '#/components/shared/tag-badge';
import {TagColorField} from '#/components/tags/tag-color-field';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {useTagCreate, useTagUpdate} from '#/api/live';
import {formatApiError} from '#/api/utils';

interface TagItem {
	id: number;
	name: string;
	color: string;
}

interface HandleTagProps {
	tagId?: number;
	tagData?: TagItem;
	onSuccess?: () => void;
}

export const HandleTag = ({tagId, tagData, onSuccess}: HandleTagProps) => {
	const [isOpen, setIsOpen] = useState(false);
	const [name, setName] = useState('');
	const [color, setColor] = useState('#3b82f6');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [errorMessage, setErrorMessage] = useState<string | null>(null);

	const createMutation = useTagCreate();
	const patchMutation = useTagUpdate();

	useEffect(() => {
		if (isOpen) {
			setErrorMessage(null);
			if (tagData) {
				setName(tagData.name || '');
				setColor(tagData.color || '#3b82f6');
			} else {
				setName('');
				setColor('#3b82f6');
			}
		}
	}, [tagData, isOpen]);

	const onSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		const trimmedName = name.trim();
		if (!trimmedName) {
			setErrorMessage('Tag name is required');
			return;
		}

		setIsSubmitting(true);
		setErrorMessage(null);
		try {
			if (tagId) {
				await patchMutation.mutateAsync({
					id: tagId,
					name: trimmedName,
					color: color || '#3b82f6',
				});
				toast.success('Tag Updated');
			} else {
				await createMutation.mutateAsync({
					name: trimmedName,
					color: color || '#3b82f6',
				});
				toast.success('Tag Created');
			}
			setIsOpen(false);
			onSuccess?.();
		} catch (err: any) {
			const msg =
				formatApiError(err) ||
				(tagId ? 'Error updating tag' : 'Error creating tag');
			setErrorMessage(msg);
			toast.error(tagId ? 'Error updating tag' : 'Error creating tag');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={setIsOpen}>
			<DialogTrigger
				render={
					tagId ? (
						<Button
							variant="ghost"
							size="icon"
							className="h-8 w-8 text-muted-foreground hover:text-foreground">
							<PenBoxIcon className="h-4 w-4" />
						</Button>
					) : (
						<Button
							size="sm"
							className="h-9 gap-1.5 bg-primary px-4 text-xs font-semibold text-primary-foreground shadow-xs hover:bg-primary/90">
							<PlusIcon className="h-3.5 w-3.5" />
							New Tag
						</Button>
					)
				}
			/>
			<DialogContent className="rounded-xl bg-card p-6 shadow-lg sm:max-w-md">
				<DialogHeader className="space-y-1">
					<DialogTitle className="text-base font-bold text-foreground">
						{tagId ? 'Update' : 'New'} Tag
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{tagId
							? 'Update the tag name and color'
							: 'Create a new tag to organize your projects'}
					</DialogDescription>
				</DialogHeader>
				{errorMessage && (
					<AlertBlock type="error">{errorMessage}</AlertBlock>
				)}

				<form
					id="hook-form-tag"
					onSubmit={onSubmit}
					className="flex flex-col gap-4 py-1">
					{/* Name Field */}
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">
							Name
						</label>
						<Input
							placeholder="e.g., Production, Client, Internal"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							autoFocus
							className="h-9 text-xs"
						/>
					</div>

					{/* Color Field */}
					<TagColorField color={color} onChange={setColor} />

					{/* Live Preview */}
					<div className="mt-1 flex items-center justify-between rounded-lg bg-muted/20 p-3">
						<span className="text-xs font-medium text-muted-foreground">
							Preview Badge:
						</span>
						<TagBadge name={name.trim() || 'Tag Name'} color={color} />
					</div>
				</form>

				<DialogFooter className="gap-2 border-t  pt-2 sm:gap-0">
					<Button
						type="button"
						variant="outline"
						onClick={() => setIsOpen(false)}
						className="h-9 text-xs font-semibold">
						Cancel
					</Button>
					<Button
						disabled={isSubmitting}
						form="hook-form-tag"
						type="submit"
						className="h-9 bg-primary px-5 text-xs font-bold text-primary-foreground hover:bg-primary/90">
						{isSubmitting ? 'Saving...' : tagId ? 'Update' : 'Create'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
};
