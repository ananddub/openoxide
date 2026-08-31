import {createClient, type Transport} from '@connectrpc/connect';
import {createConnectTransport} from '@connectrpc/connect-web';
import {getActiveOrganizationId} from '#/stores/organization-context';
import {getApiBaseUrl, refreshAccessToken} from './client';

const AUTH_STORAGE_KEY = 'openoxide-auth-session';

function readAccessToken(): string {
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

/**
 * ConnectRPC transport for the openoxide panel API.
 *
 * The backend serves Connect-protocol POSTs under `/openoxide.api.v1.*`, so
 * the transport baseUrl is the API origin. Auth token + active organization
 * header are attached per request; a single 401 triggers one refresh and
 * one retry (mirrors the REST client middleware).
 */
export const transport: Transport = createConnectTransport({
	baseUrl: getApiBaseUrl(),
	useBinaryFormat: false,
	interceptors: [
		{
			intercept: (next, request) => {
				const headers = new Headers(request.header);
				const token = readAccessToken();
				if (token) headers.set('Authorization', `Bearer ${token}`);
				const organizationId = getActiveOrganizationId();
				if (organizationId) {
					headers.set('X-Organization-Id', String(organizationId));
				}
				return next({...request, header: headers}, (response) => {
					if (response.stream || response.error?.code !== 1) return response;
					// Connect code 1 = unauthenticated: refresh once and replay.
					return Promise.resolve(refreshAccessToken()).then((newToken) => {
						if (!newToken) return response;
						const retryHeaders = new Headers(request.header);
						retryHeaders.set('Authorization', `Bearer ${newToken}`);
						return next({...request, header: retryHeaders});
					});
				});
			},
		},
	],
});

export function createServiceClient<
	T extends Parameters<typeof createClient>[0],
>(service: T) {
	return createClient(service, transport);
}
