import {useState} from 'react';
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
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useTagDelete} from '#/api/live';
import type {TagItem} from '#/components/tags/tag-edit-dialog';

/** Confirmation dialog for deleting a tag. */
export function TagDeleteDialog({
	tag,
	open,
	onOpenChange,
}: {
	tag: TagItem | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}) {
	const [isDeleting, setIsDeleting] = useState(false);
	const deleteMutation = useTagDelete();

	const confirmDeleteTag = async () => {
		if (!tag) return;
		setIsDeleting(true);
		try {
			await deleteMutation.mutateAsync(tag.id);
			toast.success('Tag removed');
			onOpenChange(false);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setIsDeleting(false);
		}
	};

	return (
		<AlertDialog open={open} onOpenChange={onOpenChange}>
			<AlertDialogContent className="rounded-xl p-5 sm:max-w-sm">
				<AlertDialogHeader>
					<AlertDialogTitle className="text-sm font-semibold">
						Delete Tag
					</AlertDialogTitle>
					<AlertDialogDescription className="text-xs">
						Remove tag "{tag?.name}"?
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter className="pt-2">
					<AlertDialogCancel disabled={isDeleting} className="h-8 text-xs">
						Cancel
					</AlertDialogCancel>
					<AlertDialogAction
						onClick={confirmDeleteTag}
						disabled={isDeleting}
						className="text-destructive-foreground h-8 bg-destructive text-xs hover:bg-destructive/90">
						{isDeleting ? 'Deleting...' : 'Delete'}
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
