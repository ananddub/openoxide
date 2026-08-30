import {createServiceClient, useResourceListQuery} from './query';
import {NotificationService} from '#/gen/connect/openoxide/api/v1/notification_pb';

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

/** Notifications for an organization (legacy live: useNotificationListByOrganization). */
export function useNotificationListByOrganization(organization_id: bigint) {
	return useResourceListQuery<NotificationPayload>(
		['notifications', 'list', organization_id.toString()],
		() => notificationClient.listNotifications({organizationId: organization_id}),
	);
}
