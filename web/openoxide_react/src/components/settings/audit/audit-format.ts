import {Activity, ShieldCheck, User as UserIcon} from 'lucide-react';
import type {AuditLogPayload} from '#/api/live/audit';

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

export type CategoryId = 'all' | 'team' | 'access' | 'system';

export type Sentence = {
	verb: string;
	tone: EntryTone;
	category: CategoryId;
};

const AUDIT_SENTENCES: Record<string, Sentence> = {
	INVITE_CREATED: {verb: 'invited', tone: 'info', category: 'team'},
	INVITE_ACCEPTED: {
		verb: 'accepted an invite to',
		tone: 'success',
		category: 'team',
	},
	INVITE_CANCELLED: {
		verb: 'cancelled an invite for',
		tone: 'neutral',
		category: 'team',
	},
	MEMBER_ROLE_UPDATED: {
		verb: 'changed the role of',
		tone: 'info',
		category: 'team',
	},
	MEMBER_REMOVED: {verb: 'removed', tone: 'danger', category: 'team'},
	PERMISSION_GROUP_CREATED: {
		verb: 'created',
		tone: 'success',
		category: 'access',
	},
	PERMISSION_GROUP_UPDATED: {
		verb: 'updated',
		tone: 'info',
		category: 'access',
	},
	PERMISSION_GROUP_DELETED: {
		verb: 'deleted',
		tone: 'danger',
		category: 'access',
	},
	PERMISSION_GROUP_ASSIGNED: {
		verb: 'assigned a permission group to',
		tone: 'info',
		category: 'access',
	},
	USER_POLICIES_REPLACED: {
		verb: 'replaced policies on',
		tone: 'warning',
		category: 'access',
	},
};

export const CATEGORIES: {
	id: CategoryId;
	label: string;
	icon: typeof Activity;
}[] = [
	{id: 'all', label: 'All', icon: Activity},
	{id: 'team', label: 'Team', icon: UserIcon},
	{id: 'access', label: 'Access', icon: ShieldCheck},
];

export function sentenceFor(action: string): Sentence {
	return (
		AUDIT_SENTENCES[action] ?? {
			verb: action.toLowerCase().replace(/_/g, ' '),
			tone: 'neutral',
			category: 'system',
		}
	);
}

/** Human label for an audit resource, from metadata/name/id. */
export function resourceLabel(e: AuditLogPayload): string {
	if (e.resource_name) return e.resource_name;
	const meta = parseMeta(e);
	const email = typeof meta.email === 'string' ? meta.email : undefined;
	const kind = (e.resource_type || '').replace(/_/g, ' ');
	if (email) return email;
	if (e.resource_id) return `${kind} #${e.resource_id}`;
	return kind;
}

export function parseMeta(e: AuditLogPayload): Record<string, unknown> {
	try {
		return e.metadata ? JSON.parse(e.metadata) : {};
	} catch {
		return {};
	}
}

/** Audit rows reach the web layer from different writers — epoch seconds, milliseconds or microseconds depending on the path. Normalize before formatting. */
export function toSec(ts: number): number {
	const n = Number(ts);
	if (!Number.isFinite(n) || n <= 0) return 0;
	if (n >= 1e14) return Math.floor(n / 1_000_000); // µs
	if (n >= 1e11) return Math.floor(n / 1_000); // ms
	return n;
}

export function timeAgo(sec: number): string {
	const diff = Math.floor(Date.now() / 1000 - toSec(sec));
	if (diff < 0) return 'just now';
	if (diff < 60) return `${diff}s ago`;
	const min = Math.floor(diff / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h ago`;
	return `${Math.floor(hr / 24)}d ago`;
}

export function clockTime(sec: number): string {
	const s = toSec(sec);
	if (!s) return '—';
	return new Date(s * 1000).toLocaleTimeString([], {
		hour: '2-digit',
		minute: '2-digit',
		hour12: true,
	});
}

export function absoluteTime(sec: number): string {
	const s = toSec(sec);
	if (!s) return 'Unknown time';
	return new Date(s * 1000).toLocaleString();
}

export function dayKey(sec: number): string {
	const d = new Date(toSec(sec) * 1000);
	return `${d.getFullYear()}-${d.getMonth()}-${d.getDate()}`;
}

export function dayLabel(sec: number): string {
	const now = new Date();
	const startOf = (x: Date) =>
		new Date(x.getFullYear(), x.getMonth(), x.getDate()).getTime();
	const d = new Date(toSec(sec) * 1000);
	const diffDays = Math.round((startOf(now) - startOf(d)) / 86400000);
	if (diffDays <= 0) return 'Today';
	if (diffDays === 1) return 'Yesterday';
	return d.toLocaleDateString(undefined, {
		weekday: 'short',
		month: 'short',
		day: 'numeric',
		year: d.getFullYear() === now.getFullYear() ? undefined : 'numeric',
	});
}

export const PER_PAGE = 25;

export type PeriodKey = 'all' | 'today' | '7d' | '30d' | '90d';

export const PERIODS: {value: PeriodKey; label: string}[] = [
	{value: 'all', label: 'All time'},
	{value: 'today', label: 'Today'},
	{value: '7d', label: 'Last 7 days'},
	{value: '30d', label: 'Last 30 days'},
	{value: '90d', label: 'Last 90 days'},
];

export function periodCutoff(period: PeriodKey): number {
	if (period === 'all') return 0;
	const now = Date.now() / 1000;
	if (period === 'today') {
		const d = new Date();
		d.setHours(0, 0, 0, 0);
		return d.getTime() / 1000;
	}
	const days = period === '7d' ? 7 : period === '30d' ? 30 : 90;
	return now - days * 86400;
}
