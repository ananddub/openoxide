// Status vocabulary shared by the app / compose / database detail hooks.

/** An action is still settling server-side. */
const BUILDING_STATUSES = [
	'QUEUED',
	'BUILDING',
	'STARTING',
	'PREPARING',
	'PENDING',
	'DEPLOYING',
];

/** Finished-good states — used to decide "is it up?". */
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

/** Deployment rows that are still running (finished rows are filtered first). */
export const ACTIVE_DEPLOYMENT_STATUSES = [
	'QUEUED',
	'PENDING',
	'BUILDING',
	'PREPARING',
	'STARTING',
	'DEPLOYING',
];

const upper = (status: string | null | undefined) =>
	String(status ?? '').toUpperCase();

export const isBuildingStatus = (status: string | null | undefined) =>
	BUILDING_STATUSES.includes(upper(status));

export const isDeployedStatus = (status: string | null | undefined) =>
	DEPLOYED_STATUSES.includes(upper(status));

// Optimistic status shown the instant an action fires, before the backend answers.
export function intermediateStatusFor(
	action:
		| 'deploy'
		| 'redeploy'
		| 'rebuild'
		| 'reload'
		| 'start'
		| 'stop'
		| 'cancel',
	currentStatus: string | null | undefined,
) {
	if (action === 'stop' || action === 'cancel') {
		return isBuildingStatus(currentStatus) ? 'CANCELLING' : 'STOPPING';
	}
	if (action === 'start') return 'STARTING';
	return 'DEPLOYING';
}

/** Fetched statuses that mean a given optimistic override has been overtaken. */
const OVERRIDE_OVERTAKEN: Record<string, string[]> = {
	DEPLOYING: [
		'BUILDING',
		'QUEUED',
		'PREPARING',
		'DEPLOYING',
		'RUNNING',
		'HEALTHY',
		'SUCCESS',
		'DONE',
	],
	STARTING: ['RUNNING', 'HEALTHY', 'STARTING'],
	STOPPING: ['STOPPED', 'IDLE', 'STOPPING', 'CANCELLED'],
	CANCELLING: ['CANCELLED', 'STOPPED', 'IDLE'],
};

// `previous` lets a caller drop the override on any movement at all (app page).
export function isOverrideStale(
	override: string | null | undefined,
	fetched: string | null | undefined,
	previous?: string | null,
) {
	if (!override || !fetched) return false;
	const overrideUpper = upper(override);
	const fetchedUpper = upper(fetched);
	if (previous != null && fetchedUpper !== upper(previous)) return true;
	if (fetchedUpper === overrideUpper) return true;
	return (OVERRIDE_OVERTAKEN[overrideUpper] ?? []).includes(fetchedUpper);
}

/** `status || app_status || …` — the payloads name the same field per kind. */
function readRawStatus(
	row: Record<string, unknown> | null | undefined,
	fields: string[],
) {
	if (!row) return undefined;
	for (const field of fields) {
		const value = row[field];
		if (value) return String(value);
	}
	return undefined;
}

// Sets both the kind-specific key and the generic `status`, as detail pages read both.
export function withStatusOverride<T extends Record<string, unknown>>(
	row: T | null,
	override: string | null,
	fallbackFields: string[],
	statusKey: string,
): T | null {
	if (!row) return null;
	const effective =
		override || readRawStatus(row, fallbackFields) || 'STOPPED';
	return {...row, status: effective, [statusKey]: effective} as T;
}
