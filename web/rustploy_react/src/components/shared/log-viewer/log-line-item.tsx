import React, {useMemo} from 'react';
import type {ParsedLogEntry} from './types';
import {LogActionsMenu} from './log-actions-menu';
import {LogStd} from './log-std';
import {LogDate} from './log-date';
import {LogLevelBar} from './log-level-bar';
import {ContainerTag} from './container-tag';
import {renderAnsiText} from './ansi';

interface LogLineItemProps {
	entry: ParsedLogEntry;
	index: number;
	searchQuery?: string;
	isWrapLines?: boolean;
	isCompact?: boolean;
	onShowDetails: (entry: ParsedLogEntry) => void;
}

export const LogLineItem = React.memo(({
	entry,
	index,
	searchQuery,
	isWrapLines = true,
	isCompact = false,
	onShowDetails,
}: LogLineItemProps) => {
	const isMultiLine = useMemo(() => {
		return entry.clean.includes('\n') || (isWrapLines && entry.clean.length > 120);
	}, [entry.clean, isWrapLines]);

	// Docker/Swarm sends error events (health fail, scheduling fail) via stdout.
	// Detect them and promote to stderr so the badge + color reflect the real severity.
	const effectiveStream = useMemo((): typeof entry.stream => {
		if (entry.stream === 'STDERR') return 'STDERR';
		const t = entry.clean;
		// If it looks like a JSON status object with a non-empty Error field
		if (t.includes('"Error"')) {
			try {
				const start = t.indexOf('{');
				const end = t.lastIndexOf('}');
				if (start !== -1 && end !== -1) {
					const obj = JSON.parse(t.slice(start, end + 1)) as Record<string, unknown>;
					if (typeof obj.Error === 'string' && obj.Error.trim() !== '' && obj.Error !== '""') {
						return 'STDERR';
					}
				}
			} catch {}
		}
		// Plain-text error keywords
		const lower = t.toLowerCase();
		if (
			/no suitable node|task failed|failed to|container failed|error:|panic:|fatal:|oom killed|exec format error/.test(lower)
		) {
			return 'STDERR';
		}
		return entry.stream;
	}, [entry.stream, entry.clean]);

	let textColor = 'text-foreground';
	if (entry.level === 'ERROR' || effectiveStream === 'STDERR') {
		textColor = 'text-rose-400 font-semibold';
	} else if (entry.level === 'WARN') {
		textColor = 'text-amber-300';
	}

	return (
		<div
			className={`group/entry relative flex w-full items-start gap-x-2 px-2 py-1 md:px-4 odd:bg-muted/30 even:bg-transparent hover:bg-accent/60 transition-colors rounded-xs text-[13px] leading-relaxed font-mono ${
				isWrapLines ? 'break-words' : 'overflow-x-auto'
			} ${isCompact ? 'py-0 text-[11.5px]' : ''}`}
			style={{
				fontFamily: `'JetBrains Mono', 'Fira Code', 'Cascadia Code', ui-monospace, SFMono-Regular, monospace`,
			}}
		>
			{/* 1. Dozzle LogActions (Hover Menu button ...) */}
			<LogActionsMenu entry={entry} onShowDetails={onShowDetails} />

			{/* Line Index */}
			<span className="text-[10.5px] text-muted-foreground select-none shrink-0 w-6 text-right font-mono font-normal self-start pt-0.5">
				{index + 1}
			</span>

			{/* 2. Dozzle LogStd (stdout / stderr Pill Tag) */}
			<LogStd std={effectiveStream} />

			{/* 3. Dozzle ContainerTag (if multicontainer / container tag exists) */}
			{entry.containerName && <ContainerTag name={entry.containerName} />}

			{/* 4. Dozzle LogDate (Timestamp Pill Tag in Blue text) */}
			{entry.timestamp && <LogDate timestamp={entry.timestamp} />}

			{/* 5. Dozzle LogLevel (Green/Red dot for single line, expands to vertical line for multiline) */}
			<LogLevelBar level={entry.level} isMultiLine={isMultiLine} />

			{/* 6. Dozzle LogMessage Content with ANSI color rendering */}
			<div className={`[word-break:break-word] whitespace-pre-wrap flex-1 min-w-0 ${textColor}`}>
				{renderAnsiText(entry.clean, searchQuery)}
			</div>
		</div>
	);
});
