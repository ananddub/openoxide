import createQueryClient from 'openapi-react-query';
import createFetchClient from 'openapi-fetch';
import {type Middleware} from 'openapi-fetch';
import type {paths} from '#/types/api.d.ts';
import {getAccessToken, applyOrganizationHeader, refreshAccessToken} from './client';
import {useAuthStore} from '#/stores/auth-store';

// DEPRECATED REST shim — only kept while the last components migrate to the
// ConnectRPC live hooks (#/api/live). Every call through here hits a 404 on
// the backend; do not add new usages. This file is deleted with the final
// REST stack removal.
const restClient = createFetchClient<paths>({
	baseUrl: import.meta.env.VITE_API_URL ?? '',
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});

const authMiddleware: Middleware = {
	async onRequest({request}) {
		applyOrganizationHeader(request);
		const token = getAccessToken();
		if (token) {
			request.headers.set('Authorization', `Bearer ${token}`);
		}
		return request;
	},
	async onResponse({request, response}) {
		if (response.status !== 401) return response;
		if (
			request.url.includes('auth/login') ||
			request.url.includes('auth/signup') ||
			request.url.includes('auth/refresh')
		) {
			return response;
		}
		const newAccessToken = await refreshAccessToken();
		if (!newAccessToken) {
			useAuthStore.getState().logout();
			return response;
		}
		const newRequest = request.clone();
		newRequest.headers.set('Authorization', `Bearer ${newAccessToken}`);
		return fetch(newRequest);
	},
};

restClient.use(authMiddleware);

// Type-safe react-query wrapper over the openapi-fetch client.
// Usage:
//   const { data, isLoading } = $api.useQuery('get', '/projects/{id}', { params: { path: { id: 1 } } })
//   const mutation = $api.useMutation('post', '/applications')
export const $api = createQueryClient(restClient);
