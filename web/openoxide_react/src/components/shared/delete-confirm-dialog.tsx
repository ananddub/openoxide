import React from 'react';
import {AlertCircle} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';

/**
 * Destructive "Delete Permanently" confirmation modal shared by provider
 * and resource list pages.
 */
export function DeleteConfirmDialog({
	open,
	onOpenChange,
	title,
	description,
	onConfirm,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	title: string;
	description: React.ReactNode;
	onConfirm: () => void;
}) {
	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="rounded-xl bg-card p-6 shadow-lg sm:max-w-sm">
				<DialogHeader className="space-y-2">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-destructive">
						<AlertCircle className="size-5" />
						{title}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{description}
					</DialogDescription>
				</DialogHeader>

				<DialogFooter className="pt-2">
					<Button
						variant="destructive"
						onClick={onConfirm}
						className="h-9 w-full text-xs font-semibold">
						Delete Permanently
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
