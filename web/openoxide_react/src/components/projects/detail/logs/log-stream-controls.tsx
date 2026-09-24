import {RefreshCw, Play, Square, Download} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

export interface LogLevelOption {
	value: string;
	label: string;
}

/** Shared live-log control bar: stream/pause toggle, timestamps checkbox, optional tail-lines select, refresh and download buttons. */
export function LogStreamControls({
	isLive,
	onToggleLive,
	timestamps,
	onToggleTimestamps,
	lines,
	onLinesChange,
	lineOptions,
	onRefresh,
	onDownload,
}: {
	isLive: boolean;
	onToggleLive: () => void;
	timestamps: boolean;
	onToggleTimestamps: (v: boolean) => void;
	lines?: string;
	onLinesChange?: (v: string) => void;
	lineOptions?: LogLevelOption[];
	onRefresh: () => void;
	onDownload: () => void;
}) {
	return (
		<div className="flex flex-wrap items-center gap-3">
			<Button
				variant={isLive ? 'default' : 'outline'}
				size="sm"
				onClick={onToggleLive}
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
					onChange={e => onToggleTimestamps(e.target.checked)}
					className="h-4 w-4 rounded accent-primary"
				/>
				Timestamps
			</label>

			{lines !== undefined && onLinesChange && (
				<Select value={lines} onValueChange={v => v && onLinesChange(v)}>
					<SelectTrigger className="h-8 w-[120px] bg-muted/30 text-xs font-semibold">
						<SelectValue placeholder="Lines" />
					</SelectTrigger>
					<SelectContent className="border- bg-card">
						{(lineOptions ?? []).map(o => (
							<SelectItem
								key={o.value}
								value={o.value}
								className="text-xs">
								{o.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			)}

			<Button
				variant="outline"
				size="sm"
				onClick={onRefresh}
				className="flex h-8 items-center gap-1.5  text-xs font-semibold text-foreground hover:bg-muted">
				<RefreshCw className="h-3.5 w-3.5" /> Refresh
			</Button>

			<Button
				variant="outline"
				size="sm"
				onClick={onDownload}
				className="flex h-8 items-center gap-1.5  text-xs font-semibold text-foreground hover:bg-muted">
				<Download className="h-3.5 w-3.5" /> Download
			</Button>
		</div>
	);
}
