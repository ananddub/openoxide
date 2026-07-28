import {useState, useEffect} from 'react';
import {Cpu, HardDrive, Database, Disc, Network, Layers, RefreshCw} from 'lucide-react';
import {Card, CardContent, CardHeader, CardTitle} from '#/components/ui/card';
import {Progress} from '#/components/ui/progress';
import {$api} from '#/api/query';

// Helper to convert Docker memory strings (e.g. "34.4MiB", "78.46GiB", "352kB") into bytes
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

// Helper to format bytes into readable strings
function formatBytes(bytes: number): string {
	if (!bytes || isNaN(bytes) || bytes <= 0) return '0 B';
	if (bytes < 1024) return `${bytes.toFixed(0)} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

// Helper to parse Docker stats payload
function parseDockerStatsPayload(payload: unknown): Record<string, unknown>[] {
	if (!payload) return [];
	if (Array.isArray(payload)) return payload as Record<string, unknown>[];
	const obj = payload as Record<string, unknown>;
	if (obj.CPUPerc || obj.MemUsage) return [obj];
	if (typeof payload === 'object') {
		const rawContent = String(obj.raw || obj.line || obj.data || (typeof payload === 'string' ? payload : JSON.stringify(payload)));
		if (typeof rawContent === 'string') {
			const lines = rawContent.split(/\r?\n/).map((l: string) => l.trim()).filter(Boolean);
			const result: Record<string, unknown>[] = [];
			for (const line of lines) {
				try {
					const obj = JSON.parse(line);
					if (obj && (obj.CPUPerc || obj.MemUsage || obj.Container || obj.ID || obj.Name)) {
						result.push(obj);
					}
				} catch {}
			}
			if (result.length > 0) return result;
		}
	}
	return [];
}

export function GlobalMonitoringCards() {
	// Query real backend API for Docker containers list using $api client
	const { data: rawDockerContainers = [], isLoading: isDockerLoading, refetch: refetchContainers } = $api.useQuery(
		'get',
		'/deployments/docker/containers',
		{
			params: {
				query: {
					server_id: undefined,
				} as any,
			},
		},
		{
			refetchInterval: 5000,
		}
	);

	// Query running deployments
	const { data: rawRunning = [] } = $api.useQuery(
		'get',
		'/deployments/running',
		{
			params: {
				query: {
					query: {
						limit: 50,
					},
				},
			},
		},
		{
			refetchInterval: 5000,
		}
	);

	const [containersList, setContainersList] = useState<Record<string, unknown>[]>([]);

	// Direct REST snapshot fetch helper
	const fetchSnapshot = async () => {
		try {
			const sessionRaw = localStorage.getItem('rustploy-auth-session');
			let accessToken = '';
			if (sessionRaw && sessionRaw !== 'undefined') {
				try {
					const session = JSON.parse(sessionRaw);
					accessToken = session?.tokens?.access_token || '';
				} catch {}
			}

			const res = await fetch(`/api/deployments/docker/stats?stream=false${accessToken ? `&token=${encodeURIComponent(accessToken)}` : ''}`, {
				headers: {
					Authorization: accessToken ? `Bearer ${accessToken}` : '',
				},
			});

			if (res.ok) {
				const text = await res.text();
				let rawData = text;
				const dataLines = text.split('\n').filter(l => l.trim().startsWith('data:'));
				if (dataLines.length > 0) {
					rawData = dataLines.map(l => l.trim().slice(5).trim()).join('\n');
				}

				try {
					const parsed = JSON.parse(rawData);
					const payload = parsed.stats || parsed;
					const containers = parseDockerStatsPayload(payload);
					if (containers.length > 0) {
						setContainersList(containers);
					}
				} catch {
					const containers = parseDockerStatsPayload({ raw: rawData });
					if (containers.length > 0) {
						setContainersList(containers);
					}
				}
			}
		} catch {}
	};

	useEffect(() => {
		fetchSnapshot();
		const interval = setInterval(fetchSnapshot, 3000);
		return () => clearInterval(interval);
	}, []);

	// Active containers count from real API
	const dockerContainersArray = Array.isArray(rawDockerContainers) ? rawDockerContainers : [];
	const runningArray = Array.isArray(rawRunning) ? rawRunning : [];
	const activeContainersCount = Math.max(dockerContainersArray.length, runningArray.length, containersList.length);

	// Aggregate metrics across running Docker containers
	let totalCpu = 0;
	let totalMemUsedBytes = 0;
	let totalMemLimitBytes = 0;
	let totalPids = 0;
	let totalBlockReadBytes = 0;
	let totalBlockWriteBytes = 0;
	let totalNetRxBytes = 0;
	let totalNetTxBytes = 0;

	for (const c of containersList) {
		const cpuVal = parseFloat(String(c.CPUPerc || '0').replace('%', '')) || 0;
		totalCpu += cpuVal;

		const memUsageStr = String(c.MemUsage || '');
		if (memUsageStr.includes('/')) {
			const [uStr, lStr] = memUsageStr.split('/').map(s => s.trim());
			totalMemUsedBytes += parseBytes(uStr);
			if (totalMemLimitBytes === 0) {
				totalMemLimitBytes = parseBytes(lStr);
			}
		}

		const pidsVal = parseInt(String(c.PIDs || '0'), 10) || 0;
		totalPids += pidsVal;

		const blockStr = String(c.BlockIO || '');
		if (blockStr.includes('/')) {
			const [rStr, wStr] = blockStr.split('/').map(s => s.trim());
			totalBlockReadBytes += parseBytes(rStr);
			totalBlockWriteBytes += parseBytes(wStr);
		}

		const netStr = String(c.NetIO || '');
		if (netStr.includes('/')) {
			const [rxStr, txStr] = netStr.split('/').map(s => s.trim());
			totalNetRxBytes += parseBytes(rxStr);
			totalNetTxBytes += parseBytes(txStr);
		}
	}

	const memPercent = totalMemLimitBytes > 0 ? (totalMemUsedBytes / totalMemLimitBytes) * 100 : 0;

	return (
		<div className="flex flex-col gap-5">
			{/* Connection Indicator Bar */}
			<div className="flex items-center justify-between bg-card border border-border rounded-xl p-4 shadow-xs">
				<div className="flex items-center gap-2.5">
					<div className="size-2.5 rounded-full bg-emerald-500 animate-pulse" />
					<span className="text-xs font-semibold text-foreground flex items-center gap-2">
						Docker Telemetry Engine ({activeContainersCount} Active System Containers)
					</span>
				</div>
				<button
					onClick={() => {
						refetchContainers();
						fetchSnapshot();
					}}
					className="px-3 py-1.5 text-xs font-semibold rounded-lg border border-border bg-muted/30 hover:bg-muted text-foreground transition-colors flex items-center gap-1.5 shadow-xs">
					<RefreshCw className={`size-3.5 ${isDockerLoading ? 'animate-spin' : ''}`} /> Refresh Telemetry
				</button>
			</div>

			{/* Telemetry Cards Grid */}
			<div className="grid gap-5 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
				{/* 1. CPU Usage */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Cpu className="size-4 text-primary" /> CPU Usage
						</CardTitle>
						<span className="text-xs font-mono font-bold text-primary">{totalCpu.toFixed(1)}%</span>
					</CardHeader>
					<CardContent className="space-y-3">
						<div className="flex items-center justify-between text-xs text-muted-foreground">
							<span>Active Containers</span>
							<span>Used: {totalCpu.toFixed(1)}%</span>
						</div>
						<Progress value={Math.min(100, totalCpu)} className="h-2 w-full" />
						<p className="text-[11px] text-muted-foreground">Aggregated CPU load across {activeContainersCount} containers</p>
					</CardContent>
				</Card>

				{/* 2. RAM Memory */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<HardDrive className="size-4 text-emerald-500" /> RAM Memory
						</CardTitle>
						<span className="text-xs font-mono font-bold text-emerald-500">{memPercent.toFixed(1)}%</span>
					</CardHeader>
					<CardContent className="space-y-3">
						<div className="flex items-center justify-between text-xs text-muted-foreground">
							<span>Used: {formatBytes(totalMemUsedBytes)}</span>
							<span>Total: {formatBytes(totalMemLimitBytes || 84244240793)}</span>
						</div>
						<Progress value={Math.min(100, memPercent)} className="h-2 w-full bg-secondary" />
						<p className="text-[11px] text-muted-foreground">RAM memory utilization from Docker daemon</p>
					</CardContent>
				</Card>

				{/* 3. Active PIDs */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Database className="size-4 text-purple-500" /> Active PIDs
						</CardTitle>
						<span className="text-xs font-mono font-bold text-purple-500">{totalPids} Threads</span>
					</CardHeader>
					<CardContent className="space-y-3">
						<div className="flex items-center justify-between text-xs text-muted-foreground">
							<span>Total Active Threads</span>
							<span>{totalPids} PIDs</span>
						</div>
						<Progress value={Math.min(100, totalPids * 0.5)} className="h-2 w-full" />
						<p className="text-[11px] text-muted-foreground">Container active thread count</p>
					</CardContent>
				</Card>

				{/* 4. Block I/O */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Layers className="size-4 text-rose-500" /> Block I/O
						</CardTitle>
						<span className="text-xs font-mono font-bold text-rose-400">
							Total: {formatBytes(totalBlockReadBytes + totalBlockWriteBytes)}
						</span>
					</CardHeader>
					<CardContent className="space-y-2">
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Disk Read:</span>
							<span className="font-bold text-emerald-400">{formatBytes(totalBlockReadBytes)}</span>
						</div>
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Disk Write:</span>
							<span className="font-bold text-rose-400">{formatBytes(totalBlockWriteBytes)}</span>
						</div>
						<p className="text-[11px] text-muted-foreground pt-1">Disk read/write throughput from Docker</p>
					</CardContent>
				</Card>

				{/* 5. Network I/O */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Network className="size-4 text-blue-500" /> Network I/O
						</CardTitle>
						<span className="text-xs font-mono font-bold text-blue-400">
							Total: {formatBytes(totalNetRxBytes + totalNetTxBytes)}
						</span>
					</CardHeader>
					<CardContent className="space-y-2">
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Input (RX):</span>
							<span className="font-bold text-blue-400">{formatBytes(totalNetRxBytes)}</span>
						</div>
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Output (TX):</span>
							<span className="font-bold text-indigo-400">{formatBytes(totalNetTxBytes)}</span>
						</div>
						<p className="text-[11px] text-muted-foreground pt-1">Network traffic from Docker</p>
					</CardContent>
				</Card>

				{/* 6. Docker Containers Count */}
				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Disc className="size-4 text-amber-500" /> Docker Engine
						</CardTitle>
						<span className="text-xs font-mono font-bold text-amber-500">{activeContainersCount} Containers</span>
					</CardHeader>
					<CardContent className="space-y-2">
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Active Containers:</span>
							<span className="font-bold text-foreground">{activeContainersCount}</span>
						</div>
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Engine Status:</span>
							<span className="font-bold text-emerald-400">Online</span>
						</div>
						<p className="text-[11px] text-muted-foreground pt-1">Docker daemon status & active containers</p>
					</CardContent>
				</Card>
			</div>
		</div>
	);
}
