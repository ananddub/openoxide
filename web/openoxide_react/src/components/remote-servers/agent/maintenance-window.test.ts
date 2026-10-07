import {describe, expect, it} from 'vitest';
import {
	defaultMaintenanceWindowDraft,
	fromLocalInputValue,
	formatLastSeen,
	hasActiveWindow,
	maintenanceWindowProblem,
	maintenanceWindowState,
	toLocalInputValue,
	type MaintenanceWindowDraft,
} from './maintenance-window';
import type {MaintenanceWindowPayload} from '#/api/live/monitoring';

const draft = (
	overrides: Partial<MaintenanceWindowDraft> = {},
): MaintenanceWindowDraft => ({
	startsAt: '2026-10-05T10:00',
	endsAt: '2026-10-05T12:00',
	reason: 'kernel patches',
	...overrides,
});

const window = (
	overrides: Partial<MaintenanceWindowPayload> = {},
): MaintenanceWindowPayload => ({
	id: 1,
	server_id: 7,
	starts_at: 1000,
	ends_at: 2000,
	reason: 'work',
	created_at: 900,
	...overrides,
});

describe('local input conversion', () => {
	it('round-trips a value through the browser zone', () => {
		const value = '2026-10-05T10:00';
		const seconds = fromLocalInputValue(value);
		expect(seconds).not.toBeNull();
		expect(toLocalInputValue(seconds ?? 0)).toBe(value);
	});

	it('rejects empty and unparseable input instead of yielding NaN', () => {
		expect(fromLocalInputValue('')).toBeNull();
		expect(fromLocalInputValue('not-a-date')).toBeNull();
	});
});

describe('defaultMaintenanceWindowDraft', () => {
	it('opens a two-hour window starting now', () => {
		const now = 1_800_000_000;
		const value = defaultMaintenanceWindowDraft(now);
		expect(fromLocalInputValue(value.startsAt)).toBe(now);
		expect(
			(fromLocalInputValue(value.endsAt) ?? 0) -
				(fromLocalInputValue(value.startsAt) ?? 0),
		).toBe(2 * 60 * 60);
	});
});

describe('maintenanceWindowProblem', () => {
	it('accepts a coherent draft', () => {
		expect(maintenanceWindowProblem(draft())).toBeNull();
	});

	it('requires both ends of the window', () => {
		expect(maintenanceWindowProblem(draft({startsAt: ''}))).toMatch(
			/starts/,
		);
		expect(maintenanceWindowProblem(draft({endsAt: ''}))).toMatch(/ends/);
	});

	it('rejects a window that ends before it starts', () => {
		expect(
			maintenanceWindowProblem(
				draft({startsAt: '2026-10-05T14:00', endsAt: '2026-10-05T12:00'}),
			),
		).toMatch(/end after it starts/);
	});

	it('treats an equal start and end as empty, matching the backend', () => {
		expect(
			maintenanceWindowProblem(
				draft({startsAt: '2026-10-05T12:00', endsAt: '2026-10-05T12:00'}),
			),
		).toMatch(/end after it starts/);
	});

	it('needs a reason and trims it for the length check', () => {
		expect(maintenanceWindowProblem(draft({reason: '   '}))).toMatch(
			/why/,
		);
		expect(
			maintenanceWindowProblem(draft({reason: 'x'.repeat(501)})),
		).toMatch(/500/);
		expect(
			maintenanceWindowProblem(draft({reason: 'x'.repeat(500)})),
		).toBeNull();
	});
});

describe('maintenanceWindowState', () => {
	it('classifies a window against the clock', () => {
		expect(maintenanceWindowState(window(), 500)).toBe('scheduled');
		expect(maintenanceWindowState(window(), 1500)).toBe('active');
		expect(maintenanceWindowState(window(), 2000)).toBe('ended');
		expect(maintenanceWindowState(window(), 5000)).toBe('ended');
	});

	it('only counts a window as active while it covers now', () => {
		expect(
			hasActiveWindow([window(), window({id: 2, starts_at: 9000})], 1500),
		).toBe(true);
		expect(hasActiveWindow([window({starts_at: 9000})], 1500)).toBe(false);
		expect(hasActiveWindow([], 1500)).toBe(false);
	});
});

describe('formatLastSeen', () => {
	it('says never without a heartbeat', () => {
		expect(formatLastSeen(null, 1000)).toBe('never');
	});

	it('uses the largest whole unit below the next one', () => {
		expect(formatLastSeen(1000, 1030)).toBe('30s ago');
		expect(formatLastSeen(1000, 1000 + 59)).toBe('59s ago');
		expect(formatLastSeen(1000, 1000 + 120)).toBe('2m ago');
		expect(formatLastSeen(1000, 1000 + 7200)).toBe('2h ago');
		expect(formatLastSeen(1000, 1000 + 172800)).toBe('2d ago');
	});

	it('clamps a heartbeat from the future to now', () => {
		expect(formatLastSeen(2000, 1000)).toBe('0s ago');
	});
});
