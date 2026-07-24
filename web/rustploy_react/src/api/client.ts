import createFetchClient, {type Middleware} from 'openapi-fetch';
import type {paths} from '#/types/api.d.ts';
import {useAuthStore} from '#/stores/auth-store';

// Middleware: auto-refresh token on 401
const authMiddleware: Middleware = {
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
		const refreshRes = await fetch('/api/auth/refresh', {
			method: 'POST',
			credentials: 'include',
		});

		if (!refreshRes.ok) {
			useAuthStore.getState().logout();
			return response;
		}

		// Retry the original request — new cookies are set by the refresh response
		return fetch(request);
	},
};

export const client = createFetchClient<paths>({
	baseUrl: '/api',
	credentials: 'include',
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});

client.use(authMiddleware);
