import {useState, useEffect} from 'react';
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

	const linePath = getSmoothPath(coords);
	const areaPath = coords.length > 0 
		? `${linePath} L ${coords[coords.length - 1].x.toFixed(1)} ${height - padding} L ${coords[0].x.toFixed(1)} ${height - padding} Z`
		: '';

	return (
		<div className="bg-card border border-border rounded-xl p-4 shadow-xs flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-2">
					<Icon className={`size-4 ${color}`} />
					<span className="text-xs font-bold text-foreground">{title}</span>
				</div>
				<span className={`text-sm font-mono font-extrabold ${color} transition-all duration-300`}>
					{latest.toFixed(1)}{unit}
				</span>
			</div>

			<div className="w-full h-32 relative overflow-hidden">
				{coords.length < 2 ? (
					<div className="w-full h-full flex items-center justify-center text-xs text-muted-foreground font-mono border border-dashed border-border/60 rounded-lg">
						Collecting live telemetry points…
					</div>
				) : (
					<svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full overflow-visible">
						<defs>
							<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
								<stop offset="0%" stopColor="currentColor" stopOpacity="0.3" />
								<stop offset="100%" stopColor="currentColor" stopOpacity="0.0" />
							</linearGradient>
						</defs>

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

						<path
							d={areaPath}
							fill={`url(#${gradientId})`}
							className={`${color} transition-all duration-700 ease-in-out`}
						/>

						<path
							d={linePath}
							fill="none"
							stroke="currentColor"
							strokeWidth="2.5"
							strokeLinecap="round"
							strokeLinejoin="round"
							className={`${color} transition-all duration-700 ease-in-out`}
						/>

						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current animate-ping transition-all duration-700 ease-out`}
							/>
						)}
						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current transition-all duration-700 ease-out`}
							/>
						)}
					</svg>
				)}
			</div>

			<div className="flex justify-between text-[10px] text-muted-foreground font-mono px-1">
				<span>{data[0]?.time || 'Start'}</span>
				<span>{data[data.length - 1]?.time || 'Live'}</span>
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

// ─── Component ────────────────────────────────────────────────────────────────

export function GlobalMonitoringCards() {
	const overviewServices = useAppStore((state) => state.overviewServices || []);
	const {data: rawDockerContainers = []} =
		$api.useQuery('get', '/deployments/docker/containers', {params: {query: {server_id: undefined} as any}});

	const {data: rawRunning} = useDeploymentRunning({
		status: null, state: null,
		application_id: null, compose_id: null, database_id: null, server_id: null,
		limit: 50n, offset: null,
	});

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
			}

			const timeStr = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit', second: '2-digit'});
			setHistory(prev => [
				...prev.slice(-49),
				{
					time: timeStr,
					cpu: cpuSum,
					memUsedGB: memUsedSum / (1024 ** 3),
					memLimitGB: (memLimitSum || 3.32 * 1024 ** 3) / (1024 ** 3),
					diskUsedGB: 12.24,
					diskTotalGB: 38.09,
					blockReadMB: blkReadSum / (1024 * 1024),
					blockWriteMB: blkWriteSum / (1024 * 1024),
					netRxMB: netRxSum / (1024 * 1024),
					netTxMB: netTxSum / (1024 * 1024),
				},
			]);
		}
	}, [containersList]);

	// ─── Aggregate metrics ────────────────────────────────────────────────────
	const dockerContainersArray = Array.isArray(rawDockerContainers) ? rawDockerContainers : [];
	const runningArray = Array.isArray(rawRunning) ? rawRunning : [];
	const activeContainersCount = Math.max(dockerContainersArray.length, runningArray.length, containersList.length);

	const last = history[history.length - 1];
	const latestCpu = last?.cpu || 0;
	const latestMemUsed = last?.memUsedGB || 0;
	const latestMemLimit = last?.memLimitGB || 3.32;
	const latestBlockR = last?.blockReadMB || 0;
	const latestBlockW = last?.blockWriteMB || 0;
	const latestNetRx = last?.netRxMB || 0;
	const latestNetTx = last?.netTxMB || 0;

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
							Used: <span className="font-bold text-foreground">{latestMemUsed.toFixed(2)}GiB</span> / Limit:{' '}
							<span className="font-bold text-foreground">{latestMemLimit.toFixed(2)}GiB</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalAreaChart
							gradientId="dok-g-mem"
							colorHex="#10b981"
							data={history}
							dataKey="memUsedGB"
							maxVal={latestMemLimit}
							yTicks={['0 GB', '0.85GB', '1.7 GB', '2.55 GB', `${latestMemLimit.toFixed(2)}GB`]}
							legendLabel="Memory (GB)"
						/>
					</CardContent>
				</Card>

				{/* 3. Disk Space */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Disk Space</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Used: <span className="font-bold text-foreground">12.24 GB</span> / Limit:{' '}
							<span className="font-bold text-foreground">38.09 GB</span>
						</span>
					</CardHeader>
					<CardContent className="pt-2">
						<GlobalAreaChart
							gradientId="dok-g-disk"
							colorHex="#a855f7"
							data={history}
							dataKey="diskUsedGB"
							maxVal={38.09}
							yTicks={['10 GB', '20 GB', '30 GB', '38.09GB']}
							legendLabel="Disk Space"
						/>
					</CardContent>
				</Card>

				{/* 4. Docker Disk Usage */}
				<Card className="bg-card border-border shadow-xs flex flex-col justify-between">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">Docker Disk Usage</CardTitle>
						<span className="text-xs font-mono text-muted-foreground">
							Total: <span className="font-bold text-foreground">5.45 GB</span>
						</span>
					</CardHeader>
					<CardContent className="pt-4 flex flex-col gap-4">
						<div className="grid grid-cols-3 gap-3 text-center">
							<div className="bg-secondary/40 border border-border/50 rounded-lg p-3">
								<span className="text-[11px] text-muted-foreground block mb-1 font-medium">Containers</span>
								<span className="text-sm font-bold font-mono text-foreground">5.45 GB</span>
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
