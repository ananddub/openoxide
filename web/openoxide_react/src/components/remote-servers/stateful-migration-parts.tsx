import {Ban, CheckCircle2, Clock, RefreshCw, XCircle} from 'lucide-react';
import type {StatefulMigrationPayload} from '#/api/live';

/** Terminal states stop polling. */
export function isActive(migration: StatefulMigrationPayload | undefined) {
	return (
		!!migration &&
		(migration.status === 'RUNNING' || migration.status === 'APPLYING')
	);
}

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
		case 'CANCELLED':
			return {
				icon: <Ban className="h-4 w-4 text-amber-500" />,
				cls: 'text-amber-500',
			};
		case 'RUNNING':
		case 'APPLYING':
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

export function formatBytes(bytes: number) {
	if (bytes <= 0) return '0 B';
	const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
	const index = Math.min(
		Math.floor(Math.log2(bytes) / 10),
		units.length - 1,
	);
	const value = bytes / 2 ** (index * 10);
	return `${value >= 100 || index === 0 ? Math.round(value) : value.toFixed(1)} ${units[index]}`;
}

/** Ordered view of the phase machine for the progress strip. */
const PHASES = [
	'PREFLIGHT',
	'QUIESCE',
	'SYNC',
	'VERIFY',
	'APPLY',
	'REPOINT',
	'REDEPLOY',
	'HEALTH',
	'DONE',
] as const;

export function PhaseStrip({phase}: {phase: string}) {
	const current = PHASES.indexOf(phase as (typeof PHASES)[number]);
	return (
		<div className="flex flex-wrap items-center gap-1 text-[10px]">
			{PHASES.map((p, i) => (
				<span
					key={p}
					className={`rounded-full px-2 py-0.5 font-mono ${
						i < current
							? 'bg-emerald-500/15 text-emerald-500'
							: i === current
								? 'bg-primary/15 font-semibold text-primary'
								: 'bg-muted/30 text-muted-foreground'
					}`}>
					{p}
				</span>
			))}
		</div>
	);
}
