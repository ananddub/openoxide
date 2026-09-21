import React from 'react';
import {URL_REGEX} from './log-constants';
import {renderPrettyJson} from './log-json';
import {runTokenRegex} from './log-tokens';

// ─── Public entry point ───────────────────────────────────────────────────────
export function renderRichLogText(
	text: string,
	searchQuery?: string,
): React.ReactNode {
	if (!text) return null;

	// URLs first (split so they don't get tokenized)
	const urlParts = text.split(URL_REGEX);
	URL_REGEX.lastIndex = 0;

	return urlParts.map((part, i) => {
		if (/^https?:\/\//i.test(part)) {
			return (
				<a
					key={i}
					href={part}
					target="_blank"
					rel="noopener noreferrer"
					className="cursor-pointer font-medium break-all text-sky-400 underline transition-colors hover:text-sky-300"
					onClick={e => e.stopPropagation()}>
					{part}
				</a>
			);
		}
		return (
			<React.Fragment key={i}>
				{applySearchThenSyntax(part, searchQuery)}
			</React.Fragment>
		);
	});
}

// ─── Search highlight ─────────────────────────────────────────────────────────
function applySearchThenSyntax(
	text: string,
	searchQuery?: string,
): React.ReactNode {
	if (!text) return null;

	if (searchQuery?.trim()) {
		const q = searchQuery.trim();
		const escaped = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
		const parts = text.split(new RegExp(`(${escaped})`, 'gi'));
		return parts.map((p, i) =>
			p.toLowerCase() === q.toLowerCase() ? (
				<mark
					key={i}
					className="rounded bg-amber-400/40 px-0.5 font-bold text-amber-100 underline">
					{p}
				</mark>
			) : (
				<React.Fragment key={i}>{tokenizeLine(p)}</React.Fragment>
			),
		);
	}

	return tokenizeLine(text);
}

// ─── Inline JSON detection ────────────────────────────────────────────────────
function tokenizeLine(text: string): React.ReactNode {
	if (!text) return null;

	// Inline JSON
	if (text.includes('{') || text.includes('[')) {
		const jm = text.match(/(\{[\s\S]*\}|\[[\s\S]*\])/);
		if (jm && jm[0].length > 4) {
			try {
				const obj = JSON.parse(jm[0]);
				const pre = text.slice(0, jm.index);
				const post = text.slice((jm.index ?? 0) + jm[0].length);
				return (
					<>
						{pre && tokenizeLine(pre)}
						{renderPrettyJson(obj)}
						{post && tokenizeLine(post)}
					</>
				);
			} catch {
				/* not JSON */
			}
		}
	}

	return runTokenRegex(text);
}

// Re-exported for external use / consumers of the highlighter barrel.
export {DEPLOY_STAGE_BARE_RE, DEPLOY_STAGE_RE} from './log-constants';
