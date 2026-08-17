import {Cpu, HardDrive, Database, Network, Disc, Layers} from 'lucide-react';
import {Card, CardContent, CardHeader, CardTitle} from '#/components/ui/card';

interface MonitoringCardsProps {
	metrics: any;
	history?: Array<{
		time: string;
		cpu: number;
		memUsedGB: number;
		memLimitGB: number;
		diskUsedGB: number;
		diskTotalGB: number;
		dockerDiskGB: number;
		blockReadMB: number;
		blockWriteMB: number;
		netRxMB: number;
		netTxMB: number;
	}>;
}

function getSmoothPath(coords: Array<{x: number; y: number}>): string {
	if (coords.length === 0) return '';
	if (coords.length === 1) return `M ${coords[0].x} ${coords[0].y}`;

	let path = `M ${coords[0].x.toFixed(1)} ${coords[0].y.toFixed(1)}`;

	for (let i = 0; i < coords.length - 1; i++) {
		const p0 = coords[Math.max(i - 1, 0)];
		const p1 = coords[i];
		const p2 = coords[i + 1];
		const p3 = coords[Math.min(i + 2, coords.length - 1)];

		const cp1x = p1.x + (p2.x - p0.x) * 0.15;
		const cp1y = p1.y + (p2.y - p0.y) * 0.15;
		const cp2x = p2.x - (p3.x - p1.x) * 0.15;
		const cp2y = p2.y - (p3.y - p1.y) * 0.15;

		path += ` C ${cp1x.toFixed(1)} ${cp1y.toFixed(1)}, ${cp2x.toFixed(1)} ${cp2y.toFixed(1)}, ${p2.x.toFixed(1)} ${p2.y.toFixed(1)}`;
	}

	return path;
}

