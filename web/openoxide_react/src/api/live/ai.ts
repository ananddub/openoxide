import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	AiService,
	AiProvider,
	type AiSetting,
} from '#/gen/connect/openoxide/api/v1/ai_pb';

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

function providerToString(provider: AiProvider): string {
	switch (provider) {
		case AiProvider.OPENAI:
			return 'openai';
		case AiProvider.ANTHROPIC:
			return 'anthropic';
		case AiProvider.GEMINI:
			return 'gemini';
		case AiProvider.OLLAMA:
			return 'ollama';
		case AiProvider.OPENAI_COMPATIBLE:
			return 'openai_compatible';
		default:
			return '';
	}
}

function toAiSettingPayload(setting: AiSetting): AiSettingPayload {
	return {
		id: Number(setting.id),
		name: setting.name,
		api_url: setting.apiUrl,
		model: setting.model,
		is_enabled: setting.isEnabled,
		provider: providerToString(setting.provider),
		has_api_key: setting.hasApiKey,
		organization_id: Number(setting.organizationId),
		created_at: Number(setting.createdAt),
		updated_at: Number(setting.updatedAt),
	};
}

/** AI provider settings (legacy live: useAiListSettings). */
export function useAiListSettings() {
	return useConnectQuery<AiSettingPayload[]>(
		['ai_settings', 'list'],
		async () => {
			const response = await aiClient.listSettings({});
			return response.settings.map(toAiSettingPayload);
		},
	);
}

/** AI provider settings with numeric coercion, matching the former zustand `aiSettings` slice data. Key prefix `['ai_settings']` matches the table-watch AI_SETTINGS entry. */
export function useAiSettingItems() {
	return useConnectQuery<AiSettingPayload[]>(['ai_settings'], async () => {
		const response = await aiClient.listSettings({});
		return response.settings.map(toAiSettingPayload);
	});
}

/** Delete an AI provider setting by id (legacy REST: DELETE /ai/settings/{id}). */
export function useAiSettingDelete() {
	return useConnectMutation(
		(id: number | bigint) => aiClient.deleteSettings({id: BigInt(id)}),
		// Self-invalidation keeps the actor's own list deterministic even while the table-watch stream is reconnecting; the stream still syncs every other client.
		{invalidate: [['ai_settings']]},
	);
}
