import {
	Code,
	ConnectError,
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
		// Connect RPC responses serialize proto fields camelCase (accessToken).
		return session?.tokens?.accessToken || '';
	} catch {
		return '';
	}
}

/** Connect 2.x surfaces failed unary calls as thrown ConnectErrors — there is no response object to inspect. An expired/invalid access token arrives as the `unauthenticated` code (the wire maps HTTP 401 onto it), which is the signal to refresh once and replay. */
function isUnauthenticatedError(error: unknown): boolean {
	return (
		error instanceof ConnectError && error.code === Code.Unauthenticated
	);
}

export const authInterceptor: Interceptor = next => async req => {
	// The refresh RPC itself must never enter the refresh-and-replay path: a rejected refresh would call refreshAccessToken again, which is already in flight (promise dedupe), deadlocking the await chain.
	const isRefreshCall = req.url.endsWith('/AuthService/Refresh');

	const headers = new Headers(req.header);
	const token = readAccessToken();
	if (token) headers.set('Authorization', `Bearer ${token}`);
	const organizationId = getActiveOrganizationId();
	if (organizationId) {
		headers.set('X-Organization-Id', String(organizationId));
	}

	try {
		return await next({...req, header: headers});
	} catch (error) {
		// Refresh-and-replay once on a rejection the backend marked as
		// unauthenticated; everything else (permissions, transport,
		// aborted calls) propagates untouched.
		if (
			isRefreshCall ||
			!isUnauthenticatedError(error) ||
			req.signal.aborted
		)
			throw error;

		const newToken = await refreshAccessToken();
		if (!newToken) throw error;

		const retryHeaders = new Headers(headers);
		retryHeaders.set('Authorization', `Bearer ${newToken}`);
		return next({...req, header: retryHeaders});
	}
};

/**
 * ConnectRPC transport for the openoxide panel API.
 *
 * The backend serves Connect-protocol POSTs under `/openoxide.api.v1.*`, so
 * the transport baseUrl is the API origin. Auth token + active organization
 * header are attached per request; a single 401 triggers one refresh and
 * one retry (mirrors the REST client middleware).
 */
// Todo: IN production switch to binary true
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
