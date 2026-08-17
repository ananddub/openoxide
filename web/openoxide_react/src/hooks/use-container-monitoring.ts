import {useState, useEffect, useCallback} from 'react';

export type MonitoringEntityType = 'application' | 'database' | 'compose';

export interface ContainerMetrics {
	cpuPercent: number;
	memPercent: number;
	memUsage: string;
	memLimit: string;
	dockerDiskUsage: string;
	dockerDiskPercent: number;
	diskSpaceUsed: string;
	diskSpaceTotal: string;
	diskSpacePercent: number;
	netRx: string;
	netTx: string;
	blockRead: string;
	blockWrite: string;
	pids: string;
}

function buildStatsUrl(entityType: MonitoringEntityType, id: number, isLive: boolean): string {
	switch (entityType) {
		case 'database':
			return `/api/deployments/database/${id}/stats?stream=${isLive}`;
		case 'compose':
			return `/api/deployments/compose/${id}/stats?stream=${isLive}`;
		case 'application':
		default:
			return `/api/deployments/application/${id}/stats?stream=${isLive}`;
	}
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

function stripAnsi(str: string): string {
	// Remove all ANSI/VT100 escape sequences: ESC [ ... letter
	return str.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '').replace(/\x1b[^[]/g, '');
}

function parseStats(raw: unknown): ContainerMetrics | null {
	if (!raw) return null;
	let s = raw as Record<string, unknown>;

	// Docker streaming mode wraps JSON in ANSI terminal codes → {raw: "\x1B[H{...}\x1B[K\n"}
	// Strip codes and re-parse the embedded JSON
	if (typeof s.raw === 'string') {
		const stripped = stripAnsi(s.raw).trim();
		const start = stripped.indexOf('{');
		const end = stripped.lastIndexOf('}');
		if (start !== -1 && end !== -1) {
			try {
				s = JSON.parse(stripped.slice(start, end + 1));
				console.log('[Monitoring] 🧹 Extracted JSON from raw ANSI output:', s);
			} catch {
				console.warn('[Monitoring] ⚠️ Could not extract JSON from raw:', stripped);
				return null;
			}
		} else {
			return null;
		}
	}

	const cpuPercent = parseFloat(String(s.CPUPerc || s.cpu_percent || '0').replace('%', ''));
	const memPercent = parseFloat(String(s.MemPerc || s.memory_percent || '0').replace('%', ''));

	const memUsageStr = String(s.MemUsage || s.mem_usage || '');
	const [memUsage, memLimit] = memUsageStr.includes('/')
		? memUsageStr.split('/').map(v => v.trim())
		: [memUsageStr || '0 B', '0 B'];

	const netIOStr = String(s.NetIO || s.net_io || '');
	const [netRx, netTx] = netIOStr.includes('/')
		? netIOStr.split('/').map(v => v.trim())
		: [netIOStr || '0 B', '0 B'];

	const blockIOStr = String(s.BlockIO || s.block_io || '');
	const [blockRead, blockWrite] = blockIOStr.includes('/')
		? blockIOStr.split('/').map(v => v.trim())
		: [blockIOStr || '0 B', '0 B'];

	const dockerDiskUsage = String(s.SizeRw || s.docker_disk_usage || s.size_rw || '0 MB');
	const dockerDiskPercent = parseFloat(String(s.DockerDiskPerc || s.docker_disk_percent || '0').replace('%', ''));
	const diskSpaceUsed = String(s.DiskUsed || s.disk_space_used || '0 GB');
	const diskSpaceTotal = String(s.DiskTotal || s.disk_space_total || '0 GB');
	const diskSpacePercent = parseFloat(String(s.DiskPerc || s.disk_space_percent || '0').replace('%', ''));

	return {
		cpuPercent: isNaN(cpuPercent) ? 0 : cpuPercent,
		memPercent: isNaN(memPercent) ? 0 : memPercent,
		memUsage: memUsage || '0 B',
		memLimit: memLimit || '0 B',
		dockerDiskUsage,
		dockerDiskPercent: isNaN(dockerDiskPercent) ? 0 : dockerDiskPercent,
		diskSpaceUsed,
		diskSpaceTotal,
		diskSpacePercent: isNaN(diskSpacePercent) ? 0 : diskSpacePercent,
		netRx: netRx || '0 B',
		netTx: netTx || '0 B',
		blockRead: blockRead || '0 B',
		blockWrite: blockWrite || '0 B',
		pids: String(s.PIDs || s.pids || '0'),
	};
}

const DISABLE_METRICS = false;

export function useContainerMonitoring(id: number, entityType: MonitoringEntityType = 'application') {
	// All hooks declared unconditionally at the top — never reorder these
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(false);
	const [hasError, setHasError] = useState(false);
	const [rawStats, setRawStats] = useState<unknown>(null);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const triggerRefresh = useCallback(() => {
		setRefetchTrigger(prev => prev + 1);
	}, []);

	useEffect(() => {
		if (!id || DISABLE_METRICS) {
			setIsLoading(false);
			return;
		}

		let isMounted = true;
		const controller = new AbortController();

		setIsLoading(true);
		setHasError(false);
		setRawStats(null);

		const startStream = async () => {
			try {
				let accessToken = '';
				const sessionRaw = localStorage.getItem('openoxide-auth-session');
				if (sessionRaw) {
					try {
						const session = JSON.parse(sessionRaw);
						accessToken = session?.tokens?.access_token || '';
					} catch {}
				}

				const url = buildStatsUrl(entityType, id, isLive);
				console.log(`[Monitoring] 🔌 Connecting → ${url}`);
				const response = await fetch(url, {
					headers: {
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					signal: controller.signal,
				});

				console.log(`[Monitoring] 📡 Response status: ${response.status} ${response.statusText}`);

				if (!response.ok) {
					console.warn(`[Monitoring] ❌ Bad response: ${response.status} for ${url}`);
					if (isMounted) {
						setIsLoading(false);
						setHasError(true);
					}
					return;
				}

				const reader = response.body?.getReader();
				const decoder = new TextDecoder();
				if (!reader) {
					console.warn(`[Monitoring] ❌ No readable body from ${url}`);
					if (isMounted) {
						setIsLoading(false);
						setHasError(true);
					}
					return;
				}

				console.log(`[Monitoring] ✅ Stream opened for ${entityType} id=${id}`);
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
									console.log(`[Monitoring] 📦 SSE event:`, data.type, data);
									if (data.type === 'stats' && data.stats && isMounted) {
										console.log(`[Monitoring] 📊 Stats received:`, data.stats);
										setRawStats(data.stats);
									}
								} catch (e) {
									console.warn(`[Monitoring] ⚠️ JSON parse fail:`, jsonStr, e);
								}
							}
						}
					}
				}
			} catch (err: unknown) {
				const isAbort = (err as {name?: string})?.name === 'AbortError';
				if (!isAbort && isMounted) {
					setIsLoading(false);
					setHasError(true);
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
	}, [id, entityType, isLive, refetchTrigger]);

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
		const parsed = parseStats(rawStats);
		if (parsed) {
			const timeStr = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit', second: '2-digit'});
			const memUsedGB = parseBytes(parsed.memUsage) / (1024 ** 3);
			const memLimitGB = parseBytes(parsed.memLimit) / (1024 ** 3);
			const diskUsedGB = parseBytes(parsed.diskSpaceUsed) / (1024 ** 3);
			const diskTotalGB = parseBytes(parsed.diskSpaceTotal) / (1024 ** 3);
			const dockerDiskGB = parseBytes(parsed.dockerDiskUsage) / (1024 ** 3);
			const blockReadMB = parseBytes(parsed.blockRead) / (1024 * 1024);
			const blockWriteMB = parseBytes(parsed.blockWrite) / (1024 * 1024);
			const netRxMB = parseBytes(parsed.netRx) / (1024 * 1024);
			const netTxMB = parseBytes(parsed.netTx) / (1024 * 1024);

			setHistory(prev => [
				...prev.slice(-49),
				{
					time: timeStr,
					cpu: parsed.cpuPercent,
					memUsedGB,
					memLimitGB,
					diskUsedGB,
					diskTotalGB,
					dockerDiskGB,
					blockReadMB,
					blockWriteMB,
					netRxMB,
					netTxMB,
				},
			]);
		}
	}, [rawStats]);

	return {
		isLive,
		setIsLive,
		isLoading,
		hasError,
		metrics: parseStats(rawStats),
		history,
		triggerRefresh,
	};
}
