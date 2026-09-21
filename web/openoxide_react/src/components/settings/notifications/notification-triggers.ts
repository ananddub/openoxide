import type {NotificationPayload} from '#/api/live';
import type {ProviderId} from './notification-providers';

export const TRIGGERS: {key: keyof NotificationPayload; label: string}[] =
	[
		{key: 'on_app_deploy', label: 'App deployed'},
		{key: 'on_app_build_error', label: 'Build failed'},
		{key: 'on_database_backup', label: 'Database backup'},
		{key: 'on_volume_backup', label: 'Volume backup'},
		{key: 'on_panel_restart', label: 'Panel restart'},
		{key: 'on_docker_cleanup', label: 'Docker cleanup'},
		{key: 'on_server_threshold', label: 'Server threshold'},
		{key: 'on_panel_backup', label: 'Panel backup'},
		{key: 'on_schedule_success', label: 'Schedule success'},
		{key: 'on_schedule_failure', label: 'Schedule failure'},
	];

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
