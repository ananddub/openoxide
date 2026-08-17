import {useState, useEffect} from 'react';
import {Cpu, HardDrive, Database, Disc, Network, Layers} from 'lucide-react';
import {Card, CardContent, CardHeader, CardTitle} from '#/components/ui/card';
import {Progress} from '#/components/ui/progress';
import {$api} from '#/api/query';
import {useDeploymentRunning} from 'virtual:openoxide-live';

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

	// ─── Aggregate metrics ────────────────────────────────────────────────────
	const dockerContainersArray = Array.isArray(rawDockerContainers) ? rawDockerContainers : [];
	const runningArray = Array.isArray(rawRunning) ? rawRunning : [];
	const activeContainersCount = Math.max(dockerContainersArray.length, runningArray.length, containersList.length);

	let totalCpu = 0, totalMemUsed = 0, totalMemLimit = 0, totalPids = 0;
	let totalBlockR = 0, totalBlockW = 0, totalNetRx = 0, totalNetTx = 0;

	for (const c of containersList) {
		totalCpu += parseFloat(String(c.CPUPerc || '0').replace('%', '')) || 0;

		const mem = String(c.MemUsage || '');
		if (mem.includes('/')) {
			const [u, l] = mem.split('/').map(s => s.trim());
			totalMemUsed += parseBytes(u);
			if (!totalMemLimit) totalMemLimit = parseBytes(l);
		}

		totalPids += parseInt(String(c.PIDs || '0'), 10) || 0;

		const blk = String(c.BlockIO || '');
		if (blk.includes('/')) {
			const [r, w] = blk.split('/').map(s => s.trim());
			totalBlockR += parseBytes(r);
			totalBlockW += parseBytes(w);
		}

		const net = String(c.NetIO || '');
		if (net.includes('/')) {
			const [rx, tx] = net.split('/').map(s => s.trim());
			totalNetRx += parseBytes(rx);
			totalNetTx += parseBytes(tx);
		}
	}

	const memPercent = totalMemLimit > 0 ? (totalMemUsed / totalMemLimit) * 100 : 0;

	return (
		<div className="flex flex-col gap-5">
			{/* Header bar */}
		<div className="flex items-center gap-2.5 bg-card border border-border rounded-xl p-4 shadow-xs">
			<div className="size-2.5 rounded-full bg-emerald-500 animate-pulse" />
			<span className="text-xs font-semibold text-foreground">
				Docker Telemetry Engine
			</span>
			<span className="text-xs text-muted-foreground">
				— {activeContainersCount} Active System Containers
			</span>
		</div>

			{/* Cards */}
			<div className="grid gap-5 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
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

				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<HardDrive className="size-4 text-emerald-500" /> RAM Memory
						</CardTitle>
						<span className="text-xs font-mono font-bold text-emerald-500">{memPercent.toFixed(1)}%</span>
					</CardHeader>
					<CardContent className="space-y-3">
						<div className="flex items-center justify-between text-xs text-muted-foreground">
							<span>Used: {formatBytes(totalMemUsed)}</span>
							<span>Total: {formatBytes(totalMemLimit)}</span>
						</div>
						<Progress value={Math.min(100, memPercent)} className="h-2 w-full bg-secondary" />
						<p className="text-[11px] text-muted-foreground">RAM memory utilization from Docker daemon</p>
					</CardContent>
				</Card>

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

				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Layers className="size-4 text-rose-500" /> Block I/O
						</CardTitle>
						<span className="text-xs font-mono font-bold text-rose-400">
							{formatBytes(totalBlockR + totalBlockW)}
						</span>
					</CardHeader>
					<CardContent className="space-y-2">
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Disk Read:</span>
							<span className="font-bold text-emerald-400">{formatBytes(totalBlockR)}</span>
						</div>
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Disk Write:</span>
							<span className="font-bold text-rose-400">{formatBytes(totalBlockW)}</span>
						</div>
						<p className="text-[11px] text-muted-foreground pt-1">Disk read/write throughput from Docker</p>
					</CardContent>
				</Card>

				<Card className="bg-card border-border shadow-xs">
					<CardHeader className="flex flex-row items-center justify-between pb-2">
						<CardTitle className="text-xs font-bold uppercase tracking-wider text-muted-foreground flex items-center gap-2">
							<Network className="size-4 text-blue-500" /> Network I/O
						</CardTitle>
						<span className="text-xs font-mono font-bold text-blue-400">
							{formatBytes(totalNetRx + totalNetTx)}
						</span>
					</CardHeader>
					<CardContent className="space-y-2">
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Input (RX):</span>
							<span className="font-bold text-blue-400">{formatBytes(totalNetRx)}</span>
						</div>
						<div className="flex items-center justify-between text-xs font-mono">
							<span className="text-muted-foreground">Output (TX):</span>
							<span className="font-bold text-indigo-400">{formatBytes(totalNetTx)}</span>
						</div>
						<p className="text-[11px] text-muted-foreground pt-1">Network traffic from Docker</p>
					</CardContent>
				</Card>

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
						<p className="text-[11px] text-muted-foreground pt-1">Docker daemon status &amp; active containers</p>
					</CardContent>
				</Card>
			</div>
		</div>
	);
}
