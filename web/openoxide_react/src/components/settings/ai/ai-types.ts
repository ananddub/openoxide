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
	model: 'gpt-4o-mini',
	is_enabled: true,
};

export interface AiPreset {
	name: string;
	url: string;
	defaultModel: string;
	description: string;
}

export const AI_PRESETS: AiPreset[] = [
	{
		name: 'OpenAI',
		url: 'https://api.openai.com/v1',
		defaultModel: 'gpt-4o-mini',
		description: 'GPT-4o, GPT-4o-mini, o1 models',
	},
	{
		name: 'Anthropic',
		url: 'https://api.anthropic.com/v1',
		defaultModel: 'claude-3-5-sonnet-20241022',
		description: 'Claude 3.5 Sonnet, Claude 3.5 Haiku',
	},
	{
		name: 'Google Gemini',
		url: 'https://generativelanguage.googleapis.com/v1beta',
		defaultModel: 'gemini-1.5-flash',
		description: 'Gemini 1.5 Pro, Gemini 1.5 Flash',
	},
	{
		name: 'Mistral AI',
		url: 'https://api.mistral.ai/v1',
		defaultModel: 'mistral-large-latest',
		description: 'Mistral Large, Mistral Nemo, Codestral',
	},
	{
		name: 'Ollama (Local)',
		url: 'http://localhost:11434/v1',
		defaultModel: 'llama3.2',
		description: 'Self-hosted local models (Llama, DeepSeek, Qwen)',
	},
	{
		name: 'OpenRouter',
		url: 'https://openrouter.ai/api/v1',
		defaultModel: 'openai/gpt-4o-mini',
		description: 'Universal unified LLM gateway',
	},
];
