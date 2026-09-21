import {Search, X} from 'lucide-react';
import {Input} from '#/components/ui/input';
import type {LogLevel, StreamType} from './types';
import {LogToolbarFilters} from './log-viewer-toolbar-filters';

interface LogViewerToolbarProps {
	totalLines: number;
	filteredCount: number;
	searchQuery: string;
	onSearchChange: (query: string) => void;
	selectedLevel: LogLevel;
	onLevelChange: (level: LogLevel) => void;
	selectedStream: StreamType;
	onStreamChange: (stream: StreamType) => void;
	isLive?: boolean;
	isAutoScroll: boolean;
	onToggleAutoScroll: () => void;
	isWrapLines: boolean;
	onToggleWrapLines: () => void;
	isCompact: boolean;
	onToggleCompact: () => void;
	onReload?: () => void;
	onDownload?: () => void;
}

export function LogViewerToolbar({
	totalLines,
	filteredCount,
	searchQuery,
	onSearchChange,
	selectedLevel,
	onLevelChange,
	selectedStream,
	onStreamChange,
	isLive = true,
	isAutoScroll,
	onToggleAutoScroll,
	isWrapLines,
	onToggleWrapLines,
	isCompact,
	onToggleCompact,
	onReload,
	onDownload,
}: LogViewerToolbarProps) {
	return (
		<div className="flex flex-wrap items-center justify-between gap-2.5 rounded-xl bg-card px-3 py-2 font-sans">
			{/* Left: Live Status & Line Count */}
			<div className="flex flex-wrap items-center gap-2">
				<div className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
					{isLive ? (
						<span className="relative flex h-2 w-2">
							<span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
							<span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-500"></span>
						</span>
					) : (
						<span className="h-2 w-2 rounded-full bg-zinc-500"></span>
					)}
					<span>{isLive ? 'Live Logs' : 'Log Output'}</span>
				</div>
				<span className="rounded bg-muted/50 px-2 py-0.5 font-mono text-[11px] text-muted-foreground">
					{searchQuery ||
					selectedLevel !== 'ALL' ||
					selectedStream !== 'ALL'
						? `${filteredCount} / ${totalLines}`
						: `${totalLines} ${totalLines === 1 ? 'line' : 'lines'}`}
				</span>

				{/* Stream Badges: STDOUT (Blue dot) / STDERR (Red dot) */}
				<div className="flex items-center gap-1 border-l  pl-1">
					<button
						type="button"
						onClick={() =>
							onStreamChange(
								selectedStream === 'STDOUT' ? 'ALL' : 'STDOUT',
							)
						}
						className={`flex cursor-pointer items-center gap-1 rounded  px-1.5 py-0.5 font-mono text-[10px] transition-colors ${
							selectedStream === 'STDOUT'
								? 'border-blue-500/40 bg-blue-500/20 font-bold text-blue-300'
								: ' bg-muted/30 text-muted-foreground hover:text-foreground'
						}`}
						title="Filter STDOUT">
						<span className="h-1.5 w-1.5 rounded-full bg-blue-400" />
						STDOUT
					</button>
					<button
						type="button"
						onClick={() =>
							onStreamChange(
								selectedStream === 'STDERR' ? 'ALL' : 'STDERR',
							)
						}
						className={`flex cursor-pointer items-center gap-1 rounded  px-1.5 py-0.5 font-mono text-[10px] transition-colors ${
							selectedStream === 'STDERR'
								? 'border-rose-500/40 bg-rose-500/20 font-bold text-rose-300'
								: ' bg-muted/30 text-muted-foreground hover:text-foreground'
						}`}
						title="Filter STDERR">
						<span className="h-1.5 w-1.5 rounded-full bg-rose-400" />
						STDERR
					</button>
				</div>
			</div>

			{/* Right: Search, Level Filter, Options & Actions */}
			<div className="flex flex-wrap items-center gap-2">
				{/* Search Input */}
				<div className="relative w-44 sm:w-52">
					<Search className="absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Filter logs..."
						value={searchQuery}
						onChange={e => onSearchChange(e.target.value)}
						className="border- h-7 bg-muted/30 pr-7 pl-8 font-mono text-xs focus:bg-card"
					/>
					{searchQuery && (
						<button
							type="button"
							onClick={() => onSearchChange('')}
							className="absolute top-1/2 right-2 -translate-y-1/2 cursor-pointer text-muted-foreground hover:text-foreground">
							<X className="h-3 w-3" />
						</button>
					)}
				</div>

				{/* Level filter, display options and reload/download actions */}
				<LogToolbarFilters
					selectedLevel={selectedLevel}
					onLevelChange={onLevelChange}
					isAutoScroll={isAutoScroll}
					onToggleAutoScroll={onToggleAutoScroll}
					isWrapLines={isWrapLines}
					onToggleWrapLines={onToggleWrapLines}
					isCompact={isCompact}
					onToggleCompact={onToggleCompact}
					onReload={onReload}
					onDownload={onDownload}
				/>
			</div>
		</div>
	);
}
