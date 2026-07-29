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
		<div className="flex items-center justify-between flex-wrap gap-2.5 px-3 py-2 bg-card border border-border rounded-xl font-sans">
			{/* Left: Live Status & Line Count */}
			<div className="flex items-center gap-2 flex-wrap">
				<div className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
					{isLive ? (
						<span className="relative flex h-2 w-2">
							<span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
							<span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
						</span>
					) : (
						<span className="h-2 w-2 rounded-full bg-zinc-500"></span>
					)}
					<span>{isLive ? 'Live Logs' : 'Log Output'}</span>
				</div>
				<span className="text-[11px] text-muted-foreground font-mono bg-muted/50 px-2 py-0.5 rounded border border-border/40">
					{searchQuery || selectedLevel !== 'ALL' || selectedStream !== 'ALL'
						? `${filteredCount} / ${totalLines}`
						: `${totalLines} ${totalLines === 1 ? 'line' : 'lines'}`}
				</span>

				{/* Stream Badges: STDOUT (Blue dot) / STDERR (Red dot) */}
				<div className="flex items-center gap-1 pl-1 border-l border-border/60">
					<button
						type="button"
						onClick={() => onStreamChange(selectedStream === 'STDOUT' ? 'ALL' : 'STDOUT')}
						className={`text-[10px] font-mono px-1.5 py-0.5 rounded border transition-colors flex items-center gap-1 cursor-pointer ${
							selectedStream === 'STDOUT'
								? 'bg-blue-500/20 text-blue-300 border-blue-500/40 font-bold'
								: 'bg-muted/30 text-muted-foreground border-border/40 hover:text-foreground'
						}`}
						title="Filter STDOUT"
					>
						<span className="w-1.5 h-1.5 rounded-full bg-blue-400" />
						STDOUT
					</button>
					<button
						type="button"
						onClick={() => onStreamChange(selectedStream === 'STDERR' ? 'ALL' : 'STDERR')}
						className={`text-[10px] font-mono px-1.5 py-0.5 rounded border transition-colors flex items-center gap-1 cursor-pointer ${
							selectedStream === 'STDERR'
								? 'bg-rose-500/20 text-rose-300 border-rose-500/40 font-bold'
								: 'bg-muted/30 text-muted-foreground border-border/40 hover:text-foreground'
						}`}
						title="Filter STDERR"
					>
						<span className="w-1.5 h-1.5 rounded-full bg-rose-400" />
						STDERR
					</button>
				</div>
			</div>

			{/* Right: Search, Level Filter, Options & Actions */}
			<div className="flex items-center gap-2 flex-wrap">
				{/* Search Input */}
				<div className="relative w-44 sm:w-52">
					<Search className="w-3.5 h-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
					<Input
						type="text"
						placeholder="Filter logs..."
						value={searchQuery}
						onChange={(e) => onSearchChange(e.target.value)}
						className="h-7 text-xs font-mono pl-8 pr-7 bg-muted/30 border-border focus:bg-card"
					/>
					{searchQuery && (
						<button
							type="button"
							onClick={() => onSearchChange('')}
							className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer"
						>
							<X className="w-3 h-3" />
						</button>
					)}
				</div>

				{/* Level Filter Dropdown */}
				<DropdownMenu>
					<DropdownMenuTrigger
						className={`h-7 text-xs font-mono border rounded-md gap-1 px-2.5 inline-flex items-center justify-center cursor-pointer transition-colors font-medium ${
							selectedLevel !== 'ALL'
								? 'bg-primary/10 text-primary border-primary/40'
								: 'border-border bg-card hover:bg-muted text-foreground'
						}`}
					>
						<Filter className="w-3 h-3" />
						{selectedLevel === 'ALL' ? 'Levels' : selectedLevel}
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-36 bg-card border-border font-mono text-xs">
						<DropdownMenuLabel className="text-[10px] text-muted-foreground uppercase">
							Filter Level
						</DropdownMenuLabel>
						<DropdownMenuSeparator />
						{LOG_LEVELS.map((level) => (
							<DropdownMenuCheckboxItem
								key={level}
								checked={selectedLevel === level}
								onCheckedChange={() => onLevelChange(level)}
								className="cursor-pointer"
							>
								{level}
							</DropdownMenuCheckboxItem>
						))}
					</DropdownMenuContent>
				</DropdownMenu>

				{/* Settings & Controls Dropdown */}
				<DropdownMenu>
					<DropdownMenuTrigger
						className="h-7 w-7 p-0 border border-border rounded-md inline-flex items-center justify-center bg-card hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer transition-colors"
						title="View Options"
					>
						<Settings2 className="w-3.5 h-3.5" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-48 bg-card border-border font-mono text-xs">
						<DropdownMenuLabel className="text-[10px] text-muted-foreground uppercase">
							Display Settings
						</DropdownMenuLabel>
						<DropdownMenuSeparator />
						<DropdownMenuCheckboxItem checked={isAutoScroll} onCheckedChange={onToggleAutoScroll}>
							Auto-scroll to bottom
						</DropdownMenuCheckboxItem>
						<DropdownMenuCheckboxItem checked={isWrapLines} onCheckedChange={onToggleWrapLines}>
							Wrap lines
						</DropdownMenuCheckboxItem>
						<DropdownMenuCheckboxItem checked={isCompact} onCheckedChange={onToggleCompact}>
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
						className="h-7 w-7 p-0 border-border hover:bg-muted"
					>
						<RefreshCw className="w-3.5 h-3.5 text-muted-foreground" />
					</Button>
				)}
				{onDownload && (
					<Button
						variant="outline"
						size="sm"
						onClick={onDownload}
						title="Download Log File"
						className="h-7 w-7 p-0 border-border hover:bg-muted"
					>
						<Download className="w-3.5 h-3.5 text-muted-foreground" />
					</Button>
				)}
			</div>
		</div>
	);
}
