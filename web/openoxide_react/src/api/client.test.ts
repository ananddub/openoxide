// @vitest-environment jsdom
import {beforeEach, describe, expect, it, vi} from 'vitest';

import {getAccessToken} from './client';

describe('auth session storage', () => {
	beforeEach(() => {
		localStorage.clear();
		vi.restoreAllMocks();
	});

	it('reads the access token from the camelCase session', () => {
		localStorage.setItem(
			'openoxide-auth-session',
			JSON.stringify({tokens: {accessToken: 'abc'}}),
		);

		expect(getAccessToken()).toBe('abc');
	});

	it('returns empty for missing or malformed sessions', () => {
		expect(getAccessToken()).toBe('');
		localStorage.setItem('openoxide-auth-session', 'not-json');
		expect(getAccessToken()).toBe('');
	});
});
