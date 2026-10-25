// @vitest-environment jsdom
import {Code, ConnectError} from '@connectrpc/connect';
import {beforeEach, describe, expect, it, vi} from 'vitest';

const mocks = vi.hoisted(() => ({
	refresh: vi.fn(),
	logout: vi.fn(),
	setAuth: vi.fn(),
}));

vi.mock('#/api/live/query', () => ({
	createServiceClient: () => ({refresh: mocks.refresh}),
}));

vi.mock('#/stores/session-context', () => ({
	getSessionActions: () => ({
		logout: mocks.logout,
		setAuth: mocks.setAuth,
	}),
}));

import {getAccessToken, refreshAccessToken} from './client';

const AUTH_STORAGE_KEY = 'openoxide-auth-session';

function storeSession(refreshToken: string) {
	localStorage.setItem(
		AUTH_STORAGE_KEY,
		JSON.stringify({tokens: {accessToken: 'old', refreshToken}}),
	);
}

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

describe('refreshAccessToken', () => {
	beforeEach(() => {
		localStorage.clear();
		mocks.refresh.mockReset();
		mocks.logout.mockReset();
		mocks.setAuth.mockReset();
	});

	it('ends the session and clears storage when the refresh token is rejected', async () => {
		storeSession('rt-rejected');
		mocks.refresh.mockRejectedValue(
			new ConnectError('invalid or revoked token', Code.Unauthenticated),
		);

		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'expired',
		});

		expect(mocks.logout).toHaveBeenCalledOnce();
		expect(localStorage.getItem(AUTH_STORAGE_KEY)).toBeNull();
	});

	it('ends the session when no refresh token is stored', async () => {
		localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify({tokens: {}}));

		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'expired',
		});

		expect(mocks.logout).toHaveBeenCalledOnce();
		expect(localStorage.getItem(AUTH_STORAGE_KEY)).toBeNull();
		expect(mocks.refresh).not.toHaveBeenCalled();
	});

	it('keeps the session on transport failures so it can retry', async () => {
		storeSession('rt-transient');
		mocks.refresh.mockRejectedValue(
			new ConnectError('backend down', Code.Unavailable),
		);

		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'transient',
		});

		expect(mocks.logout).not.toHaveBeenCalled();
		expect(localStorage.getItem(AUTH_STORAGE_KEY)).not.toBeNull();
	});

	it('rewrites the stored tokens on a successful refresh', async () => {
		storeSession('rt-ok');
		mocks.refresh.mockResolvedValue({
			user: {
				id: 1n,
				email: 'test@gmail.com',
				firstName: 'Test',
				lastName: 'User',
				avatar: '',
			},
			tokens: {accessToken: 'new-access', refreshToken: 'rt-ok'},
		});

		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'ok',
			token: 'new-access',
		});

		expect(getAccessToken()).toBe('new-access');
		expect(mocks.setAuth).toHaveBeenCalledOnce();
		expect(mocks.logout).not.toHaveBeenCalled();
	});

	it('does not cache an early result across sessions', async () => {
		// No session → synchronous `expired` early return. A settled promise
		// cached here used to make every later refresh replay that result, so
		// a fresh sign-in could never renew its access token again.
		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'expired',
		});

		storeSession('rt-after-login');
		mocks.refresh.mockResolvedValue({
			user: {id: 1n, email: 'test@gmail.com', avatar: ''},
			tokens: {accessToken: 'fresh', refreshToken: 'rt-after-login'},
		});

		await expect(refreshAccessToken()).resolves.toEqual({
			status: 'ok',
			token: 'fresh',
		});
		expect(mocks.refresh).toHaveBeenCalledOnce();
	});
});
