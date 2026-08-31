import {AiService} from '#/gen/connect/openoxide/api/v1/ai_pb';
import {createServiceClient, useConnectQuery} from '#/api/live/query';
import {readResource, readResourceList} from '#/api/live/resource';

export type AiSettingPayload = {
	id: number;
	provider: string;
	api_url: string;
	api_key: string;
	model: string;
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

/** All configured AI providers. */
export function useAiSettingsList() {
	return useConnectQuery(['ai_settings', 'list'], async () =>
		readResourceList<AiSettingPayload>(await aiClient.listSettings({})),
	);
}

export async function listAiSettings() {
	return readResourceList<AiSettingPayload>(await aiClient.listSettings({}));
}

export async function createAiSetting(body: Partial<AiSettingPayload>) {
	return readResource<AiSettingPayload>(
		await aiClient.createSettings({payload: JSON.stringify(body)}),
	);
}

export async function updateAiSetting(
	id: number,
	body: Partial<AiSettingPayload>,
) {
	return readResource<AiSettingPayload>(
		await aiClient.updateSettings({
			id: BigInt(id),
			payload: JSON.stringify(body),
		}),
	);
}

export async function deleteAiSetting(id: number) {
	await aiClient.deleteSettings({id: BigInt(id)});
}

export async function discoverAiModels(body: {
	api_url: string;
	api_key: string;
}) {
	return readResourceList<AiModelPayload>(
		await aiClient.discoverModels({payload: JSON.stringify(body)}),
	);
}

export async function discoverSettingModels(id: number) {
	return readResourceList<AiModelPayload>(
		await aiClient.listModels({id: BigInt(id)}),
	);
}

export async function testAiConnection(body: {
	api_url: string;
	api_key: string;
	model: string;
}) {
	return aiClient.testConnection({payload: JSON.stringify(body)});
}

export async function testAiSetting(id: number) {
	return aiClient.testSettings({id: BigInt(id)});
}
