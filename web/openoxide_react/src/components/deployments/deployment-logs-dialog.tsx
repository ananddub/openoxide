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

	const isRunning =
		selectedDeployment?.status?.toUpperCase() === 'RUNNING';

	return (
		<Dialog
			open={!!selectedDeployment}
			onOpenChange={open => !open && onClose()}>
			<DialogContent className="flex h-[650px] flex-col justify-between overflow-hidden border-zinc-800/80 bg-[#09090b] shadow-2xl shadow-primary/5 sm:max-w-4xl">
				<DialogHeader className="border-b border-zinc-800/50 pb-4">
					<div className="flex items-center justify-between pr-6">
						<div>
							<DialogTitle className="text-md flex items-center gap-2.5 font-semibold text-zinc-100">
								<Terminal className="size-4.5 text-primary" />
								<span>Deployment Console #{selectedDeployment?.id}</span>
								{isRunning && (
									<span className="ml-2 flex animate-pulse items-center gap-1.5 rounded-full border border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 text-[10px] font-bold tracking-wider text-emerald-400 uppercase">
										<span className="size-1.5 rounded-full bg-emerald-400" />
										Streaming
									</span>
								)}
							</DialogTitle>
							<DialogDescription className="mt-1 max-w-xl truncate text-xs text-zinc-400">
								{selectedDeployment?.title} —{' '}
								{selectedDeployment?.description}
							</DialogDescription>
						</div>

						<div className="flex items-center gap-2">
							<Button
								variant="outline"
								size="sm"
								onClick={onCopyLogs}
								className="flex h-8 items-center gap-1.5 border-zinc-800 bg-zinc-900/50 px-2.5 text-xs font-semibold text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200">
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

				<div className="my-2 grow overflow-y-auto">
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