// Single series SVG Chart matching Dokploy style
function DokploySingleChart({
	gradientId,
	colorHex,
	data = [],
	dataKey,
	maxVal = 100,
	yTicks = ['0%', '25%', '50%', '75%', '100%'],
	legendLabel,
}: {
	gradientId: string;
	colorHex: string;
	data: Array<any>;
	dataKey: string;
	maxVal?: number;
	yTicks?: string[];
	legendLabel: string;
}) {
	const points = data.map(d => Number(d[dataKey]) || 0);
	const width = 500;
	const height = 130;
	const paddingLeft = 55;
	const paddingRight = 15;
	const paddingTop = 10;
	const paddingBottom = 20;

	const chartWidth = width - paddingLeft - paddingRight;
	const chartHeight = height - paddingTop - paddingBottom;

	const coords = points.map((val, idx) => {
		const x = paddingLeft + (idx / Math.max(points.length - 1, 1)) * chartWidth;
		const normalizedVal = Math.min(Math.max(val, 0), maxVal);
		const y = height - paddingBottom - (normalizedVal / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const linePath = getSmoothPath(coords);
	const areaPath = coords.length > 0
		? `${linePath} L ${coords[coords.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords[0].x.toFixed(1)} ${height - paddingBottom} Z`
		: '';

	return (
		<div className="flex flex-col gap-2 w-full">
			<div className="w-full h-36 relative">
				{coords.length < 2 ? (
					<div className="w-full h-full flex items-center justify-center text-xs text-muted-foreground font-mono border border-dashed border-border/60 rounded-lg">
						Collecting telemetry points...
					</div>
				) : (
					<svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full overflow-visible">
						<defs>
							<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
								<stop offset="5%" stopColor={colorHex} stopOpacity={0.45} />
								<stop offset="95%" stopColor={colorHex} stopOpacity={0.05} />
							</linearGradient>
						</defs>

						{/* Horizontal Gridlines & Y-Axis Labels */}
						{yTicks.map((tick, idx) => {
							const ratio = idx / Math.max(yTicks.length - 1, 1);
							const y = height - paddingBottom - ratio * chartHeight;
							return (
								<g key={tick}>
									<line
										x1={paddingLeft}
										y1={y}
										x2={width - paddingRight}
										y2={y}
										stroke="currentColor"
										strokeDasharray="3 3"
										className="text-border/30"
									/>
									<text
										x={paddingLeft - 8}
										y={y + 3}
										textAnchor="end"
										className="text-[10px] fill-muted-foreground font-mono"
									>
										{tick}
									</text>
								</g>
							);
						})}

						{/* Area Fill */}
						<path
							d={areaPath}
							fill={`url(#${gradientId})`}
							className="transition-all duration-700 ease-in-out"
						/>

						{/* Line Stroke */}
						<path
							d={linePath}
							fill="none"
							stroke={colorHex}
							strokeWidth="2"
							strokeLinecap="round"
							strokeLinejoin="round"
							className="transition-all duration-700 ease-in-out"
						/>
					</svg>
				)}
			</div>

			{/* Dokploy Chart Legend */}
			<div className="flex items-center justify-center gap-2 text-xs text-muted-foreground font-medium pt-1">
				<span className="size-2.5 rounded-xs" style={{backgroundColor: colorHex}} />
				<span>{legendLabel}</span>
			</div>
		</div>
	);
}

// Dual series SVG Chart for Block I/O & Network I/O matching Dokploy style
function DokployDualChart({
	gradientId1,
	gradientId2,
	colorHex1,
	colorHex2,
	data = [],
	dataKey1,
	dataKey2,
	legendLabel1,
	legendLabel2,
}: {
	gradientId1: string;
	gradientId2: string;
	colorHex1: string;
	colorHex2: string;
	data: Array<any>;
	dataKey1: string;
	dataKey2: string;
	legendLabel1: string;
	legendLabel2: string;
}) {
	const points1 = data.map(d => Number(d[dataKey1]) || 0);
	const points2 = data.map(d => Number(d[dataKey2]) || 0);
	const maxVal = Math.max(10, ...points1, ...points2);

	const width = 500;
	const height = 130;
	const paddingLeft = 55;
	const paddingRight = 15;
	const paddingTop = 10;
	const paddingBottom = 20;

	const chartWidth = width - paddingLeft - paddingRight;
	const chartHeight = height - paddingTop - paddingBottom;

	const coords1 = points1.map((val, idx) => {
		const x = paddingLeft + (idx / Math.max(points1.length - 1, 1)) * chartWidth;
		const y = height - paddingBottom - (val / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const coords2 = points2.map((val, idx) => {
		const x = paddingLeft + (idx / Math.max(points2.length - 1, 1)) * chartWidth;
		const y = height - paddingBottom - (val / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const linePath1 = getSmoothPath(coords1);
	const areaPath1 = coords1.length > 0
		? `${linePath1} L ${coords1[coords1.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords1[0].x.toFixed(1)} ${height - paddingBottom} Z`
		: '';

	const linePath2 = getSmoothPath(coords2);
	const areaPath2 = coords2.length > 0
		? `${linePath2} L ${coords2[coords2.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords2[0].x.toFixed(1)} ${height - paddingBottom} Z`
		: '';

	const yTicks = [0, maxVal * 0.25, maxVal * 0.5, maxVal * 0.75, maxVal].map(v => {
		if (v >= 1000) return (v / 1000).toFixed(1) + 'k';
		return Math.round(v).toString();
	});

	return (
		<div className="flex flex-col gap-2 w-full">
			<div className="w-full h-36 relative">
				{coords1.length < 2 ? (
					<div className="w-full h-full flex items-center justify-center text-xs text-muted-foreground font-mono border border-dashed border-border/60 rounded-lg">
						Collecting telemetry points...
					</div>
				) : (
					<svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full overflow-visible">
						<defs>
							<linearGradient id={gradientId1} x1="0" y1="0" x2="0" y2="1">
								<stop offset="5%" stopColor={colorHex1} stopOpacity={0.4} />
								<stop offset="95%" stopColor={colorHex1} stopOpacity={0.0} />
							</linearGradient>
							<linearGradient id={gradientId2} x1="0" y1="0" x2="0" y2="1">
								<stop offset="5%" stopColor={colorHex2} stopOpacity={0.4} />
								<stop offset="95%" stopColor={colorHex2} stopOpacity={0.0} />
							</linearGradient>
						</defs>

						{yTicks.map((tick, idx) => {
							const ratio = idx / Math.max(yTicks.length - 1, 1);
							const y = height - paddingBottom - ratio * chartHeight;
							return (
								<g key={idx}>
									<line
										x1={paddingLeft}
										y1={y}
										x2={width - paddingRight}
										y2={y}
										stroke="currentColor"
										strokeDasharray="3 3"
										className="text-border/30"
									/>
									<text
										x={paddingLeft - 8}
										y={y + 3}
										textAnchor="end"
										className="text-[10px] fill-muted-foreground font-mono"
									>
										{tick}
									</text>
								</g>
							);
						})}

						{/* Area 1 */}
						<path d={areaPath1} fill={`url(#${gradientId1})`} className="transition-all duration-700 ease-in-out" />
						<path d={linePath1} fill="none" stroke={colorHex1} strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="transition-all duration-700 ease-in-out" />

						{/* Area 2 */}
						<path d={areaPath2} fill={`url(#${gradientId2})`} className="transition-all duration-700 ease-in-out" />
						<path d={linePath2} fill="none" stroke={colorHex2} strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="transition-all duration-700 ease-in-out" />
					</svg>
				)}
			</div>

			{/* Dual Legends */}
			<div className="flex items-center justify-center gap-6 text-xs text-muted-foreground font-medium pt-1">
				<div className="flex items-center gap-2">
					<span className="size-2.5 rounded-xs" style={{backgroundColor: colorHex1}} />
					<span>{legendLabel1}</span>
				</div>
				<div className="flex items-center gap-2">
					<span className="size-2.5 rounded-xs" style={{backgroundColor: colorHex2}} />
					<span>{legendLabel2}</span>
				</div>
			</div>
		</div>
	);
}

export function MonitoringCards({metrics, history = []}: MonitoringCardsProps) {
	// Dynamic Max bounds for Y-Axis domains
	const lastPoint = history[history.length - 1];
	const maxMemUsed = Math.max(0, ...(history.map(h => h.memUsedGB || 0)));
	const maxMemLimit = lastPoint?.memLimitGB || 0;
	const memLimitGB = Math.max(maxMemLimit, maxMemUsed, 0.5);

	const memTicks = [
		'0 GB',
		(memLimitGB * 0.25).toFixed(2) + ' GB',
		(memLimitGB * 0.5).toFixed(2) + ' GB',
		(memLimitGB * 0.75).toFixed(2) + ' GB',
		memLimitGB.toFixed(2) + ' GB',
	];

	const maxDiskUsed = Math.max(0, ...(history.map(h => h.diskUsedGB || 0)));
	const maxDiskTotal = lastPoint?.diskTotalGB || 0;
	const diskTotalGB = Math.max(maxDiskTotal, maxDiskUsed, 10);

	const diskTicks = [
		'0 GB',
		(diskTotalGB * 0.25).toFixed(1) + ' GB',
		(diskTotalGB * 0.5).toFixed(1) + ' GB',
		(diskTotalGB * 0.75).toFixed(1) + ' GB',
		diskTotalGB.toFixed(2) + ' GB',
	];

	return (
		<div className="flex flex-col gap-6">
			{/* Dokploy Container Monitoring Grid (2 Columns) */}
			<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
				{/* 1. CPU Usage */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">CPU Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">{metrics?.cpuPercent?.toFixed(2) || '0.00'}%</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DokploySingleChart
							gradientId="dok-cpu-grad"
							colorHex="#3b82f6"
							data={history}
							dataKey="cpu"
							maxVal={100}
							yTicks={['0%', '25%', '50%', '75%', '100%']}
							legendLabel="CPU Usage"
						/>
					</CardContent>
				</Card>

				{/* 2. Memory Usage */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Memory Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">{metrics?.memUsage || '0 B'}</span> / Limit:{' '}
							<span className="font-bold text-foreground">{metrics?.memLimit || '0 B'}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DokploySingleChart
							gradientId="dok-mem-grad"
							colorHex="#10b981"
							data={history}
							dataKey="memUsedGB"
							maxVal={memLimitGB}
							yTicks={memTicks}
							legendLabel="Memory (GB)"
						/>
					</CardContent>
				</Card>

				{/* 3. Disk Space */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Disk Space</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">{metrics?.diskSpaceUsed || '0 GB'}</span> / Limit:{' '}
							<span className="font-bold text-foreground">{metrics?.diskSpaceTotal || '0 GB'}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DokploySingleChart
							gradientId="dok-disk-grad"
							colorHex="#a855f7"
							data={history}
							dataKey="diskUsedGB"
							maxVal={diskTotalGB}
							yTicks={diskTicks}
							legendLabel="Disk Space"
						/>
					</CardContent>
				</Card>

				{/* 4. Docker Disk Usage */}
				<Card className="bg-card border-border shadow-xs flex flex-col justify-between">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Docker Disk Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Total: <span className="font-bold text-foreground">{metrics?.dockerDiskUsage || '0 MB'}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-4 flex flex-col gap-4">
						<div className="grid grid-cols-3 gap-3 text-center">
							<div className="bg-secondary/40 border border-border/50 rounded-lg p-3">
								<span className="text-[11px] text-muted-foreground block mb-1 font-medium">Containers</span>
								<span className="text-sm font-bold font-mono text-foreground">{metrics?.dockerDiskUsage || '0 MB'}</span>
							</div>
							<div className="bg-secondary/40 border border-border/50 rounded-lg p-3">
								<span className="text-[11px] text-muted-foreground block mb-1 font-medium">Images</span>
								<span className="text-sm font-bold font-mono text-muted-foreground">0 MB</span>
							</div>
							<div className="bg-secondary/40 border border-border/50 rounded-lg p-3">
								<span className="text-[11px] text-muted-foreground block mb-1 font-medium">Volumes</span>
								<span className="text-sm font-bold font-mono text-muted-foreground">0 MB</span>
							</div>
						</div>
						<div className="flex items-center justify-center gap-2 text-xs text-muted-foreground font-medium pt-2">
							<span className="size-2.5 rounded-xs bg-amber-500" />
							<span>Docker Usage</span>
						</div>
					</CardContent>
				</Card>

				{/* 5. Block I/O */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Block I/O</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Read: <span className="font-bold text-emerald-500">{metrics?.blockRead || '0 B'}</span> / Write:{' '}
							<span className="font-bold text-rose-500">{metrics?.blockWrite || '0 B'}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DokployDualChart
							gradientId1="dok-blk-r"
							gradientId2="dok-blk-w"
							colorHex1="#10b981"
							colorHex2="#f43f5e"
							data={history}
							dataKey1="blockReadMB"
							dataKey2="blockWriteMB"
							legendLabel1="Read (MB)"
							legendLabel2="Write (MB)"
						/>
					</CardContent>
				</Card>

				{/* 6. Network I/O */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Network I/O</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							In: <span className="font-bold text-blue-500">{metrics?.netRx || '0 B'}</span> / Out:{' '}
							<span className="font-bold text-indigo-500">{metrics?.netTx || '0 B'}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DokployDualChart
							gradientId1="dok-net-in"
							gradientId2="dok-net-out"
							colorHex1="#3b82f6"
							colorHex2="#6366f1"
							data={history}
							dataKey1="netRxMB"
							dataKey2="netTxMB"
							legendLabel1="In (MB)"
							legendLabel2="Out (MB)"
						/>
					</CardContent>
				</Card>
			</div>
		</div>
	);
}
