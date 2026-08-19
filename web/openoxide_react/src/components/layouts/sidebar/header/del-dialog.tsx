import * as React from 'react';
import {Trash2, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {cn} from '#/api/utils';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';

type Props = {
	organizationId: string;
	onDelete: () => void;
	disabled?: boolean;
};

// Confirmation dialog before deleting an organization.
// Disabled when only one org remains to prevent leaving an empty state.
export function DeleteOrganization({
	organizationId,
	onDelete,
	disabled,
}: Props) {
	const [open, setOpen] = React.useState(false);
	const [isPending, setIsPending] = React.useState(false);
	const queryClient = useQueryClient();

	const deleteMutation = $api.useMutation('delete', '/organizations/{id}');

	const handleConfirm = async () => {
		if (disabled) return;
		setIsPending(true);
		try {
			await deleteMutation.mutateAsync({
				params: {
					path: {
						id: Number(organizationId),
					},
				},
			});

			toast.success('Organization deleted successfully');
			queryClient.invalidateQueries({queryKey: ['get', '/organizations']});
			onDelete();
			setOpen(false);
		} catch {
			toast.error('An unexpected error occurred');
		} finally {
			setIsPending(false);
		}
	};

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger
				render={
					<button
						className={cn(
							'shrink-0 rounded-md p-1.5 transition-colors',
							disabled
								? 'cursor-not-allowed text-muted-foreground/20'
								: 'hover:bg-destructive/10 hover:text-destructive',
						)}
						disabled={disabled}
					/>
				}>
				<Trash2 className="size-4" />
			</DialogTrigger>
			<DialogContent className="sm:max-w-106.25">
				<DialogHeader>
					<DialogTitle>Delete Organization</DialogTitle>
					<DialogDescription>
						Are you sure you want to delete this organization? This action
						cannot be undone.
					</DialogDescription>
				</DialogHeader>
				<div className="mt-4 flex justify-end">
					<Button
						variant="destructive"
						disabled={isPending}
						onClick={handleConfirm}
						className="flex gap-2">
						{isPending && <Loader2 className="size-4 animate-spin" />}
						Delete
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
