export type LogLevel =
	| 'ALL'
	| 'ERROR'
	| 'WARN'
	| 'SUCCESS'
	| 'DEBUG'
	| 'INFO';

export const getLogLevel = (line: string): LogLevel => {
	if (!line) return 'INFO';
	const restLower = line.toLowerCase();
	if (
		restLower.includes('error') ||
		restLower.includes('failed') ||
		restLower.includes('err!') ||
		restLower.includes('❌') ||
		restLower.includes('exception')
	) {
		return 'ERROR';
	}
	if (restLower.includes('warn') || restLower.includes('warning')) {
		return 'WARN';
	}
	if (
		restLower.includes('success') ||
		restLower.includes('deployed') ||
		restLower.includes('built') ||
		restLower.includes('done 0.')
	) {
		return 'SUCCESS';
	}
	if (restLower.includes('debug') || restLower.includes('trace')) {
		return 'DEBUG';
	}
	return 'INFO';
};

// Sanitize raw Rust tracing headers e.g. "2026-07-24T21:21:30.497011Z INFO openoxide::utils::exec::exec_local: command output ... line=#12 exporting config..."
export const cleanLogLine = (line: string): string => {
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

	// Match Rust tracing pattern: "... INFO openoxide::... line=..."
	const tracingMatch = cleaned.match(
		/.*(?:INFO|ERROR|WARN|DEBUG|TRACE)\s+openoxide::[^:]+:\s*command\s+output[^\n]*?line=(.*)/i,
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

	// Strip "INFO:" or "ERROR:" or "DEBUG:" prefix at start of line
	cleaned = cleaned.replace(/^(INFO|ERROR|WARN|DEBUG|TRACE):\s*/i, '');

	return cleaned;
};

/** Tailwind classes for a deployment-stage badge pill. */
export function stageBadgeStyle(tag: string): string {
	switch (tag.toUpperCase()) {
		case 'QUEUED':
			return 'bg-amber-500/20 text-amber-300 border-amber-500/40 font-bold';
		case 'PREPARING':
			return 'bg-sky-500/20 text-sky-300 border-sky-500/40 font-bold';
		case 'SOURCE_READY':
			return 'bg-purple-500/20 text-purple-300 border-purple-500/40 font-bold';
		case 'BUILDING':
		case 'DEPLOYING':
			return 'bg-blue-500/20 text-blue-300 border-blue-500/40 font-bold';
		case 'ROUTING':
			return 'bg-indigo-500/20 text-indigo-300 border-indigo-500/40 font-bold';
		case 'HEALTH_CHECK':
			return 'bg-teal-500/20 text-teal-300 border-teal-500/40 font-bold';
		case 'DEPLOYED':
		case 'SUCCESS':
			return 'bg-emerald-500/25 text-emerald-300 border-emerald-500/50 font-bold';
		case 'ERROR':
		case 'FAILED':
		case 'CANCELLED':
			return 'bg-rose-500/25 text-rose-300 border-rose-500/50 font-bold';
		default:
			return 'bg-zinc-800 text-zinc-300 border-zinc-700 font-semibold';
	}
}
