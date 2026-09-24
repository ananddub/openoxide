import {useEffect, useMemo, useState} from 'react';
import {
	isBuildingStatus,
	isDeployedStatus,
	isOverrideStale,
} from '../shared/resource-status';

/** Merges the live-query row with the store row, layers a transient local status override (set optimistically by action handlers) over the fetched status, and auto-clears that override once the backend status catches up or after a 4s safety timeout. */
export function useDatabaseStatus(liveDb: any, storeDb: any) {
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);

	// Auto-clear localStatusOverride when live/store status updates or safety timeout occurs
	useEffect(() => {
		const fetchedStatus =
			liveDb?.status ||
			liveDb?.app_status ||
			storeDb?.status ||
			storeDb?.app_status;
		if (isOverrideStale(localStatusOverride, fetchedStatus)) {
			setLocalStatusOverride(null);
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
		isDeployed: isDeployedStatus(statusUpper),
		isBuilding: isBuildingStatus(statusUpper),
		setLocalStatusOverride,
	};
}
