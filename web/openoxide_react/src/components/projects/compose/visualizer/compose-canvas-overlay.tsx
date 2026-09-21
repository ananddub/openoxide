// ─── Canvas overlays for the compose visualizer ───────────────────────────────

const legendDot = (color: string): React.CSSProperties => ({
	width: 8,
	height: 8,
	borderRadius: 2,
	background: color,
});

const LEGEND_ITEMS: {color: string; label: string}[] = [
	{color: '#30363d', label: 'Service (Click for Actions)'},
	{color: '#10b981', label: 'Domain (Click to Delete)'},
	{color: '#38bdf8', label: 'Volume Backup (Click to Delete)'},
	{color: '#c084fc', label: 'CronJob (Click to Delete)'},
];

export function CanvasLegend() {
	return (
		<div
			style={{
				position: 'absolute',
				top: 12,
				left: 16,
				zIndex: 5,
				display: 'flex',
				alignItems: 'center',
				gap: 12,
				padding: '6px 12px',
				borderRadius: 8,
				background: 'rgba(22,27,34,0.85)',
				backdropFilter: 'blur(8px)',
				border: '1px solid rgba(255,255,255,0.08)',
				fontSize: 10.5,
				color: '#8b949e',
			}}>
			{LEGEND_ITEMS.map(({color, label}) => (
				<div
					key={label}
					style={{display: 'flex', alignItems: 'center', gap: 5}}>
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
		<div
			style={{
				position: 'absolute',
				inset: 0,
				zIndex: 5,
				display: 'flex',
				flexDirection: 'column',
				alignItems: 'center',
				justifyContent: 'center',
				gap: 8,
				pointerEvents: 'none',
				padding: 24,
				textAlign: 'center',
			}}>
			<svg
				viewBox="0 0 24 24"
				style={{width: 32, height: 32, opacity: 0.25, fill: '#e6edf3'}}>
				<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
			</svg>
			<p
				style={{
					color: '#e6edf3',
					fontSize: 13,
					fontWeight: 600,
					margin: 0,
				}}>
				{isGitSource
					? `Configured File: ${gitBuildPath || 'docker-compose.yml'}`
					: 'No services found'}
			</p>
			<p
				style={{
					color: '#8b949e',
					fontSize: 11.5,
					maxWidth: 360,
					margin: 0,
					lineHeight: 1.5,
				}}>
				{isGitSource
					? 'Deploy/Build the project to fetch this file from Git and auto-wire your service dependency graph.'
					: 'Save a valid docker-compose.yml to visualize the dependency graph'}
			</p>
		</div>
	);
}
