import {createServiceClient, useResourceListQuery} from './query';
import {AiService} from '#/gen/connect/openoxide/api/v1/ai_pb';

export type AiSettingPayload = {
	id: number;
	name: string;
	api_url: string;
	model: string;
	is_enabled: boolean;
	provider: string;
	has_api_key: boolean;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

const aiClient = createServiceClient(AiService);

/** AI provider settings (legacy live: useAiListSettings). */
export function useAiListSettings() {
	return useResourceListQuery<AiSettingPayload>(
		['ai_settings', 'list'],
		() => aiClient.listSettings({}),
	);
}
