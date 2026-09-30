import type {ReactNode} from 'react';
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

/** Quiet AlertDialog confirm for the destructive swarm actions — same grammar as Delete SSH Key. */
export function SwarmDangerDialog({
	title,
	description,
	confirmLabel,
	busyLabel,
	busy,
	onCancel,
	onConfirm,
}: {
	title: string;
	description: ReactNode;
	confirmLabel: string;
	busyLabel: string;
	busy: boolean;
	onCancel: () => void;
	onConfirm: () => void;
}) {
	return (
		<AlertDialog open onOpenChange={open => !open && onCancel()}>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>{title}</AlertDialogTitle>
					<AlertDialogDescription>{description}</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel onClick={onCancel}>Cancel</AlertDialogCancel>
					<AlertDialogAction
						className="bg-danger text-white hover:bg-danger/90"
						disabled={busy}
						onClick={e => {
							e.preventDefault();
							onConfirm();
						}}>
						{busy ? busyLabel : confirmLabel}
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
