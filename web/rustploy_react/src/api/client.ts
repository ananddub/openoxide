import createFetchClient, {type Middleware} from 'openapi-fetch';
import type {paths} from '#/types/api.d.ts';
import {useAuthStore} from '#/stores/auth-store';

const AUTH_STORAGE_KEY = 'rustploy-auth-session';

const authMiddleware: Middleware = {
	async onRequest({request}) {
		const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (sessionRaw) {
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
		if (!sessionRaw) {
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

			const refreshRes = await fetch('http://das.tail25b5a0.ts.net:4000/auth/refresh', {
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
	baseUrl: 'http://das.tail25b5a0.ts.net:4000',
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});

client.use(authMiddleware);
