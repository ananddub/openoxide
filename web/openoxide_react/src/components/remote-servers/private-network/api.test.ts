import {beforeEach, describe, expect, it, vi} from 'vitest';

import {privateNetworkApi} from './api';

class MemoryStorage {
	private values = new Map<string, string>();
	getItem(key: string) {
		return this.values.get(key) ?? null;
	}
	setItem(key: string, value: string) {
		this.values.set(key, value);
	}
	removeItem(key: string) {
		this.values.delete(key);
	}
	clear() {
		this.values.clear();
	}
}

const storage = new MemoryStorage();

beforeEach(() => {
	storage.clear();
	vi.stubGlobal('localStorage', storage);
	vi.restoreAllMocks();
});

describe('private network browser API flow', () => {
	it('refreshes an expired token and retries the original request', async () => {
		storage.setItem(
			'openoxide-auth-session',
			JSON.stringify({tokens: {access_token: 'expired', refresh_token: 'refresh-token'}}),
		);
		const fetchMock = vi
			.fn()
			.mockResolvedValueOnce(new Response('{"error":"invalid token"}', {status: 401}))
			.mockResolvedValueOnce(
				new Response(JSON.stringify({tokens: {access_token: 'fresh', refresh_token: 'next-refresh'}}), {
					status: 200,
					headers: {'Content-Type': 'application/json'},
				}),
			)
			.mockResolvedValueOnce(new Response('null', {status: 200, headers: {'Content-Type': 'application/json'}}));
		vi.stubGlobal('fetch', fetchMock);

		await expect(privateNetworkApi.get(13)).resolves.toBeNull();
		expect(fetchMock).toHaveBeenCalledTimes(3);
		expect(fetchMock.mock.calls[1][0]).toBe('http://127.0.0.1:4000/auth/refresh');
		expect(new Headers(fetchMock.mock.calls[2][1]?.headers).get('Authorization')).toBe('Bearer fresh');
	});

	it('surfaces the backend validation message instead of a generic 409', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				new Response(JSON.stringify({error: 'managed WireGuard requires endpoint'}), {
					status: 409,
					headers: {'Content-Type': 'application/json'},
				}),
			),
		);

		await expect(
			privateNetworkApi.update(10, {
				connection_mode: 'MANAGED_WIREGUARD',
				provider: 'WIREGUARD',
				private_host: '10.77.11.2',
				tunnel_address: '10.77.11.0/24',
				public_key: null,
				endpoint: null,
				listen_port: 51820,
				persistent_keepalive: 25,
				dns_name: null,
				routes: [],
			}),
		).rejects.toThrow('managed WireGuard requires endpoint');
	});

	it('calls the explicit re-setup route with the current token', async () => {
		storage.setItem(
			'openoxide-auth-session',
			JSON.stringify({tokens: {access_token: 'valid', refresh_token: 'refresh-token'}}),
		);
		const response = {
			server_id: 13,
			connection_mode: 'MANAGED_WIREGUARD',
			provider: 'WIREGUARD',
			private_host: '10.77.14.2',
			tunnel_address: '10.77.14.0/24',
			public_key: null,
			endpoint: 'panel.example.com:51820',
			listen_port: 51820,
			persistent_keepalive: 25,
			status: 'CONFIGURING',
			dns_name: null,
			routes: [],
			health_status: 'UNKNOWN',
			health_error: null,
		};
		const fetchMock = vi.fn().mockResolvedValue(
			new Response(JSON.stringify(response), {status: 200, headers: {'Content-Type': 'application/json'}}),
		);
		vi.stubGlobal('fetch', fetchMock);

		await expect(privateNetworkApi.reSetup(13)).resolves.toMatchObject({status: 'CONFIGURING'});
		expect(fetchMock.mock.calls[0][0]).toBe('http://127.0.0.1:4000/servers/13/private-network/re-setup');
		expect(fetchMock.mock.calls[0][1]?.method).toBe('POST');
	});
});
