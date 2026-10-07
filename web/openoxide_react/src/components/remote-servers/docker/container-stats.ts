import type {ContainerStatsPayload} from '#/api/live/monitoring';

/** Byte totals the agent reports as `…_kbps`/`…_mb` are easier to read scaled up. */
export function formatMegabytes(value: number): string {
	if (!Number.isFinite(value) || value <= 0) return '0 MB';
	if (value >= 1024) return `${(value / 1024).toFixed(2)} GB`;
	return `${value.toFixed(1)} MB`;
}

/** Kilo-bytes per second, scaled to MB/s once it passes a megabyte. */
export function formatKilobytesPerSecond(value: number): string {
	if (!Number.isFinite(value) || value <= 0) return '0 KB/s';
	if (value >= 1024) return `${(value / 1024).toFixed(2)} MB/s`;
	return `${value.toFixed(1)} KB/s`;
}

export function formatPercent(value: number): string {
	if (!Number.isFinite(value)) return '—';
	return `${value.toFixed(1)}%`;
}

/** `limit: 0` means Docker saw no memory limit for this container. */
export function formatMemoryRow(row: ContainerStatsPayload): string {
	const used = formatMegabytes(row.memory_used_mb);
	return row.memory_limit_mb > 0
		? `${used} of ${formatMegabytes(row.memory_limit_mb)}`
		: `${used} (no limit)`;
}
