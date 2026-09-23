import type {Deployment} from '#/hooks/deployments/use-deployments';

/** Application | Compose | Database | Generic, from the owning id fields. */
export function deploymentType(d: Deployment): string {
	if (d.application_id != null) return 'Application';
	if (d.compose_id != null) return 'Compose';
	if (d.database_id != null) return 'Database';
	return 'Generic';
}

/** True while a build is in flight (running, queued, building, pending). */
export function isBusyStatus(status: string): boolean {
	const s = (status || '').toUpperCase();
	return (
		s === 'RUNNING' ||
		s === 'QUEUED' ||
		s.includes('BUILD') ||
		s.includes('PEND')
	);
}

/** Seconds- or millis-epoch to a short date string. */
export function formatDeploymentDate(raw: unknown): string {
	if (!raw) return 'N/A';
	const num = Number(raw);
	if (isNaN(num)) return String(raw);
	const ms = num < 1e11 ? num * 1000 : num;
	return new Date(ms).toLocaleDateString();
}

/** Compact build duration ("48s", "2m 14s", "1h 3m") from epoch seconds. */
export function formatDeploymentDuration(
	startSec: number,
	endSec: number,
): string {
	const s = Math.max(0, Math.round(endSec - startSec));
	if (s < 60) return `${s}s`;
	const m = Math.floor(s / 60);
	if (m < 60) return `${m}m ${s % 60}s`;
	return `${Math.floor(m / 60)}h ${m % 60}m`;
}
