import React from 'react';
import {
	STAGE_BADGE_CLASSES,
	stageGroup,
	stageLabel,
} from './log-constants';

export function renderStageBadge(
	tag: string,
	key: string | number,
): React.ReactNode {
	const group = stageGroup(tag);
	const cls = STAGE_BADGE_CLASSES[group];
	return (
		<span
			key={key}
			className={`inline-flex items-center gap-1 rounded  px-1.5 py-0.5 font-mono text-[10px] leading-none font-bold tracking-wide ${cls}`}>
			{stageLabel(tag)}
		</span>
	);
}

// ─── Pretty JSON block ────────────────────────────────────────────────────────
export function renderPrettyJson(obj: unknown): React.ReactNode {
	const lines = JSON.stringify(obj, null, 2).split('\n');
	return (
		<span className="my-0.5 inline-flex w-full flex-col overflow-x-auto rounded bg-zinc-950/70 px-3 py-2 font-mono text-xs">
			{lines.map((line, i) => {
				const km = line.match(/^(\s*)("([^"]+)")\s*:\s*(.*)/);
				if (km) {
					const [, indent, , key, rest] = km;
					const val = rest.replace(/,$/, '').trim();
					const hasComma = rest.trimEnd().endsWith(',');
					return (
						<div key={i} className="leading-relaxed whitespace-pre">
							{indent}
							<span className="text-purple-300">"{key}"</span>
							<span className="text-zinc-600">: </span>
							{jsonValueNode(val)}
							{hasComma && <span className="text-zinc-600">,</span>}
						</div>
					);
				}
				return (
					<div
						key={i}
						className="leading-relaxed whitespace-pre text-zinc-600">
						{line}
					</div>
				);
			})}
		</span>
	);
}

export function jsonValueNode(val: string): React.ReactNode {
	if (val.startsWith('"'))
		return <span className="text-emerald-300">{val}</span>;
	if (val === 'true')
		return <span className="font-bold text-amber-300">true</span>;
	if (val === 'false')
		return <span className="font-bold text-amber-300">false</span>;
	if (val === 'null')
		return <span className="text-rose-400 italic">null</span>;
	if (/^-?\d+(\.\d+)?$/.test(val))
		return <span className="text-sky-300">{val}</span>;
	return <span className="text-zinc-300">{val}</span>;
}
