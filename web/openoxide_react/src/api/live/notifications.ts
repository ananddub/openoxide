import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	NotificationService,
	NotificationProviderType,
	type Notification,
	type NotificationDeliveryAttempt,
} from '#/gen/connect/openoxide/api/v1/notification_pb';

export type NotificationPayload = {
	id: number;
	name: string;
	notification_type: string;
	on_app_deploy: boolean;
	on_app_build_error: boolean;
	on_database_backup: boolean;
	on_volume_backup: boolean;
	on_panel_restart: boolean;
	on_docker_cleanup: boolean;
	on_server_threshold: boolean;
	on_panel_backup: boolean;
	on_schedule_success: boolean;
	on_schedule_failure: boolean;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

const notificationClient = createServiceClient(NotificationService);

/** One row of `notification_delivery_attempts` — a dispatch attempt the
 * backend made for a channel (status: PENDING | DELIVERED | FAILED). */
export type NotificationDeliveryPayload = {
	id: number;
	notification_id: number;
	organization_id: number;
	trigger_name: string;
	correlation_id: string;
	status: string;
	attempt: number;
	title: string;
	body: string;
	error: string | null;
	created_at: number;
	finished_at: number | null;
};

/** Enum value → member name ('SLACK', 'TELEGRAM', …). */
function enumName<E extends Record<number, string>>(
	enumeration: E,
	value: number,
): string {
	return enumeration[value] ?? '';
}

/** Loose 0/1/boolean/undefined → proto bool defaulting to false. */
function toBool(value: unknown, fallback = false): boolean {
	if (value === undefined || value === null || value === '')
		return fallback;
	return Boolean(Number(value));
}

/** Loose number/string/bigint → proto int64 (undefined stays unset). */
function toBigInt(value: unknown): bigint | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	return BigInt(value as number | string | bigint);
}

/** Loose optional string → proto `optional string` (undefined stays unset). */
function toOptString(value: unknown): string | undefined {
	if (value === undefined || value === null) return undefined;
	return String(value);
}

function toNotificationPayload(
	notification: Notification,
): NotificationPayload {
	return {
		id: Number(notification.id),
		name: notification.name,
		notification_type: enumName(
			NotificationProviderType,
			notification.notificationType,
		),
		on_app_deploy: notification.onAppDeploy,
		on_app_build_error: notification.onAppBuildError,
		on_database_backup: notification.onDatabaseBackup,
		on_volume_backup: notification.onVolumeBackup,
		on_panel_restart: notification.onPanelRestart,
		on_docker_cleanup: notification.onDockerCleanup,
		on_server_threshold: notification.onServerThreshold,
		on_panel_backup: notification.onPanelBackup,
		on_schedule_success: notification.onScheduleSuccess,
		on_schedule_failure: notification.onScheduleFailure,
		organization_id: Number(notification.organizationId),
		created_at: Number(notification.createdAt),
		updated_at: Number(notification.updatedAt),
	};
}

function toDeliveryPayload(
	attempt: NotificationDeliveryAttempt,
): NotificationDeliveryPayload {
	return {
		id: Number(attempt.id),
		notification_id: Number(attempt.notificationId),
		organization_id: Number(attempt.organizationId),
		trigger_name: attempt.triggerName,
		correlation_id: attempt.correlationId,
		status: attempt.status,
		attempt: Number(attempt.attempt),
		title: attempt.title,
		body: attempt.body,
		error: attempt.error ?? null,
		created_at: Number(attempt.createdAt),
		finished_at:
			attempt.finishedAt === undefined ? null : Number(attempt.finishedAt),
	};
}

/** Recent delivery attempts for an organization, newest first (the backend
 * caps the page at 200). */
export function useNotificationDeliveries(organization_id: bigint) {
	return useConnectQuery<NotificationDeliveryPayload[]>(
		['notifications', 'deliveries', organization_id.toString()],
		async () => {
			const response = await notificationClient.listDeliveryHistory({
				organizationId: organization_id,
			});
			return response.attempts.map(toDeliveryPayload);
		},
	);
}

