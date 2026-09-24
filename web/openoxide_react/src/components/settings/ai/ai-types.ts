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
	{
		name: 'Cohere',
		url: 'https://api.cohere.ai/compatibility/v1',
		defaultModel: 'command-a-07-2025',
		description: 'Command R+ and North models',
	},
	{
		name: 'Perplexity',
		url: 'https://api.perplexity.ai',
		defaultModel: 'sonar',
		description: 'Search-grounded answers (sonar, sonar-pro)',
	},
	{
		name: 'DeepInfra',
		url: 'https://api.deepinfra.com/v1/openai',
		defaultModel: 'deepseek-ai/DeepSeek-V3',
		description: 'OpenAI-compatible hosted open models',
	},
	{
		name: 'Z.AI',
		url: 'https://api.z.ai/api/paas/v4',
		defaultModel: 'glm-4.7',
		description: 'Zhipu GLM coding and chat models',
	},
	{
		name: 'MiniMax',
		url: 'https://api.minimax.io/v1',
		defaultModel: 'MiniMax-M2.7',
		description: 'MiniMax agentic coding models',
	},
];
