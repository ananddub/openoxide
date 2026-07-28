import {useMemo} from 'react';
import {Terminal, Copy, Check} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {DeploymentViewer} from '#/components/shared/deployment-viewer';
import type {Deployment} from '#/hooks/deployments/use-deployments';

interface DeploymentLogsDialogProps {
	selectedDeployment: Deployment | null;
	onClose: () => void;
	logs: string;
	isLogsLoading: boolean;
	copied: boolean;
	onCopyLogs: () => void;
}

export function DeploymentLogsDialog({
	selectedDeployment,
	onClose,
	logs,
	isLogsLoading,
	copied,
	onCopyLogs,
}: DeploymentLogsDialogProps) {
	const logLines = useMemo(() => {
		if (!logs) return [];
		return logs.split('\n');
	}, [logs]);

	const isRunning = selectedDeployment?.status?.toUpperCase() === 'RUNNING';

	return (
		<Dialog open={!!selectedDeployment} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-4xl bg-[#09090b] border-zinc-800/80 h-[650px] flex flex-col justify-between overflow-hidden shadow-2xl shadow-primary/5">
				<DialogHeader className="border-b border-zinc-800/50 pb-4">
					<div className="flex items-center justify-between pr-6">
						<div>
							<DialogTitle className="text-md font-semibold flex items-center gap-2.5 text-zinc-100">
								<Terminal className="size-4.5 text-primary" />
								<span>Deployment Console #{selectedDeployment?.id}</span>
								{isRunning && (
									<span className="flex items-center gap-1.5 ml-2 bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 rounded-full text-[10px] text-emerald-400 font-bold tracking-wider uppercase animate-pulse">
										<span className="size-1.5 rounded-full bg-emerald-400" />
										Streaming
									</span>
								)}
							</DialogTitle>
							<DialogDescription className="text-xs text-zinc-400 mt-1 truncate max-w-xl">
								{selectedDeployment?.title} — {selectedDeployment?.description}
							</DialogDescription>
						</div>

						<div className="flex items-center gap-2">
							<Button
								variant="outline"
								size="sm"
								onClick={onCopyLogs}
								className="border-zinc-800 bg-zinc-900/50 hover:bg-zinc-800 text-zinc-400 hover:text-zinc-200 h-8 px-2.5 text-xs font-semibold flex items-center gap-1.5">
								{copied ? (
									<>
										<Check className="size-3.5 text-emerald-500" />
										Copied
									</>
								) : (
									<>
										<Copy className="size-3.5" />
										Copy Logs
									</>
								)}
							</Button>
						</div>
					</div>
				</DialogHeader>

				<div className="grow overflow-y-auto my-2">
					<DeploymentViewer
						logs={logLines}
						isLoading={isLogsLoading}
						isLive={isRunning}
						isDeployment={true}
						heightClass="h-[480px]"
						loadingText="Connecting to console stream..."
						emptyText="Waiting for build console outputs..."
					/>
				</div>
			</DialogContent>
		</Dialog>
	);
}
