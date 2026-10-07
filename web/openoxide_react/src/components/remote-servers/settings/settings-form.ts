import type {RemoteServerPayload} from '#/api/live/servers';

/** Everything the Settings tab edits; every field maps onto a `UpdateRemoteServer` (PATCH) slot. */
export type ServerSettingsForm = {
	name: string;
	description: string;
	ip_address: string;
	port: string;
	username: string;
	ssh_key_id: string;
	server_type: string;
	build_memory_limit: string;
	build_cpu_limit: string;
	enable_docker_cleanup: boolean;
	log_cleanup_cron: string;
};

export function toSettingsForm(
	server: RemoteServerPayload,
): ServerSettingsForm {
	return {
		name: server.name ?? '',
		description: server.description ?? '',
		ip_address: server.ip_address ?? '',
		port: String(server.port || 22),
		username: server.username || 'root',
		ssh_key_id: server.ssh_key_id ? String(server.ssh_key_id) : '',
		server_type: (server.server_type || 'DEPLOY').toUpperCase(),
		build_memory_limit: server.build_memory_limit ?? '',
		build_cpu_limit: server.build_cpu_limit ?? '',
		enable_docker_cleanup: Number(server.enable_docker_cleanup ?? 0) > 0,
		log_cleanup_cron: server.log_cleanup_cron ?? '',
	};
}

/** PATCH body carrying only what the operator actually changed. An optional int64 has no "unset" on the wire, so clearing the SSH key is not offered. */
export type ServerSettingsPatch = {
	name?: string;
	description?: string;
	ip_address?: string;
	port?: number;
	username?: string;
	ssh_key_id?: number;
	server_type?: string;
	build_memory_limit?: string;
	build_cpu_limit?: string;
	enable_docker_cleanup?: boolean;
	log_cleanup_cron?: string;
};

export function buildSettingsPatch(
	form: ServerSettingsForm,
	initial: ServerSettingsForm,
): ServerSettingsPatch {
	const patch: ServerSettingsPatch = {};
	const trim = (value: string) => value.trim();

	if (trim(form.name) !== trim(initial.name) && trim(form.name))
		patch.name = trim(form.name);
	if (trim(form.description) !== trim(initial.description))
		patch.description = trim(form.description);
	if (
		trim(form.ip_address) !== trim(initial.ip_address) &&
		trim(form.ip_address)
	)
		patch.ip_address = trim(form.ip_address);
	if (form.port !== initial.port) patch.port = Number(form.port) || 22;
	if (
		trim(form.username) !== trim(initial.username) &&
		trim(form.username)
	)
		patch.username = trim(form.username);
	if (form.ssh_key_id !== initial.ssh_key_id && form.ssh_key_id)
		patch.ssh_key_id = Number(form.ssh_key_id);
	if (form.server_type !== initial.server_type)
		patch.server_type = form.server_type;
	if (trim(form.build_memory_limit) !== trim(initial.build_memory_limit))
		patch.build_memory_limit = trim(form.build_memory_limit);
	if (trim(form.build_cpu_limit) !== trim(initial.build_cpu_limit))
		patch.build_cpu_limit = trim(form.build_cpu_limit);
	if (form.enable_docker_cleanup !== initial.enable_docker_cleanup)
		patch.enable_docker_cleanup = form.enable_docker_cleanup;
	if (trim(form.log_cleanup_cron) !== trim(initial.log_cleanup_cron))
		patch.log_cleanup_cron = trim(form.log_cleanup_cron);

	return patch;
}

/** Server name and address are required by the backend; the rest may stay as-is. */
export function settingsProblem(form: ServerSettingsForm): string | null {
	if (!form.name.trim()) return 'Give this server a name.';
	if (!form.ip_address.trim())
		return 'Set the address OpenOxide reaches over SSH.';
	if (!form.username.trim()) return 'Set the SSH user.';
	const port = Number(form.port);
	if (!Number.isInteger(port) || port < 1 || port > 65_535)
		return 'SSH port must be between 1 and 65535.';
	return null;
}

/** Build limits only mean something for a build node. */
export const isBuildNode = (form: ServerSettingsForm): boolean =>
	form.server_type === 'BUILD';

/** Cleanup needs a schedule; the backend keeps the flag off without one. */
export const cleanupProblem = (
	form: ServerSettingsForm,
): string | null => {
	if (!form.enable_docker_cleanup) return null;
	const cron = form.log_cleanup_cron.trim();
	if (!cron) return 'Docker cleanup is on but has no schedule.';
	if (cron.split(/\s+/).length !== 5)
		return 'Use a 5-field cron expression, e.g. “0 2 * * *”.';
	return null;
};
