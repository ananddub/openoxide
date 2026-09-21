export type ActiveModal =
	| 'domain'
	| 'schedule'
	| 'backup'
	| 'terminal'
	| 'logs'
	| 'deployLogs'
	| null;

/** Identifies the resource (application or compose service) that owns
 *  domains / schedules / volume backups created from the architecture tab. */
export interface ArchitectureOwner {
	kind: 'app' | 'compose';
	id: number;
	appName?: string;
	name?: string;
}
