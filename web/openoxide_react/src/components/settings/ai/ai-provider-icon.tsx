import {Bot} from 'lucide-react';
import {
	AnthropicIcon,
	GeminiIcon,
	MistralIcon,
	OllamaIcon,
	OpenAIIcon,
	OpenRouterIcon,
} from '#/components/icons/ai-provider-icons';

/** Resolves a provider label (preset name or the `provider` field the API returns) to its brand glyph. Falls back to a neutral robot mark for custom / unknown endpoints. */
export function AiProviderIcon({
	name,
	className = 'size-4',
}: {
	name?: string;
	className?: string;
}) {
	const key = (name || '').toLowerCase();
	if (key.includes('openai')) return <OpenAIIcon className={className} />;
	if (key.includes('anthropic') || key.includes('claude'))
		return <AnthropicIcon className={className} />;
	if (key.includes('gemini') || key.includes('google'))
		return <GeminiIcon className={className} />;
	if (key.includes('mistral'))
		return <MistralIcon className={className} />;
	if (key.includes('ollama') || key.includes('llama'))
		return <OllamaIcon className={className} />;
	if (key.includes('openrouter'))
		return <OpenRouterIcon className={className} />;
	return <Bot className={className} />;
}
