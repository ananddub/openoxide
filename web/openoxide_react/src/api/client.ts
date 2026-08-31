import {useAuthStore} from '#/stores/auth-store';
import {getActiveOrganizationId} from '#/stores/organization-context';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';

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

const authClient = createServiceClient(AuthService);

export function applyOrganizationHeader(request: Request) {
	const organizationId = getActiveOrganizationId();
	if (organizationId && !request.url.includes('/organizations')) {
		request.headers.set('X-Organization-Id', String(organizationId));
	}
}

/**
 * Refreshes the access token through the ConnectRPC AuthService and
 * rewrites the stored session in the legacy REST-era shape (user.user_id /
 * tokens.access_token snake_case), which the stores and interceptors read.
 */
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

			const response = await authClient.refresh({refreshToken});
			if (!response.user || !response.tokens) {
				useAuthStore.getState().logout();
				return null;
			}

			const newSession = {
				user: {
					user_id: Number(response.user.id),
					email: response.user.email,
					first_name: response.user.firstName,
					last_name: response.user.lastName,
					avatar: response.user.avatar,
				},
				tokens: response.tokens,
			};
			localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(newSession));
			refreshRetryToken = '';
			refreshRetryAfter = 0;

			useAuthStore.getState().setAuth({
				id: Number(response.user.id),
				email: response.user.email || '',
				firstName: response.user.firstName,
				lastName: response.user.lastName,
			});

			return newSession.tokens.accessToken || null;
		} catch {
			// A failed refresh means the session is gone; the cooldown keeps
			// a broken backend from being hammered on every request.
			refreshRetryToken = refreshToken;
			refreshRetryAfter = Date.now() + REFRESH_RETRY_COOLDOWN_MS;
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}
