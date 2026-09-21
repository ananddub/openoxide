import {useEffect, useMemo, useState} from 'react';

const DEPLOYED_STATUSES = [
	'RUNNING',
	'DONE',
	'HEALTHY',
	'SUCCESS',
	'COMPLETED',
	'UP',
	'ACTIVE',
	'OK',
];
const BUILDING_STATUSES = [
	'QUEUED',
	'BUILDING',
	'STARTING',
	'PREPARING',
	'PENDING',
	'DEPLOYING',
];

/**
 * Merges the live-query row with the store row, layers a transient local
 * status override (set optimistically by action handlers) over the fetched
 * status, and auto-clears that override once the backend status catches up
 * or after a 4s safety timeout.
 */
export function useDatabaseStatus(liveDb: any, storeDb: any) {
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);

	// Auto-clear localStatusOverride when live/store status updates or safety timeout occurs
	useEffect(() => {
		const fetchedStatus = (
			liveDb?.status ||
			liveDb?.app_status ||
			storeDb?.status ||
			storeDb?.app_status ||
			''
		).toUpperCase();

		if (localStatusOverride && fetchedStatus) {
			const overrideUpper = localStatusOverride.toUpperCase();
			const resolved =
				fetchedStatus === overrideUpper ||
				(overrideUpper === 'DEPLOYING' &&
					[
						'BUILDING',
						'QUEUED',
						'PREPARING',
						'DEPLOYING',
						'RUNNING',
						'HEALTHY',
						'SUCCESS',
						'DONE',
					].includes(fetchedStatus)) ||
				(overrideUpper === 'STARTING' &&
					['RUNNING', 'HEALTHY', 'STARTING'].includes(fetchedStatus)) ||
				(overrideUpper === 'STOPPING' &&
					['STOPPED', 'IDLE', 'STOPPING'].includes(fetchedStatus)) ||
				(overrideUpper === 'CANCELLING' &&
					['CANCELLED', 'STOPPED', 'IDLE'].includes(fetchedStatus));
			if (resolved) setLocalStatusOverride(null);
		}
	}, [liveDb, storeDb, localStatusOverride]);

	useEffect(() => {
		if (localStatusOverride) {
			const timer = setTimeout(() => setLocalStatusOverride(null), 4000);
			return () => clearTimeout(timer);
		}
	}, [localStatusOverride]);

	const raw = liveDb || storeDb;
	const database = useMemo(() => {
		if (!raw) return null;
		const liveStatus = liveDb?.status || liveDb?.app_status;
		const storeStatus = storeDb?.status || storeDb?.app_status;
		const rawStatus = raw.status || raw.app_status;
		const effectiveStatus =
			localStatusOverride ||
			liveStatus ||
			storeStatus ||
			rawStatus ||
			'STOPPED';
		return {
			...raw,
			status: effectiveStatus,
			app_status: effectiveStatus,
		};
	}, [raw, liveDb, storeDb, localStatusOverride]);

	const statusUpper = (
		database?.status ||
		database?.app_status ||
		database?.application_status ||
		''
	).toUpperCase();

	return {
		database,
		statusUpper,
		isDeployed: DEPLOYED_STATUSES.includes(statusUpper),
		isBuilding: BUILDING_STATUSES.includes(statusUpper),
		setLocalStatusOverride,
	};
}
