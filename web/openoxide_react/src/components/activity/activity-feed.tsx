import {useMemo} from 'react';
import {Link} from '@tanstack/react-router';
import {useAuditLogItems} from '#/api/live/audit';
import {useDeploymentItems} from '#/api/live/deployments';
import {
	toEntries,
	dayLabel,
	timeAgo,
	deploymentTone,
	TONE_DOT,
	type ActivityEntry,
	type EntryTone,
} from './activity-model';

/** Merged activity feed — deployments and audit events in one day-grouped, sentence-rendered timeline. Pure model helpers live in `./activity-model`. */

export type {ActivityEntry, EntryTone} from './activity-model';
export {
	TONE_DOT,
	deploymentTone,
	deploymentVerb,
	dayLabel,
	timeAgo,
} from './activity-model';

export function useActivityEntries(limit?: number) {
	const {data: deployments = [], isLoading: depLoading} =
		useDeploymentItems();
	const {data: audits = [], isLoading: auditLoading} = useAuditLogItems({
		limit: limit ?? 200,
	});
	const entries = useMemo(
		() => toEntries(deployments, audits),
		[deployments, audits],
	);
	const days = useMemo(() => {
		const groups = new Map<string, ActivityEntry[]>();
		for (const e of entries) {
			const label = dayLabel(e.createdAtSec);
			const list = groups.get(label);
			if (list) list.push(e);
			else groups.set(label, [e]);
		}
		return [...groups.entries()];
	}, [entries]);
	return {
		entries,
		days,
		isLoading: depLoading || auditLoading,
	};
}

/** Compact recent-activity list for the home page (flat, no day cards). */
export function ActivitySummary({count = 6}: {count?: number}) {
	const {entries, isLoading} = useActivityEntries(count * 4);
	if (isLoading) {
		return (
			<div className="space-y-2">
				{[...Array(3)].map((_, i) => (
					<div
						key={i}
						className="h-10 animate-pulse rounded-xl bg-muted/40"
					/>
				))}
			</div>
		);
	}
	if (entries.length === 0) {
		return (
			<p className="py-2 text-sm text-muted-foreground/70">
				No activity yet — your deployments and changes will show up here.
			</p>
		);
	}
	return (
		<div className="overflow-hidden rounded-xl bg-muted/10">
			{entries.slice(0, count).map((e, i) => {
				const tone: EntryTone =
					e.kind === 'deployment'
						? deploymentTone(e.status)
						: ((e.status as EntryTone) ?? 'neutral');
				const row = (
					<>
						<span
							className={`size-2 shrink-0 rounded-full ${TONE_DOT[tone]}`}
							aria-hidden
						/>
						<div className="min-w-0 flex-1">
							<p className="truncate text-sm font-medium text-foreground">
								{e.resource}
							</p>
							<p className="truncate text-xs text-muted-foreground/70">
								{e.actor} {e.action}
							</p>
						</div>
						<span className="shrink-0 text-xs text-muted-foreground/70">
							{timeAgo(e.createdAtSec)}
						</span>
					</>
				);
				const className = `flex items-center gap-2.5 px-3 py-2.5 text-left transition-colors hover:bg-muted/40 ${
					i > 0 ? 'border-t ' : ''
				}`;
				return e.kind === 'deployment' && e.deploymentId ? (
					<Link
						key={e.key}
						to="/Deployments"
						search={{id: e.deploymentId}}
						className={className}>
						{row}
					</Link>
				) : (
					<div key={e.key} className={className}>
						{row}
					</div>
				);
			})}
		</div>
	);
}
