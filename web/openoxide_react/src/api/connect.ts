import {
	createClient,
	type Interceptor,
	type Transport,
} from '@connectrpc/connect';
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
		// Connect RPC responses serialize proto fields camelCase
		// (accessToken); legacy REST-era sessions used snake_case
		// (access_token). Accept both.
		return (
			session?.tokens?.accessToken || session?.tokens?.access_token || ''
		);
	} catch {
		return '';
	}
}

/**
 * ConnectRPC interceptor (connect 2.x onion shape): attaches the bearer
 * token + active organization header, and replays a unary call once after
 * a token refresh when the backend rejects it as unauthenticated.
 */
export const authInterceptor: Interceptor = (next) => async (req) => {
	const headers = new Headers(req.header);
	const token = readAccessToken();
	if (token) headers.set('Authorization', `Bearer ${token}`);
	const organizationId = getActiveOrganizationId();
	if (organizationId) {
		headers.set('X-Organization-Id', String(organizationId));
	}

	const response = await next({...req, header: headers});

	// Unary responses carry the HTTP status; refresh-and-replay once.
	const status = (response as {response?: {status?: number}}).response?.status;
	if (status !== 401 || req.signal.aborted) return response;

	const newToken = await refreshAccessToken();
	if (!newToken) return response;

	const retryHeaders = new Headers(headers);
	retryHeaders.set('Authorization', `Bearer ${newToken}`);
	return next({...req, header: retryHeaders});
};

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
	interceptors: [authInterceptor],
});

export function createServiceClient<
	T extends Parameters<typeof createClient>[0],
>(service: T) {
	return createClient(service, transport);
}
