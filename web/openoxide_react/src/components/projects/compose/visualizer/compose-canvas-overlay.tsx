// ─── Canvas overlays for the compose visualizer ───────────────────────────────

import {cn} from '#/api/utils';

const legendDot = (color: string): React.CSSProperties => ({
	width: 8,
	height: 8,
	borderRadius: 2,
	background: color,
});

const LEGEND_ITEMS: {color: string; label: string}[] = [
	{color: 'var(--muted-foreground)', label: 'Service (Click for Actions)'},
	{color: '#10b981', label: 'Domain (Click to Delete)'},
	{color: '#38bdf8', label: 'Volume Backup (Click to Delete)'},
	{color: '#c084fc', label: 'Schedule (Click to Delete)'},
];

export function CanvasLegend() {
	return (
		<div className="absolute top-3 left-4 z-5 flex items-center gap-3 rounded-lg border border-border bg-popover/85 px-3 py-1.5 text-[10.5px] text-muted-foreground backdrop-blur-[8px]">
			{LEGEND_ITEMS.map(({color, label}) => (
				<div key={label} className="flex items-center gap-[5px]">
					<span style={legendDot(color)} /> {label}
				</div>
			))}
		</div>
	);
}

export function CanvasEmptyState({
	isGitSource,
	gitBuildPath,
}: {
	isGitSource?: boolean;
	gitBuildPath?: string;
}) {
	return (
		<div className="pointer-events-none absolute inset-0 z-5 flex select-none flex-col items-center justify-center gap-2 p-6 text-center">
			<svg
				viewBox="0 0 24 24"
				className={cn(
					'size-8 opacity-25',
					'fill-foreground',
				)}>
				<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
			</svg>
			<p className="m-0 text-[13px] font-semibold text-foreground">
				{isGitSource
					? `Configured File: ${gitBuildPath || 'docker-compose.yml'}`
					: 'No services found'}
			</p>
			<p className="m-0 max-w-[360px] text-[11.5px] leading-normal text-muted-foreground">
				{isGitSource
					? 'Deploy/Build the project to fetch this file from Git and auto-wire your service dependency graph.'
					: 'Save a valid docker-compose.yml to visualize the dependency graph'}
			</p>
		</div>
	);
}
