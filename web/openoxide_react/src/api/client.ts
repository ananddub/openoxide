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

let refreshPromise: Promise<string | null> | null = null;
let refreshRetryAfter = 0;
let refreshRetryToken = '';
const REFRESH_RETRY_COOLDOWN_MS = 15_000;

// NOTE: created lazily inside refreshAccessToken(), never at module scope.
// client.ts ↔ connect.ts ↔ query.ts is an import cycle; a module-level
// createServiceClient() runs while connect.ts's transport is still
// undefined (bundler-evaluated), which silently breaks every refresh call.

/** Refreshes the access token through the ConnectRPC AuthService and rewrites the stored session (user.id / tokens.accessToken, camelCase). */
export async function refreshAccessToken(): Promise<string | null> {
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
				return null;
			}

			const session = JSON.parse(sessionRaw);
			refreshToken = session?.tokens?.refreshToken || '';
			if (!refreshToken) {
				logout();
				return null;
			}
			if (
				refreshRetryToken === refreshToken &&
				Date.now() < refreshRetryAfter
			)
				return null;

			const response = await createServiceClient(AuthService).refresh({
				refreshToken,
			});
			if (!response.user || !response.tokens) {
				logout();
				return null;
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

			return newSession.tokens.accessToken || null;
		} catch {
			// A failed refresh means the session is gone; the cooldown keeps a broken backend from being hammered on every request.
			refreshRetryToken = refreshToken;
			refreshRetryAfter = Date.now() + REFRESH_RETRY_COOLDOWN_MS;
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}
