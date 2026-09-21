import {Box, Hammer, Terminal} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	LogStreamControls,
	type LogLevelOption,
} from '#/components/projects/detail/logs/log-stream-controls';

const LINE_OPTIONS: LogLevelOption[] = [
	{value: '100', label: '100 Lines'},
	{value: '300', label: '300 Lines'},
	{value: '500', label: '500 Lines'},
	{value: '1000', label: '1000 Lines'},
	{value: 'all', label: 'All Lines'},
];

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
		<section className="flex flex-col gap-4 rounded-xl bg-card p-5 shadow-sm">
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

					{logMode === 'container' && (
						<Select
							value={activeService}
							onValueChange={val => {
								if (val) setSelectedContainer(val);
							}}>
							<SelectTrigger className="h-9 min-w-[170px]  bg-muted/30 font-mono text-xs font-bold shadow-2xs hover:bg-muted/60">
								<Box className="mr-1 size-3.5 shrink-0 text-primary" />
								<SelectValue placeholder="Select Service" />
							</SelectTrigger>
							<SelectContent className="border- bg-card">
								<div className="mb-1 border-b  px-3 py-1.5 text-[9px] font-bold tracking-wider text-muted-foreground uppercase">
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
				<div className="flex items-center gap-1 rounded-lg bg-muted p-1">
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
			<div className="border-t  pt-2">
				<LogStreamControls
					isLive={isLive}
					onToggleLive={() => setIsLive(!isLive)}
					timestamps={timestamps}
					onToggleTimestamps={setTimestamps}
					lines={logMode === 'container' ? lines : undefined}
					onLinesChange={logMode === 'container' ? setLines : undefined}
					lineOptions={LINE_OPTIONS}
					onRefresh={onRefresh}
					onDownload={onDownload}
				/>
			</div>
		</section>
	);
}
