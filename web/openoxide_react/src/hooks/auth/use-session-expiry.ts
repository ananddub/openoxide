import {useEffect} from 'react';
import {useAuthSession} from '#/stores/session-context';
import {refreshTokenExpiry} from '#/stores/session-expiry';

// setTimeout saturates above this; a 7-day refresh token fits, but clamp anyway.
const MAX_TIMER_MS = 2_147_483_647;

/**
 * Ends the session the moment its refresh token expires.
 *
 * API-driven detection (401 → refresh) misses an idle tab entirely: no request,
 * no rejection, so the UI keeps rendering a session the backend has already
 * stopped accepting. The router redirect follows from the auth state change.
 */
export function useSessionExpiry(): void {
	const {isAuth, logout} = useAuthSession();

	useEffect(() => {
		if (!isAuth) return;
		let timer: ReturnType<typeof setTimeout> | undefined;

		const schedule = () => {
			if (timer) clearTimeout(timer);
			// Re-read every time: another tab may have stored a fresh session.
			const expiresAt = refreshTokenExpiry();
			if (expiresAt == null) return;
			const remaining = expiresAt - Date.now();
			if (remaining <= 0) {
				logout();
				return;
			}
			timer = setTimeout(schedule, Math.min(remaining, MAX_TIMER_MS));
		};

		schedule();
		// Background tabs throttle timers, so re-check as soon as the tab wakes.
		const onVisibilityChange = () => {
			if (document.visibilityState === 'visible') schedule();
		};
		document.addEventListener('visibilitychange', onVisibilityChange);
		window.addEventListener('focus', schedule);
		return () => {
			if (timer) clearTimeout(timer);
			document.removeEventListener('visibilitychange', onVisibilityChange);
			window.removeEventListener('focus', schedule);
		};
	}, [isAuth, logout]);
}
