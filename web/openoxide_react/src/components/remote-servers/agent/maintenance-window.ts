import type {
	AgentStatusPayload,
	MaintenanceWindowPayload,
} from '#/api/live/monitoring';

/** Draft of a new maintenance window, held as `datetime-local` strings because the browser picker cannot emit epoch seconds. */
export type MaintenanceWindowDraft = {
	startsAt: string;
	endsAt: string;
	reason: string;
};

/** Epoch seconds → the value a `datetime-local` input expects, in the browser's zone. */
export function toLocalInputValue(epochSeconds: number): string {
	const date = new Date(epochSeconds * 1000);
	const pad = (value: number) => String(value).padStart(2, '0');
	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/** `datetime-local` value → epoch seconds. A zoneless date-time string is already local, so no offset correction. */
export function fromLocalInputValue(value: string): number | null {
	if (!value) return null;
	const millis = new Date(value).getTime();
	return Number.isNaN(millis) ? null : Math.floor(millis / 1000);
}

/** A window starting now and ending in two hours — the common case, so the form opens ready to submit. */
export function defaultMaintenanceWindowDraft(
	nowSeconds = Math.floor(Date.now() / 1000),
): MaintenanceWindowDraft {
	return {
		startsAt: toLocalInputValue(nowSeconds),
		endsAt: toLocalInputValue(nowSeconds + 2 * 60 * 60),
		reason: '',
	};
}

/** Why the draft cannot be submitted yet, or `null` when it can. Mirrors the backend's `ends_at must follow starts_at`. */
export function maintenanceWindowProblem(
	draft: MaintenanceWindowDraft,
): string | null {
	const startsAt = fromLocalInputValue(draft.startsAt);
	const endsAt = fromLocalInputValue(draft.endsAt);
	if (startsAt === null) return 'Pick when the window starts.';
	if (endsAt === null) return 'Pick when the window ends.';
	if (endsAt <= startsAt) return 'The window must end after it starts.';
	if (!draft.reason.trim())
		return 'Say why — it shows up in the alert suppression.';
	if (draft.reason.trim().length > 500)
		return 'Keep the reason under 500 characters.';
	return null;
}

/** Where a window sits relative to `nowSeconds`; drives both the pill and the ordering. */
export function maintenanceWindowState(
	window: MaintenanceWindowPayload,
	nowSeconds: number,
): 'active' | 'scheduled' | 'ended' {
	if (window.ends_at <= nowSeconds) return 'ended';
	if (window.starts_at > nowSeconds) return 'scheduled';
	return 'active';
}

/** True when alerting for this server is currently muted by any window. */
export function hasActiveWindow(
	windows: MaintenanceWindowPayload[],
	nowSeconds: number,
): boolean {
	return windows.some(
		window => maintenanceWindowState(window, nowSeconds) === 'active',
	);
}

/** Human summary of how long ago a heartbeat arrived, for the agent status line. */
export function formatLastSeen(
	lastSeenAt: number | null,
	nowSeconds: number,
): string {
	if (lastSeenAt === null) return 'never';
	const seconds = Math.max(0, nowSeconds - lastSeenAt);
	if (seconds < 60) return `${seconds}s ago`;
	const minutes = Math.floor(seconds / 60);
	if (minutes < 60) return `${minutes}m ago`;
	const hours = Math.floor(minutes / 60);
	if (hours < 24) return `${hours}h ago`;
	return `${Math.floor(hours / 24)}d ago`;
}

/** Copy the panel uses to describe what each agent state means for the operator. */
export const AGENT_STATE_LABELS: Record<
	AgentStatusPayload['state'],
	{label: string; hint: string}
> = {
	online: {
		label: 'Online',
		hint: 'Heartbeat arrived in the last 30 seconds.',
	},
	offline: {
		label: 'Offline',
		hint: 'Registered, but no heartbeat lately — the container is probably stopped.',
	},
	never_seen: {
		label: 'Never connected',
		hint: 'Installed but has not reported yet. Check the server time and the agent token.',
	},
	unregistered: {
		label: 'Not installed',
		hint: 'This server has no monitoring agent. Run the server setup to install one.',
	},
};
