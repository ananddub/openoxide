import {useState, useMemo, useRef, useEffect} from 'react';
import {Search, X, Terminal, Download, RefreshCw} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {LogLine} from '#/components/shared/log-line';

interface LogViewerProps {
	logs?: string[] | null;
	isLoading?: boolean;
	loadingText?: string;
	emptyText?: string;
	heightClass?: string;
	onDownload?: () => void;
	onReload?: () => void;
	isLive?: boolean;
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
}: LogViewerProps) {
	const [searchQuery, setSearchQuery] = useState('');
	const scrollRef = useRef<HTMLDivElement>(null);

	const safeLogs = useMemo(() => (Array.isArray(logs) ? logs : []), [logs]);

	// Filter logs based on search query
	const filteredLogs = useMemo(() => {
		if (!searchQuery.trim()) return safeLogs;
		const query = searchQuery.toLowerCase();
		return safeLogs.filter(line => (line || '').toLowerCase().includes(query));
	}, [safeLogs, searchQuery]);

	// Auto-scroll when live
	useEffect(() => {
		if (isLive && scrollRef.current) {
			scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
		}
	}, [filteredLogs.length, isLive]);

	return (
		<div className="flex flex-col gap-2 w-full">
			{/* Sleek Terminal Header */}
			<div className="flex items-center justify-between flex-wrap gap-3 px-3 py-2 bg-card border border-border rounded-xl">
				{/* Live Status & Line Count */}
				<div className="flex items-center gap-2.5">
					<div className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
						<span className="relative flex h-2 w-2">
							<span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
							<span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
						</span>
						<span>Live Output</span>
					</div>
					<span className="text-[11px] text-muted-foreground font-mono bg-muted/50 px-2 py-0.5 rounded border border-border/40">
						{filteredLogs.length} {filteredLogs.length === 1 ? 'line' : 'lines'}
					</span>
				</div>

				{/* Search & Actions */}
				<div className="flex items-center gap-2">
					<div className="relative w-48">
						<Search className="w-3.5 h-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
						<Input
							type="text"
							placeholder="Filter logs..."
							value={searchQuery}
							onChange={e => setSearchQuery(e.target.value)}
							className="h-7 text-xs font-mono pl-8 pr-7 bg-muted/30 border-border focus:bg-card"
						/>
						{searchQuery && (
							<button
								onClick={() => setSearchQuery('')}
								className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								<X className="w-3 h-3" />
							</button>
						)}
					</div>

					{onReload && (
						<Button
							variant="outline"
							size="sm"
							onClick={onReload}
							title="Reload Logs"
							className="h-7 text-xs font-semibold border-border hover:bg-muted px-2 flex items-center gap-1"
						>
							<RefreshCw className="w-3.5 h-3.5" />
						</Button>
					)}
					{onDownload && (
						<Button
							variant="outline"
							size="sm"
							onClick={onDownload}
							title="Download Log File"
							className="h-7 text-xs font-semibold border-border hover:bg-muted px-2 flex items-center gap-1"
						>
							<Download className="w-3.5 h-3.5" />
						</Button>
					)}
				</div>
			</div>

			{/* Terminal Window */}
			<div
				ref={scrollRef}
				className={`bg-[#090d16] border border-border/80 rounded-xl p-3 font-mono text-xs ${heightClass} overflow-y-auto shadow-inner flex flex-col gap-0.5 w-full`}
			>
				{isLoading && safeLogs.length === 0 ? (
					<div className="flex items-center justify-center h-full text-zinc-500 gap-2">
						<RefreshCw className="w-4 h-4 animate-spin text-primary" /> {loadingText}
					</div>
				) : filteredLogs.length === 0 ? (
					<div className="flex flex-col items-center justify-center h-full text-zinc-500 gap-1">
						<Terminal className="w-6 h-6 opacity-30" />
						<p className="text-xs">
							{searchQuery ? `No log lines match '${searchQuery}'` : emptyText}
						</p>
					</div>
				) : (
					filteredLogs.map((line, idx) => (
						<LogLine key={idx} line={line} index={idx} />
					))
				)}
			</div>
		</div>
	);
}
