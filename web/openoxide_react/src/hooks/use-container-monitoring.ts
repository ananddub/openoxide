import {useState, useEffect, useCallback} from 'react';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';

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

const deploymentClient = createServiceClient(DeploymentService);

function stripAnsi(str: string): string {
	// Remove all ANSI/VT100 escape sequences: ESC [ ... letter
	return str
		.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '')
		.replace(/\x1b[^[]/g, '');
}

function parseStats(raw: unknown): ContainerMetrics | null {
	if (!raw) return null;

	let target: unknown = raw;
	if (Array.isArray(raw)) {
		target = raw[0];
	}

	if (typeof target === 'string') {
		const stripped = stripAnsi(target).trim();
		const start = stripped.indexOf('{');
		const end = stripped.lastIndexOf('}');
		if (start !== -1 && end !== -1) {
			try {
				target = JSON.parse(stripped.slice(start, end + 1));
			} catch {}
		}
	}

	let s = (target || {}) as Record<string, unknown>;

	if (typeof s.raw === 'string') {
		const stripped = stripAnsi(s.raw).trim();
		const start = stripped.indexOf('{');
		const end = stripped.lastIndexOf('}');
		if (start !== -1 && end !== -1) {
			try {
				s = JSON.parse(stripped.slice(start, end + 1));
			} catch {
				return null;
			}
		} else {
			return null;
		}
	}

	const cpuPercent = parseFloat(
		String(s.CPUPerc || s.cpu_percent || '0').replace('%', ''),
	);
	const memPercent = parseFloat(
		String(s.MemPerc || s.memory_percent || '0').replace('%', ''),
	);

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

	const dockerDiskUsage = String(
		s.SizeRw || s.docker_disk_usage || s.size_rw || '0 MB',
	);
	const dockerDiskPercent = parseFloat(
		String(s.DockerDiskPerc || s.docker_disk_percent || '0').replace(
			'%',
			'',
		),
	);
	const diskSpaceUsed = String(s.DiskUsed || s.disk_space_used || '0 GB');
	const diskSpaceTotal = String(
		s.DiskTotal || s.disk_space_total || '0 GB',
	);
	const diskSpacePercent = parseFloat(
		String(s.DiskPerc || s.disk_space_percent || '0').replace('%', ''),
	);

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

/**
 * Live docker stats for an application / database / compose project over
 * the panel's ConnectRPC stream. Each event payload is raw
 * `docker stats --format '{{json .}}'` output (one JSON object per line);
 * the first object of a chunk drives the UI.
 */
export function useContainerMonitoring(
	id: number,
	entityType: MonitoringEntityType = 'application',
	enabled = true,
) {
	const [isLive, setIsLive] = useState(true);
	const [isLoading, setIsLoading] = useState(false);
	const [hasError, setHasError] = useState(false);
	const [rawStats, setRawStats] = useState<unknown>(null);
	const [refetchTrigger, setRefetchTrigger] = useState(0);

	const triggerRefresh = useCallback(() => {
		setRefetchTrigger(prev => prev + 1);
	}, []);

	useEffect(() => {
		if (!id || !enabled) {
			setIsLoading(false);
			return;
		}

		let isMounted = true;
		const controller = new AbortController();

		setIsLoading(true);
		setHasError(false);
		setRawStats(null);

		const streamStats = async () => {
			const request = {id: BigInt(id)};
			try {
				const call =
					entityType === 'database'
						? deploymentClient.streamDatabaseStats(request, {
								signal: controller.signal,
							})
						: entityType === 'compose'
							? deploymentClient.streamComposeStats(request, {
									signal: controller.signal,
								})
							: deploymentClient.streamApplicationStats(request, {
									signal: controller.signal,
								});
				for await (const event of call) {
					if (!isMounted) return;
					setIsLoading(false);
					if (event.error) {
						setHasError(true);
						continue;
					}
					const firstLine = event.payload
						.split('\n')
						.map(line => line.trim())
						.find(Boolean);
					if (firstLine) setRawStats(firstLine);
					// Live mode keeps consuming; one-shot mode stops after
					// the first snapshot.
					if (!isLive) break;
				}
			} catch (err: unknown) {
				const isAbort = (err as {name?: string})?.name === 'AbortError';
				if (!isAbort && isMounted) {
					setHasError(true);
				}
			} finally {
				if (isMounted) setIsLoading(false);
			}
		};

		void streamStats();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [id, entityType, isLive, refetchTrigger, enabled]);


	return {
		isLive,
		setIsLive,
		isLoading,
		hasError,
		metrics: parseStats(rawStats),
		triggerRefresh,
	};
}
