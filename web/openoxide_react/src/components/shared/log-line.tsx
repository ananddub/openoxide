import React from 'react';

export type LogLevel = 'ALL' | 'ERROR' | 'WARN' | 'SUCCESS' | 'DEBUG' | 'INFO';

export const getLogLevel = (line: string): LogLevel => {
	if (!line) return 'INFO';
	const restLower = line.toLowerCase();
	if (restLower.includes('error') || restLower.includes('failed') || restLower.includes('err!') || restLower.includes('❌') || restLower.includes('exception')) {
		return 'ERROR';
	}
	if (restLower.includes('warn') || restLower.includes('warning')) {
		return 'WARN';
	}
	if (restLower.includes('success') || restLower.includes('deployed') || restLower.includes('built') || restLower.includes('done 0.')) {
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
	const tracingMatch = cleaned.match(/.*(?:INFO|ERROR|WARN|DEBUG|TRACE)\s+openoxide::[^\:]+:\s*command\s+output[^\n]*?line=(.*)/i);
	if (tracingMatch && tracingMatch[1] !== undefined) {
		cleaned = tracingMatch[1].trim();
		if ((cleaned.startsWith('"') && cleaned.endsWith('"')) || (cleaned.startsWith("'") && cleaned.endsWith("'"))) {
			cleaned = cleaned.slice(1, -1);
		}
	}

	// Strip "INFO:" or "ERROR:" or "DEBUG:" prefix at start of line
	cleaned = cleaned.replace(/^(INFO|ERROR|WARN|DEBUG|TRACE):\s*/i, '');

	return cleaned;
};

interface LogLineProps {
	line: string;
	index?: number;
	stage?: string;
}

export const LogLine = React.memo(({line, index, stage}: LogLineProps) => {
	if (!line) return <div className="h-4" />;

	// Apply cleanLogLine to strip Rust tracing headers (INFO/ERROR openoxide::... line=)
	const sanitized = cleanLogLine(line);
	if (!sanitized) return <div className="h-4" />;

	// Parse ISO Timestamp at start e.g. [2026-07-24T21:36:01Z] or 2026-07-24T21:36:01Z
	let timestampStr = '';
	let restStr = sanitized;

	const tsMatch = sanitized.match(/^(\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?\]|\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?)\s*(.*)/);
	if (tsMatch) {
		timestampStr = tsMatch[1].replace(/^\[|\]$/g, '');
		restStr = tsMatch[2];
	}

	// Check if line itself defines a stage marker or use passed stage prop
	let badgeTag = stage || '';
	let remainingText = restStr;

	// Check for bracket stage e.g. [DEPLOYING]
	const bracketMatch = restStr.match(/^\[(QUEUED|PREPARING|SOURCE_READY|BUILDING|DEPLOYING|ROUTING|HEALTH_CHECK|DEPLOYED|SUCCESS|ERROR|FAILED|CANCELLED)\]\s*(.*)/i);
	if (bracketMatch) {
		badgeTag = bracketMatch[1].toUpperCase();
		remainingText = bracketMatch[2] || bracketMatch[1];
	} else {
		// Check for standalone stage line e.g. "DEPLOYING" or "QUEUED"
		const standaloneMatch = restStr.match(/^(QUEUED|PREPARING|SOURCE_READY|BUILDING|DEPLOYING|ROUTING|HEALTH_CHECK|DEPLOYED)\s*$/i);
		if (standaloneMatch) {
			badgeTag = standaloneMatch[1].toUpperCase();
		}
	}

	let badgeStyle = '';
	if (badgeTag) {
		const tagUpper = badgeTag.toUpperCase();
		switch (tagUpper) {
			case 'QUEUED':
				badgeStyle = 'bg-amber-500/20 text-amber-300 border-amber-500/40 font-bold';
				break;
			case 'PREPARING':
				badgeStyle = 'bg-sky-500/20 text-sky-300 border-sky-500/40 font-bold';
				break;
			case 'SOURCE_READY':
				badgeStyle = 'bg-purple-500/20 text-purple-300 border-purple-500/40 font-bold';
				break;
			case 'BUILDING':
			case 'DEPLOYING':
				badgeStyle = 'bg-blue-500/20 text-blue-300 border-blue-500/40 font-bold';
				break;
			case 'ROUTING':
				badgeStyle = 'bg-indigo-500/20 text-indigo-300 border-indigo-500/40 font-bold';
				break;
			case 'HEALTH_CHECK':
				badgeStyle = 'bg-teal-500/20 text-teal-300 border-teal-500/40 font-bold';
				break;
			case 'DEPLOYED':
			case 'SUCCESS':
				badgeStyle = 'bg-emerald-500/25 text-emerald-300 border-emerald-500/50 font-bold';
				break;
			case 'ERROR':
			case 'FAILED':
			case 'CANCELLED':
				badgeStyle = 'bg-rose-500/25 text-rose-300 border-rose-500/50 font-bold';
				break;
			default:
				badgeStyle = 'bg-zinc-800 text-zinc-300 border-zinc-700 font-semibold';
				break;
		}
	}

	// Syntax highlighting for log content
	const restLower = remainingText.toLowerCase();
	let textColor = 'text-zinc-200';
	let lineBg = 'hover:bg-zinc-900/60';

	if (restLower.includes('error') || restLower.includes('failed') || restLower.includes('err!') || restLower.includes('❌') || restLower.includes('exception')) {
		textColor = 'text-rose-400 font-semibold';
		lineBg = 'bg-rose-950/20 hover:bg-rose-950/30 border-l-2 border-rose-500 pl-1.5';
	} else if (restLower.includes('warn') || restLower.includes('warning')) {
		textColor = 'text-amber-300';
	} else if (restLower.includes('success') || restLower.includes('deployed') || restLower.includes('built') || restLower.includes('done 0.')) {
		textColor = 'text-emerald-400 font-medium';
	} else if (restLower.startsWith('#')) {
		textColor = 'text-zinc-400';
	}

	return (
		<div className={`flex items-start gap-2.5 font-mono text-[12px] leading-relaxed whitespace-pre-wrap break-all py-0.5 px-2 rounded transition-colors ${lineBg}`}>
			{/* Line Number */}
			{index !== undefined && (
				<span className="text-[10px] text-zinc-600 select-none shrink-0 w-7 text-right font-mono font-normal">
					{index + 1}
				</span>
			)}

			{/* ISO Timestamp if present */}
			{timestampStr && (
				<span className="text-[10px] text-zinc-500 select-none shrink-0 font-mono font-normal">
					{timestampStr.length > 8 ? timestampStr.split('T')[1]?.replace('Z', '') || timestampStr : timestampStr}
				</span>
			)}

			{/* Stage Badge Pill on Left side of Log Line (e.g. [DEPLOYING], [ROUTING], [PREPARING]) */}
			{badgeTag && (
				<span className={`inline-flex items-center px-1.5 py-0.1 rounded text-[9px] tracking-wider uppercase border shrink-0 font-mono ${badgeStyle}`}>
					{badgeTag}
				</span>
			)}

			{/* Clean Log Line Text */}
			<span className={`flex-1 ${textColor}`}>
				{remainingText}
			</span>
		</div>
	);
});
