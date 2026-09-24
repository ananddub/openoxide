import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	AiService,
	AiProvider,
	AiLogContext,
	type AiSetting,
	type AiComposeSuggestion,
	type AiGeneration,
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

// ─── AI Assistant: compose generations + log analysis ────────────────────────

/** LLM unary calls run well past the default deadline; give them minutes. */
const AI_CALL_OPTIONS = {timeoutMs: 300_000};

export type AiEnvVarDraft = {name: string; value: string};
export type AiDomainDraft = {
	host: string;
	port: number;
	service_name: string;
};
export type AiFileDraft = {file_path: string; content: string};

export type AiSuggestionPayload = {
	id: string;
	name: string;
	short_description: string;
	description: string;
	docker_compose: string;
	env_variables: AiEnvVarDraft[];
	domains: AiDomainDraft[];
	config_files: AiFileDraft[];
};

export type AiGenerationPayload = {
	id: number;
	ai_setting_id: number;
	prompt: string;
	status: string;
	compose_id: number | null;
	suggestions: AiSuggestionPayload[];
};

function toSuggestionPayload(s: AiComposeSuggestion): AiSuggestionPayload {
	return {
		id: s.id,
		name: s.name,
		short_description: s.shortDescription,
		description: s.description,
		docker_compose: s.dockerCompose,
		env_variables: s.envVariables.map(e => ({
			name: e.name,
			value: e.value,
		})),
		domains: s.domains.map(d => ({
			host: d.host,
			port: Number(d.port),
			service_name: d.serviceName,
		})),
		config_files: s.configFiles.map(f => ({
			file_path: f.filePath,
			content: f.content,
		})),
	};
}

function fromSuggestionPayload(s: AiSuggestionPayload) {
	return {
		id: s.id,
		name: s.name,
		shortDescription: s.short_description,
		description: s.description,
		dockerCompose: s.docker_compose,
		envVariables: s.env_variables.map(e => ({
			name: e.name,
			value: e.value,
		})),
		domains: s.domains.map(d => ({
			host: d.host,
			port: BigInt(d.port),
			serviceName: d.service_name,
		})),
		configFiles: s.config_files.map(f => ({
			filePath: f.file_path,
			content: f.content,
		})),
	};
}

function toGenerationPayload(g?: AiGeneration): AiGenerationPayload {
	return {
		id: Number(g?.id ?? 0),
		ai_setting_id: Number(g?.aiSettingId ?? 0),
		prompt: g?.prompt ?? '',
		status: g?.status ?? '',
		compose_id: g?.composeId == null ? null : Number(g.composeId),
		suggestions: (g?.output?.suggestions ?? []).map(toSuggestionPayload),
	};
}

/** AI provider settings that can drive generation/analysis (isEnabled only). */
export function useAiEnabledSettings() {
	const query = useAiListSettings();
	return {
		...query,
		settings: (query.data ?? []).filter(s => s.is_enabled),
	};
}

/** Run the compose-generation prompt against one enabled provider setting. */
export function useAiCreateGeneration() {
	return useConnectMutation(
		(input: {aiSettingId: number; request: string}) =>
			aiClient
				.createGeneration(
					{
						aiSettingId: BigInt(input.aiSettingId),
						request: input.request,
					},
					AI_CALL_OPTIONS,
				)
				.then(r => toGenerationPayload(r.generation)),
		{invalidate: [['ai_generations']]},
	);
}

export function useAiReviewGeneration() {
	return useConnectMutation(
		(input: {id: number; suggestions: AiSuggestionPayload[]}) =>
			aiClient
				.reviewGeneration({
					id: BigInt(input.id),
					output: {
						suggestions: input.suggestions.map(fromSuggestionPayload),
					},
				})
				.then(r => toGenerationPayload(r.generation)),
		{invalidate: [['ai_generations']]},
	);
}

export type AiDeployResult = {
	generation: AiGenerationPayload;
	compose_id: number;
	deployment_id: number | null;
};

export function useAiDeployGeneration() {
	return useConnectMutation(
		(input: {
			id: number;
			suggestionId: string;
			environmentId: number;
			serverId?: number | null;
			deployNow: boolean;
		}) =>
			aiClient
				.deployGeneration(
					{
						id: BigInt(input.id),
						suggestionId: input.suggestionId,
						environmentId: BigInt(input.environmentId),
						serverId:
							input.serverId == null ? undefined : BigInt(input.serverId),
						deployNow: input.deployNow,
					},
					AI_CALL_OPTIONS,
				)
				.then(r => ({
					generation: toGenerationPayload(r.generation),
					compose_id: Number(r.composeId),
					deployment_id:
						r.deploymentId == null ? null : Number(r.deploymentId),
				})),
		{invalidate: [['ai_generations'], ['composes'], ['services']]},
	);
}

export type AiLogAnalysisKind = 'BUILD' | 'RUNTIME';

export function useAiAnalyzeLogs() {
	return useConnectMutation(
		(input: {
			aiSettingId: number;
			context: AiLogAnalysisKind;
			logs: string;
		}) =>
			aiClient.analyzeLogs(
				{
					aiSettingId: BigInt(input.aiSettingId),
					context:
						input.context === 'BUILD'
							? AiLogContext.BUILD
							: AiLogContext.RUNTIME,
					logs: input.logs,
				},
				AI_CALL_OPTIONS,
			),
	);
}
