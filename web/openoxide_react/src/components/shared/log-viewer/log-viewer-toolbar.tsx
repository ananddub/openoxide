import {Search, X, RefreshCw, Download, Pause, Play} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import type {LogLevel, StreamType} from './types';

interface LogViewerToolbarProps {
	totalLines: number;
	filteredCount: number;
	searchQuery: string;
	onSearchChange: (query: string) => void;
	/* Retained for existing callers; the minimal bar no longer renders them. */
	selectedLevel?: LogLevel;
	onLevelChange?: (level: LogLevel) => void;
	selectedStream?: StreamType;
	onStreamChange?: (stream: StreamType) => void;
	isLive?: boolean;
	onToggleLive?: () => void;
	isAutoScroll?: boolean;
	onToggleAutoScroll?: () => void;
	isWrapLines?: boolean;
	onToggleWrapLines?: () => void;
	isCompact?: boolean;
	onToggleCompact?: () => void;
	onReload?: () => void;
	onDownload?: () => void;
}

/** Quiet, monochrome log bar: status + count on the left, one search box and
 * the stream/reload/download actions on the right. No per-line filters. */
export function LogViewerToolbar({
	totalLines,
	filteredCount,
	searchQuery,
	onSearchChange,
	isLive = true,
	onToggleLive,
	onReload,
	onDownload,
}: LogViewerToolbarProps) {
	return (
		<div className="flex flex-wrap items-center justify-between gap-2 rounded-lg border bg-card px-2.5 py-1.5 font-sans">
			{/* Left: status + line count */}
			<div className="flex min-w-0 items-center gap-2">
				{onToggleLive ? (
					<button
						type="button"
						onClick={onToggleLive}
						className="flex cursor-pointer items-center gap-1.5 rounded-md px-1.5 py-0.5 text-xs font-medium text-foreground transition-colors hover:bg-accent/40">
						{isLive ? (
							<Pause className="size-3 text-muted-foreground" />
						) : (
							<Play className="size-3 text-muted-foreground" />
						)}
						{isLive ? 'Live' : 'Paused'}
					</button>
				) : (
					<span className="flex items-center gap-1.5 text-xs font-medium text-foreground">
						<span
							className={cn(
								'size-1.5 rounded-full',
								isLive ? 'bg-foreground' : 'bg-muted-foreground/40',
							)}
						/>
						{isLive ? 'Live' : 'Paused'}
					</span>
				)}
				<span className="font-mono text-[11px] text-muted-foreground">
					{searchQuery
						? `${filteredCount} / ${totalLines}`
						: `${totalLines} ${totalLines === 1 ? 'line' : 'lines'}`}
				</span>
			</div>

			{/* Right: search + actions */}
			<div className="flex items-center gap-1.5">
				<div className="relative w-40 sm:w-56">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Search logs…"
						value={searchQuery}
						onChange={e => onSearchChange(e.target.value)}
						className="h-7 rounded-md bg-background pr-7 pl-8 text-xs"
					/>
					{searchQuery && (
						<button
							type="button"
							onClick={() => onSearchChange('')}
							className="absolute top-1/2 right-2 -translate-y-1/2 cursor-pointer text-muted-foreground hover:text-foreground">
							<X className="size-3" />
						</button>
					)}
				</div>

				{onReload && (
					<Button
						variant="ghost"
						size="icon"
						onClick={onReload}
						title="Reload"
						className="size-7 text-muted-foreground hover:text-foreground">
						<RefreshCw className="size-3.5" />
					</Button>
				)}
				{onDownload && (
					<Button
						variant="ghost"
						size="icon"
						onClick={onDownload}
						title="Download"
						className="size-7 text-muted-foreground hover:text-foreground">
						<Download className="size-3.5" />
					</Button>
				)}
			</div>
		</div>
	);
}
