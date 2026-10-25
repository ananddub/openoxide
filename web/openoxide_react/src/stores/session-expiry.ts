const AUTH_STORAGE_KEY = 'openoxide-auth-session';

/** `exp` (epoch ms) inside a JWT, or null when the token is opaque/malformed. */
export function jwtExpiryMs(token: string): number | null {
	const payload = token.split('.')[1];
	if (!payload) return null;
	try {
		const base64 = payload.replace(/-/g, '+').replace(/_/g, '/');
		const padded = base64 + '='.repeat((4 - (base64.length % 4)) % 4);
		const claims = JSON.parse(atob(padded));
		return typeof claims?.exp === 'number' ? claims.exp * 1000 : null;
	} catch {
		return null;
	}
}

/**
 * When the stored session actually dies: the refresh token's `exp`.
 *
 * Access tokens (15 min) expiring is not a logout — the interceptor renews
 * them silently. Only a dead refresh token (7 days) means the user has to
 * sign in again, and nothing API-driven detects that on an idle tab.
 */
export function refreshTokenExpiry(): number | null {
	try {
		const raw = localStorage.getItem(AUTH_STORAGE_KEY);
		if (!raw || raw === 'undefined') return null;
		const token = JSON.parse(raw)?.tokens?.refreshToken;
		return typeof token === 'string' ? jwtExpiryMs(token) : null;
	} catch {
		return null;
	}
}
