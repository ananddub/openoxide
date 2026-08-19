import {
	RefreshCw,
	Play,
	Square,
	Box,
	Download,
	Hammer,
	Terminal,
} from 'lucide-react';
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
		<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5 shadow-sm">
			<div className="flex flex-wrap items-center justify-between gap-4">
				<div className="flex flex-wrap items-center gap-3">
					<div>
						<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
							<Terminal className="h-4 w-4 text-primary" /> Compose Logs
							Console
						</h3>
						<p className="mt-1 text-xs text-muted-foreground">
							Real-time terminal stream output for build assembly and
							container execution
						</p>
					</div>

					{/* Premium Shadcn Service Select Dropdown */}
					{logMode === 'container' && (
						<Select
							value={activeService}
							onValueChange={val => {
								if (val) setSelectedContainer(val);
							}}>
							<SelectTrigger className="h-9 min-w-[170px] border-border/80 bg-muted/30 font-mono text-xs font-bold shadow-2xs hover:bg-muted/60">
								<Box className="mr-1 size-3.5 shrink-0 text-primary" />
								<SelectValue placeholder="Select Service" />
							</SelectTrigger>
							<SelectContent className="border-border bg-card">
								<div className="mb-1 border-b border-border/40 px-3 py-1.5 text-[9px] font-bold tracking-wider text-muted-foreground uppercase">
									Compose Services
								</div>
								{availableServices.map(srv => (
									<SelectItem
										key={srv}
										value={srv}
										className="font-mono text-xs font-semibold">
										Service: {srv}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}
				</div>

				{/* Dual Log Mode Segmented Switch */}
				<div className="flex items-center gap-1 rounded-lg border border-border/40 bg-muted p-1">
					<button
						type="button"
						onClick={() => setLogMode('container')}
						className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-bold transition-all ${
							logMode === 'container'
								? 'bg-card text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'
						}`}>
						<Terminal className="h-3.5 w-3.5" /> Container Logs
					</button>
					<button
						type="button"
						onClick={() => setLogMode('build')}
						className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-bold transition-all ${
							logMode === 'build'
								? 'bg-card text-foreground shadow-sm'
								: 'text-muted-foreground hover:text-foreground'
						}`}>
						<Hammer className="h-3.5 w-3.5" /> Build & Deploy Logs
					</button>
				</div>
			</div>

			{/* Filter Controls Action Bar */}
			<div className="flex flex-wrap items-center justify-between gap-3 border-t border-border/40 pt-2">
				<div className="flex flex-wrap items-center gap-3">
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="flex h-8 items-center gap-1.5 text-xs font-semibold">
						{isLive ? (
							<Square className="h-3 w-3 fill-current" />
						) : (
							<Play className="h-3 w-3 fill-current" />
						)}
						{isLive ? 'Live Stream' : 'Paused'}
					</Button>

					<label className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-muted-foreground">
						<input
							type="checkbox"
							checked={timestamps}
							onChange={e => setTimestamps(e.target.checked)}
							className="h-4 w-4 rounded accent-primary"
						/>
						Timestamps
					</label>

					{logMode === 'container' && (
						<Select
							value={lines}
							onValueChange={val => val && setLines(val)}>
							<SelectTrigger className="h-8 w-[120px] border border-border/60 bg-muted/30 text-xs font-semibold">
								<SelectValue placeholder="Lines" />
							</SelectTrigger>
							<SelectContent className="border-border bg-card">
								<SelectItem value="100" className="text-xs">
									100 Lines
								</SelectItem>
								<SelectItem value="300" className="text-xs">
									300 Lines
								</SelectItem>
								<SelectItem value="500" className="text-xs">
									500 Lines
								</SelectItem>
								<SelectItem value="1000" className="text-xs">
									1000 Lines
								</SelectItem>
								<SelectItem value="all" className="text-xs">
									All Lines
								</SelectItem>
							</SelectContent>
						</Select>
					)}
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<RefreshCw className="h-3.5 w-3.5" /> Refresh
					</Button>

					<Button
						variant="outline"
						size="sm"
						onClick={onDownload}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<Download className="h-3.5 w-3.5" /> Download
					</Button>
				</div>
			</div>
		</section>
	);
}
