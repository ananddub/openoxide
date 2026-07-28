import {RefreshCw, Play, Square, Box, Download, Hammer, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

interface ComposeLogsHeaderProps {
	logMode: 'container' | 'build';
	setLogMode: (mode: 'container' | 'build') => void;
	activeService: string;
	setSelectedContainer: (v: string) => void;
	availableServices: string[];
	isLive: boolean;
	setIsLive: (v: boolean) => void;
	timestamps: boolean;
	setTimestamps: (v: boolean) => void;
	lines: string;
	setLines: (v: string) => void;
	onRefresh: () => void;
	onDownload: () => void;
}

export function ComposeLogsHeader({
	logMode,
	setLogMode,
	activeService,
	setSelectedContainer,
	availableServices,
	isLive,
	setIsLive,
	timestamps,
	setTimestamps,
	lines,
	setLines,
	onRefresh,
	onDownload,
}: ComposeLogsHeaderProps) {
	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			<div className="flex items-center justify-between gap-4 flex-wrap">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Terminal className="w-4 h-4 text-primary" /> Compose Logs Console
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time terminal stream output for build assembly and container execution</p>
				</div>

				{/* Dual Log Mode Segmented Switch */}
				<div className="bg-muted p-1 rounded-lg flex items-center gap-1 border border-border/40">
					<button
						type="button"
						onClick={() => setLogMode('container')}
						className={`px-3 py-1.5 rounded-md text-xs font-bold transition-all flex items-center gap-1.5 ${
							logMode === 'container'
								? 'bg-card text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						<Terminal className="w-3.5 h-3.5" /> Container Logs
					</button>
					<button
						type="button"
						onClick={() => setLogMode('build')}
						className={`px-3 py-1.5 rounded-md text-xs font-bold transition-all flex items-center gap-1.5 ${
							logMode === 'build'
								? 'bg-card text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						<Hammer className="w-3.5 h-3.5" /> Build & Deploy Logs
					</button>
				</div>
			</div>

			{/* Filter Controls Action Bar */}
			<div className="flex items-center justify-between gap-3 flex-wrap pt-2 border-t border-border/40">
				<div className="flex items-center gap-3 flex-wrap">
					{logMode === 'container' && (
						<Select value={activeService} onValueChange={v => v && setSelectedContainer(v)}>
							<SelectTrigger className="h-8 text-xs font-semibold w-44 bg-muted/30 border-border">
								<Box className="w-3.5 h-3.5 mr-1.5 text-primary shrink-0" />
								<SelectValue placeholder="Select Container" />
							</SelectTrigger>
							<SelectContent>
								{availableServices.map((srv) => (
									<SelectItem key={srv} value={srv} className="text-xs font-mono">
										Service: {srv}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}

					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="h-8 text-xs font-semibold flex items-center gap-1.5"
					>
						{isLive ? <Square className="w-3 h-3 fill-current" /> : <Play className="w-3 h-3 fill-current" />}
						{isLive ? 'Live Stream' : 'Paused'}
					</Button>

					<label className="flex items-center gap-2 cursor-pointer text-xs font-semibold text-muted-foreground">
						<input
							type="checkbox"
							checked={timestamps}
							onChange={e => setTimestamps(e.target.checked)}
							className="accent-primary w-4 h-4 rounded"
						/>
						Timestamps
					</label>

					{logMode === 'container' && (
						<Select value={lines} onValueChange={v => v && setLines(v)}>
							<SelectTrigger className="h-8 text-xs font-semibold w-28 border-border">
								<SelectValue placeholder="Lines" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="100" className="text-xs">100 Lines</SelectItem>
								<SelectItem value="300" className="text-xs">300 Lines</SelectItem>
								<SelectItem value="500" className="text-xs">500 Lines</SelectItem>
								<SelectItem value="1000" className="text-xs">1000 Lines</SelectItem>
							</SelectContent>
						</Select>
					)}
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5"
					>
						<RefreshCw className="w-3.5 h-3.5" /> Refresh
					</Button>

					<Button
						variant="outline"
						size="sm"
						onClick={onDownload}
						className="border-border text-foreground hover:bg-muted font-semibold h-8 text-xs flex items-center gap-1.5"
					>
						<Download className="w-3.5 h-3.5" /> Download
					</Button>
				</div>
			</div>
		</section>
	);
}
