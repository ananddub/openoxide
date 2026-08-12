import type {LogLevel, StreamType, ParsedLogEntry} from './types';

export function getLogLevel(line: string): LogLevel {
	if (!line) return 'INFO';
	const lower = line.toLowerCase();
	// Use word-boundary patterns so JSON key names like "Error":"" don't trigger ERROR level
	if (
		/\berror\b/.test(lower) ||
		/\bfailed\b/.test(lower) ||
		lower.includes('err!') ||
		lower.includes('❌') ||
		/\bexception\b/.test(lower) ||
		lower.includes('stderr')
	) {
		// Don't flag as ERROR if the match is just a JSON key with an empty value
		// e.g. "Error": "" — check the value isn't empty
		const errorKeyEmpty = /"[Ee]rror"\s*:\s*""/.test(line);
		if (!errorKeyEmpty) return 'ERROR';
	}
	if (/\bwarn(ing)?\b/.test(lower)) {
		return 'WARN';
	}
	if (
		/\bsuccess\b/.test(lower) ||
		/\bdeployed\b/.test(lower) ||
		/\bbuilt\b/.test(lower) ||
		lower.includes('done 0.')
	) {
		return 'SUCCESS';
	}
	if (/\bdebug\b/.test(lower) || /\btrace\b/.test(lower)) {
		return 'DEBUG';
	}
	return 'INFO';
}

export function cleanLogLine(line: string): string {
	if (!line) return '';
	let cleaned = line.trim();

	if (
		cleaned.startsWith('event:') ||
		cleaned.includes('event: log') ||
		cleaned.includes('event: deployment') ||
		cleaned.includes('event: keep-alive') ||
		cleaned.startsWith('id:') ||
		cleaned === ':' ||
		cleaned.includes('keep-alive')
	) {
		return '';
	}

	const tracingMatch = cleaned.match(
		/.*(?:INFO|ERROR|WARN|DEBUG|TRACE)\s+openoxide::[^\:]+:\s*command\s+output[^\n]*?line=(.*)/i
	);
	if (tracingMatch && tracingMatch[1] !== undefined) {
		cleaned = tracingMatch[1].trim();
		if (
			(cleaned.startsWith('"') && cleaned.endsWith('"')) ||
			(cleaned.startsWith("'") && cleaned.endsWith("'"))
		) {
			cleaned = cleaned.slice(1, -1);
		}
	}

	cleaned = cleaned.replace(/^(INFO|ERROR|WARN|DEBUG|TRACE):\s*/i, '');
	return cleaned;
}

export function parseLogEntry(raw: string, index: number, stage?: string): ParsedLogEntry {
	let clean = cleanLogLine(raw);
	let isJson = false;
	let jsonObject: Record<string, unknown> | undefined;

	let containerName: string | undefined;
	let stream: StreamType = 'STDOUT';
	let timestamp: string = new Date().toISOString();
	let level: LogLevel = getLogLevel(clean || raw);

	// Check if the log line is a JSON payload
	if (clean.startsWith('{') && clean.endsWith('}')) {
		try {
			const parsed = JSON.parse(clean);
			if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
				jsonObject = parsed as Record<string, unknown>;
				isJson = true;

				// Extract container or service name from JSON fields
				const jsonContainer = jsonObject.container || jsonObject.service || jsonObject.app || jsonObject.container_name || jsonObject.name;
				if (typeof jsonContainer === 'string' && jsonContainer.trim()) {
					containerName = jsonContainer.trim();
				}

				// Extract stream (stdout / stderr) from JSON fields — exact match only
				const jsonStream = jsonObject.stream || jsonObject.std;
				if (typeof jsonStream === 'string') {
					const lowerStream = jsonStream.toLowerCase().trim();
					if (lowerStream === 'stderr') {
						stream = 'STDERR';
					} else if (lowerStream === 'stdout') {
						stream = 'STDOUT';
					}
				}

				// Extract level from JSON fields
				const jsonLevel = jsonObject.level || jsonObject.severity;
				if (typeof jsonLevel === 'string') {
					level = getLogLevel(jsonLevel);
				}

				// Extract timestamp from JSON fields
				const jsonTime = jsonObject.time || jsonObject.timestamp || jsonObject.date || jsonObject.datetime;
				if (typeof jsonTime === 'string' && jsonTime.trim()) {
					timestamp = jsonTime.replace(/^\[|\]$/g, '');
				}

				// Extract clean log line message text
				const jsonMessage = jsonObject.line || jsonObject.log || jsonObject.message || jsonObject.msg || jsonObject.text || jsonObject.content;
				if (typeof jsonMessage === 'string' && jsonMessage.trim()) {
					clean = jsonMessage;
				}
			}
		} catch {
			// Not valid JSON
		}
	}

	if (!isJson) {
		if (clean.startsWith('[STDERR]')) {
			stream = 'STDERR';
			clean = clean.replace(/^\[STDERR\]\s*/, '');
		} else {
			const lowerRaw = raw.toLowerCase();
			if (lowerRaw.includes('stderr')) {
				stream = 'STDERR';
			}
		}

		// Extract timestamp e.g. [2026-07-29T19:50:00Z] or 2026-07-29T19:50:00.123Z
		const tsMatch = clean.match(
			/^(\[\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?\]|\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?)\s*(.*)/i
		);
		if (tsMatch) {
			timestamp = tsMatch[1].replace(/^\[|\]$/g, '');
			clean = tsMatch[2];
		}

		// Extract container/service name tag e.g. [container:web] or [service:db] or container=app-1
		const containerMatch = clean.match(/^\[(?:container|service|app):\s*([^\]]+)\]\s*(.*)/i) ||
			clean.match(/^(?:container|service)=([a-zA-Z0-9_\-]+)\s*(.*)/i);
		if (containerMatch) {
			containerName = containerMatch[1];
			clean = containerMatch[2];
		}
	}

	return {
		id: `log-${index}-${Date.now()}`,
		raw,
		clean: clean || raw,
		timestamp,
		level,
		stream,
		stage,
		containerName,
		isJson,
		jsonObject,
	};
}
