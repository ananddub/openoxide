// @vitest-environment jsdom
import {beforeEach, describe, expect, it, vi} from 'vitest';

import {
	ACTIVE_ORGANIZATION_STORAGE_KEY,
	ORGANIZATION_CHANGED_EVENT,
	getActiveOrganizationId,
	persistActiveOrganizationId,
} from './organization-context';

describe('active organization browser context', () => {
	beforeEach(() => localStorage.clear());

	it('persists the selected organization and notifies live hooks', () => {
		const listener = vi.fn();
		window.addEventListener(ORGANIZATION_CHANGED_EVENT, listener);

		persistActiveOrganizationId(3);

		expect(getActiveOrganizationId()).toBe(3);
		expect(localStorage.getItem(ACTIVE_ORGANIZATION_STORAGE_KEY)).toBe(
			'3',
		);
		expect(listener).toHaveBeenCalledOnce();
		const [event] = listener.mock.calls[0] as [CustomEvent];
		expect(event.detail).toBe(3);
	});

	it('ignores malformed persisted values', () => {
		localStorage.setItem(ACTIVE_ORGANIZATION_STORAGE_KEY, 'not-an-id');
		expect(getActiveOrganizationId()).toBeNull();
	});
});
