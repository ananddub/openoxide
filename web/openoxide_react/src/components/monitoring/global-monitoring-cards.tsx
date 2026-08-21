import {useState, useEffect, useMemo} from 'react';
import {
	Server,
} from 'lucide-react';
import {
	Card,
	CardContent,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {$api} from '#/api/query';
import {useMonitoringContainerStates} from 'virtual:openoxide-live';
import {useAppStore} from '#/stores/app-store';
import {getAccessToken, refreshAccessToken} from '#/api/client';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

// ─── Helpers ─────────────────────────────────────────────────────────────────

function stripAnsi(str: string): string {
	return str
		.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '')
		.replace(/\x1b[^[]/g, '');
}

function parseBytes(str?: string): number {
	if (!str || typeof str !== 'string') return 0;
	const val = parseFloat(str) || 0;
	const unit = str
		.replace(/[0-9.]/g, '')
		.trim()
		.toUpperCase();
	if (unit.startsWith('K')) return val * 1024;
	if (unit.startsWith('M')) return val * 1024 * 1024;
	if (unit.startsWith('G')) return val * 1024 * 1024 * 1024;
	if (unit.startsWith('T')) return val * 1024 * 1024 * 1024 * 1024;
	return val;
}

type DockerStat = Record<string, unknown>;

function isStatObject(x: any): boolean {
	if (!x || typeof x !== 'object') return false;
	return Boolean(
		x.CPUPerc ||
		x.cpu_percent ||
		x.cpuPerc ||
		x.cpu ||
		x.MemUsage ||
		x.mem_usage ||
		x.memPerc ||
		x.MemPerc ||
		x.Container ||
		x.container ||
		x.name ||
		x.container_name ||
		x.ID ||
		x.id,
	);
}

/** Extract one or more Docker stat objects from an SSE stats payload */
function extractDockerStats(payload: unknown): DockerStat[] {
	if (!payload) return [];

	if (Array.isArray(payload)) {
		return (payload as DockerStat[]).filter(isStatObject);
	}

	if (isStatObject(payload)) return [payload as DockerStat];

	// Raw ANSI-wrapped string → strip + find JSON lines
	const rawStr =
		typeof (payload as any)?.raw === 'string'
			? (payload as any).raw
			: typeof payload === 'string'
				? payload
				: '';
	if (rawStr) {
		const stripped = stripAnsi(rawStr);
		const results: DockerStat[] = [];
		for (const chunk of stripped.split('\n')) {
			const trimmed = chunk.trim();
			const start = trimmed.indexOf('{');
			const end = trimmed.lastIndexOf('}');
			if (start === -1 || end === -1) continue;
			try {
				const parsed = JSON.parse(
					trimmed.slice(start, end + 1),
				) as DockerStat;
				if (isStatObject(parsed)) {
					results.push(parsed);
				}
			} catch {}
		}
		if (results.length > 0) return results;
	}

	return [];
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

// ─── Component ────────────────────────────────────────────────────────────────

export function GlobalMonitoringCards() {
	// `0` was only a UI placeholder. Passing it to the monitoring endpoint made
	// the local agent query an invalid server and left the dashboard stale.
	const LOCAL_SERVER_ID = 0;
	const servers = useAppStore(state => state.servers || []);
	const [selectedServerId, setSelectedServerId] = useState<number | null>(null);
	const serverLabel = (server: any) => {
		const isLocal =
			String(server.server_type || server.type || '').toLowerCase() === 'local' ||
			server.ip === '127.0.0.1' ||
			server.ip_address === '127.0.0.1';
		if (isLocal || Number(server.id) === LOCAL_SERVER_ID && !server.name)
			return 'Localhost (Main server)';
		return server.name || server.hostname || server.app_name || `Server ${server.id}`;
	};
	const localServer = servers.find((server: any) => {
		const address = String(server.ip || server.ip_address || '').toLowerCase();
		return address === '127.0.0.1' || address === 'localhost' || address === 'openoxide-monitor' || server.app_name === 'localhost';
	});
	const localServerId = Number(localServer?.id || 0);
	const effectiveServerId = selectedServerId ?? localServerId;
	const selectedServer = servers.find(
		(server: any) => Number(server.id) === effectiveServerId,
	);
	const selectedServerName = selectedServer
		? serverLabel(selectedServer)
		: effectiveServerId === localServerId
			? 'Localhost (Main server)'
			: `Server ${effectiveServerId}`;
	useEffect(() => {
		if (selectedServerId !== null && !servers.some((server: any) => Number(server.id) === selectedServerId)) {
			setSelectedServerId(null);
		}
	}, [servers, selectedServerId]);
	const {data: rawDockerContainers = []} = $api.useQuery(
		'get',
		'/deployments/docker/containers',
		{params: {query: {server_id: effectiveServerId || undefined} as any}},
	);


	const {data: rawContainerStates} = useMonitoringContainerStates(
		BigInt(effectiveServerId),
	);

	const [containersList, setContainersList] = useState<DockerStat[]>([]);
	const [streamState, setStreamState] = useState<'connecting' | 'connected' | 'disconnected'>('connecting');

	// SSE live stream
	useEffect(() => {
		const DISABLE_METRICS = false;
		if (DISABLE_METRICS) return;
		let isMounted = true;
		let controller: AbortController | null = null;
		let retryTimer: ReturnType<typeof setTimeout> | undefined;
		let retryDelay = 2000;

		const run = async () => {
			if (!isMounted) return;
			setStreamState('connecting');
			controller = new AbortController();
			try {
				let token = getAccessToken();
				if (!token) token = (await refreshAccessToken()) || '';
				const statsUrl = effectiveServerId
					? `/api/deployments/docker/stats?stream=true&server_id=${effectiveServerId}`
					: '/api/deployments/docker/stats?stream=true';
				const res = await fetch(
					statsUrl,
					{
						headers: {Authorization: token ? `Bearer ${token}` : ''},
						signal: controller.signal,
					},
				);
				if (res.status === 401) {
					const refreshed = await refreshAccessToken();
					if (refreshed) {
						controller.abort();
						throw new Error('retry metrics stream with refreshed token');
					}
				}
				if (!res.ok || !res.body) throw new Error(`metrics stream returned ${res.status}`);
				setStreamState('connected');
				retryDelay = 2000;

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
								if (stats.length > 0 && isMounted) {
									setContainersList([...stats]);

									let cpuSum = 0,
										memUsedSum = 0,
										memLimitSum = 0;
									let blkReadSum = 0,
										blkWriteSum = 0,
										netRxSum = 0,
										netTxSum = 0;
									let diskUsedSum = 0,
										diskTotalSum = 0;

									for (const c of stats) {
										const rawCpu =
											c.CPUPerc ||
											c.cpu_percent ||
											c.cpuPerc ||
											c.cpu ||
											0;
										cpuSum +=
											parseFloat(String(rawCpu).replace('%', '')) || 0;

										const memVal =
											c.MemUsage || c.mem_usage || c.memUsage || '';
										const memStr = String(memVal);
										if (memStr.includes('/')) {
											const [u, l] = memStr.split('/').map(s => s.trim());
											memUsedSum += parseBytes(u);
											const limitBytes = parseBytes(l);
											if (limitBytes > memLimitSum) {
												memLimitSum = limitBytes;
											}
										}

										const blkVal =
											c.BlockIO || c.block_io || c.blockIO || '';
										const blkStr = String(blkVal);
										if (blkStr.includes('/')) {
											const [r, w] = blkStr.split('/').map(s => s.trim());
											blkReadSum += parseBytes(r);
											blkWriteSum += parseBytes(w);
										}

										const netVal = c.NetIO || c.net_io || c.netIO || '';
										const netStr = String(netVal);
										if (netStr.includes('/')) {
											const [rx, tx] = netStr
												.split('/')
												.map(s => s.trim());
											netRxSum += parseBytes(rx);
											netTxSum += parseBytes(tx);
										}

										if (c.SizeRw)
											diskUsedSum += parseBytes(String(c.SizeRw));
										if (c.DiskUsed || c.disk_used)
											diskUsedSum += parseBytes(
												String(c.DiskUsed || c.disk_used),
											);
										if ((c.DiskTotal || c.total_disk) && !diskTotalSum)
											diskTotalSum = parseBytes(
												String(c.DiskTotal || c.total_disk),
											);
									}

									const timeStr = new Date().toLocaleTimeString([], {
										hour: '2-digit',
										minute: '2-digit',
										second: '2-digit',
									});
									const finalMemLimitGB =
										memLimitSum > memUsedSum
											? memLimitSum / 1024 ** 3
											: memUsedSum > 0
												? (memUsedSum * 1.5) / 1024 ** 3
												: 1;
									const finalDiskUsedGB =
										diskUsedSum > 0 ? diskUsedSum / 1024 ** 3 : 0;
									const finalDiskTotalGB =
										diskTotalSum > 0
											? diskTotalSum / 1024 ** 3
											: Math.max(finalDiskUsedGB * 1.5, 1);

									setHistory(prev => [
										...prev.slice(-49),
										{
											time: timeStr,
											cpu: cpuSum,
											memUsedGB: memUsedSum / 1024 ** 3,
											memLimitGB: finalMemLimitGB,
											diskUsedGB: finalDiskUsedGB,
											diskTotalGB: finalDiskTotalGB,
											dockerDiskGB: finalDiskUsedGB,
											blockReadMB: blkReadSum / (1024 * 1024),
											blockWriteMB: blkWriteSum / (1024 * 1024),
											netRxMB: netRxSum / (1024 * 1024),
											netTxMB: netTxSum / (1024 * 1024),
										},
									]);
								}
							}
						} catch {}
					}
				}
				if (isMounted) throw new Error('metrics stream closed');
			} catch {
				if (!isMounted) return;
				setStreamState('disconnected');
				retryTimer = setTimeout(run, retryDelay);
				retryDelay = Math.min(retryDelay * 2, 15000);
			}
		};

		run();
		return () => {
			isMounted = false;
			if (retryTimer) clearTimeout(retryTimer);
			controller?.abort();
		};
	}, [effectiveServerId]);

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
		setHistory([]);
		setContainersList([]);
	}, [effectiveServerId]);

	// ─── Aggregate metrics ────────────────────────────────────────────────────
	const dockerContainersArray = Array.isArray(rawDockerContainers)
		? rawDockerContainers
		: [];
	const containerStates = Array.isArray(rawContainerStates)
		? rawContainerStates as Array<Record<string, unknown>>
		: [];
	const runningStatesCount = containerStates.filter(container =>
		String(container.state || '').toLowerCase() === 'running',
	).length;
	const localRunningCount = dockerContainersArray.filter((container: any) => {
		const state = String(container?.state || container?.State || '').toLowerCase();
		return state === 'running' || state === 'up';
	}).length;
	const activeContainersCount = effectiveServerId === localServerId
		? (containerStates.length > 0 ? runningStatesCount : localRunningCount)
		: Math.max(runningStatesCount, containersList.length);

	const last = history[history.length - 1];
	const latestCpu = last?.cpu || 0;
	const latestMemUsed = last?.memUsedGB || 0;
	const rawMemLimit = last?.memLimitGB || 1;
	const latestMemLimit = Math.max(rawMemLimit, latestMemUsed, 0.1);

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
			<div className="flex items-center justify-between gap-4 rounded-xl border border-border bg-card p-4 shadow-xs">
				<div className="flex items-center gap-2 text-sm font-semibold text-foreground">
					<Server className="size-4 text-muted-foreground" />
					<span>Monitoring Server</span>
				</div>
				<Select
					value={String(effectiveServerId)}
					onValueChange={value => setSelectedServerId(Number(value))}>
					<SelectTrigger className="w-full max-w-xs">
						<SelectValue placeholder="Select server">
							{selectedServerName}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{localServerId === 0 && (
							<SelectItem value={String(LOCAL_SERVER_ID)}>
								Localhost (Main server)
							</SelectItem>
						)}
						{servers.map((server: any) => (
							<SelectItem key={server.id} value={String(server.id)}>
								{serverLabel(server)}
								{(server.ip || server.ip_address) &&
									!['127.0.0.1', 'localhost'].includes(
										String(server.ip || server.ip_address).toLowerCase(),
									) &&
									` (${server.ip || server.ip_address})`}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
			{/* Header bar */}
			<div className="flex items-center justify-between rounded-xl border border-border bg-card p-4 shadow-xs">
				<div className="flex items-center gap-2.5">
					<div className={`size-2.5 rounded-full ${streamState === 'connected' ? 'animate-pulse bg-emerald-500' : streamState === 'connecting' ? 'animate-pulse bg-amber-500' : 'bg-rose-500'}`} />
					<span className="text-xs font-semibold text-foreground">
						Docker Telemetry Engine
					</span>
					<span className="text-xs text-muted-foreground">
						— {activeContainersCount} Active System Containers
					</span>
				</div>
				<span className="font-mono text-xs text-muted-foreground">
					{streamState === 'connected' ? 'Realtime Monitoring' : streamState === 'connecting' ? 'Connecting...' : 'Reconnecting...'}
				</span>
			</div>

			{/* Dokploy 6-Card Monitoring Grid */}
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
								{latestCpu.toFixed(2)}%
							</span>
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
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Memory Usage
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Used:{' '}
							<span className="font-bold text-foreground">
								{formatGB(latestMemUsed)}
							</span>{' '}
							/ Limit:{' '}
							<span className="font-bold text-foreground">
								{formatGB(latestMemLimit)}
							</span>
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

				{/* 3. Block I/O */}
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Block I/O
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							Read:{' '}
							<span className="font-bold text-emerald-500">
								{latestBlockR.toFixed(2)} MB
							</span>{' '}
							/ Write:{' '}
							<span className="font-bold text-rose-500">
								{latestBlockW.toFixed(2)} MB
							</span>
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

				{/* 5. Network I/O */}
				<Card className="border-border bg-card shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-sm font-bold text-foreground">
							Network I/O
						</CardTitle>
						<span className="font-mono text-xs text-muted-foreground">
							In:{' '}
							<span className="font-bold text-blue-500">
								{latestNetRx.toFixed(2)} MB
							</span>{' '}
							/ Out:{' '}
							<span className="font-bold text-indigo-500">
								{latestNetTx.toFixed(2)} MB
							</span>
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
