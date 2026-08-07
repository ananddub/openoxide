import {getApiBaseUrl} from '#/api/client';

import type {PrivateNetworkConfig, UpdatePrivateNetworkInput} from './types';

function authHeaders(json = false): HeadersInit {
	const sessionRaw = localStorage.getItem('rustploy-auth-session');
	let token = '';
	try {
		token = sessionRaw ? JSON.parse(sessionRaw)?.tokens?.access_token || '' : '';
	} catch {
		// Invalid sessions are handled by the global authentication flow.
	}
	return {
		Accept: 'application/json',
		...(json ? {'Content-Type': 'application/json'} : {}),
		...(token ? {Authorization: `Bearer ${token}`} : {}),
	};
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
	const response = await fetch(`${getApiBaseUrl()}${path}`, init);
	if (!response.ok) {
		const message = await response.text();
		throw new Error(message || `Request failed (${response.status})`);
	}
	if (response.status === 204) return undefined as T;
	return response.json() as Promise<T>;
}

const pathFor = (serverId: number, action = '') => `/servers/${serverId}/private-network${action}`;

export const privateNetworkApi = {
	get: (serverId: number) =>
		request<PrivateNetworkConfig | null>(pathFor(serverId), {
			headers: authHeaders(),
		}),
	update: (serverId: number, input: UpdatePrivateNetworkInput) =>
		request<PrivateNetworkConfig>(pathFor(serverId), {
			method: 'PUT',
			headers: authHeaders(true),
			body: JSON.stringify(input),
		}),
	setup: (serverId: number) =>
		request<PrivateNetworkConfig>(pathFor(serverId, '/setup'), {
			method: 'POST',
			headers: authHeaders(),
		}),
	health: (serverId: number) =>
		request<{status: string; error: string | null}>(pathFor(serverId, '/health'), {headers: authHeaders()}),
	repair: (serverId: number) =>
		request<PrivateNetworkConfig>(pathFor(serverId, '/repair'), {
			method: 'POST',
			headers: authHeaders(),
		}),
	rotateKeys: (serverId: number) =>
		request<PrivateNetworkConfig>(pathFor(serverId, '/rotate-keys'), {
			method: 'POST',
			headers: authHeaders(),
		}),
	teardown: (serverId: number) =>
		request<void>(pathFor(serverId, '/teardown'), {
			method: 'POST',
			headers: authHeaders(),
		}),
};
