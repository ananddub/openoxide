// Pure docker-stats parsing for the container monitoring hook: normalizes the raw `docker stats --format '{{json .}}'` output (possibly ANSI-wrapped or nested under a `raw` field) into a flat metrics object.

export type MonitoringEntityType = 'application' | 'database' | 'compose';

export interface ContainerMetrics {
	name: string;
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

function stripAnsi(str: string): string {
	// Remove all ANSI/VT100 escape sequences: ESC [ ... letter
	const esc = '\u001b';
	return str
		.replace(new RegExp(`${esc}\\[[0-9;]*[a-zA-Z]`, 'g'), '')
		.replace(new RegExp(`${esc}[^[]`, 'g'), '');
}

function extractJsonObject(str: string): Record<string, unknown> | null {
	const stripped = stripAnsi(str).trim();
	const start = stripped.indexOf('{');
	const end = stripped.lastIndexOf('}');
	if (start === -1 || end === -1) return null;
	try {
		return JSON.parse(stripped.slice(start, end + 1));
	} catch {
		return null;
	}
}

const num = (value: string) => {
	const parsed = parseFloat(value.replace('%', ''));
	return isNaN(parsed) ? 0 : parsed;
};

const slashPair = (raw: string): [string, string] =>
	raw.includes('/')
		? (raw.split('/').map(v => v.trim()) as [string, string])
		: [raw || '0 B', '0 B'];

export function parseStats(raw: unknown): ContainerMetrics | null {
	if (!raw) return null;

	let target: unknown = raw;
	if (Array.isArray(raw)) target = raw[0];
	if (typeof target === 'string') target = extractJsonObject(target) ?? {};

	let s = (target || {}) as Record<string, unknown>;
	if (typeof s.raw === 'string') {
		const nested = extractJsonObject(s.raw);
		if (!nested) return null;
		s = nested;
	}

	const [memUsage, memLimit] = slashPair(
		String(s.MemUsage || s.mem_usage || ''),
	);
	const [netRx, netTx] = slashPair(String(s.NetIO || s.net_io || ''));
	const [blockRead, blockWrite] = slashPair(
		String(s.BlockIO || s.block_io || ''),
	);

	return {
		name: String(s.Name || s.name || s.Container || ''),
		cpuPercent: num(String(s.CPUPerc || s.cpu_percent || '0')),
		memPercent: num(String(s.MemPerc || s.memory_percent || '0')),
		memUsage: memUsage || '0 B',
		memLimit: memLimit || '0 B',
		dockerDiskUsage: String(
			s.SizeRw || s.docker_disk_usage || s.size_rw || '0 MB',
		),
		dockerDiskPercent: num(
			String(s.DockerDiskPerc || s.docker_disk_percent || '0'),
		),
		diskSpaceUsed: String(s.DiskUsed || s.disk_space_used || '0 GB'),
		diskSpaceTotal: String(s.DiskTotal || s.disk_space_total || '0 GB'),
		diskSpacePercent: num(
			String(s.DiskPerc || s.disk_space_percent || '0'),
		),
		netRx: netRx || '0 B',
		netTx: netTx || '0 B',
		blockRead: blockRead || '0 B',
		blockWrite: blockWrite || '0 B',
		pids: String(s.PIDs || s.pids || '0'),
	};
}
