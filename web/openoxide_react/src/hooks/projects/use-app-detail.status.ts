import {useEffect, useRef, useState} from 'react';

/**
 * Transient optimistic status override for the application detail page, plus
 * the in-flight guard refs shared with the action handlers. The override is
 * auto-cleared once the fetched status transitions past the pre-action value
 * or reaches the override, or after a 30s safety timeout.
 */
export function useAppStatusOverride(appStatus: string | undefined) {
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);
	const actionInFlight = useRef(false);
	const statusBeforeAction = useRef<string | null>(null);

	// Auto-clear localStatusOverride when status updates (primitive string to
	// prevent infinite loop)
	useEffect(() => {
		if (localStatusOverride && appStatus) {
			const fetchedStatus = String(appStatus).toUpperCase();
			const overrideUpper = localStatusOverride.toUpperCase();
			const statusTransitioned =
				statusBeforeAction.current !== null &&
				fetchedStatus !== statusBeforeAction.current;
			if (
				statusTransitioned ||
				fetchedStatus === overrideUpper ||
				(overrideUpper === 'STARTING' &&
					(fetchedStatus === 'RUNNING' || fetchedStatus === 'HEALTHY')) ||
				(overrideUpper === 'STOPPING' &&
					(fetchedStatus === 'STOPPED' ||
						fetchedStatus === 'IDLE' ||
						fetchedStatus === 'CANCELLED')) ||
				(overrideUpper === 'CANCELLING' &&
					(fetchedStatus === 'CANCELLED' ||
						fetchedStatus === 'STOPPED' ||
						fetchedStatus === 'IDLE'))
			) {
				setLocalStatusOverride(null);
				actionInFlight.current = false;
				statusBeforeAction.current = null;
			}
		}
	}, [appStatus, localStatusOverride]);

	useEffect(() => {
		if (!localStatusOverride) return;
		const timeout = window.setTimeout(() => {
			setLocalStatusOverride(null);
			actionInFlight.current = false;
			statusBeforeAction.current = null;
		}, 30_000);
		return () => window.clearTimeout(timeout);
	}, [localStatusOverride]);

	const clearOverride = () => {
		setLocalStatusOverride(null);
		actionInFlight.current = false;
		statusBeforeAction.current = null;
	};

	return {
		localStatusOverride,
		setLocalStatusOverride,
		actionInFlight,
		statusBeforeAction,
		clearOverride,
	};
}
