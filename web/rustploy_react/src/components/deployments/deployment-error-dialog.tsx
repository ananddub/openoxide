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
		<Dialog open={!!errorDetailDeployment} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl bg-card border-border">
				<DialogHeader className="border-b border-border/30 pb-3">
					<DialogTitle className="text-md font-bold text-rose-500 flex items-center gap-2">
						<AlertCircle className="size-5" />
						Deployment Failure Details
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Error trace registered for deployment #{errorDetailDeployment?.id} ({errorDetailDeployment?.title})
					</DialogDescription>
				</DialogHeader>

				<div className="my-4">
					<div className="bg-rose-500/5 border border-rose-500/20 text-rose-200 rounded-lg p-4 font-mono text-xs max-h-96 overflow-y-auto whitespace-pre-wrap select-text leading-relaxed">
						{errorDetailDeployment?.error_message || 'No failure trace registered.'}
					</div>
				</div>

				<div className="flex justify-end pt-2">
					<Button
						variant="outline"
						onClick={onClose}
						className="border-border hover:bg-muted font-semibold text-xs">
						Dismiss
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
