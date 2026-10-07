import type {ServerManagementPayload} from '#/api/live/server-management';

/** Epoch-seconds stamp from the management tables; `null` or 0 means the run never finished. */
export function formatEpochSeconds(value: bigint | null): string {
	if (value === null || value <= 0n) return '—';
	return new Date(Number(value) * 1000).toLocaleString();
}

/** Wall-clock duration between two epoch-seconds stamps, for the cleanup history row. */
export function formatRunDuration(
	startedAt: bigint,
	finishedAt: bigint | null,
): string {
	if (finishedAt === null || finishedAt <= 0n || finishedAt < startedAt) {
		return '—';
	}
	const seconds = Number(finishedAt - startedAt);
	if (seconds < 60) return `${seconds}s`;
	const minutes = Math.floor(seconds / 60);
	if (minutes < 60) return `${minutes}m ${seconds % 60}s`;
	return `${Math.floor(minutes / 60)}h ${minutes % 60}m`;
}

/** Cleanup policy snapshot (JSON document) → the scopes that were switched on. */
export function summarizePolicy(policyJson: string): string {
	const trimmed = policyJson.trim();
	if (!trimmed) return '—';
	try {
		const parsed = JSON.parse(trimmed) as Record<string, unknown>;
		const enabled = Object.entries(parsed)
			.filter(([, value]) => value === true)
			.map(([key]) => key);
		return enabled.length > 0 ? enabled.join(', ') : 'no scopes';
	} catch {
		return trimmed;
	}
}

/** True when the edited form differs from the saved record, so Save stays inert until something changed. */
export function managementDirty(
	current: ServerManagementPayload,
	saved: ServerManagementPayload,
): boolean {
	return (
		current.maintenanceMode !== saved.maintenanceMode ||
		current.maintenanceMessage !== saved.maintenanceMessage ||
		current.gpuEnabled !== saved.gpuEnabled ||
		CLEANUP_KEYS.some(
			key => current.cleanupPolicy[key] !== saved.cleanupPolicy[key],
		) ||
		!sameLabels(current.labels, saved.labels)
	);
}

const CLEANUP_KEYS = [
	'containers',
	'images',
	'networks',
	'volumes',
	'packages',
] as const;

function sameLabels(
	current: Array<[string, string]>,
	saved: Array<[string, string]>,
): boolean {
	const filled = current.filter(([key]) => key.trim().length > 0);
	if (filled.length !== saved.length) return false;
	return saved.every(([key, value]) =>
		filled.some(
			([candidateKey, candidateValue]) =>
				candidateKey.trim() === key && candidateValue === value,
		),
	);
}
