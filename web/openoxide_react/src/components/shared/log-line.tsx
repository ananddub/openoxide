import React from 'react';
import {cleanLogLine, stageBadgeStyle} from './log-line-utils';

export {getLogLevel, cleanLogLine, type LogLevel} from './log-line-utils';

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

	const tsMatch = sanitized.match(
		/^(\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?\]|\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?)\s*(.*)/,
	);
	if (tsMatch) {
		timestampStr = tsMatch[1].replace(/^\[|\]$/g, '');
		restStr = tsMatch[2];
	}

	// Check if line itself defines a stage marker or use passed stage prop
	let badgeTag = stage || '';
	let remainingText = restStr;

	// Check for bracket stage e.g. [DEPLOYING]
	const bracketMatch = restStr.match(
		/^\[(QUEUED|PREPARING|SOURCE_READY|BUILDING|DEPLOYING|ROUTING|HEALTH_CHECK|DEPLOYED|SUCCESS|ERROR|FAILED|CANCELLED)\]\s*(.*)/i,
	);
	if (bracketMatch) {
		badgeTag = bracketMatch[1].toUpperCase();
		remainingText = bracketMatch[2] || bracketMatch[1];
	} else {
		// Check for standalone stage line e.g. "DEPLOYING" or "QUEUED"
		const standaloneMatch = restStr.match(
			/^(QUEUED|PREPARING|SOURCE_READY|BUILDING|DEPLOYING|ROUTING|HEALTH_CHECK|DEPLOYED)\s*$/i,
		);
		if (standaloneMatch) {
			badgeTag = standaloneMatch[1].toUpperCase();
		}
	}

	let badgeStyle = '';
	if (badgeTag) {
		badgeStyle = stageBadgeStyle(badgeTag);
	}

	// Syntax highlighting for log content
	const restLower = remainingText.toLowerCase();
	let textColor = 'text-zinc-200';
	let lineBg = 'hover:bg-zinc-900/60';

	if (
		restLower.includes('error') ||
		restLower.includes('failed') ||
		restLower.includes('err!') ||
		restLower.includes('❌') ||
		restLower.includes('exception')
	) {
		textColor = 'text-rose-400 font-semibold';
		lineBg =
			'bg-rose-950/20 hover:bg-rose-950/30 border-l-2 border-rose-500 pl-1.5';
	} else if (restLower.includes('warn') || restLower.includes('warning')) {
		textColor = 'text-amber-300';
	} else if (
		restLower.includes('success') ||
		restLower.includes('deployed') ||
		restLower.includes('built') ||
		restLower.includes('done 0.')
	) {
		textColor = 'text-emerald-400 font-medium';
	} else if (restLower.startsWith('#')) {
		textColor = 'text-zinc-400';
	}

	return (
		<div
			className={`flex items-start gap-2.5 rounded px-2 py-0.5 font-mono text-[12px] leading-relaxed break-all whitespace-pre-wrap transition-colors ${lineBg}`}>
			{/* Line Number */}
			{index !== undefined && (
				<span className="w-7 shrink-0 text-right font-mono text-[10px] font-normal text-zinc-600 select-none">
					{index + 1}
				</span>
			)}

			{/* ISO Timestamp if present */}
			{timestampStr && (
				<span className="shrink-0 font-mono text-[10px] font-normal text-zinc-500 select-none">
					{timestampStr.length > 8
						? timestampStr.split('T')[1]?.replace('Z', '') || timestampStr
						: timestampStr}
				</span>
			)}

			{/* Stage Badge Pill on Left side of Log Line (e.g. [DEPLOYING], [ROUTING], [PREPARING]) */}
			{badgeTag && (
				<span
					className={`py-0.1 inline-flex shrink-0 items-center rounded  px-1.5 font-mono text-[9px] tracking-wider uppercase ${badgeStyle}`}>
					{badgeTag}
				</span>
			)}

			{/* Clean Log Line Text */}
			<span className={`flex-1 ${textColor}`}>{remainingText}</span>
		</div>
	);
});
