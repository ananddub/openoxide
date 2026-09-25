import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	SettingsCertificateType,
	SettingsService,
	type Settings,
} from '#/gen/connect/openoxide/api/v1/settings_pb';

/** Instance settings payload mirrors the legacy live shape (snake_case JSON payload of the settings DTO). `certificate_type` is "NONE" | "LETSENCRYPT" | "CUSTOM"; `metrics_config` is a JSON string. */
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

function certificateTypeName(type: SettingsCertificateType): string {
	switch (type) {
		case SettingsCertificateType.LETSENCRYPT:
			return 'LETSENCRYPT';
		case SettingsCertificateType.CUSTOM:
			return 'CUSTOM';
		case SettingsCertificateType.NONE:
		case SettingsCertificateType.UNSPECIFIED:
		default:
			return 'NONE';
	}
}

/** Accepts the legacy serde variant names ("None" | "Letsencrypt" | "Custom") as well as the uppercased forms; unknown values are omitted. */
function parseCertificateType(
	value: unknown,
): SettingsCertificateType | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	switch (String(value).toLowerCase()) {
		case 'none':
			return SettingsCertificateType.NONE;
		case 'letsencrypt':
			return SettingsCertificateType.LETSENCRYPT;
		case 'custom':
			return SettingsCertificateType.CUSTOM;
		default:
			return undefined;
	}
}

function toSettingsPayload(settings: Settings): SettingsPayload {
	return {
		id: settings.id,
		server_ip: settings.serverIp ?? null,
		certificate_type: certificateTypeName(settings.certificateType),
		custom_cert_resolver: settings.customCertResolver ?? null,
		https: settings.https,
		host: settings.host ?? null,
		lets_encrypt_email: settings.letsEncryptEmail ?? null,
		enable_docker_cleanup: settings.enableDockerCleanup,
		log_cleanup_cron: settings.logCleanupCron ?? null,
		metrics_config: settings.metricsConfig,
		panel_backup_cron: settings.panelBackupCron,
		log_retention_days: settings.logRetentionDays,
		panel_backup_enabled: settings.panelBackupEnabled,
		log_cleanup_enabled: settings.logCleanupEnabled,
		updated_at: settings.updatedAt,
	};
}

/** Instance settings (legacy live: useSettingsGet). */
export function useSettingsGet(options?: {enabled?: boolean}) {
	return useConnectQuery<SettingsPayload>(
		['settings', 'get'],
		async () => {
			const response = await settingsClient.getSettings({});
			if (!response.settings) throw new Error('Settings not initialized');
			return toSettingsPayload(response.settings);
		},
		options,
	);
}

/** Update payload mirrors `UpdateSettingsDto` (all fields optional). */
export type SettingsUpdatePayload = {
	server_ip?: string | null;
	/** "None" | "Letsencrypt" | "Custom" (serde variant names). */
	certificate_type?: string;
	custom_cert_resolver?: string | null;
	https?: boolean;
	host?: string | null;
	lets_encrypt_email?: string | null;
	enable_docker_cleanup?: boolean;
	log_cleanup_cron?: string | null;
	metrics_config?: string;
};

/** Legacy JSON nulls meant "leave the stored value alone" (patch semantics); protobuf absence carries the same meaning, so nulls drop out of the request while empty strings are sent and clear the column. */
function nullable(value: string | null | undefined): string | undefined {
	return value === null ? undefined : value;
}

/** Persist instance settings (backend gate: org:update). */
export function useSettingsUpdate() {
	return useConnectMutation(
		(input: SettingsUpdatePayload) =>
			settingsClient.updateSettings({
				serverIp: nullable(input.server_ip),
				certificateType: parseCertificateType(input.certificate_type),
				customCertResolver: nullable(input.custom_cert_resolver),
				https: input.https,
				host: nullable(input.host),
				letsEncryptEmail: nullable(input.lets_encrypt_email),
				enableDockerCleanup: input.enable_docker_cleanup,
				logCleanupCron: nullable(input.log_cleanup_cron),
				metricsConfig: input.metrics_config,
			}),
		{invalidate: [['settings']]},
	);
}
