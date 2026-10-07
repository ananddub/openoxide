import {describe, expect, it} from 'vitest';

import {diskBytes, formatBytes} from './disk-metrics';

describe('server disk metrics', () => {
	it('adds reclaimable sizes across docker display units', () => {
		expect(diskBytes('1.5GB')).toBeCloseTo(1.5e9);
		expect(diskBytes('500B')).toBeCloseTo(500);
		expect(diskBytes('2 M')).toBeCloseTo(2e6);
	});

	it('counts an unreadable cell as zero', () => {
		expect(diskBytes('—')).toBe(0);
		expect(diskBytes('')).toBe(0);
	});

	it('renders bytes with one decimal at MB and above', () => {
		expect(formatBytes(1.5e9)).toBe('1.5 GB');
		expect(formatBytes(2.34e6)).toBe('2.3 MB');
		expect(formatBytes(64e3)).toBe('64 kB');
	});
});
