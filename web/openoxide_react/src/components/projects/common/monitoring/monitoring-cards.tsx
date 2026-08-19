import {
	Cpu,
	HardDrive,
	Database,
	Network,
	Disc,
	Layers,
} from 'lucide-react';
import {
	Card,
	CardContent,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';

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
		const x =
			paddingLeft + (idx / Math.max(points.length - 1, 1)) * chartWidth;
		const normalizedVal = Math.min(Math.max(val, 0), maxVal);
		const y =
			height -
			paddingBottom -
			(normalizedVal / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const linePath = getSmoothPath(coords);
	const areaPath =
		coords.length > 0
			? `${linePath} L ${coords[coords.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords[0].x.toFixed(1)} ${height - paddingBottom} Z`
			: '';

	return (
		<div className="flex w-full flex-col gap-2">
			<div className="relative h-36 w-full">
				{coords.length < 2 ? (
					<div className="flex h-full w-full items-center justify-center rounded-lg border border-dashed border-border/60 font-mono text-xs text-muted-foreground">
						Collecting telemetry points...
					</div>
				) : (
					<svg
						viewBox={`0 0 ${width} ${height}`}
						className="h-full w-full overflow-visible">
						<defs>
							<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
								<stop
									offset="5%"
									stopColor={colorHex}
									stopOpacity={0.45}
								/>
								<stop
									offset="95%"
									stopColor={colorHex}
									stopOpacity={0.05}
								/>
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
										className="fill-muted-foreground font-mono text-[10px]">
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
			<div className="flex items-center justify-center gap-2 pt-1 text-xs font-medium text-muted-foreground">
				<span
					className="size-2.5 rounded-xs"
					style={{backgroundColor: colorHex}}
				/>
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
		const x =
			paddingLeft + (idx / Math.max(points1.length - 1, 1)) * chartWidth;
		const y =
			height -
			paddingBottom -
			(val / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const coords2 = points2.map((val, idx) => {
		const x =
			paddingLeft + (idx / Math.max(points2.length - 1, 1)) * chartWidth;
		const y =
			height -
			paddingBottom -
			(val / Math.max(maxVal, 0.001)) * chartHeight;
		return {x, y, val};
	});

	const linePath1 = getSmoothPath(coords1);
	const areaPath1 =
		coords1.length > 0
			? `${linePath1} L ${coords1[coords1.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords1[0].x.toFixed(1)} ${height - paddingBottom} Z`
			: '';

	const linePath2 = getSmoothPath(coords2);
	const areaPath2 =
		coords2.length > 0
			? `${linePath2} L ${coords2[coords2.length - 1].x.toFixed(1)} ${height - paddingBottom} L ${coords2[0].x.toFixed(1)} ${height - paddingBottom} Z`
			: '';

	const yTicks = [
		0,
		maxVal * 0.25,
		maxVal * 0.5,
		maxVal * 0.75,
		maxVal,
	].map(v => {
		if (v >= 1000) return (v / 1000).toFixed(1) + 'k';
		return Math.round(v).toString();
	});

	return (
		<div className="flex w-full flex-col gap-2">
			<div className="relative h-36 w-full">
				{coords1.length < 2 ? (
					<div className="flex h-full w-full items-center justify-center rounded-lg border border-dashed border-border/60 font-mono text-xs text-muted-foreground">
						Collecting telemetry points...
					</div>
				) : (
					<svg
						viewBox={`0 0 ${width} ${height}`}
						className="h-full w-full overflow-visible">
						<defs>
							<linearGradient id={gradientId1} x1="0" y1="0" x2="0" y2="1">
								<stop
									offset="5%"
									stopColor={colorHex1}
									stopOpacity={0.4}
								/>
								<stop
									offset="95%"
									stopColor={colorHex1}
									stopOpacity={0.0}
								/>
							</linearGradient>
							<linearGradient id={gradientId2} x1="0" y1="0" x2="0" y2="1">
								<stop
									offset="5%"
									stopColor={colorHex2}
									stopOpacity={0.4}
								/>
								<stop
									offset="95%"
									stopColor={colorHex2}
									stopOpacity={0.0}
								/>
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
										className="fill-muted-foreground font-mono text-[10px]">
										{tick}
									</text>
								</g>
							);
						})}

						{/* Area 1 */}
						<path
							d={areaPath1}
							fill={`url(#${gradientId1})`}
							className="transition-all duration-700 ease-in-out"
						/>
						<path
							d={linePath1}
							fill="none"
							stroke={colorHex1}
							strokeWidth="2"
							strokeLinecap="round"
							strokeLinejoin="round"
							className="transition-all duration-700 ease-in-out"
						/>

						{/* Area 2 */}
						<path
							d={areaPath2}
							fill={`url(#${gradientId2})`}
							className="transition-all duration-700 ease-in-out"
						/>
						<path
							d={linePath2}
							fill="none"
							stroke={colorHex2}
							strokeWidth="2"
							strokeLinecap="round"
							strokeLinejoin="round"
							className="transition-all duration-700 ease-in-out"
						/>
					</svg>
				)}
			</div>

			{/* Dual Legends */}
			<div className="flex items-center justify-center gap-6 pt-1 text-xs font-medium text-muted-foreground">
				<div className="flex items-center gap-2">
					<span
						className="size-2.5 rounded-xs"
						style={{backgroundColor: colorHex1}}
					/>
					<span>{legendLabel1}</span>
				</div>
				<div className="flex items-center gap-2">
					<span
						className="size-2.5 rounded-xs"
						style={{backgroundColor: colorHex2}}
					/>
					<span>{legendLabel2}</span>
				</div>
			</div>
		</div>
	);
}

function DockerDiskDonutChart({
	totalStr = '5.45 GB',
	containersStr = '5.45 GB',
	imagesStr = '0 MB',
	volumesStr = '0 MB',
}: {
	totalStr?: string;
	containersStr?: string;
	imagesStr?: string;
	volumesStr?: string;
}) {
	const cVal = parseBytes(containersStr) || 1;
	const iVal = parseBytes(imagesStr) || 0;
	const vVal = parseBytes(volumesStr) || 0;
	const totalVal = cVal + iVal + vVal || 1;

	const cPercent = cVal / totalVal;
	const iPercent = iVal / totalVal;
	const vPercent = vVal / totalVal;

	const radius = 45;
	const circumference = 2 * Math.PI * radius;

	const cStroke = cPercent * circumference;
	const iStroke = iPercent * circumference;
	const vStroke = vPercent * circumference;

	const cOffset = 0;
	const iOffset = -cStroke;
	const vOffset = -(cStroke + iStroke);

	return (
		<div className="flex w-full flex-col items-center justify-center gap-3 py-1">
			<div className="relative flex size-36 items-center justify-center">
				<svg
					viewBox="0 0 120 120"
					className="size-full -rotate-90 overflow-visible">
					<circle
						cx="60"
						cy="60"
						r={radius}
						fill="transparent"
						stroke="currentColor"
						strokeWidth="14"
						className="text-secondary/40"
					/>
					{cPercent > 0 && (
						<circle
							cx="60"
							cy="60"
							r={radius}
							fill="transparent"
							stroke="#3b82f6"
							strokeWidth="14"
							strokeDasharray={`${cStroke} ${circumference - cStroke}`}
							strokeDashoffset={cOffset}
							className="transition-all duration-700 ease-in-out"
						/>
					)}
					{iPercent > 0 && (
						<circle
							cx="60"
							cy="60"
							r={radius}
							fill="transparent"
							stroke="#10b981"
							strokeWidth="14"
							strokeDasharray={`${iStroke} ${circumference - iStroke}`}
							strokeDashoffset={iOffset}
							className="transition-all duration-700 ease-in-out"
						/>
					)}
					{vPercent > 0 && (
						<circle
							cx="60"
							cy="60"
							r={radius}
							fill="transparent"
							stroke="#a855f7"
							strokeWidth="14"
							strokeDasharray={`${vStroke} ${circumference - vStroke}`}
							strokeDashoffset={vOffset}
							className="transition-all duration-700 ease-in-out"
						/>
					)}
				</svg>

				<div className="absolute inset-0 flex flex-col items-center justify-center text-center">
					<span className="font-mono text-sm leading-none font-extrabold text-foreground">
						{totalStr}
					</span>
					<span className="mt-1 text-[10px] font-medium text-muted-foreground">
						Docker Usage
					</span>
				</div>
			</div>

			<div className="flex flex-wrap items-center justify-center gap-4 text-[11px] font-medium text-muted-foreground">
				<div className="flex items-center gap-1.5">
					<span className="size-2.5 rounded-xs bg-blue-500" />
					<span>Containers ({containersStr})</span>
				</div>
				<div className="flex items-center gap-1.5">
					<span className="size-2.5 rounded-xs bg-emerald-500" />
					<span>Images ({imagesStr})</span>
				</div>
				<div className="flex items-center gap-1.5">
					<span className="size-2.5 rounded-xs bg-purple-500" />
					<span>Volumes ({volumesStr})</span>
				</div>
			</div>
		</div>
	);
}

export function MonitoringCards({
	metrics,
	history = [],
}: MonitoringCardsProps) {
	const formatGB = (gb: number) => {
		if (gb >= 1) return `${gb.toFixed(2)} GiB`;
		return `${(gb * 1024).toFixed(0)} MiB`;
	};

	const lastPoint = history[history.length - 1];
	const maxMemUsed = Math.max(0, ...history.map(h => h.memUsedGB || 0));
	const maxMemLimit = lastPoint?.memLimitGB || 0;
	const memLimitGB = Math.max(maxMemLimit, maxMemUsed, 0.1);

	const memTicks = [
		'0 GB',
		formatGB(memLimitGB * 0.25),
		formatGB(memLimitGB * 0.5),
		formatGB(memLimitGB * 0.75),
		formatGB(memLimitGB),
	];

	const maxDiskUsed = Math.max(0, ...history.map(h => h.diskUsedGB || 0));
	const maxDiskTotal = lastPoint?.diskTotalGB || 0;
	const diskTotalGB = Math.max(maxDiskTotal, maxDiskUsed, 0.1);

	const diskTicks = [
		'0 GB',
		formatGB(diskTotalGB * 0.25),
		formatGB(diskTotalGB * 0.5),
		formatGB(diskTotalGB * 0.75),
		formatGB(diskTotalGB),
	];

	return (
		<div className="flex flex-col gap-6">
			{/* Dokploy Container Monitoring Grid (2 Columns) */}
			<div className="grid grid-cols-1 gap-6 md:grid-cols-2">
				{/* 1. CPU Usage */}
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							CPU Usage
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Used:{' '}
							<span className="font-bold text-foreground">
								{metrics?.cpuPercent?.toFixed(2) || '0.00'}%
							</span>
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
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Memory Usage
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Used:{' '}
							<span className="font-bold text-foreground">
								{metrics?.memUsage || '0 B'}
							</span>{' '}
							/ Limit:{' '}
							<span className="font-bold text-foreground">
								{metrics?.memLimit || '0 B'}
							</span>
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
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Disk Space
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Used:{' '}
							<span className="font-bold text-foreground">
								{metrics?.diskSpaceUsed || '0 GB'}
							</span>{' '}
							/ Limit:{' '}
							<span className="font-bold text-foreground">
								{metrics?.diskSpaceTotal || '0 GB'}
							</span>
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
				<Card className="flex flex-col justify-between border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Docker Disk Usage
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Total:{' '}
							<span className="font-bold text-foreground">
								{metrics?.dockerDiskUsage || '0 MB'}
							</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<DockerDiskDonutChart
							totalStr={metrics?.dockerDiskUsage || '0 MB'}
							containersStr={metrics?.dockerDiskUsage || '0 MB'}
							imagesStr="0 MB"
							volumesStr="0 MB"
						/>
					</CardContent>
				</Card>

				{/* 5. Block I/O */}
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Block I/O
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Read:{' '}
							<span className="font-bold text-emerald-500">
								{metrics?.blockRead || '0 B'}
							</span>{' '}
							/ Write:{' '}
							<span className="font-bold text-rose-500">
								{metrics?.blockWrite || '0 B'}
							</span>
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
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Network I/O
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							In:{' '}
							<span className="font-bold text-blue-500">
								{metrics?.netRx || '0 B'}
							</span>{' '}
							/ Out:{' '}
							<span className="font-bold text-indigo-500">
								{metrics?.netTx || '0 B'}
							</span>
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
