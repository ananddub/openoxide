import {Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
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

interface DatabaseDeploymentsModalProps {
	activeLogId: number | null;
	liveLogs: string[];
	cancelingId: number | null;
	onCloseLogs: () => void;
	onCloseCancel: () => void;
	onConfirmCancel: () => Promise<void>;
}

export function DatabaseDeploymentsModal({
	activeLogId,
	liveLogs,
	cancelingId,
	onCloseLogs,
	onCloseCancel,
	onConfirmCancel,
}: DatabaseDeploymentsModalProps) {
	return (
		<>
			{/* Cancel Confirmation Dialog */}
			<AlertDialog open={cancelingId !== null} onOpenChange={(open) => !open && onCloseCancel()}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Database Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this database container deployment? This action will stop the running container setup.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={onCloseCancel}>Keep Running</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={onConfirmCancel}
						>
							Yes, Cancel Deployment
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>

			{/* Realtime Stream Logs Modal */}
			{activeLogId && (
				<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
					<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-4xl flex flex-col overflow-hidden max-h-[85vh]">
						<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
							<div className="flex items-center gap-2">
								<Terminal className="w-4 h-4 text-foreground" />
								<h3 className="text-xs font-bold text-foreground">Live Database Container Deployment Stream #{activeLogId}</h3>
							</div>
							<Button variant="ghost" size="sm" onClick={onCloseLogs} className="h-7 w-7 p-0 rounded-lg hover:bg-muted text-muted-foreground">
								<X className="w-4 h-4" />
							</Button>
						</div>
						<div className="p-4 overflow-y-auto">
							<DeploymentViewer
								logs={liveLogs}
								isLoading={liveLogs.length === 0}
								isLive={true}
								loadingText="Connecting to realtime SSE database deployment stream..."
								emptyText="No container deployment build log entries available."
							/>
						</div>
					</div>
				</div>
			)}
		</>
	);
}
