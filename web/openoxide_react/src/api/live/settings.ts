import {createServiceClient, useResourceQuery} from './query';
import {SettingsService} from '#/gen/connect/openoxide/api/v1/settings_pb';

/**
 * Instance settings payload mirrors the legacy live shape (snake_case JSON
 * payload of the settings DTO). `certificate_type` is "NONE" | "LETSENCRYPT"
 * | "CUSTOM"; `metrics_config` is a JSON string.
 */
export type SettingsPayload = {
	id: bigint;
	server_ip: string | null;
	certificate_type: string;
	custom_cert_resolver: string | null;
	https: boolean;
	host: string | null;
	lets_encrypt_email: string | null;
	enable_docker_cleanup: boolean;
	log_cleanup_cron: string | null;
	metrics_config: string;
	panel_backup_cron: string;
	log_retention_days: bigint;
	panel_backup_enabled: boolean;
	log_cleanup_enabled: boolean;
	updated_at: bigint;
};

const settingsClient = createServiceClient(SettingsService);

/** Instance settings (legacy live: useSettingsGet). */
export function useSettingsGet() {
	// Request is `Empty` (no id), but the response still wraps the settings
	// JSON in a `ResourceResponse` payload.
	return useResourceQuery<SettingsPayload>(
		['settings', 'get'],
		() => settingsClient.getSettings({}),
	);
}
