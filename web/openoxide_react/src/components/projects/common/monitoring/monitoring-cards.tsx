import {Cpu, HardDrive, Network, Layers, Disc, Database} from 'lucide-react';

interface MonitoringCardsProps {
	metrics: any;
	history?: Array<{time: string; cpu: number; mem: number; pids?: number}>;
}

function MetricAreaChart({
	title,
	icon: Icon,
	color,
	gradientId,
	data = [],
	dataKey,
	unit = '%',
	maxVal = 100,
}: {
	title: string;
	icon: any;
	color: string;
	gradientId: string;
	data?: Array<{time: string; [key: string]: any}>;
	dataKey: string;
	unit?: string;
	maxVal?: number;
}) {
	const points = data.map(d => Number(d[dataKey]) || 0);
	const latest = points.length > 0 ? points[points.length - 1] : 0;
	
	const width = 500;
	const height = 130;
	const padding = 15;

	const chartWidth = width - padding * 2;
	const chartHeight = height - padding * 2;

	const coords = points.map((val, idx) => {
		const x = padding + (idx / Math.max(points.length - 1, 1)) * chartWidth;
		const normalizedVal = Math.min(Math.max(val, 0), maxVal);
		const y = height - padding - (normalizedVal / maxVal) * chartHeight;
		return {x, y, val};
	});

	const linePath = coords.map((c, i) => `${i === 0 ? 'M' : 'L'} ${c.x} ${c.y}`).join(' ');
	const areaPath = coords.length > 0 
		? `${linePath} L ${coords[coords.length - 1].x} ${height - padding} L ${coords[0].x} ${height - padding} Z`
		: '';

	return (
		<div className="bg-card border border-border rounded-xl p-4 shadow-xs flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-2">
					<Icon className={`size-4 ${color}`} />
					<span className="text-xs font-bold text-foreground">{title}</span>
				</div>
				<span className={`text-sm font-mono font-extrabold ${color}`}>
					{latest.toFixed(1)}{unit}
				</span>
			</div>

			<div className="w-full h-32 relative">
				{coords.length < 2 ? (
					<div className="w-full h-full flex items-center justify-center text-xs text-muted-foreground font-mono border border-dashed border-border/60 rounded-lg">
						Collecting telemetry points...
					</div>
				) : (
					<svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full overflow-visible">
						<defs>
							<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
								<stop offset="0%" stopColor="currentColor" stopOpacity="0.3" />
								<stop offset="100%" stopColor="currentColor" stopOpacity="0.0" />
							</linearGradient>
						</defs>

						{/* Horizontal Gridlines */}
						{[0.25, 0.5, 0.75].map((ratio) => (
							<line
								key={ratio}
								x1={padding}
								y1={height - padding - ratio * chartHeight}
								x2={width - padding}
								y2={height - padding - ratio * chartHeight}
								stroke="currentColor"
								strokeDasharray="3 3"
								className="text-border/40"
							/>
						))}

						{/* Fill Area */}
						<path d={areaPath} fill={`url(#${gradientId})`} className={color} />

						{/* Line */}
						<path d={linePath} fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" className={color} />

						{/* Live Pulse Dot */}
						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current animate-ping`}
							/>
						)}
						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current`}
							/>
						)}
					</svg>
				)}
			</div>

			{/* Time Range */}
			<div className="flex justify-between text-[10px] text-muted-foreground font-mono px-1">
				<span>{data[0]?.time || 'Start'}</span>
				<span>{data[data.length - 1]?.time || 'Live'}</span>
			</div>
		</div>
	);
}

interface MonitoringCardsProps {
	metrics: any;
	history?: Array<{time: string; cpu: number; mem: number; disk: number; net: number}>;
}

export function MonitoringCards({metrics, history = []}: MonitoringCardsProps) {
	return (
		<div className="flex flex-col gap-6">
			{/* Real-time Telemetry Graphs (2x2 Grid for CPU, RAM, Disk, Network) */}
			<div className="grid grid-cols-1 md:grid-cols-2 gap-5">
				{/* 1. CPU Usage Graph */}
				<MetricAreaChart
					title="CPU Utilization Graph"
					icon={Cpu}
					color="text-primary"
					gradientId="cpu-grad"
					data={history}
					dataKey="cpu"
					unit="%"
				/>

				{/* 2. RAM Memory Graph */}
				<MetricAreaChart
					title="RAM Memory Utilization Graph"
					icon={HardDrive}
					color="text-emerald-500"
					gradientId="mem-grad"
					data={history}
					dataKey="mem"
					unit="%"
				/>

				{/* 3. Disk Space & I/O Graph */}
				<MetricAreaChart
					title="Disk Space & I/O Utilization Graph"
					icon={Database}
					color="text-purple-500"
					gradientId="disk-grad"
					data={history}
					dataKey="disk"
					unit="%"
				/>

				{/* 4. Network I/O Traffic Graph */}
				<MetricAreaChart
					title="Network I/O Traffic Graph"
					icon={Network}
					color="text-blue-500"
					gradientId="net-grad"
					data={history}
					dataKey="net"
					unit=" MB"
					maxVal={Math.max(10, ...(history.map(h => h.net || 0)))}
				/>
			</div>
		</div>
	);
}
