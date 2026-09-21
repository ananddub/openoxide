import {Terminal, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';

export interface StreamLogsModalProps {
	deploymentId: number | null;
	logs: string[];
	onClose: () => void;
	streamLabel?: string;
	loadingText?: string;
	emptyText?: string;
}

/** Fullscreen realtime build-log stream overlay for a deployment. */
export function StreamLogsModal({
	deploymentId,
	logs,
	onClose,
	streamLabel = 'Live Deployment Build Stream',
	loadingText = 'Connecting to realtime deployment stream...',
	emptyText = 'No deployment build log entries available.',
}: StreamLogsModalProps) {
	if (!deploymentId) return null;
	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="flex max-h-[85vh] w-full max-w-4xl flex-col overflow-hidden rounded-xl bg-card shadow-lg">
				<div className="border- flex items-center justify-between border-b bg-muted/30 p-4">
					<div className="flex items-center gap-2">
						<Terminal className="h-4 w-4 text-foreground" />
						<h3 className="text-xs font-bold text-foreground">
							{streamLabel} #{deploymentId}
						</h3>
					</div>
					<Button
						variant="ghost"
						size="sm"
						onClick={onClose}
						className="h-7 w-7 rounded-lg p-0 text-muted-foreground hover:bg-muted">
						<X className="h-4 w-4" />
					</Button>
				</div>
				<div className="overflow-y-auto p-4">
					<DeploymentViewer
						logs={logs}
						isLoading={logs.length === 0}
						isLive={true}
						loadingText={loadingText}
						emptyText={emptyText}
					/>
				</div>
			</div>
		</div>
	);
}
