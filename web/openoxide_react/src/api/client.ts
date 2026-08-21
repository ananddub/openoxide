import createFetchClient, {type Middleware} from 'openapi-fetch';
import type {paths} from '#/types/api.d.ts';
import {useAuthStore} from '#/stores/auth-store';
import {getActiveOrganizationId} from '#/stores/organization-context';

const AUTH_STORAGE_KEY = 'openoxide-auth-session';

export function getAccessToken(): string {
	if (typeof localStorage === 'undefined') return '';
	try {
		const raw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (!raw || raw === 'undefined') return '';
		const session = JSON.parse(raw);
		return session?.tokens?.access_token || '';
	} catch {
		return '';
	}
}

export const getApiBaseUrl = () => {
	if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL;
	if (import.meta.env.DEV) return '/api';
	if (typeof window !== 'undefined' && window.location.hostname) {
		const host =
			window.location.hostname === 'localhost'
				? '127.0.0.1'
				: window.location.hostname;
		return `${window.location.protocol}//${host}:4000`;
	}
	return 'http://127.0.0.1:4000';
};

let refreshPromise: Promise<string | null> | null = null;
let refreshRetryAfter = 0;
let refreshRetryToken = '';
const REFRESH_RETRY_COOLDOWN_MS = 15_000;

export function applyOrganizationHeader(request: Request) {
	const organizationId = getActiveOrganizationId();
	if (organizationId && !request.url.includes('/organizations')) {
		request.headers.set('X-Organization-Id', String(organizationId));
	}
}

export async function refreshAccessToken(): Promise<string | null> {
	if (refreshPromise) return refreshPromise;

	refreshPromise = (async () => {
		let refreshToken = '';
		try {
			const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
			if (!sessionRaw || sessionRaw === 'undefined') {
				useAuthStore.getState().logout();
				return null;
			}

			const session = JSON.parse(sessionRaw);
			refreshToken = session?.tokens?.refresh_token || '';
			if (!refreshToken) {
				useAuthStore.getState().logout();
				return null;
			}
			if (
				refreshRetryToken === refreshToken &&
				Date.now() < refreshRetryAfter
			)
				return null;

			const refreshRes = await fetch(`${getApiBaseUrl()}/auth/refresh`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({refresh_token: refreshToken}),
			});

			if (!refreshRes.ok) {
				if ([400, 401, 403].includes(refreshRes.status)) {
					useAuthStore.getState().logout();
				} else {
					refreshRetryToken = refreshToken;
					refreshRetryAfter = Date.now() + REFRESH_RETRY_COOLDOWN_MS;
				}
				return null;
			}

			const newSession = await refreshRes.json();
			localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(newSession));
			refreshRetryToken = '';
			refreshRetryAfter = 0;

			if (newSession?.user) {
				useAuthStore.getState().setAuth({
					id: newSession.user.user_id,
					email: newSession.user.email || '',
					firstName: newSession.user.first_name,
					lastName: newSession.user.last_name,
				});
			}

			return newSession?.tokens?.access_token || null;
		} catch {
			if (refreshToken) {
				refreshRetryToken = refreshToken;
				refreshRetryAfter = Date.now() + REFRESH_RETRY_COOLDOWN_MS;
			}
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}

const authMiddleware: Middleware = {
	async onRequest({request}) {
		applyOrganizationHeader(request);
		const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (sessionRaw && sessionRaw !== 'undefined') {
			if (sessionRaw.length > 4000) {
				localStorage.removeItem(AUTH_STORAGE_KEY);
				useAuthStore.getState().logout();
				return request;
			}
			try {
				const session = JSON.parse(sessionRaw);
				const accessToken = session?.tokens?.access_token;
				if (accessToken) {
					request.headers.set('Authorization', `Bearer ${accessToken}`);
				}
			} catch {}
		}
		return request;
	},

	async onResponse({request, response}) {
		if (response.status !== 401) return response;

		// Don't retry auth endpoints to avoid infinite loops
		if (
			request.url.includes('auth/login') ||
			request.url.includes('auth/signup') ||
			request.url.includes('auth/refresh')
		) {
			return response;
		}

		// Use single refresh promise mutex so parallel 401s don't bombard backend
		const newAccessToken = await refreshAccessToken();
		if (!newAccessToken) {
			return response;
		}

		// Retry the original request with the new access token
		const newRequest = request.clone();
		newRequest.headers.set('Authorization', `Bearer ${newAccessToken}`);
		return fetch(newRequest);
	},
};

export const client = createFetchClient<paths>({
	baseUrl: getApiBaseUrl(),
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});

client.use(authMiddleware);

export async function authFetch(
	path: string,
	options: RequestInit = {},
): Promise<Response> {
	const baseUrl = getApiBaseUrl();
	const token = getAccessToken();

	const headers = new Headers(options.headers || {});
	if (token && !headers.has('Authorization')) {
		headers.set('Authorization', `Bearer ${token}`);
	}

	const cleanPath = path.startsWith('/api') ? path.slice(4) : path;
	const url = cleanPath.startsWith('http')
		? cleanPath
		: `${baseUrl}${cleanPath.startsWith('/') ? cleanPath : `/${cleanPath}`}`;
	return fetch(url, {...options, headers});
}
