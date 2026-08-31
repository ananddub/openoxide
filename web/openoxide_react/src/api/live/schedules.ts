import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
// Only ScheduleOperationsService is served by the backend (ScheduleService
// has no controller), so the live layer targets the operations variant.
import {ScheduleOperationsService} from '#/gen/connect/openoxide/api/v1/schedule_operations_pb';

/**
 * Schedule JSON payload (backend `ScheduleResponseDto`). The linkage fields
 * `application_id`/`compose_id`/`database_id` ride inside the payload rather
 * than the resource envelope, so they stay optional extras.
 */
export type SchedulePayload = {
	id: bigint;
	name: string;
	description?: string;
	cron_expression: string;
	app_name: string;
	service_name?: string;
	shell_type: string;
	schedule_type: string;
	schedule_action: string;
	command: string;
	script?: string;
	timezone?: string;
	enabled: bigint;
	service_id?: bigint;
	server_id?: bigint;
	organization_id?: bigint;
	created_at: bigint;
	updated_at: bigint;
	application_id?: bigint;
	compose_id?: bigint;
	database_id?: bigint;
	[key: string]: unknown;
};

const scheduleClient = createServiceClient(ScheduleOperationsService);

/** Schedules for an organization (legacy live: useScheduleListByOrganization). */
export function useScheduleListByOrganization(organizationId: bigint) {
	return useResourceListQuery<SchedulePayload>(
		['schedules', 'list', organizationId.toString()],
		() => scheduleClient.listOrganizationSchedules({organizationId}),
	);
}

/**
 * Schedules attached to a compose (legacy live: useScheduleListByCompose).
 * The backend reads the compose id from the query's `organization_id` slot,
 * so ListComposeSchedules filters server-side.
 */
export function useScheduleListByCompose(composeId: bigint) {
	return useResourceListQuery<SchedulePayload>(
		['schedules', 'list', 'compose', composeId.toString()],
		() => scheduleClient.listComposeSchedules({organizationId: composeId}),
	);
}

/**
 * Schedules attached to a database (legacy live: useScheduleListByDatabase).
 * ListDatabaseSchedules carries the database id in the query's
 * `organization_id` slot; the backend currently returns an empty list for it.
 */
export function useScheduleListByDatabase(databaseId: bigint) {
	return useResourceListQuery<SchedulePayload>(
		['schedules', 'list', 'database', databaseId.toString()],
		() => scheduleClient.listDatabaseSchedules({organizationId: databaseId}),
	);
}

/** Create a schedule (cron, command and linkage fields in the payload). */
export function useScheduleCreate() {
	return useConnectMutation((input: Record<string, unknown>) =>
		scheduleClient.createSchedule({payload: JSON.stringify(input)}),
	);
}

/** Update a schedule by id (fields to change in the payload). */
export function useScheduleUpdate() {
	return useConnectMutation((input: {id: number | bigint; [field: string]: unknown}) => {
		const {id, ...body} = input;
		return scheduleClient.updateSchedule({id: BigInt(id), payload: JSON.stringify(body)});
	});
}

/** Delete a schedule by id. */
export function useScheduleDelete() {
	return useConnectMutation((id: number | bigint) =>
		scheduleClient.deleteSchedule({id: BigInt(id)}),
	);
}

/** Trigger an immediate schedule run by id (legacy REST: POST /schedules/{id}/trigger). */
export function useScheduleRun() {
	return useConnectMutation((id: number | bigint) =>
		scheduleClient.triggerSchedule({id: BigInt(id)}),
	);
}

/**
 * Pause a schedule by id. There is no dedicated backend RPC; pausing is the
 * legacy REST patch with `enabled: 0`.
 */
export function useSchedulePause() {
	return useConnectMutation((id: number | bigint) =>
		scheduleClient.updateSchedule({id: BigInt(id), payload: JSON.stringify({enabled: 0})}),
	);
}

/**
 * Resume a schedule by id. There is no dedicated backend RPC; resuming is
 * the legacy REST patch with `enabled: 1`.
 */
export function useScheduleResume() {
	return useConnectMutation((id: number | bigint) =>
		scheduleClient.updateSchedule({id: BigInt(id), payload: JSON.stringify({enabled: 1})}),
	);
}
