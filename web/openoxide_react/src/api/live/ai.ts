import {createServiceClient, useConnectMutation, useResourceListQuery, useConnectQuery} from './query';
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

/**
 * AI provider settings with numeric coercion, matching the former zustand
 * `aiSettings` slice data. Key prefix `['ai_settings']` matches the
 * table-watch AI_SETTINGS entry.
 */
export function useAiSettingItems() {
	return useConnectQuery<AiSettingPayload[]>(['ai_settings'], async () => {
		const response = await aiClient.listSettings({});
		return (response as any).items.map((item: Record<string, unknown>) => ({
			...item,
			id: Number(item.id),
			organization_id: Number(item.organization_id),
			created_at: Number(item.created_at),
			updated_at: Number(item.updated_at),
		})) as AiSettingPayload[];
	});
}

/** Delete an AI provider setting by id (legacy REST: DELETE /ai/settings/{id}). */
export function useAiSettingDelete() {
	return useConnectMutation(
		(id: number | bigint) => aiClient.deleteSettings({id: BigInt(id)}),
		// Self-invalidation keeps the actor's own list deterministic even
		// while the table-watch stream is reconnecting; the stream still
		// syncs every other client.
		{invalidate: [['ai_settings']]},
	);
}