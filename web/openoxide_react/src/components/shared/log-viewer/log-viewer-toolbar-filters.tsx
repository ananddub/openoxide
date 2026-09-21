import {Download, RefreshCw, Settings2, Filter} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuCheckboxItem,
	DropdownMenuContent,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {LogLevel} from './types';

const LOG_LEVELS: LogLevel[] = ['ALL', 'INFO', 'WARN', 'ERROR', 'DEBUG'];

interface LogToolbarFiltersProps {
	selectedLevel: LogLevel;
	onLevelChange: (level: LogLevel) => void;
	isAutoScroll: boolean;
	onToggleAutoScroll: () => void;
	isWrapLines: boolean;
	onToggleWrapLines: () => void;
	isCompact: boolean;
	onToggleCompact: () => void;
	onReload?: () => void;
	onDownload?: () => void;
}

/** Level filter + display options + reload/download actions for the toolbar. */
export function LogToolbarFilters({
	selectedLevel,
	onLevelChange,
	isAutoScroll,
	onToggleAutoScroll,
	isWrapLines,
	onToggleWrapLines,
	isCompact,
	onToggleCompact,
	onReload,
	onDownload,
}: LogToolbarFiltersProps) {
	return (
		<>
			{/* Level Filter Dropdown */}
			<DropdownMenu>
				<DropdownMenuTrigger
					className={`inline-flex h-7 cursor-pointer items-center justify-center gap-1 rounded-md  px-2.5 font-mono text-xs font-medium transition-colors ${
						selectedLevel !== 'ALL'
							? ' bg-primary/10 text-primary'
							: 'border- bg-card text-foreground hover:bg-muted'
					}`}>
					<Filter className="h-3 w-3" />
					{selectedLevel === 'ALL' ? 'Levels' : selectedLevel}
				</DropdownMenuTrigger>
				<DropdownMenuContent
					align="end"
					className="border- w-36 bg-card font-mono text-xs">
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
					className="inline-flex h-7 w-7 cursor-pointer items-center justify-center rounded-md bg-card p-0 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
					title="View Options">
					<Settings2 className="h-3.5 w-3.5" />
				</DropdownMenuTrigger>
				<DropdownMenuContent
					align="end"
					className="border- w-48 bg-card font-mono text-xs">
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
					className="h-7 w-7  p-0 hover:bg-muted">
					<RefreshCw className="h-3.5 w-3.5 text-muted-foreground" />
				</Button>
			)}
			{onDownload && (
				<Button
					variant="outline"
					size="sm"
					onClick={onDownload}
					title="Download Log File"
					className="h-7 w-7  p-0 hover:bg-muted">
					<Download className="h-3.5 w-3.5 text-muted-foreground" />
				</Button>
			)}
		</>
	);
}
