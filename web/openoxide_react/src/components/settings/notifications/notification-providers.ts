import type {LucideIcon} from 'lucide-react';
import {
	Bell,
	GitBranch,
	Mail,
	MessageSquare,
	Send,
	Voicemail,
	Wrench,
	Zap,
} from 'lucide-react';

export type ProviderId =
	| 'DISCORD'
	| 'SLACK'
	| 'TELEGRAM'
	| 'EMAIL'
	| 'NTFY'
	| 'GOTIFY'
	| 'MATTERMOST'
	| 'TEAMS'
	| 'CUSTOM';

export type NotificationField = {
	key: string;
	label: string;
	placeholder?: string;
	type?: string;
	optional?: boolean;
};

export type ProviderMeta = {
	id: ProviderId;
	label: string;
	icon: LucideIcon;
	tile: string;
	description: string;
	fields: NotificationField[];
};

const f = (
	key: string,
	label: string,
	extra: Partial<NotificationField> = {},
): NotificationField => ({key, label, ...extra});

export const PROVIDERS: ProviderMeta[] = [
	{
		id: 'DISCORD',
		label: 'Discord',
		icon: MessageSquare,
		tile: 'bg-indigo-500/10 text-indigo-400',
		description: 'Post alerts to a channel webhook.',
		fields: [
			f('webhook_url', 'Webhook URL', {
				placeholder: 'https://discord.com/api/webhooks/…',
			}),
		],
	},
	{
		id: 'SLACK',
		label: 'Slack',
		icon: GitBranch,
		tile: 'bg-emerald-500/10 text-emerald-400',
		description: 'Incoming webhook to any Slack channel.',
		fields: [
			f('webhook_url', 'Webhook URL', {
				placeholder: 'https://hooks.slack.com/services/…',
			}),
			f('channel', 'Channel (optional)', {optional: true}),
		],
	},
	{
		id: 'TELEGRAM',
		label: 'Telegram',
		icon: Send,
		tile: 'bg-sky-500/10 text-sky-400',
		description: 'Bot message to a chat.',
		fields: [
			f('bot_token', 'Bot token', {placeholder: '123456:ABC-DEF…'}),
			f('chat_id', 'Chat ID', {placeholder: '-1001234567890'}),
		],
	},
	{
		id: 'EMAIL',
		label: 'SMTP Email',
		icon: Mail,
		tile: 'bg-rose-500/10 text-rose-400',
		description: 'Send alerts over SMTP.',
		fields: [
			f('smtp_server', 'SMTP server', {placeholder: 'smtp.example.com'}),
			f('smtp_port', 'SMTP port', {type: 'number', placeholder: '587'}),
			f('username', 'Username'),
			f('password', 'Password', {type: 'password'}),
			f('from_address', 'From address', {
				placeholder: 'alerts@example.com',
			}),
			f('to_addresses', 'To addresses', {placeholder: 'a@b.com,c@d.com'}),
		],
	},
	{
		id: 'NTFY',
		label: 'Ntfy',
		icon: Voicemail,
		tile: 'bg-amber-500/10 text-amber-400',
		description: 'Pub-sub push notifications.',
		fields: [
			f('server_url', 'Server URL', {placeholder: 'https://ntfy.sh'}),
			f('topic', 'Topic'),
			f('access_token', 'Access token (optional)', {
				type: 'password',
				optional: true,
			}),
			f('priority', 'Priority (1-5)', {type: 'number', placeholder: '3'}),
		],
	},
	{
		id: 'GOTIFY',
		label: 'Gotify',
		icon: Zap,
		tile: 'bg-violet-500/10 text-violet-400',
		description: 'Self-hosted message server.',
		fields: [
			f('server_url', 'Server URL', {
				placeholder: 'https://gotify.example.com',
			}),
			f('app_token', 'App token', {type: 'password'}),
			f('priority', 'Priority', {type: 'number', placeholder: '5'}),
		],
	},
	{
		id: 'MATTERMOST',
		label: 'Mattermost',
		icon: MessageSquare,
		tile: 'bg-blue-500/10 text-blue-400',
		description: 'Team webhook.',
		fields: [f('webhook_url', 'Webhook URL')],
	},
	{
		id: 'TEAMS',
		label: 'Microsoft Teams',
		icon: GitBranch,
		tile: 'bg-cyan-500/10 text-cyan-400',
		description: 'Workflows / connector webhook.',
		fields: [f('webhook_url', 'Webhook URL')],
	},
	{
		id: 'CUSTOM',
		label: 'Custom webhook',
		icon: Wrench,
		tile: 'bg-muted text-muted-foreground',
		description: 'POST the alert JSON to any endpoint.',
		fields: [
			f('endpoint', 'Endpoint URL'),
			f('headers', 'Headers JSON (optional)', {
				optional: true,
				placeholder: '{"Authorization":"Bearer …"}',
			}),
		],
	},
];

export function providerMeta(type: string): ProviderMeta {
	return (
		PROVIDERS.find(p => p.id === type) ?? {
			id: type as ProviderId,
			label: type,
			icon: Bell,
			tile: 'bg-muted text-muted-foreground',
			description: '',
			fields: [],
		}
	);
}