/** Notifications for an organization (legacy live: useNotificationListByOrganization). */
export function useNotificationListByOrganization(
	organization_id: bigint,
) {
	return useConnectQuery<NotificationPayload[]>(
		['notifications', 'list', organization_id.toString()],
		async () => {
			const response = await notificationClient.listNotifications({
				organizationId: organization_id,
			});
			return response.notifications.map(toNotificationPayload);
		},
	);
}

/** Provider config keys mirror the UI dialog (snake_case, see `notification-providers.ts`); the proto wants exactly one provider message on the `provider` oneof, whose case also fixes the stored notification_type. */
function buildProviderOneof(
	type: string,
	config: Record<string, unknown>,
) {
	switch (type.toUpperCase()) {
		case 'SLACK':
			return {
				case: 'slack' as const,
				value: {
					webhookUrl: String(config.webhook_url ?? ''),
					channel: toOptString(config.channel),
				},
			};
		case 'TELEGRAM':
			return {
				case: 'telegram' as const,
				value: {
					botToken: String(config.bot_token ?? ''),
					chatId: String(config.chat_id ?? ''),
					messageThreadId: toOptString(config.message_thread_id),
				},
			};
		case 'DISCORD':
			return {
				case: 'discord' as const,
				value: {
					webhookUrl: String(config.webhook_url ?? ''),
					decoration: toBool(config.decoration),
				},
			};
		case 'EMAIL':
			return {
				case: 'email' as const,
				value: {
					smtpServer: String(config.smtp_server ?? ''),
					smtpPort: toBigInt(config.smtp_port) ?? 0n,
					username: String(config.username ?? ''),
					password: String(config.password ?? ''),
					fromAddress: String(config.from_address ?? ''),
					toAddresses: Array.isArray(config.to_addresses)
						? config.to_addresses.map(String)
						: [],
				},
			};
		case 'RESEND':
			return {
				case: 'resend' as const,
				value: {
					apiKey: String(config.api_key ?? ''),
					fromAddress: String(config.from_address ?? ''),
					toAddresses: Array.isArray(config.to_addresses)
						? config.to_addresses.map(String)
						: [],
				},
			};
		case 'GOTIFY':
			return {
				case: 'gotify' as const,
				value: {
					serverUrl: String(config.server_url ?? ''),
					appToken: String(config.app_token ?? ''),
					priority: toBigInt(config.priority) ?? 0n,
					decoration: toBool(config.decoration),
				},
			};
		case 'NTFY':
			return {
				case: 'ntfy' as const,
				value: {
					serverUrl: String(config.server_url ?? ''),
					topic: String(config.topic ?? ''),
					accessToken: toOptString(config.access_token),
					priority: toBigInt(config.priority) ?? 0n,
				},
			};
		case 'MATTERMOST':
			return {
				case: 'mattermost' as const,
				value: {
					webhookUrl: String(config.webhook_url ?? ''),
					channel: toOptString(config.channel),
					username: toOptString(config.username),
				},
			};
		case 'CUSTOM':
			return {
				case: 'custom' as const,
				value: {
					endpoint: String(config.endpoint ?? ''),
					headers: toOptString(config.headers),
				},
			};
		case 'LARK':
			return {
				case: 'lark' as const,
				value: {webhookUrl: String(config.webhook_url ?? '')},
			};
		case 'PUSHOVER':
			return {
				case: 'pushover' as const,
				value: {
					userKey: String(config.user_key ?? ''),
					apiToken: String(config.api_token ?? ''),
					priority: toBigInt(config.priority) ?? 0n,
					retry: toBigInt(config.retry),
					expire: toBigInt(config.expire),
				},
			};
		case 'TEAMS':
			return {
				case: 'teams' as const,
				value: {webhookUrl: String(config.webhook_url ?? '')},
			};
		default:
			return {case: undefined, value: undefined};
	}
}

