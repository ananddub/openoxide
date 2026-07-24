import {useState, useEffect} from 'react';
import {Activity, Cpu, HardDrive, Network, RefreshCw, Radio, Layers} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface MonitoringTabProps {
	app: any;
	appId: number;
}

export function MonitoringTab({app, appId}: MonitoringTabProps) {
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [rawStats, setRawStats] = useState<any>(null);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	// Connect to backend Server-Sent Events (SSE) telemetry stats stream
	useEffect(() => {
		if (!appId) return;
		let isMounted = true;
		const controller = new AbortController();
		setIsLoading(true);

		const startStream = async () => {
			try {
				let accessToken = '';
				const sessionRaw = localStorage.getItem('rustploy-auth-session');
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const response = await fetch(
					`/api/deployments/application/${appId}/stats?stream=${isLive}`,
					{
						headers: {
							Authorization: accessToken ? `Bearer ${accessToken}` : '',
						},
						signal: controller.signal,
					}
				);

				if (!response.ok) {
					if (isMounted) setIsLoading(false);
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) return;

				if (isMounted) setIsLoading(false);

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						const trimmed = line.trim();
						if (!trimmed || trimmed.startsWith('event:') || trimmed.startsWith(':')) continue;

						if (line.startsWith('data:')) {
							const jsonStr = line.slice(5).trim();
							if (jsonStr) {
								try {
									const data = JSON.parse(jsonStr);
									if (data.type === 'stats' && data.stats && isMounted) {
										setRawStats(data.stats);
									}
								} catch {}
							}
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError' && isMounted) {
					setIsLoading(false);
				}
			} finally {
				if (isMounted) setIsLoading(false);
			}
		};

		startStream();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [appId, isLive, refetchTrigger]);

	// Parse Docker CLI stats format
	const parseStats = (s: any) => {
		if (!s) return null;

		// CPUPerc: "0.45%"
		const cpuPercent = parseFloat(String(s.CPUPerc || s.cpu_percent || '0').replace('%', ''));
		const memPercent = parseFloat(String(s.MemPerc || s.memory_percent || '0').replace('%', ''));

		// MemUsage: "45.2MiB / 7.82GiB"
		const memUsageStr = String(s.MemUsage || s.mem_usage || '');
		const [memUsage, memLimit] = memUsageStr.includes('/')
			? memUsageStr.split('/').map(v => v.trim())
			: [memUsageStr || '0 B', '0 B'];

		// NetIO: "12.4kB / 8.2kB"
		const netIOStr = String(s.NetIO || s.net_io || '');
		const [netRx, netTx] = netIOStr.includes('/')
			? netIOStr.split('/').map(v => v.trim())
			: [netIOStr || '0 B', '0 B'];

		// BlockIO: "1.2MB / 0B"
		const blockIOStr = String(s.BlockIO || s.block_io || '');
		const [blockRead, blockWrite] = blockIOStr.includes('/')
			? blockIOStr.split('/').map(v => v.trim())
			: [blockIOStr || '0 B', '0 B'];

		return {
			cpuPercent: isNaN(cpuPercent) ? 0 : cpuPercent,
			memPercent: isNaN(memPercent) ? 0 : memPercent,
			memUsage: memUsage || '0 B',
			memLimit: memLimit || '0 B',
			netRx: netRx || '0 B',
			netTx: netTx || '0 B',
			blockRead: blockRead || '0 B',
			blockWrite: blockWrite || '0 B',
			pids: s.PIDs || s.pids || '0',
		};
	};

	const parsed = parseStats(rawStats);

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Metrics & Live Telemetry</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time resource utilization for container: <span className="font-mono text-foreground font-semibold">{app?.app_name || app?.name || 'app'}</span></p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="h-8 text-xs font-semibold flex items-center gap-1.5 rounded-lg"
					>
						<Radio className={`w-3.5 h-3.5 ${isLive ? 'animate-pulse text-emerald-400' : ''}`} />
						{isLive ? 'Live SSE Stream' : 'Paused'}
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => setRefetchTrigger(prev => prev + 1)}
						disabled={isLoading}
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs rounded-lg"
					>
						<RefreshCw className={`w-3.5 h-3.5 ${isLoading ? 'animate-spin' : ''}`} /> Refresh
					</Button>
				</div>
			</section>

			{isLoading && !parsed ? (
				<div className="flex justify-center py-20 bg-card border border-border rounded-xl shadow-sm">
					<div className="flex items-center gap-2 text-xs font-semibold text-muted-foreground">
						<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Connecting to real-time container metrics stream...
					</div>
				</div>
			) : !parsed ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground bg-card border border-border rounded-xl shadow-sm">
					<Activity className="w-8 h-8 opacity-30 mb-2" />
					<p className="text-xs font-semibold text-foreground">No container metrics available</p>
					<p className="text-[11px] text-muted-foreground mt-1">Ensure the application container is running to view live CPU and memory telemetry</p>
				</div>
			) : (
				<>
					{/* Core Metrics Grid */}
					<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
						{/* CPU */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<div className="flex items-center justify-between">
								<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
									<Cpu className="w-3.5 h-3.5 text-primary" /> CPU Usage
								</span>
								<span className="text-xs font-mono font-bold text-primary">{parsed.cpuPercent.toFixed(2)}%</span>
							</div>
							<div className="flex flex-col gap-1">
								<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
									<div
										className="h-full bg-primary transition-all duration-500 rounded-full"
										style={{width: `${Math.min(parsed.cpuPercent, 100)}%`}}
									/>
								</div>
								<span className="text-[10px] text-muted-foreground mt-1">Processor cores utilization</span>
							</div>
						</div>

						{/* Memory */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<div className="flex items-center justify-between">
								<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
									<HardDrive className="w-3.5 h-3.5 text-emerald-500" /> RAM Memory
								</span>
								<span className="text-xs font-mono font-bold text-emerald-500">{parsed.memPercent.toFixed(2)}%</span>
							</div>
							<div className="flex flex-col gap-1">
								<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
									<div
										className="h-full bg-emerald-500 transition-all duration-500 rounded-full"
										style={{width: `${Math.min(parsed.memPercent, 100)}%`}}
									/>
								</div>
								<span className="text-[10px] text-muted-foreground mt-1">
									{parsed.memUsage} / {parsed.memLimit}
								</span>
							</div>
						</div>

						{/* Network RX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
								<Network className="w-3.5 h-3.5 text-blue-500" /> Network In (RX)
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground font-mono">{parsed.netRx}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Received ingress traffic</span>
							</div>
						</div>

						{/* Network TX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
								<Network className="w-3.5 h-3.5 text-indigo-500" /> Network Out (TX)
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground font-mono">{parsed.netTx}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Transmitted egress traffic</span>
							</div>
						</div>
					</div>

					{/* Extended Block I/O & PIDs metrics */}
					<div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
							<div>
								<span className="text-xs font-semibold text-foreground block">Block Disk Read</span>
								<span className="text-[10px] text-muted-foreground">Volume data read from host disk</span>
							</div>
							<span className="text-sm font-mono font-bold text-foreground">{parsed.blockRead}</span>
						</div>

						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
							<div>
								<span className="text-xs font-semibold text-foreground block">Block Disk Write</span>
								<span className="text-[10px] text-muted-foreground">Volume data written to host disk</span>
							</div>
							<span className="text-sm font-mono font-bold text-foreground">{parsed.blockWrite}</span>
						</div>

						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
							<div>
								<span className="text-xs font-semibold text-foreground block">Active Process Threads</span>
								<span className="text-[10px] text-muted-foreground">Active container process PIDs count</span>
							</div>
							<span className="text-sm font-mono font-bold text-primary flex items-center gap-1.5">
								<Layers className="w-3.5 h-3.5" /> {parsed.pids}
							</span>
						</div>
					</div>
				</>
			)}
		</div>
	);
}
