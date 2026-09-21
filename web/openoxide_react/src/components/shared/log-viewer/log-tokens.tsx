import React from 'react';
import {TOKEN_RE, METHOD_COLORS, httpStatusBadge} from './log-constants';
import {renderStageBadge} from './log-json';

// ─── Master tokenizer ───────────────────────────────────────────────────────
export function runTokenRegex(text: string): React.ReactNode {
	const parts: React.ReactNode[] = [];
	let lastIndex = 0;
	TOKEN_RE.lastIndex = 0;

	let m: RegExpExecArray | null;
	while ((m = TOKEN_RE.exec(text)) !== null) {
		if (m.index > lastIndex) parts.push(text.slice(lastIndex, m.index));

		const [full, g1, g2, g3, g4, g5, g6, g7, g8] = m;
		const key = `${m.index}`;

		if (g1) {
			const tag = g1.replace(/^\[|\]$/g, '').toUpperCase();
			parts.push(renderStageBadge(tag, key));
		} else if (g2) {
			const upper = g2.toUpperCase();
			parts.push(
				<span
					key={key}
					className={`inline-block rounded  px-1 py-0 font-mono text-[10px] leading-5 font-bold ${METHOD_COLORS[upper] ?? 'border-zinc-600/30 bg-zinc-500/15 text-zinc-300'}`}>
					{upper}
				</span>,
			);
		} else if (g3) {
			const prefixLen = full.length - g3.length;
			if (prefixLen > 0)
				parts.push(text.slice(m.index, m.index + prefixLen));
			parts.push(
				<span
					key={key + 'sc'}
					className={`inline-block rounded  px-1 py-0 font-mono text-[10px] leading-5 font-bold ${httpStatusBadge(g3)}`}>
					{g3}
				</span>,
			);
		} else if (g4) {
			parts.push(
				<span
					key={key}
					className="rounded bg-cyan-950/25 px-0.5 font-mono font-medium text-cyan-300">
					{g4}
				</span>,
			);
		} else if (g5) {
			parts.push(
				<span
					key={key}
					className="font-mono text-[11px] text-violet-400/70">
					{g5}
				</span>,
			);
		} else if (g6) {
			parts.push(
				<span
					key={key}
					className="rounded bg-zinc-800/50 px-0.5 font-mono text-[10px] text-zinc-500">
					{g6}
				</span>,
			);
		} else if (g7) {
			parts.push(
				<span key={key} className="font-mono text-emerald-300/90">
					{g7}
				</span>,
			);
		} else if (g8) {
			const eq = g8.indexOf('=');
			const kk = g8.slice(0, eq);
			const vv = g8.slice(eq + 1);
			parts.push(
				<span
					key={key}
					className="inline-flex items-baseline font-mono text-xs">
					<span className="text-zinc-500">{kk}</span>
					<span className="text-zinc-600">=</span>
					<span className="font-medium text-amber-200/90">{vv}</span>
				</span>,
			);
		} else {
			parts.push(full);
		}

		lastIndex = TOKEN_RE.lastIndex;
	}

	if (lastIndex < text.length) parts.push(text.slice(lastIndex));
	return parts;
}
