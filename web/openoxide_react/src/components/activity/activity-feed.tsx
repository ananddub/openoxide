import {useMemo} from 'react';
import {Link} from '@tanstack/react-router';
import {useAuditLogItems, type AuditLogPayload} from '#/api/live/audit';
import {useDeploymentItems, type DeploymentItemPayload} from '#/api/live/deployments';

/**
 * Merged activity feed — deployments and audit events in one day-grouped,
 * sentence-rendered timeline (OpenShip's audit page pattern: "actor
 * deployed api-gateway", not table rows).
 *
 * Deployment rows link to `/Deployments?id=<id>` (opens the log stream);
 * audit rows are the panel's `audit_logs` table (permission/member/invite
 * mutations), rendered as past-tense sentences.
 */

export type ActivityEntry = {
	key: string;
	kind: 'deployment' | 'audit';
	/** Actor sentence fragment: bold subject. */
	actor: string;
	/** Past-tense verb phrase. */
	action: string;
	/** Bold object (resource name / title). */
	resource: string;
	createdAtSec: number;
	status?: string;
	deploymentId?: string;
};

export type EntryTone = 'success' | 'danger' | 'info' | 'warning' | 'neutral';

export const TONE_DOT: Record<EntryTone, string> = {
	success: 'bg-emerald-500',
	danger: 'bg-rose-500',
	info: 'bg-sky-500',
	warning: 'bg-amber-500',
	neutral: 'bg-muted-foreground/40',
};

const AUDIT_SENTENCES: Record<string, {verb: string; tone: EntryTone}> = {
	INVITE_CREATED: {verb: 'invited', tone: 'info'},
	INVITE_ACCEPTED: {verb: 'accepted an invite to', tone: 'success'},
	INVITE_CANCELLED: {verb: 'cancelled an invite for', tone: 'neutral'},
	MEMBER_ROLE_UPDATED: {verb: 'changed the role of', tone: 'info'},
	MEMBER_REMOVED: {verb: 'removed', tone: 'danger'},
	PERMISSION_GROUP_CREATED: {verb: 'created', tone: 'success'},
	PERMISSION_GROUP_UPDATED: {verb: 'updated', tone: 'info'},
	PERMISSION_GROUP_DELETED: {verb: 'deleted', tone: 'danger'},
	PERMISSION_GROUP_ASSIGNED: {verb: 'assigned a permission group to', tone: 'info'},
	USER_POLICIES_REPLACED: {verb: 'replaced policies on', tone: 'warning'},
};

export function deploymentTone(status?: string): EntryTone {
	const s = (status || '').toUpperCase();
	if (s.includes('RUN') || s.includes('BUILD') || s.includes('QUEU'))
		return 'warning';
	if (s.includes('ERR') || s.includes('FAIL') || s.includes('CRASH'))
		return 'danger';
	if (s.includes('DONE') || s.includes('SUCCESS') || s.includes('DEPLOY'))
		return 'success';
	return 'neutral';
}

export function deploymentVerb(status?: string): string {
	const s = (status || '').toUpperCase();
	if (s.includes('RUN') || s.includes('BUILD')) return 'is deploying';
	if (s.includes('QUEU')) return 'queued a deploy for';
	if (s.includes('ERR') || s.includes('FAIL') || s.includes('CRASH'))
		return 'failed deploying';
	return 'deployed';
}

/** Human label for an audit resource, from metadata/name/id. */
function auditResourceLabel(e: AuditLogPayload): string {
	if (e.resource_name) return e.resource_name;
	let meta: Record<string, unknown> = {};
	try {
		meta = e.metadata ? JSON.parse(e.metadata) : {};
	} catch {
		meta = {};
	}
	const email = typeof meta.email === 'string' ? meta.email : undefined;
	const kind = (e.resource_type || '').replace(/_/g, ' ');
	if (email) return email;
	if (e.resource_id) return `${kind} #${e.resource_id}`;
	return kind;
}

function toEntries(
	deployments: DeploymentItemPayload[],
	audits: AuditLogPayload[],
): ActivityEntry[] {
	const depRows: ActivityEntry[] = deployments.map(d => {
		const id = String(d.id);
		return {
			key: `d-${id}`,
			kind: 'deployment',
			actor: String(d.user_email ?? '') || 'System',
			action: deploymentVerb(d.status),
			resource: String(d.title || d.description || `deployment #${id}`),
			createdAtSec: Number(d.created_at ?? 0),
			status: d.status,
			deploymentId: id,
		};
	});
	const auditRows: ActivityEntry[] = audits.map(a => {
		const info = AUDIT_SENTENCES[a.action];
		return {
			key: `a-${a.id}`,
			kind: 'audit',
			actor: a.user_email || 'System',
			action: info?.verb ?? a.action.toLowerCase().replace(/_/g, ' '),
			resource: auditResourceLabel(a),
			createdAtSec: Number(a.created_at ?? 0),
			status: info?.tone,
		};
	});
	return [...depRows, ...auditRows].sort(
		(x, y) => y.createdAtSec - x.createdAtSec,
	);
}

export function dayLabel(sec: number): string {
	const d = new Date(sec * 1000);
	const today = new Date();
	const startOf = (x: Date) =>
		new Date(x.getFullYear(), x.getMonth(), x.getDate()).getTime();
	const diffDays = Math.round((startOf(today) - startOf(d)) / 86400000);
	if (diffDays <= 0) return 'Today';
	if (diffDays === 1) return 'Yesterday';
	return d.toLocaleDateString(undefined, {
		weekday: 'short',
		month: 'short',
		day: 'numeric',
	});
}

export function timeAgo(sec: number): string {
	const diff = Math.floor(Date.now() / 1000 - sec);
	if (diff < 60) return `${Math.max(diff, 0)}s ago`;
	const min = Math.floor(diff / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h ago`;
	return `${Math.floor(hr / 24)}d ago`;
}


export function useActivityEntries(limit?: number) {
	const {data: deployments = [], isLoading: depLoading} = useDeploymentItems();
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
					<div key={i} className="h-10 animate-pulse rounded-xl bg-muted/40" />
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
