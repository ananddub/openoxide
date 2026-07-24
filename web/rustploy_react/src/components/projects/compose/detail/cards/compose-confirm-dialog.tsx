import {Button} from '#/components/ui/button';

export type ActionType = 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel' | 'stop';

interface ComposeConfirmDialogProps {
	confirmAction: ActionType | null;
	onClose: () => void;
	onConfirm: (action: ActionType) => Promise<void>;
}

export function ComposeConfirmDialog({confirmAction, onClose, onConfirm}: ComposeConfirmDialogProps) {
	if (!confirmAction) return null;

	const getActionTitle = (action: ActionType) => {
		if (action === 'deploy') return 'Deploy Compose Stack';
		if (action === 'reload') return 'Reload Compose Stack';
		if (action === 'rebuild') return 'Rebuild Compose Stack';
		if (action === 'start') return 'Start Compose Services';
		if (action === 'stop') return 'Stop Compose Services';
		return 'Cancel Active Build';
	};

	const getActionDesc = (action: ActionType) => {
		if (action === 'deploy') return 'Are you sure you want to deploy this compose stack?';
		if (action === 'reload') return 'Are you sure you want to reload this compose stack without rebuilding?';
		if (action === 'rebuild') return 'Are you sure you want to rebuild this compose stack?';
		if (action === 'start') return 'Are you sure you want to start all compose stack services?';
		if (action === 'stop') return 'Are you sure you want to stop all running compose stack containers?';
		return 'Are you sure you want to cancel the active deployment build?';
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl p-5 shadow-2xl max-w-md w-full flex flex-col gap-4">
				<div>
					<h3 className="text-sm font-bold text-foreground">{getActionTitle(confirmAction)}</h3>
					<p className="text-xs text-muted-foreground mt-1">{getActionDesc(confirmAction)}</p>
				</div>

				<div className="flex items-center justify-end gap-2 pt-2 border-t border-border">
					<Button variant="outline" size="sm" onClick={onClose} className="h-8 text-xs font-semibold">
						Cancel
					</Button>
					<Button
						variant={confirmAction === 'stop' || confirmAction === 'cancel' ? 'destructive' : 'default'}
						size="sm"
						onClick={() => onConfirm(confirmAction)}
						className="h-8 text-xs font-semibold"
					>
						Confirm Action
					</Button>
				</div>
			</div>
		</div>
	);
}
