import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	ScheduleService,
	ScheduleAction,
	ScheduleShellType,
	ScheduleType,
	type Schedule,
} from '#/gen/connect/openoxide/api/v1/schedule_pb';

/** Schedule payload (legacy `ScheduleResponseDto` shape). The proto carries `shell_type`/`schedule_type`/`schedule_action` as numeric enums and `enabled` as a bool; the legacy surface exposed the enum *names* (e.g. 'BASH', 'APPLICATION', 'EXEC') and 0/1 integers, so the mapper below reverse-translates to keep every consumer working unchanged. */
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

/** Enum value → member name ('BASH', 'APPLICATION', 'EXEC', …). */
function enumName<E extends Record<number, string>>(
	enumeration: E,
	value: number,
): string {
	return enumeration[value] ?? '';
}

/** Loose 0/1/boolean/undefined → proto bool (undefined stays unset). */
function toBool(value: unknown): boolean | undefined {
	if (value === undefined || value === null) return undefined;
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

/** Legacy consumers send the enum names in arbitrary casing ('EXEC', 'exec', 'compose', ...); map them back to the numeric enum with an UNSPECIFIED fallback. */
function parseEnum<E extends Record<string, number | string>>(
	enumeration: E,
	name: unknown,
): number {
	if (typeof name === 'number') return name;
	const key = String(name ?? '').toUpperCase();
	const value = enumeration[key as keyof E];
	return typeof value === 'number' ? value : 0;
}

function toSchedulePayload(schedule: Schedule): SchedulePayload {
	return {
		id: schedule.id,
		name: schedule.name,
		description: schedule.description,
		cron_expression: schedule.cronExpression,
		app_name: schedule.appName,
		service_name: schedule.serviceName,
		shell_type: enumName(
			ScheduleShellType as unknown as Record<number, string>,
			schedule.shellType,
		),
		schedule_type: enumName(
			ScheduleType as unknown as Record<number, string>,
			schedule.scheduleType,
		),
		schedule_action: enumName(
			ScheduleAction as unknown as Record<number, string>,
			schedule.scheduleAction,
		),
		command: schedule.command,
		script: schedule.script,
		timezone: schedule.timezone,
		enabled: schedule.enabled ? 1n : 0n,
		service_id: schedule.serviceId,
		server_id: schedule.serverId,
		organization_id: schedule.organizationId,
		created_at: schedule.createdAt,
		updated_at: schedule.updatedAt,
		application_id: schedule.applicationId,
		compose_id: schedule.composeId,
		database_id: schedule.databaseId,
	};
}

/** Schedules for an organization (legacy live: useScheduleListByOrganization). */
export function useScheduleListByOrganization(organizationId: bigint) {
	return useConnectQuery<SchedulePayload[]>(
		['schedules', 'list', organizationId.toString()],
		async () => {
			const response = await scheduleClient.listOrganizationSchedules({
				organizationId,
			});
			return response.schedules.map(toSchedulePayload);
		},
		{enabled: organizationId > 0n},
	);
}

/** Schedules attached to a compose (legacy live: useScheduleListByCompose). */
export function useScheduleListByCompose(composeId: bigint) {
	return useConnectQuery<SchedulePayload[]>(
		['schedules', 'list', 'compose', composeId.toString()],
		async () => {
			const response = await scheduleClient.listComposeSchedules({
				composeId,
			});
			return response.schedules.map(toSchedulePayload);
		},
		{enabled: composeId > 0n},
	);
}

/** Schedules attached to a database (legacy live: useScheduleListByDatabase). */
export function useScheduleListByDatabase(databaseId: bigint) {
	return useConnectQuery<SchedulePayload[]>(
		['schedules', 'list', 'database', databaseId.toString()],
		async () => {
			const response = await scheduleClient.listDatabaseSchedules({
				databaseId,
			});
			return response.schedules.map(toSchedulePayload);
		},
		{enabled: databaseId > 0n},
	);
}

/** Create a schedule. The legacy REST body used the same snake_case keys as the proto fields plus string enum names; convert to the typed request. */
function toCreateRequest(input: Record<string, unknown>) {
	return {
		name: String(input.name ?? ''),
		description: toOptString(input.description),
		cronExpression: String(input.cron_expression ?? ''),
		appName: toOptString(input.app_name),
		serviceName: toOptString(input.service_name),
		shellType: parseEnum(ScheduleShellType, input.shell_type),
		scheduleType: parseEnum(ScheduleType, input.schedule_type),
		scheduleAction: parseEnum(ScheduleAction, input.schedule_action),
		command: String(input.command ?? ''),
		script: toOptString(input.script),
		timezone: toOptString(input.timezone),
		enabled: toBool(input.enabled),
		applicationId: toBigInt(input.application_id),
		composeId: toBigInt(input.compose_id),
		serverId: toBigInt(input.server_id),
		organizationId: toBigInt(input.organization_id),
	};
}

/** Create a schedule (cron, command and linkage fields in the input). */
export function useScheduleCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			scheduleClient.createSchedule(toCreateRequest(input)),
		{invalidate: [['schedules']]},
	);
}

/** Update a schedule by id (fields to change in the input). */
export function useScheduleUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return scheduleClient.updateSchedule({
				id: BigInt(id),
				...toCreateRequest(body),
				// UpdateScheduleRequest keeps every field optional: drop the create-time '' defaults for anything the caller omitted.
				name: toOptString(body.name),
				cronExpression: toOptString(body.cron_expression),
				shellType:
					body.shell_type === undefined
						? undefined
						: parseEnum(ScheduleShellType, body.shell_type),
				scheduleType:
					body.schedule_type === undefined
						? undefined
						: parseEnum(ScheduleType, body.schedule_type),
				scheduleAction:
					body.schedule_action === undefined
						? undefined
						: parseEnum(ScheduleAction, body.schedule_action),
				command: toOptString(body.command),
			});
		},
		{invalidate: [['schedules']]},
	);
}

/** Delete a schedule by id. */
export function useScheduleDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			scheduleClient.deleteSchedule({id: BigInt(id)}),
		{invalidate: [['schedules']]},
	);
}

/** Trigger an immediate schedule run by id (legacy REST: POST /schedules/{id}/trigger). */
export function useScheduleRun() {
	return useConnectMutation(
		(id: number | bigint) =>
			scheduleClient.triggerSchedule({id: BigInt(id)}),
		{invalidate: [['schedules']]},
	);
}

/** Pause a schedule by id (dedicated PauseSchedule RPC). */
export function useSchedulePause() {
	return useConnectMutation(
		(id: number | bigint) =>
			scheduleClient.pauseSchedule({id: BigInt(id)}),
		{invalidate: [['schedules']]},
	);
}

/** Resume a schedule by id (dedicated ResumeSchedule RPC). */
export function useScheduleResume() {
	return useConnectMutation(
		(id: number | bigint) =>
			scheduleClient.resumeSchedule({id: BigInt(id)}),
		{invalidate: [['schedules']]},
	);
}
