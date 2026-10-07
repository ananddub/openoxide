import {describe, expect, it} from 'vitest';
import type {ContainerStatsPayload} from '#/api/live/monitoring';
import {
	formatKilobytesPerSecond,
	formatMegabytes,
	formatMemoryRow,
	formatPercent,
} from './container-stats';

function stats(
	overrides: Partial<ContainerStatsPayload> = {},
): ContainerStatsPayload {
	return {
		timestamp: 1_730_000_000,
		container_name: 'web',
		cpu_percent: 0,
		memory_percent: 0,
		memory_used_mb: 0,
		memory_limit_mb: 0,
		network_in_kbps: 0,
		network_out_kbps: 0,
		disk_read_mb: 0,
		disk_write_mb: 0,
		...overrides,
	};
}

describe('formatMegabytes', () => {
	it('scales gigabyte-sized values', () => {
		expect(formatMegabytes(2048)).toBe('2.00 GB');
		expect(formatMegabytes(512.4)).toBe('512.4 MB');
	});

	it('treats zero and unreadable values as nothing used', () => {
		expect(formatMegabytes(0)).toBe('0 MB');
		expect(formatMegabytes(-5)).toBe('0 MB');
		expect(formatMegabytes(Number.NaN)).toBe('0 MB');
	});
});

describe('formatKilobytesPerSecond', () => {
	it('switches to MB/s past a megabyte', () => {
		expect(formatKilobytesPerSecond(2048)).toBe('2.00 MB/s');
		expect(formatKilobytesPerSecond(12.5)).toBe('12.5 KB/s');
		expect(formatKilobytesPerSecond(0)).toBe('0 KB/s');
	});
});

describe('formatPercent', () => {
	it('keeps one decimal and hides unreadable readings', () => {
		expect(formatPercent(12.34)).toBe('12.3%');
		expect(formatPercent(Number.NaN)).toBe('—');
	});
});

describe('formatMemoryRow', () => {
	it('compares usage against the limit when Docker saw one', () => {
		expect(
			formatMemoryRow(stats({memory_used_mb: 240, memory_limit_mb: 512})),
		).toBe('240.0 MB of 512.0 MB');
	});

	it('says so when the container is unrestricted', () => {
		expect(formatMemoryRow(stats({memory_used_mb: 240}))).toBe(
			'240.0 MB (no limit)',
		);
	});
});
