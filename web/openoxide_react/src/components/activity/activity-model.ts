import type {AuditLogPayload} from '#/api/live/audit';
import type {DeploymentItemPayload} from '#/api/live/deployments';

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

export type EntryTone =
	| 'success'
	| 'danger'
	| 'info'
	| 'warning'
	| 'neutral';

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
	PERMISSION_GROUP_ASSIGNED: {
		verb: 'assigned a permission group to',
		tone: 'info',
	},
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

export function toEntries(
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
