import {describe, expect, it} from 'vitest';
import {
	formatEpochSeconds,
	formatRunDuration,
	managementDirty,
	summarizePolicy,
} from './management-metrics';
import type {ServerManagementPayload} from '#/api/live/server-management';

const base: ServerManagementPayload = {
	maintenanceMode: false,
	maintenanceMessage: '',
	labels: [['env', 'prod']],
	cleanupPolicy: {
		containers: true,
		images: true,
		networks: false,
		volumes: false,
		packages: false,
	},
	gpuEnabled: false,
	updatedAt: 0n,
};

describe('formatEpochSeconds', () => {
	it('renders a dash for missing or zero stamps', () => {
		expect(formatEpochSeconds(null)).toBe('—');
		expect(formatEpochSeconds(0n)).toBe('—');
	});

	it('converts epoch seconds to a local date', () => {
		const value = formatEpochSeconds(1_700_000_000n);
		expect(value).not.toBe('—');
		expect(value).toContain(
			String(new Date(1_700_000_000_000).getFullYear()),
		);
	});
});

describe('formatRunDuration', () => {
	it('keeps unfinished runs blank', () => {
		expect(formatRunDuration(100n, null)).toBe('—');
		expect(formatRunDuration(100n, 50n)).toBe('—');
	});

	it('scales seconds, minutes and hours', () => {
		expect(formatRunDuration(0n, 42n)).toBe('42s');
		expect(formatRunDuration(0n, 125n)).toBe('2m 5s');
		expect(formatRunDuration(0n, 3_720n)).toBe('1h 2m');
	});
});

describe('summarizePolicy', () => {
	it('lists only the enabled scopes', () => {
		expect(
			summarizePolicy('{"containers":true,"images":false,"volumes":true}'),
		).toBe('containers, volumes');
	});

	it('calls out a policy with nothing enabled', () => {
		expect(summarizePolicy('{"containers":false}')).toBe('no scopes');
	});

	it('falls back to the raw document when it is not JSON', () => {
		expect(summarizePolicy('legacy')).toBe('legacy');
		expect(summarizePolicy('  ')).toBe('—');
	});
});

describe('managementDirty', () => {
	it('stays clean for an identical record', () => {
		expect(managementDirty(base, {...base})).toBe(false);
	});

	it('detects policy, flag and label edits', () => {
		expect(
			managementDirty(
				{...base, cleanupPolicy: {...base.cleanupPolicy, volumes: true}},
				base,
			),
		).toBe(true);
		expect(managementDirty({...base, gpuEnabled: true}, base)).toBe(true);
		expect(
			managementDirty({...base, labels: [['env', 'staging']]}, base),
		).toBe(true);
	});

	it('ignores label ordering', () => {
		expect(
			managementDirty(
				{
					...base,
					labels: [
						['a', '1'],
						['env', 'prod'],
					],
				},
				{
					...base,
					labels: [
						['env', 'prod'],
						['a', '1'],
					],
				},
			),
		).toBe(false);
	});
});
