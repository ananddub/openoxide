import {AlertCircle} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import type {Deployment} from '#/hooks/deployments/use-deployments';

interface DeploymentErrorDialogProps {
	errorDetailDeployment: Deployment | null;
	onClose: () => void;
}

export function DeploymentErrorDialog({
	errorDetailDeployment,
	onClose,
}: DeploymentErrorDialogProps) {
	return (
		<Dialog
			open={!!errorDetailDeployment}
			onOpenChange={open => !open && onClose()}>
			<DialogContent className="border-border bg-card sm:max-w-xl">
				<DialogHeader className="border-b border-border/30 pb-3">
					<DialogTitle className="text-md flex items-center gap-2 font-bold text-rose-500">
						<AlertCircle className="size-5" />
						Deployment Failure Details
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Error trace registered for deployment #
						{errorDetailDeployment?.id} ({errorDetailDeployment?.title})
					</DialogDescription>
				</DialogHeader>

				<div className="my-4">
					<div className="max-h-96 overflow-y-auto rounded-lg border border-rose-500/20 bg-rose-500/5 p-4 font-mono text-xs leading-relaxed whitespace-pre-wrap text-rose-200 select-text">
						{errorDetailDeployment?.error_message ||
							'No failure trace registered.'}
					</div>
				</div>

				<div className="flex justify-end pt-2">
					<Button
						variant="outline"
						onClick={onClose}
						className="border-border text-xs font-semibold hover:bg-muted">
						Dismiss
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
