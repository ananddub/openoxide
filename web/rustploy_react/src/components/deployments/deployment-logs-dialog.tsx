import {useMemo} from 'react';
import {Terminal, Copy, Check, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import type {Deployment} from '#/hooks/deployments/use-deployments';

interface DeploymentLogsDialogProps {
	selectedDeployment: Deployment | null;
	onClose: () => void;
	logs: string;
	isLogsLoading: boolean;
	copied: boolean;
	onCopyLogs: () => void;
}

// ANSI Escape Code Parser to React Elements
function parseAnsiToReact(text: string): React.ReactNode[] {
	if (!text) return [];

	const regex = /(\x1B\[[0-9;]*m)/g;
	const parts = text.split(regex);

	let currentClasses: string[] = [];
	let isBold = false;

	return parts.map((part, index) => {
		if (part.startsWith('\x1B[')) {
			const codes = part.replace('\x1B[', '').replace('m', '').split(';').map(Number);
			for (const code of codes) {
				if (code === 0) {
					currentClasses = [];
					isBold = false;
				} else if (code === 1) {
					isBold = true;
				} else if (code === 31 || code === 91) {
					currentClasses = ['text-red-400 font-semibold'];
				} else if (code === 32 || code === 92) {
					currentClasses = ['text-emerald-400 font-semibold'];
				} else if (code === 33 || code === 93) {
					currentClasses = ['text-amber-400 font-medium'];
				} else if (code === 34 || code === 94) {
					currentClasses = ['text-blue-400 font-medium'];
				} else if (code === 35 || code === 95) {
					currentClasses = ['text-fuchsia-400 font-medium'];
				} else if (code === 36 || code === 96) {
					currentClasses = ['text-cyan-400 font-medium'];
				} else if (code === 37) {
					currentClasses = ['text-zinc-100'];
				} else if (code === 90) {
					currentClasses = ['text-zinc-500'];
				}
			}
			return null;
		}

		if (!part) return null;

		let finalClass = currentClasses.join(' ');
		if (!finalClass) {
			const lowerPart = part.toLowerCase();
			if (lowerPart.includes('error') || lowerPart.includes('failed') || lowerPart.includes('exception')) {
				finalClass = 'text-red-400 font-semibold';
			} else if (lowerPart.includes('warning') || lowerPart.includes('warn:')) {
				finalClass = 'text-amber-400 font-medium';
			} else if (lowerPart.includes('success') || lowerPart.includes('successfully') || lowerPart.includes('done')) {
				finalClass = 'text-emerald-400 font-semibold';
			} else if (lowerPart.includes('info') || lowerPart.includes('building') || lowerPart.includes('compiling')) {
				finalClass = 'text-cyan-400';
			}
		}

		return (
			<span key={index} className={cn(finalClass, isBold && 'font-bold')}>
				{part}
			</span>
		);
	});
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

				<div className="grow overflow-hidden my-4 relative">
					{isLogsLoading ? (
						<div className="absolute inset-0 flex flex-col items-center justify-center gap-2 bg-[#08080a]/95 rounded-lg border border-zinc-900 z-10">
							<RefreshCw className="size-6 animate-spin text-primary" />
							<p className="text-xs font-medium text-zinc-400">
								Connecting to console stream...
							</p>
						</div>
					) : null}

					<div className="h-full w-full bg-[#050506] text-zinc-300 border border-zinc-900 rounded-lg font-mono text-[11px] leading-relaxed overflow-y-auto select-text p-4">
						{logLines.length > 0 ? (
							logLines.map((line, idx) => (
								<div key={idx} className="flex hover:bg-zinc-900/30 py-0.5 px-1 rounded transition-colors group">
									<span className="w-9 text-zinc-700 select-none text-right pr-3 font-mono opacity-50 group-hover:opacity-90 transition-opacity">
										{idx + 1}
									</span>
									<span className="flex-1 whitespace-pre-wrap break-all text-zinc-300">
										{parseAnsiToReact(line)}
									</span>
								</div>
							))
						) : (
							<div className="text-zinc-600 italic p-2 animate-pulse">
								Waiting for build console outputs...
							</div>
						)}
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
