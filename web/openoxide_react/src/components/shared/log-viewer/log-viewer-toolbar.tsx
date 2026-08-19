import {
	Search,
	X,
	Download,
	RefreshCw,
	Settings2,
	Filter,
} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuCheckboxItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {LogLevel, StreamType} from './types';

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

const LOG_LEVELS: LogLevel[] = ['ALL', 'INFO', 'WARN', 'ERROR', 'DEBUG'];

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
		<div className="flex flex-wrap items-center justify-between gap-2.5 rounded-xl border border-border bg-card px-3 py-2 font-sans">
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
				<span className="rounded border border-border/40 bg-muted/50 px-2 py-0.5 font-mono text-[11px] text-muted-foreground">
					{searchQuery ||
					selectedLevel !== 'ALL' ||
					selectedStream !== 'ALL'
						? `${filteredCount} / ${totalLines}`
						: `${totalLines} ${totalLines === 1 ? 'line' : 'lines'}`}
				</span>

				{/* Stream Badges: STDOUT (Blue dot) / STDERR (Red dot) */}
				<div className="flex items-center gap-1 border-l border-border/60 pl-1">
					<button
						type="button"
						onClick={() =>
							onStreamChange(
								selectedStream === 'STDOUT' ? 'ALL' : 'STDOUT',
							)
						}
						className={`flex cursor-pointer items-center gap-1 rounded border px-1.5 py-0.5 font-mono text-[10px] transition-colors ${
							selectedStream === 'STDOUT'
								? 'border-blue-500/40 bg-blue-500/20 font-bold text-blue-300'
								: 'border-border/40 bg-muted/30 text-muted-foreground hover:text-foreground'
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
						className={`flex cursor-pointer items-center gap-1 rounded border px-1.5 py-0.5 font-mono text-[10px] transition-colors ${
							selectedStream === 'STDERR'
								? 'border-rose-500/40 bg-rose-500/20 font-bold text-rose-300'
								: 'border-border/40 bg-muted/30 text-muted-foreground hover:text-foreground'
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
						className="h-7 border-border bg-muted/30 pr-7 pl-8 font-mono text-xs focus:bg-card"
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

				{/* Level Filter Dropdown */}
				<DropdownMenu>
					<DropdownMenuTrigger
						className={`inline-flex h-7 cursor-pointer items-center justify-center gap-1 rounded-md border px-2.5 font-mono text-xs font-medium transition-colors ${
							selectedLevel !== 'ALL'
								? 'border-primary/40 bg-primary/10 text-primary'
								: 'border-border bg-card text-foreground hover:bg-muted'
						}`}>
						<Filter className="h-3 w-3" />
						{selectedLevel === 'ALL' ? 'Levels' : selectedLevel}
					</DropdownMenuTrigger>
					<DropdownMenuContent
						align="end"
						className="w-36 border-border bg-card font-mono text-xs">
						<DropdownMenuLabel className="text-[10px] text-muted-foreground uppercase">
							Filter Level
						</DropdownMenuLabel>
						<DropdownMenuSeparator />
						{LOG_LEVELS.map(level => (
							<DropdownMenuCheckboxItem
								key={level}
								checked={selectedLevel === level}
								onCheckedChange={() => onLevelChange(level)}
								className="cursor-pointer">
								{level}
							</DropdownMenuCheckboxItem>
						))}
					</DropdownMenuContent>
				</DropdownMenu>

				{/* Settings & Controls Dropdown */}
				<DropdownMenu>
					<DropdownMenuTrigger
						className="inline-flex h-7 w-7 cursor-pointer items-center justify-center rounded-md border border-border bg-card p-0 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
						title="View Options">
						<Settings2 className="h-3.5 w-3.5" />
					</DropdownMenuTrigger>
					<DropdownMenuContent
						align="end"
						className="w-48 border-border bg-card font-mono text-xs">
						<DropdownMenuLabel className="text-[10px] text-muted-foreground uppercase">
							Display Settings
						</DropdownMenuLabel>
						<DropdownMenuSeparator />
						<DropdownMenuCheckboxItem
							checked={isAutoScroll}
							onCheckedChange={onToggleAutoScroll}>
							Auto-scroll to bottom
						</DropdownMenuCheckboxItem>
						<DropdownMenuCheckboxItem
							checked={isWrapLines}
							onCheckedChange={onToggleWrapLines}>
							Wrap lines
						</DropdownMenuCheckboxItem>
						<DropdownMenuCheckboxItem
							checked={isCompact}
							onCheckedChange={onToggleCompact}>
							Compact line height
						</DropdownMenuCheckboxItem>
					</DropdownMenuContent>
				</DropdownMenu>

				{/* Reload & Download Buttons */}
				{onReload && (
					<Button
						variant="outline"
						size="sm"
						onClick={onReload}
						title="Reload Logs"
						className="h-7 w-7 border-border p-0 hover:bg-muted">
						<RefreshCw className="h-3.5 w-3.5 text-muted-foreground" />
					</Button>
				)}
				{onDownload && (
					<Button
						variant="outline"
						size="sm"
						onClick={onDownload}
						title="Download Log File"
						className="h-7 w-7 border-border p-0 hover:bg-muted">
						<Download className="h-3.5 w-3.5 text-muted-foreground" />
					</Button>
				)}
			</div>
		</div>
	);
}
