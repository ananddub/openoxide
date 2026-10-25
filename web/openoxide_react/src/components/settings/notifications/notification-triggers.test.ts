import {describe, expect, it} from 'vitest';
import {triggerEventLabel, triggerOutcome} from './notification-triggers';

describe('triggerOutcome', () => {
	it('reports successful events even when delivery failed', () => {
		expect(triggerOutcome('APP_DEPLOY')).toBe('success');
		expect(triggerOutcome('SCHEDULE_SUCCESS')).toBe('success');
	});

	it('reports failed events', () => {
		expect(triggerOutcome('APP_BUILD_ERROR')).toBe('failure');
		expect(triggerOutcome('SCHEDULE_FAILURE')).toBe('failure');
	});

	it('treats informational events as neutral', () => {
		expect(triggerOutcome('SERVER_THRESHOLD')).toBe('info');
		expect(triggerOutcome('SOMETHING_NEW')).toBe('info');
	});
});

describe('triggerEventLabel', () => {
	it('maps known events and humanizes unknown ones', () => {
		expect(triggerEventLabel('APP_DEPLOY')).toBe('App deployed');
		expect(triggerEventLabel('APP_BUILD_ERROR')).toBe('Build failed');
		expect(triggerEventLabel('NEW_EVENT')).toBe('new event');
	});
});
