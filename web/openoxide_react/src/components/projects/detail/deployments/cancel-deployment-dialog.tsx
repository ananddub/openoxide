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

export interface CancelDeploymentDialogProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onConfirm: () => void;
	title?: string;
	description?: string;
	cancelLabel?: string;
	confirmLabel?: string;
}

/** Confirmation dialog before aborting a running deployment build. */
export function CancelDeploymentDialog({
	open,
	onOpenChange,
	onConfirm,
	title = 'Cancel Deployment',
	description = 'Are you sure you want to cancel this deployment? This action will stop the running build process.',
	cancelLabel = 'Keep Running',
	confirmLabel = 'Yes, Cancel Deployment',
}: CancelDeploymentDialogProps) {
	return (
		<AlertDialog open={open} onOpenChange={onOpenChange}>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>{title}</AlertDialogTitle>
					<AlertDialogDescription>{description}</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel onClick={() => onOpenChange(false)}>
						{cancelLabel}
					</AlertDialogCancel>
					<AlertDialogAction
						className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
						onClick={onConfirm}>
						{confirmLabel}
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
