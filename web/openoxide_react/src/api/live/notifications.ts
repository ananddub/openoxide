import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
// Only NotificationOperationsService is served by the backend
// (NotificationService has no controller), so the live layer targets the
// operations variant.
import {NotificationOperationsService} from '#/gen/connect/openoxide/api/v1/notification_operations_pb';

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

const notificationClient = createServiceClient(NotificationOperationsService);

/** Notifications for an organization (legacy live: useNotificationListByOrganization). */
export function useNotificationListByOrganization(organization_id: bigint) {
	return useResourceListQuery<NotificationPayload>(
		['notifications', 'list', organization_id.toString()],
		() => notificationClient.listNotifications({organizationId: organization_id}),
	);
}

/**
 * Create a notification (legacy REST: POST /notifications). The settings
 * object and provider fields ride in the payload (backend
 * `CreateNotificationDto`).
 */
export function useNotificationCreate() {
	return useConnectMutation((input: Record<string, unknown>) =>
		notificationClient.createNotification({payload: JSON.stringify(input)}),
	);
}

/** Update a notification by id (fields to change in the payload). */
export function useNotificationUpdate() {
	return useConnectMutation((input: {id: number | bigint; [field: string]: unknown}) => {
		const {id, ...body} = input;
		return notificationClient.updateNotification({id: BigInt(id), payload: JSON.stringify(body)});
	});
}

/** Delete a notification by id. */
export function useNotificationDelete() {
	return useConnectMutation((id: number | bigint) =>
		notificationClient.deleteNotification({id: BigInt(id)}),
	);
}

/**
 * Send a test notification by id (legacy REST: POST /notifications/{id}/test).
 * The backend dispatches a fixed test message, so only the id matters.
 */
export function useNotificationTest() {
	return useConnectMutation((id: number | bigint) =>
		notificationClient.testNotification({id: BigInt(id), payload: '{}'}),
	);
}
