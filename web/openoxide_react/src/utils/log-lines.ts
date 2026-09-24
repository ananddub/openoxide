/** Normalizes a log payload (SSE frame, JSON envelope or plain text) into display lines. Shared by the deployment log panels and the server-setup streams, which all receive differently-shaped payloads from the backend. */
export function extractLogLines(rawData: unknown): string[] {
	if (!rawData) return [];
	if (typeof rawData === 'string') {
		const trimmed = rawData.trim();
		if (
			!trimmed ||
			trimmed.startsWith('event:') ||
			trimmed.includes('event: log') ||
			trimmed.includes('event: deployment') ||
			trimmed.startsWith('id:') ||
			trimmed.startsWith(':') ||
			trimmed.includes('keep-alive')
		) {
			return [];
		}
		if (trimmed.startsWith('data:')) {
			const content = trimmed.slice(5).trim();
			if (!content || content.includes('keep-alive')) return [];
			try {
				const parsed = JSON.parse(content);
				return extractLogLines(parsed);
			} catch {
				return content
					.split('\n')
					.filter(l => l.trim() && !l.includes('keep-alive'));
			}
		}
		return rawData.split('\n');
	}

	if (typeof rawData === 'object' && rawData !== null) {
		const obj = rawData as Record<string, unknown>;
		if (obj.type === 'keep-alive' || obj.event === 'keep-alive') return [];
		const candidate =
			obj.line ??
			obj.message ??
			obj.data ??
			obj.log ??
			obj.text ??
			obj.content ??
			obj.output;
		if (candidate !== undefined && candidate !== null) {
			if (typeof candidate === 'object') {
				return [JSON.stringify(candidate)];
			}
			return String(candidate)
				.split('\n')
				.filter(l => l.trim() && !l.includes('keep-alive'));
		}
		return [JSON.stringify(rawData)];
	}

	return [String(rawData)];
}
