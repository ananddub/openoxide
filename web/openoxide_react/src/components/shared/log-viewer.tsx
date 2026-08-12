import {Terminal, RefreshCw} from 'lucide-react';
import {LogViewerToolbar} from './log-viewer/log-viewer-toolbar';
import {LogLineItem} from './log-viewer/log-line-item';
import {LogDetailModal} from './log-viewer/log-detail-modal';
import {useLogViewer} from './log-viewer/use-log-viewer';

export interface LogViewerProps {
	logs?: string[] | null;
	isLoading?: boolean;
	loadingText?: string;
	emptyText?: string;
	heightClass?: string;
	onDownload?: () => void;
	onReload?: () => void;
	isLive?: boolean;
	showFilter?: boolean;
}

export function LogViewer({
	logs,
	isLoading = false,
	loadingText = 'Connecting to log stream...',
	emptyText = 'No log output received.',
	heightClass = 'h-[480px]',
	onDownload,
	onReload,
	isLive = true,
	showFilter = true,
}: LogViewerProps) {
	const {
		scrollRef,
		parsedEntries,
		filteredEntries,
		searchQuery,
		setSearchQuery,
		selectedLevel,
		setSelectedLevel,
		selectedStream,
		setSelectedStream,
		isAutoScroll,
		setIsAutoScroll,
		isWrapLines,
		setIsWrapLines,
		isCompact,
		setIsCompact,
		selectedDetailEntry,
		setSelectedDetailEntry,
	} = useLogViewer({logs, isLive, showFilter, isDeployment: false});

	return (
		<div className="flex flex-col gap-2 w-full font-mono">
			{/* Dozzle-Style Toolbar */}
			<LogViewerToolbar
				totalLines={parsedEntries.length}
				filteredCount={filteredEntries.length}
				searchQuery={searchQuery}
				onSearchChange={setSearchQuery}
				selectedLevel={selectedLevel}
				onLevelChange={setSelectedLevel}
				selectedStream={selectedStream}
				onStreamChange={setSelectedStream}
				isLive={isLive}
				isAutoScroll={isAutoScroll}
				onToggleAutoScroll={() => setIsAutoScroll(!isAutoScroll)}
				isWrapLines={isWrapLines}
				onToggleWrapLines={() => setIsWrapLines(!isWrapLines)}
				isCompact={isCompact}
				onToggleCompact={() => setIsCompact(!isCompact)}
				onReload={onReload}
				onDownload={onDownload}
			/>

			{/* Terminal Window Container using Shadcn UI bg-card & border-border */}
			<div
				ref={scrollRef}
				className={`bg-card text-card-foreground border border-border rounded-xl p-3 text-xs ${heightClass} overflow-y-auto shadow-sm flex flex-col gap-0.5 w-full [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:bg-muted-foreground/30 [&::-webkit-scrollbar-thumb]:rounded-full`}
				style={{
					fontFamily: `'JetBrains Mono', 'Fira Code', 'Cascadia Code', ui-monospace, SFMono-Regular, monospace`,
				}}
			>
				{isLoading && parsedEntries.length === 0 ? (
					<div className="flex items-center justify-center h-full text-muted-foreground gap-2 font-sans">
						<RefreshCw className="w-4 h-4 animate-spin text-primary" /> {loadingText}
					</div>
				) : filteredEntries.length === 0 ? (
					<div className="flex flex-col items-center justify-center h-full text-muted-foreground gap-1 font-sans">
						<Terminal className="w-6 h-6 opacity-30" />
						<p className="text-xs">
							{searchQuery || selectedLevel !== 'ALL' || selectedStream !== 'ALL'
								? 'No log lines match current filters'
								: emptyText}
						</p>
					</div>
				) : (
					filteredEntries.map((entry, idx) => (
						<LogLineItem
							key={entry.id || idx}
							entry={entry}
							index={idx}
							searchQuery={searchQuery}
							isWrapLines={isWrapLines}
							isCompact={isCompact}
							onShowDetails={setSelectedDetailEntry}
						/>
					))
				)}
			</div>

			{/* JSON/Detail Modal */}
			<LogDetailModal entry={selectedDetailEntry} onClose={() => setSelectedDetailEntry(null)} />
		</div>
	);
}
