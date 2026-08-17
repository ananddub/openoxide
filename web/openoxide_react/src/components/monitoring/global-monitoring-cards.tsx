import {useState, useEffect, useMemo} from 'react';
import {Cpu, HardDrive, Database, Disc, Network, Layers} from 'lucide-react';
import {Card, CardContent, CardHeader, CardTitle} from '#/components/ui/card';
import {Progress} from '#/components/ui/progress';
import {$api} from '#/api/query';
import {useDeploymentRunning} from 'virtual:openoxide-live';
import {useAppStore} from '#/stores/app-store';

// ─── Helpers ─────────────────────────────────────────────────────────────────

function stripAnsi(str: string): string {
	return str.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '').replace(/\x1b[^[]/g, '');
}

function parseBytes(str?: string): number {
	if (!str || typeof str !== 'string') return 0;
	const val = parseFloat(str) || 0;
	const unit = str.replace(/[0-9.]/g, '').trim().toUpperCase();
	if (unit.startsWith('K')) return val * 1024;
	if (unit.startsWith('M')) return val * 1024 * 1024;
	if (unit.startsWith('G')) return val * 1024 * 1024 * 1024;
	if (unit.startsWith('T')) return val * 1024 * 1024 * 1024 * 1024;
	return val;
}

function formatBytes(bytes: number): string {
	if (!bytes || isNaN(bytes) || bytes <= 0) return '0 B';
	if (bytes < 1024) return `${bytes.toFixed(0)} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

type DockerStat = Record<string, unknown>;

/** Extract one or more Docker stat objects from an SSE stats payload */
function extractDockerStats(payload: unknown): DockerStat[] {
	if (!payload) return [];

	// Already a clean array of stat objects
	if (Array.isArray(payload)) {
		return (payload as DockerStat[]).filter(x => x.CPUPerc || x.MemUsage);
	}

	const obj = payload as DockerStat;

	// Already a clean stat object
	if (obj.CPUPerc || obj.MemUsage) return [obj];

	// Raw ANSI-wrapped string → strip + find JSON lines
	const rawStr = typeof obj.raw === 'string' ? obj.raw : '';
	if (rawStr) {
		const stripped = stripAnsi(rawStr);
		const results: DockerStat[] = [];
		// Each docker stat is one JSON object per line
		for (const chunk of stripped.split('\n')) {
			const trimmed = chunk.trim();
			const start = trimmed.indexOf('{');
			const end = trimmed.lastIndexOf('}');
			if (start === -1 || end === -1) continue;
			try {
				const parsed = JSON.parse(trimmed.slice(start, end + 1)) as DockerStat;
				if (parsed.CPUPerc || parsed.MemUsage || parsed.Container) {
					results.push(parsed);
				}
			} catch {}
		}
		if (results.length > 0) return results;
	}

	return [];
}

function getAccessToken(): string {
	try {
		const session = JSON.parse(localStorage.getItem('openoxide-auth-session') || '{}');
		return session?.tokens?.access_token || '';
	} catch {
		return '';
	}
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

function GlobalAreaChart({
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

function GlobalDualChart({
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

function GlobalDockerDiskDonutChart({
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
		<div className="flex flex-col items-center justify-center gap-3 py-1 w-full">
			<div className="relative size-36 flex items-center justify-center">
				<svg viewBox="0 0 120 120" className="size-full -rotate-90 overflow-visible">
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
					<span className="text-sm font-extrabold font-mono text-foreground leading-none">
						{totalStr}
					</span>
					<span className="text-[10px] font-medium text-muted-foreground mt-1">
						Docker Usage
					</span>
				</div>
			</div>

			<div className="flex items-center justify-center gap-4 text-[11px] font-medium text-muted-foreground flex-wrap">
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

// ─── Component ────────────────────────────────────────────────────────────────

export function GlobalMonitoringCards() {
	const overviewServices = useAppStore((state) => state.overviewServices || []);
	const {data: rawDockerContainers = []} =
		$api.useQuery('get', '/deployments/docker/containers', {params: {query: {server_id: undefined} as any}});

	const {data: rawDiskUsage} =
		$api.useQuery('get', '/deployments/docker/disk-usage', {params: {query: {server_id: undefined} as any}});

	const {data: rawServerMetrics} =
		$api.useQuery('get', '/monitoring/server/{id}', {params: {path: {id: 1} as any}});

	const {data: rawRunning} = useDeploymentRunning({
		status: null, state: null,
		application_id: null, compose_id: null, database_id: null, server_id: null,
		limit: 50n, offset: null,
	});

	const diskUsageFormatted = useMemo(() => {
		if (!rawDiskUsage || typeof rawDiskUsage !== 'object') {
			return {containersStr: '0 B', imagesStr: '0 B', volumesStr: '0 B', totalStr: '0 B', totalBytes: 0};
		}
		const obj = rawDiskUsage as Record<string, any>;
		const containers = Array.isArray(obj.Containers) ? obj.Containers : [];
		const images = Array.isArray(obj.Images) ? obj.Images : [];
		const volumes = Array.isArray(obj.Volumes) ? obj.Volumes : [];

		const cBytes = containers.reduce((sum: number, item: any) => sum + (Number(item.Size || item.size || item.sizeBytes) || 0), 0);
		const iBytes = images.reduce((sum: number, item: any) => sum + (Number(item.Size || item.size || item.sizeBytes) || 0), 0);
		const vBytes = volumes.reduce((sum: number, item: any) => sum + (Number(item.Size || item.size || item.sizeBytes) || 0), 0);
		const total = cBytes + iBytes + vBytes;

		const formatBytes = (bytes: number) => {
			if (bytes >= 1024 ** 3) return `${(bytes / (1024 ** 3)).toFixed(2)} GB`;
			if (bytes >= 1024 ** 2) return `${(bytes / (1024 ** 2)).toFixed(1)} MB`;
			if (bytes >= 1024) return `${(bytes / 1024).toFixed(1)} KB`;
			return `${bytes} B`;
		};

		return {
			containersStr: formatBytes(cBytes),
			imagesStr: formatBytes(iBytes),
			volumesStr: formatBytes(vBytes),
			totalStr: formatBytes(total),
			totalBytes: total,
		};
	}, [rawDiskUsage]);

	const hostDiskSpace = useMemo(() => {
		if (Array.isArray(rawServerMetrics) && rawServerMetrics.length > 0) {
			const m = rawServerMetrics[rawServerMetrics.length - 1] as any;
			if (m && m.total_disk > 0) {
				return {
					diskUsedGB: (m.disk_used || 0) / (1024 ** 3),
					diskTotalGB: (m.total_disk || 0) / (1024 ** 3),
				};
			}
		}
		return null;
	}, [rawServerMetrics]);

	const [containersList, setContainersList] = useState<DockerStat[]>([]);

	// SSE live stream
	useEffect(() => {
		const DISABLE_METRICS = false;
		if (DISABLE_METRICS) return;
		const token = getAccessToken();
		let isMounted = true;
		const controller = new AbortController();

		const run = async () => {
			try {
				const res = await fetch('/api/deployments/docker/stats?stream=true', {
					headers: {Authorization: token ? `Bearer ${token}` : ''},
					signal: controller.signal,
				});
				if (!res.ok || !res.body) return;

				const reader = res.body.getReader();
				const dec = new TextDecoder();
				let buf = '';

				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;
					buf += dec.decode(value, {stream: true});
					const lines = buf.split('\n');
					buf = lines.pop() || '';
					for (const line of lines) {
						if (!line.startsWith('data:')) continue;
						try {
							const evt = JSON.parse(line.slice(5).trim());
							if (evt.type === 'stats' && evt.stats) {
								const stats = extractDockerStats(evt.stats);
								if (stats.length > 0 && isMounted) setContainersList(stats);
							}
						} catch {}
					}
				}
			} catch {}
		};

		run();
		return () => {
			isMounted = false;
			controller.abort();
		};
	}, []);

	const [history, setHistory] = useState<
		Array<{
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
		}>
	>([]);

	useEffect(() => {
		if (containersList.length > 0) {
			let cpuSum = 0, memUsedSum = 0, memLimitSum = 0;
			let blkReadSum = 0, blkWriteSum = 0, netRxSum = 0, netTxSum = 0;
			let diskUsedSum = 0, diskTotalSum = 0;

			for (const c of containersList) {
				cpuSum += parseFloat(String(c.CPUPerc || '0').replace('%', '')) || 0;
				const mem = String(c.MemUsage || '');
				if (mem.includes('/')) {
					const [u, l] = mem.split('/').map(s => s.trim());
					memUsedSum += parseBytes(u);
					if (!memLimitSum) memLimitSum = parseBytes(l);
				}

				const blk = String(c.BlockIO || '');
				if (blk.includes('/')) {
					const [r, w] = blk.split('/').map(s => s.trim());
					blkReadSum += parseBytes(r);
					blkWriteSum += parseBytes(w);
				}

				const net = String(c.NetIO || '');
				if (net.includes('/')) {
					const [rx, tx] = net.split('/').map(s => s.trim());
					netRxSum += parseBytes(rx);
					netTxSum += parseBytes(tx);
				}

				if (c.SizeRw) diskUsedSum += parseBytes(String(c.SizeRw));
				if (c.DiskUsed) diskUsedSum += parseBytes(String(c.DiskUsed));
				if (c.DiskTotal && !diskTotalSum) diskTotalSum = parseBytes(String(c.DiskTotal));
			}

			const timeStr = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit', second: '2-digit'});
			const finalMemLimitGB = memLimitSum > 0 ? memLimitSum / (1024 ** 3) : (memUsedSum > 0 ? (memUsedSum * 1.5) / (1024 ** 3) : 1);
			const finalDiskUsedGB = hostDiskSpace?.diskUsedGB || (diskUsedSum > 0 ? diskUsedSum / (1024 ** 3) : (diskUsageFormatted.totalBytes / (1024 ** 3)));
			const finalDiskTotalGB = hostDiskSpace?.diskTotalGB || (diskTotalSum > 0 ? diskTotalSum / (1024 ** 3) : Math.max(finalDiskUsedGB * 1.5, 1));

			setHistory(prev => [
				...prev.slice(-49),
				{
					time: timeStr,
					cpu: cpuSum,
					memUsedGB: memUsedSum / (1024 ** 3),
					memLimitGB: finalMemLimitGB,
					diskUsedGB: finalDiskUsedGB,
					diskTotalGB: finalDiskTotalGB,
					dockerDiskGB: diskUsageFormatted.totalBytes / (1024 ** 3),
					blockReadMB: blkReadSum / (1024 * 1024),
					blockWriteMB: blkWriteSum / (1024 * 1024),
					netRxMB: netRxSum / (1024 * 1024),
					netTxMB: netTxSum / (1024 * 1024),
				},
			]);
		}
	}, [containersList, hostDiskSpace, diskUsageFormatted]);

	// ─── Aggregate metrics ────────────────────────────────────────────────────
	const dockerContainersArray = Array.isArray(rawDockerContainers) ? rawDockerContainers : [];
	const runningArray = Array.isArray(rawRunning) ? rawRunning : [];
	const activeContainersCount = Math.max(dockerContainersArray.length, runningArray.length, containersList.length);

	const last = history[history.length - 1];
	const latestCpu = last?.cpu || 0;
	const latestMemUsed = last?.memUsedGB || 0;
	const rawMemLimit = last?.memLimitGB || 1;
	const latestMemLimit = Math.max(rawMemLimit, latestMemUsed, 0.1);

	const latestDiskUsed = hostDiskSpace?.diskUsedGB || last?.diskUsedGB || (diskUsageFormatted.totalBytes / (1024 ** 3));
	const latestDiskTotal = hostDiskSpace?.diskTotalGB || last?.diskTotalGB || Math.max(latestDiskUsed * 1.5, 1);

	const latestBlockR = last?.blockReadMB || 0;
	const latestBlockW = last?.blockWriteMB || 0;
	const latestNetRx = last?.netRxMB || 0;
	const latestNetTx = last?.netTxMB || 0;

	const formatGB = (gb: number) => {
		if (gb >= 1) return `${gb.toFixed(2)} GiB`;
		return `${(gb * 1024).toFixed(0)} MiB`;
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Header bar */}
			<div className="flex items-center justify-between bg-card border border-border rounded-xl p-4 shadow-xs">
				<div className="flex items-center gap-2.5">
					<div className="size-2.5 rounded-full bg-emerald-500 animate-pulse" />
					<span className="text-xs font-semibold text-foreground">
						Docker Telemetry Engine
					</span>
					<span className="text-xs text-muted-foreground">
						— {activeContainersCount} Active System Containers
					</span>
				</div>
				<span className="text-xs text-muted-foreground font-mono">Realtime Monitoring</span>
			</div>

			{/* Dokploy 6-Card Monitoring Grid */}
			<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
				{/* 1. CPU Usage */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">CPU Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">{latestCpu.toFixed(2)}%</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalAreaChart
							gradientId="dok-g-cpu"
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
							Used: <span className="font-bold text-foreground">{formatGB(latestMemUsed)}</span> / Limit:{' '}
							<span className="font-bold text-foreground">{formatGB(latestMemLimit)}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalAreaChart
							gradientId="dok-g-mem"
							colorHex="#10b981"
							data={history}
							dataKey="memUsedGB"
							maxVal={latestMemLimit}
							yTicks={[
								'0 GB',
								formatGB(latestMemLimit * 0.25),
								formatGB(latestMemLimit * 0.5),
								formatGB(latestMemLimit * 0.75),
								formatGB(latestMemLimit),
							]}
							legendLabel="Memory"
						/>
					</CardContent>
				</Card>

				{/* 3. Disk Space */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Disk Space</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">{formatGB(latestDiskUsed)}</span> / Limit:{' '}
							<span className="font-bold text-foreground">{formatGB(latestDiskTotal)}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalAreaChart
							gradientId="dok-g-disk"
							colorHex="#a855f7"
							data={history}
							dataKey="diskUsedGB"
							maxVal={latestDiskTotal}
							yTicks={[
								'0 GB',
								formatGB(latestDiskTotal * 0.25),
								formatGB(latestDiskTotal * 0.5),
								formatGB(latestDiskTotal * 0.75),
								formatGB(latestDiskTotal),
							]}
							legendLabel="Disk Space"
						/>
					</CardContent>
				</Card>

				{/* 4. Docker Disk Usage */}
				<Card className="bg-card border-border shadow-xs flex flex-col justify-between">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Docker Disk Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Total: <span className="font-bold text-foreground">{diskUsageFormatted.totalStr}</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalDockerDiskDonutChart
							totalStr={diskUsageFormatted.totalStr}
							containersStr={diskUsageFormatted.containersStr}
							imagesStr={diskUsageFormatted.imagesStr}
							volumesStr={diskUsageFormatted.volumesStr}
						/>
					</CardContent>
				</Card>

				{/* 5. Block I/O */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Block I/O</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Read: <span className="font-bold text-emerald-500">{latestBlockR.toFixed(2)} MB</span> / Write:{' '}
							<span className="font-bold text-rose-500">{latestBlockW.toFixed(2)} MB</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalDualChart
							gradientId1="dok-g-blk-r"
							gradientId2="dok-g-blk-w"
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
							In: <span className="font-bold text-blue-500">{latestNetRx.toFixed(2)} MB</span> / Out:{' '}
							<span className="font-bold text-indigo-500">{latestNetTx.toFixed(2)} MB</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalDualChart
							gradientId1="dok-g-net-in"
							gradientId2="dok-g-net-out"
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
