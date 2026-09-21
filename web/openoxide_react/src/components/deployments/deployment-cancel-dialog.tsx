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

/** Confirmation before terminating an in-flight build task. */
export function DeploymentCancelDialog({
	cancelingId,
	onClose,
	onConfirm,
}: {
	cancelingId: number | null;
	onClose: () => void;
	onConfirm: () => Promise<void>;
}) {
	return (
		<AlertDialog
			open={cancelingId !== null}
			onOpenChange={open => !open && onClose()}>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>Cancel Deployment</AlertDialogTitle>
					<AlertDialogDescription>
						Are you sure you want to cancel this deployment build? This
						action will terminate the build task.
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel onClick={onClose}>
						Keep Running
					</AlertDialogCancel>
					<AlertDialogAction
						className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
						onClick={() => void onConfirm()}>
						Yes, Cancel Build
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
