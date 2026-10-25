// @vitest-environment jsdom
import {beforeEach, describe, expect, it} from 'vitest';
import {jwtExpiryMs, refreshTokenExpiry} from './session-expiry';

const AUTH_STORAGE_KEY = 'openoxide-auth-session';

function base64url(value: string): string {
	return btoa(value)
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=+$/, '');
}

function jwt(claims: Record<string, unknown>): string {
	return `${base64url('{"alg":"HS256","typ":"JWT"}')}.${base64url(JSON.stringify(claims))}.sig`;
}

describe('jwtExpiryMs', () => {
	it('returns the exp claim in epoch milliseconds', () => {
		expect(jwtExpiryMs(jwt({exp: 1_800_000_000}))).toBe(1_800_000_000_000);
	});

	it('returns null for opaque or malformed tokens', () => {
		expect(jwtExpiryMs('not-a-jwt')).toBeNull();
		expect(jwtExpiryMs('a.!!!.c')).toBeNull();
		expect(jwtExpiryMs(jwt({sub: '1'}))).toBeNull();
	});
});

describe('refreshTokenExpiry', () => {
	beforeEach(() => localStorage.clear());

	it('reads the refresh token, not the short-lived access token', () => {
		localStorage.setItem(
			AUTH_STORAGE_KEY,
			JSON.stringify({
				tokens: {
					accessToken: jwt({exp: 1_000}),
					refreshToken: jwt({exp: 1_800_000_000}),
				},
			}),
		);

		expect(refreshTokenExpiry()).toBe(1_800_000_000_000);
	});

	it('returns null when the session is missing or malformed', () => {
		expect(refreshTokenExpiry()).toBeNull();
		localStorage.setItem(AUTH_STORAGE_KEY, 'not-json');
		expect(refreshTokenExpiry()).toBeNull();
		localStorage.setItem(
			AUTH_STORAGE_KEY,
			JSON.stringify({tokens: {refreshToken: 'opaque'}}),
		);
		expect(refreshTokenExpiry()).toBeNull();
	});
});
