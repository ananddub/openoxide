export type AiSetting = {
	id: number;
	name: string;
	api_url: string;
	model: string;
	provider: string;
	is_enabled: boolean;
	has_api_key: boolean;
};

export type AiForm = {
	name: string;
	api_url: string;
	api_key: string;
	model: string;
	is_enabled: boolean;
};

export const EMPTY_AI_FORM: AiForm = {
	name: '',
	api_url: 'https://api.openai.com/v1',
	api_key: '',
	model: '',
	is_enabled: true,
};

export const AI_PRESETS = [
	['OpenAI', 'https://api.openai.com/v1'],
	['Anthropic', 'https://api.anthropic.com/v1'],
	['Google Gemini', 'https://generativelanguage.googleapis.com/v1beta'],
	['Mistral', 'https://api.mistral.ai/v1'],
	['Ollama', 'http://localhost:11434'],
	['OpenRouter', 'https://openrouter.ai/api/v1'],
] as const;
