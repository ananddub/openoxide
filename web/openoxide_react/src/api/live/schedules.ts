import {createServiceClient, useResourceListQuery} from './query';
import {ScheduleService} from '#/gen/connect/openoxide/api/v1/schedule_pb';

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

const scheduleClient = createServiceClient(ScheduleService);

/** Payload ids deserialize from JSON as numbers; coerce before comparing. */
function payloadIdEquals(value: unknown, expected: bigint): boolean {
	if (value === undefined || value === null) return false;
	try {
		return BigInt(value as string | number | bigint) === expected;
	} catch {
		return false;
	}
}

/** Schedules for an organization (legacy live: useScheduleListByOrganization). */
export function useScheduleListByOrganization(organizationId: bigint) {
	return useResourceListQuery<SchedulePayload>(
		['schedules', 'list', organizationId.toString()],
		() => scheduleClient.listSchedules({organizationId}),
	);
}

/**
 * Schedules attached to a compose (legacy live: useScheduleListByCompose).
 * ListSchedules only filters server-side by organization; the compose
 * linkage lives inside each JSON payload, so fetch all and filter
 * client-side on `compose_id`.
 */
export function useScheduleListByCompose(composeId: bigint) {
	const query = useResourceListQuery<SchedulePayload>(
		['schedules', 'list', 'compose', composeId.toString()],
		() => scheduleClient.listSchedules({}),
	);
	return {
		...query,
		data: query.data?.filter((schedule) => payloadIdEquals(schedule.compose_id, composeId)),
	};
}

/**
 * Schedules attached to a database (legacy live: useScheduleListByDatabase).
 * Same reason as compose: the database linkage only exists inside the JSON
 * payload, so filter client-side on `database_id`.
 */
export function useScheduleListByDatabase(databaseId: bigint) {
	const query = useResourceListQuery<SchedulePayload>(
		['schedules', 'list', 'database', databaseId.toString()],
		() => scheduleClient.listSchedules({}),
	);
	return {
		...query,
		data: query.data?.filter((schedule) => payloadIdEquals(schedule.database_id, databaseId)),
	};
}
