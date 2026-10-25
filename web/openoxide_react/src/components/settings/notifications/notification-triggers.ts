import type {NotificationPayload} from '#/api/live';
import type {ProviderId} from './notification-providers';

export type TriggerGroup =
	| 'Deployments'
	| 'Backups'
	| 'Platform'
	| 'Schedules';

/** The ten events a channel can fire on. `key` is the payload column the backend persists; `event` is the `trigger_name` the delivery-attempt history stores for the same event (APP_DEPLOY for on_app_deploy, …). */
export const TRIGGERS: {
	key: keyof NotificationPayload;
	label: string;
	group: TriggerGroup;
	event: string;
}[] = [
	{
		key: 'on_app_deploy',
		label: 'App deployed',
		group: 'Deployments',
		event: 'APP_DEPLOY',
	},
	{
		key: 'on_app_build_error',
		label: 'Build failed',
		group: 'Deployments',
		event: 'APP_BUILD_ERROR',
	},
	{
		key: 'on_database_backup',
		label: 'Database backup',
		group: 'Backups',
		event: 'DATABASE_BACKUP',
	},
	{
		key: 'on_volume_backup',
		label: 'Volume backup',
		group: 'Backups',
		event: 'VOLUME_BACKUP',
	},
	{
		key: 'on_panel_backup',
		label: 'Panel backup',
		group: 'Backups',
		event: 'PANEL_BACKUP',
	},
	{
		key: 'on_panel_restart',
		label: 'Panel restart',
		group: 'Platform',
		event: 'PANEL_RESTART',
	},
	{
		key: 'on_docker_cleanup',
		label: 'Docker cleanup',
		group: 'Platform',
		event: 'DOCKER_CLEANUP',
	},
	{
		key: 'on_server_threshold',
		label: 'Server threshold',
		group: 'Platform',
		event: 'SERVER_THRESHOLD',
	},
	{
		key: 'on_schedule_success',
		label: 'Schedule success',
		group: 'Schedules',
		event: 'SCHEDULE_SUCCESS',
	},
	{
		key: 'on_schedule_failure',
		label: 'Schedule failure',
		group: 'Schedules',
		event: 'SCHEDULE_FAILURE',
	},
];

export const TRIGGER_GROUPS = [
	'Deployments',
	'Backups',
	'Platform',
	'Schedules',
] as const;

/** Human label for a delivery history `trigger_name`. */
export function triggerEventLabel(event: string): string {
	return (
		TRIGGERS.find(t => t.event === event)?.label ??
		event.toLowerCase().replace(/_/g, ' ')
	);
}

/** Whether the event itself reports a success or a failure — independent of
 *  whether the external channel accepted the message. */
export type EventOutcome = 'success' | 'failure' | 'info';

export function triggerOutcome(event: string): EventOutcome {
	switch (event) {
		case 'APP_DEPLOY':
		case 'SCHEDULE_SUCCESS':
			return 'success';
		case 'APP_BUILD_ERROR':
		case 'SCHEDULE_FAILURE':
			return 'failure';
		default:
			return 'info';
	}
}

export function triggerSummary(n: NotificationPayload): string {
	const on = TRIGGERS.filter(t => n[t.key] === true).map(t => t.label);
	if (on.length === 0) return 'No events selected';
	if (on.length === TRIGGERS.length) return 'All events';
	return (
		on.slice(0, 3).join(', ') +
		(on.length > 3 ? ` +${on.length - 3} more` : '')
	);
}

export type DraftState = {
	provider: ProviderId;
	name: string;
	config: Record<string, string>;
	triggers: Record<string, boolean>;
};

export const emptyDraft = (): DraftState => ({
	provider: 'DISCORD',
	name: '',
	config: {},
	triggers: Object.fromEntries(TRIGGERS.map(t => [t.key, false])),
});

export const draftFromChannel = (n: NotificationPayload): DraftState => ({
	provider: (n.notification_type as ProviderId) ?? 'CUSTOM',
	name: n.name,
	config: {},
	triggers: Object.fromEntries(
		TRIGGERS.map(t => [t.key, n[t.key] === true]),
	),
});
