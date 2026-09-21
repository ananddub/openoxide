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
