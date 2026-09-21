import {CheckCircle2, Clock, RefreshCw, XCircle} from 'lucide-react';
import type {ServerMigrationPayload} from '#/api/live';

/** Terminal states — polling stops and the rollback affordance is offered. */
export const TERMINAL = new Set(['SUCCEEDED', 'FAILED', 'ROLLED_BACK']);

export function statusMeta(status: string) {
	switch (status) {
		case 'SUCCEEDED':
			return {
				icon: <CheckCircle2 className="h-4 w-4 text-emerald-500" />,
				cls: 'text-emerald-500',
			};
		case 'FAILED':
			return {
				icon: <XCircle className="h-4 w-4 text-rose-500" />,
				cls: 'text-rose-500',
			};
		case 'RUNNING':
			return {
				icon: <RefreshCw className="h-4 w-4 animate-spin text-primary" />,
				cls: 'text-primary',
			};
		default:
			return {
				icon: <Clock className="h-4 w-4 text-amber-500" />,
				cls: 'text-amber-500',
			};
	}
}

/** Resource counts surfaced once a migration row exists. */
export function ProgressGrid({
	migration,
}: {
	migration: ServerMigrationPayload;
}) {
	const rows: Array<[string, number]> = [
		['Applications', migration.applications],
		['Compose projects', migration.compose_projects],
		['Databases', migration.databases],
		['Build assignments', migration.build_assignments],
		['Certificates', migration.certificates],
		['Schedules', migration.schedules],
	];
	return (
		<div className="grid grid-cols-2 gap-2 rounded-xl bg-muted/20 p-3.5 text-xs">
			{rows.map(([label, value]) => (
				<div key={label} className="flex items-center justify-between">
					<span className="text-muted-foreground">{label}</span>
					<span className="font-mono font-semibold text-foreground">
						{value}
					</span>
				</div>
			))}
			{(migration.queued_applications > 0 ||
				migration.queued_compose_projects > 0) && (
				<div className="col-span-2 flex items-center justify-between border-t pt-2">
					<span className="text-muted-foreground">
						Redeploys queued on target
					</span>
					<span className="font-mono font-semibold text-foreground">
						{migration.queued_applications +
							migration.queued_compose_projects}
					</span>
				</div>
			)}
		</div>
	);
}