/** Create a notification (legacy REST: POST /notifications). The loose input keeps the dialog's `{settings, provider:{notification_type, config}}` body; the mapper translates it to the typed request. */
export function useNotificationCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) => {
			const settings = (input.settings ?? {}) as Record<string, unknown>;
			const provider = (input.provider ?? {}) as {
				notification_type?: string;
				config?: Record<string, unknown>;
			};
			return notificationClient.createNotification({
				name: String(settings.name ?? ''),
				organizationId: toBigInt(settings.organization_id) ?? 0n,
				onAppDeploy: toBool(settings.on_app_deploy),
				onAppBuildError: toBool(settings.on_app_build_error),
				onDatabaseBackup: toBool(settings.on_database_backup),
				onVolumeBackup: toBool(settings.on_volume_backup),
				onPanelRestart: toBool(settings.on_panel_restart),
				onDockerCleanup: toBool(settings.on_docker_cleanup),
				onServerThreshold: toBool(settings.on_server_threshold),
				onPanelBackup: toBool(settings.on_panel_backup),
				onScheduleSuccess: toBool(settings.on_schedule_success),
				onScheduleFailure: toBool(settings.on_schedule_failure),
				provider: buildProviderOneof(
					provider.notification_type ?? '',
					provider.config ?? {},
				),
			});
		},
		{invalidate: [['notifications']]},
	);
}

/** PATCH input helper: only present keys become optional proto fields. */
function optBoolField(
	body: Record<string, unknown>,
	key: string,
): boolean | undefined {
	return key in body ? toBool(body[key], false) : undefined;
}

/** Update a notification by id (legacy patch body: name + `on_*` triggers, optional flat provider fields). PATCH semantics: absent keys keep their stored values. */
export function useNotificationUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return notificationClient.updateNotification({
				id: BigInt(id),
				organizationId: toBigInt(body.organization_id),
				name: toOptString(body.name),
				onAppDeploy: optBoolField(body, 'on_app_deploy'),
				onAppBuildError: optBoolField(body, 'on_app_build_error'),
				onDatabaseBackup: optBoolField(body, 'on_database_backup'),
				onVolumeBackup: optBoolField(body, 'on_volume_backup'),
				onPanelRestart: optBoolField(body, 'on_panel_restart'),
				onDockerCleanup: optBoolField(body, 'on_docker_cleanup'),
				onServerThreshold: optBoolField(body, 'on_server_threshold'),
				onPanelBackup: optBoolField(body, 'on_panel_backup'),
				onScheduleSuccess: optBoolField(body, 'on_schedule_success'),
				onScheduleFailure: optBoolField(body, 'on_schedule_failure'),
				webhookUrl: toOptString(body.webhook_url),
				channel: toOptString(body.channel),
				username: toOptString(body.username),
				decoration: toBigInt(body.decoration),
				botToken: toOptString(body.bot_token),
				chatId: toOptString(body.chat_id),
				messageThreadId: toOptString(body.message_thread_id),
				smtpServer: toOptString(body.smtp_server),
				smtpPort: toBigInt(body.smtp_port),
				password: toOptString(body.password),
				fromAddress: toOptString(body.from_address),
				toAddresses: Array.isArray(body.to_addresses)
					? body.to_addresses.map(String)
					: [],
				apiKey: toOptString(body.api_key),
				serverUrl: toOptString(body.server_url),
				appToken: toOptString(body.app_token),
				priority: toBigInt(body.priority),
				topic: toOptString(body.topic),
				accessToken: toOptString(body.access_token),
				endpoint: toOptString(body.endpoint),
				headers: toOptString(body.headers),
				userKey: toOptString(body.user_key),
				apiToken: toOptString(body.api_token),
				retry: toBigInt(body.retry),
				expire: toBigInt(body.expire),
			});
		},
		{invalidate: [['notifications']]},
	);
}

/** Delete a notification by id. */
export function useNotificationDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			notificationClient.deleteNotification({id: BigInt(id)}),
		{invalidate: [['notifications']]},
	);
}

/** Send a test notification by id (legacy REST: POST /notifications/{id}/test). The backend dispatches a fixed test message, so only the id matters. */
export function useNotificationTest() {
	return useConnectMutation((id: number | bigint) =>
		notificationClient.testNotification({id: BigInt(id)}),
	);
}
