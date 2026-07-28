import createFetchClient, {type Middleware} from 'openapi-fetch';
import type {paths} from '#/types/api.d.ts';
import {useAuthStore} from '#/stores/auth-store';

const AUTH_STORAGE_KEY = 'rustploy-auth-session';

export const getApiBaseUrl = () => {
	if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL;
	if (typeof window !== 'undefined' && window.location.hostname) {
		const host = window.location.hostname === 'localhost' ? '127.0.0.1' : window.location.hostname;
		return `${window.location.protocol}//${host}:4000`;
	}
	return 'http://127.0.0.1:4000';
};

const authMiddleware: Middleware = {
	async onRequest({request}) {
		const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (sessionRaw && sessionRaw !== 'undefined') {
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

		// Attempt token refresh
		const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (!sessionRaw || sessionRaw === 'undefined') {
			useAuthStore.getState().logout();
			return response;
		}

		try {
			const session = JSON.parse(sessionRaw);
			const refreshToken = session?.tokens?.refresh_token;
			if (!refreshToken) {
				useAuthStore.getState().logout();
				return response;
			}

			const refreshRes = await fetch(`${getApiBaseUrl()}/auth/refresh`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({refresh_token: refreshToken}),
			});

			if (!refreshRes.ok) {
				useAuthStore.getState().logout();
				return response;
			}

			const newSession = await refreshRes.json();
			localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(newSession));

			// Update auth store with user
			if (newSession?.user) {
				useAuthStore.getState().setAuth({
					id: newSession.user.user_id,
					email: newSession.user.email || '',
					firstName: newSession.user.first_name,
					lastName: newSession.user.last_name,
				});
			}

			// Retry the original request with the new access token
			const newRequest = request.clone();
			newRequest.headers.set(
				'Authorization',
				`Bearer ${newSession?.tokens?.access_token}`,
			);
			return fetch(newRequest);
		} catch {
			useAuthStore.getState().logout();
			return response;
		}
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
