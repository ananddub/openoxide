import {Code, ConnectError} from '@connectrpc/connect';
import {getSessionActions} from '#/stores/session-context';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';

const AUTH_STORAGE_KEY = 'openoxide-auth-session';

export function getAccessToken(): string {
	if (typeof localStorage === 'undefined') return '';
	try {
		const raw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (!raw || raw === 'undefined') return '';
		const session = JSON.parse(raw);
		// Connect responses serialize proto fields camelCase (accessToken).
		return session?.tokens?.accessToken || '';
	} catch {
		return '';
	}
}
export const getApiBaseUrl = () => {
	if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL;
	// Same-origin by default (the Caddy proxy in the dev stack and the embedded serving in production both expose the panel API on the UI origin) — mirrors the prototype's `baseUrl: ""` pattern. Cross-origin direct-panel callers can still pin VITE_API_URL.
	return '';
};

let refreshPromise: Promise<RefreshResult> | null = null;
let refreshRetryAfter = 0;
let refreshRetryToken = '';
const REFRESH_RETRY_COOLDOWN_MS = 15_000;

/**
 * Outcome of a refresh attempt:
 * - `ok` — new access token issued.
 * - `expired` — the backend rejected the refresh token (logout performed);
 *   the session is genuinely dead.
 * - `transient` — network/transport/server failure; the stored refresh
 *   token is untouched and the session must NOT be dropped.
 */
export type RefreshResult =
	| {status: 'ok'; token: string}
	| {status: 'expired'}
	| {status: 'transient'};

// The backend maps invalid/expired/revoked refresh tokens to Unauthenticated
// (map_auth_error); a transport-level failure surfaces as Unknown/Unavailable
// instead, so only an explicit server rejection ends the session.
function isSessionRejected(error: unknown): boolean {
	return (
		error instanceof ConnectError && error.code === Code.Unauthenticated
	);
}

// NOTE: created lazily inside refreshAccessToken(), never at module scope.
// client.ts ↔ connect.ts ↔ query.ts is an import cycle; a module-level
// createServiceClient() runs while connect.ts's transport is still
// undefined (bundler-evaluated), which silently breaks every refresh call.

/** Refreshes the access token through the ConnectRPC AuthService and rewrites the stored session (user.id / tokens.accessToken, camelCase). */
export async function refreshAccessToken(): Promise<RefreshResult> {
	if (refreshPromise) return refreshPromise;

	refreshPromise = (async () => {
		let refreshToken = '';
		// The api layer may load before the provider mounts; fall back to no-ops so token refresh never crashes on early calls.
		const {logout, setAuth} = getSessionActions() ?? {
			logout: () => {},
			setAuth: () => {},
		};
		try {
			const sessionRaw = localStorage.getItem(AUTH_STORAGE_KEY);
			if (!sessionRaw || sessionRaw === 'undefined') {
				logout();
				return {status: 'expired'};
			}

			const session = JSON.parse(sessionRaw);
			refreshToken = session?.tokens?.refreshToken || '';
			if (!refreshToken) {
				logout();
				return {status: 'expired'};
			}
			if (
				refreshRetryToken === refreshToken &&
				Date.now() < refreshRetryAfter
			)
				return {status: 'transient'};

			const response = await createServiceClient(AuthService).refresh({
				refreshToken,
			});
			if (!response.user || !response.tokens) {
				logout();
				return {status: 'expired'};
			}

			const newSession = {
				user: {
					id: Number(response.user.id),
					email: response.user.email,
					firstName: response.user.firstName,
					lastName: response.user.lastName,
					avatar: response.user.avatar,
				},
				tokens: response.tokens,
			};
			localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(newSession));
			refreshRetryToken = '';
			refreshRetryAfter = 0;

			setAuth({
				id: Number(response.user.id),
				email: response.user.email || '',
				firstName: response.user.firstName,
				lastName: response.user.lastName,
			});

			if (!newSession.tokens.accessToken) return {status: 'transient'};
			return {status: 'ok', token: newSession.tokens.accessToken};
		} catch (error) {
			if (isSessionRejected(error)) {
				// The server itself rejected the refresh token: genuinely expired
				// or revoked. End the session now.
				refreshRetryToken = '';
				refreshRetryAfter = 0;
				logout();
				return {status: 'expired'};
			}
			// Network/transport/server failure: keep the session so the next
			// attempt can succeed; the cooldown keeps a broken backend from
			// being hammered on every request.
			refreshRetryToken = refreshToken;
			refreshRetryAfter = Date.now() + REFRESH_RETRY_COOLDOWN_MS;
			return {status: 'transient'};
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}
