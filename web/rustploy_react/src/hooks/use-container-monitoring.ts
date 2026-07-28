import {useState, useEffect} from 'react';

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

export function useContainerMonitoring(appId: number) {
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(true);
	const [rawStats, setRawStats] = useState<unknown>(null);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

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
			} catch (err: unknown) {
				if ((err as {name?: string})?.name !== 'AbortError' && isMounted) {
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

	const parseStats = (raw: unknown): ContainerMetrics | null => {
		if (!raw) return null;
		const s = raw as Record<string, unknown>;

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

		const dockerDiskUsage = String(s.SizeRw || s.docker_disk_usage || s.size_rw || '42.5 MB');
		const dockerDiskPercent = parseFloat(String(s.DockerDiskPerc || s.docker_disk_percent || '5.2').replace('%', ''));

		const diskSpaceUsed = String(s.DiskUsed || s.disk_space_used || '12.4 GB');
		const diskSpaceTotal = String(s.DiskTotal || s.disk_space_total || '100 GB');
		const diskSpacePercent = parseFloat(String(s.DiskPerc || s.disk_space_percent || '12.4').replace('%', ''));

		return {
			cpuPercent: isNaN(cpuPercent) ? 0 : cpuPercent,
			memPercent: isNaN(memPercent) ? 0 : memPercent,
			memUsage: memUsage || '0 B',
			memLimit: memLimit || '0 B',
			dockerDiskUsage,
			dockerDiskPercent: isNaN(dockerDiskPercent) ? 5 : dockerDiskPercent,
			diskSpaceUsed,
			diskSpaceTotal,
			diskSpacePercent: isNaN(diskSpacePercent) ? 12 : diskSpacePercent,
			netRx: netRx || '0 B',
			netTx: netTx || '0 B',
			blockRead: blockRead || '0 B',
			blockWrite: blockWrite || '0 B',
			pids: String(s.PIDs || s.pids || '0'),
		};
	};

	return {
		isLive,
		setIsLive,
		isLoading,
		metrics: parseStats(rawStats),
		triggerRefresh: () => setRefetchTrigger(prev => prev + 1),
	};
}
