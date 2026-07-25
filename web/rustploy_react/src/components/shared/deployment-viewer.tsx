import React, {useMemo, useRef, useEffect} from 'react';
import {Terminal, Download, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {LogLine} from '#/components/shared/log-line';

interface DeploymentViewerProps {
	logs?: string[] | null;
	isLoading?: boolean;
	loadingText?: string;
	emptyText?: string;
	heightClass?: string;
	onDownload?: () => void;
	onReload?: () => void;
	isLive?: boolean;
	isDeployment?: boolean;
}

const STAGE_MARKERS = ['QUEUED', 'PREPARING', 'SOURCE_READY', 'BUILDING', 'DEPLOYING', 'ROUTING', 'HEALTH_CHECK', 'DEPLOYED'];

export function DeploymentViewer({
	logs,
	isLoading = false,
	loadingText = 'Connecting to deployment log stream...',
	emptyText = 'No deployment logs received.',
	heightClass = 'h-[480px]',
	onDownload,
	onReload,
	isLive = true,
	isDeployment = false,
}: DeploymentViewerProps) {
	const scrollRef = useRef<HTMLDivElement>(null);
	const safeLogs = useMemo(() => (Array.isArray(logs) ? logs : []), [logs]);

	// Map lines to active deployment stage ONLY if isDeployment is true
	const parsedLines = useMemo(() => {
		if (!isDeployment) {
			return safeLogs.map((line) => ({line, stage: undefined}));
		}

		let currentStage: string | undefined = undefined;
		return safeLogs.map((line) => {
			const upper = (line || '').toUpperCase();
			for (const marker of STAGE_MARKERS) {
				if (upper.includes(`[${marker}]`) || upper.trim() === marker) {
					currentStage = marker;
					break;
				}
			}
			return {line, stage: currentStage};
		});
	}, [safeLogs, isDeployment]);

	// Auto-scroll terminal when live
	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [safeLogs.length, isLive]);

	return (
		<div className="flex flex-col gap-2 w-full">
			{/* Terminal Action Header */}
			<div className="flex items-center justify-between flex-wrap gap-3 px-3 py-2 bg-card border border-border rounded-xl">
				<div className="flex items-center gap-2.5">
					<div className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
						<span className="relative flex h-2 w-2">
							<span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
							<span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
						</span>
						<span>{isDeployment ? 'Deployment Stream' : 'Container Terminal Output'}</span>
					</div>
					<span className="text-[11px] text-muted-foreground font-mono bg-muted/50 px-2 py-0.5 rounded border border-border/40">
						{safeLogs.length} {safeLogs.length === 1 ? 'line' : 'lines'}
					</span>
				</div>

				<div className="flex items-center gap-2">
					{onReload && (
						<Button
							variant="outline"
							size="sm"
							onClick={onReload}
							title="Reload Logs"
							className="h-8 text-xs font-semibold border-border hover:bg-muted px-2.5 flex items-center gap-1"
						>
							<RefreshCw className="w-3.5 h-3.5" /> Refresh
						</Button>
					)}
					{onDownload && (
						<Button
							variant="outline"
							size="sm"
							onClick={onDownload}
							title="Download Log File"
							className="h-8 text-xs font-semibold border-border hover:bg-muted px-2.5 flex items-center gap-1"
						>
							<Download className="w-3.5 h-3.5" /> Download
						</Button>
					)}
				</div>
			</div>

			{/* Pristine Full-Width Dark Terminal Window */}
			<div
				ref={scrollRef}
				className={`bg-[#090d16] border border-border/80 rounded-xl p-3 font-mono text-xs ${heightClass} overflow-y-auto shadow-inner flex flex-col gap-0.5 w-full`}
			>
				{isLoading && safeLogs.length === 0 ? (
					<div className="flex items-center justify-center h-full text-zinc-500 gap-2">
						<RefreshCw className="w-4 h-4 animate-spin text-primary" /> {loadingText}
					</div>
				) : safeLogs.length === 0 ? (
					<div className="flex flex-col items-center justify-center h-full text-zinc-500 gap-1">
						<Terminal className="w-6 h-6 opacity-30" />
						<p className="text-xs">{emptyText}</p>
					</div>
				) : (
					parsedLines.map((item, idx) => (
						<LogLine key={idx} line={item.line} index={idx} stage={item.stage} />
					))
				)}
			</div>
		</div>
	);
}
