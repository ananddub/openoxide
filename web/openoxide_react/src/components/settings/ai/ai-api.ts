import {
	AiProvider,
	AiService,
	type AiSetting,
} from '#/gen/connect/openoxide/api/v1/ai_pb';
import {createServiceClient, useConnectQuery} from '#/api/live/query';

export type AiSettingPayload = {
	id: number;
	name: string;
	provider: string;
	api_url: string;
	api_key: string;
	model: string;
	is_enabled?: boolean;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

export type AiModelPayload = {
	id?: string;
	model?: string;
	name?: string;
	[key: string]: unknown;
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

// `api_key` is never echoed back by the typed API — the presence flag (`has_api_key`) lives on the message instead — so the legacy payload key stays as an empty string.
function toAiSettingPayload(setting: AiSetting): AiSettingPayload {
	return {
		id: Number(setting.id),
		name: setting.name,
		provider: providerToString(setting.provider),
		api_url: setting.apiUrl,
		api_key: '',
		model: setting.model,
		is_enabled: setting.isEnabled,
		organization_id: Number(setting.organizationId),
		created_at: Number(setting.createdAt),
		updated_at: Number(setting.updatedAt),
	};
}

// Discovery/list-models now return plain string arrays; wrap them in the legacy model payload shape the dialog reads via `model` / `name`.
function toAiModelPayloads(models: string[]): AiModelPayload[] {
	return models.map(model => ({model, name: model}));
}

/** All configured AI providers. */
export function useAiSettingsList() {
	return useConnectQuery<AiSettingPayload[]>(
		['ai_settings', 'list'],
		async () => {
			const response = await aiClient.listSettings({});
			return response.settings.map(toAiSettingPayload);
		},
	);
}

export async function listAiSettings() {
	const response = await aiClient.listSettings({});
	return response.settings.map(toAiSettingPayload);
}

export async function createAiSetting(body: Partial<AiSettingPayload>) {
	const response = await aiClient.createSettings({
		name: body.name ?? '',
		apiUrl: body.api_url ?? '',
		apiKey: body.api_key ?? '',
		model: body.model ?? '',
		isEnabled: body.is_enabled ?? true,
	});
	if (!response.setting) {
		throw new Error('AI settings response contained no setting');
	}
	return toAiSettingPayload(response.setting);
}

export async function updateAiSetting(
	id: number,
	body: Partial<AiSettingPayload>,
) {
	const response = await aiClient.updateSettings({
		id: BigInt(id),
		name: body.name,
		apiUrl: body.api_url,
		// An absent `api_key` keeps the stored key server-side.
		apiKey: body.api_key,
		model: body.model,
		isEnabled: body.is_enabled,
	});
	if (!response.setting) {
		throw new Error('AI settings response contained no setting');
	}
	return toAiSettingPayload(response.setting);
}

export async function deleteAiSetting(id: number) {
	await aiClient.deleteSettings({id: BigInt(id)});
}

export async function discoverAiModels(body: {
	api_url: string;
	api_key: string;
}) {
	const response = await aiClient.discoverModels({
		apiUrl: body.api_url,
		apiKey: body.api_key,
	});
	return toAiModelPayloads(response.models);
}

export async function discoverSettingModels(id: number) {
	const response = await aiClient.listModels({settingId: BigInt(id)});
	return toAiModelPayloads(response.models);
}

export async function testAiConnection(body: {
	api_url: string;
	api_key: string;
	model: string;
}) {
	return aiClient.testConnection({
		apiUrl: body.api_url,
		apiKey: body.api_key,
		model: body.model,
	});
}

export async function testAiSetting(id: number) {
	return aiClient.testSettings({id: BigInt(id)});
}
