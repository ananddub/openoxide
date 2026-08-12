import React from 'react';

// ─── URL regex ────────────────────────────────────────────────────────────────
const URL_REGEX = /(https?:\/\/[^\s<>'"]+)/gi;

// ─── Deployment Stage Badges ──────────────────────────────────────────────────
// Matches [BUILDING], [HEALTH_CHECK], [DEPLOYING], etc. — from DeployState enum
const DEPLOY_STAGE_RE =
	/\[(QUEUED|PREPARING|GIT_CLONE|GIT_RETRY|GIT_SUCCESS|DOCKER_IMAGE_PULL|DOCKER_IMAGE_BUILD|DOCKER_BUILD_RETRY|DOCKER_BUILD_SUCCESS|DOCKER_COMPOSE_PULL|DOCKER_COMPOSE_DOWN|DOCKER_COMPOSE_UP|DOCKER_COMPOSE_RESTART|DOCKER_COMPOSE_RETRY|CONTAINER_CREATING|CONTAINER_STARTING|CONTAINER_RUNNING|CONTAINER_STOPPING|CONTAINER_STOPPED|CONTAINER_RESTARTING|BUILDING|BUILDING_RETRY|BUILD_SUCCESS|DEPLOYING|HEALTH_CHECK|WAITING_FOR_HEALTHY|DEPLOYED|CANCELLED|STOPPED_BY_USER|ROLLING_BACK|ROLLBACK_SUCCESS|ROLLBACK_FAILED|CLEANING_UP|CLEANUP_COMPLETE|FAILED|RECOVER_AFTER_RESTART|SOURCE_READY|ROUTING)\]/gi;

// Also match unbracketed standalone stage labels at line start
const DEPLOY_STAGE_BARE_RE =
	/^(QUEUED|PREPARING|BUILDING|DEPLOYING|HEALTH_CHECK|DEPLOYED|CANCELLED|FAILED|SOURCE_READY|ROUTING)\s*$/i;

type StageGroup = 'success' | 'error' | 'building' | 'health' | 'git' | 'docker' | 'container' | 'routing' | 'default';

function stageGroup(tag: string): StageGroup {
	const t = tag.toUpperCase();
	if (/DEPLOYED|BUILD_SUCCESS|GIT_SUCCESS|ROLLBACK_SUCCESS|CLEANUP_COMPLETE/.test(t)) return 'success';
	if (/FAILED|CANCELLED|ROLLBACK_FAILED|STOPPED_BY_USER/.test(t)) return 'error';
	if (/BUILDING|BUILD|PREPARING|QUEUED|SOURCE_READY|RECOVER/.test(t)) return 'building';
	if (/HEALTH_CHECK|WAITING_FOR_HEALTHY/.test(t)) return 'health';
	if (/GIT/.test(t)) return 'git';
	if (/DOCKER/.test(t)) return 'docker';
	if (/CONTAINER/.test(t)) return 'container';
	if (/ROUTING|DEPLOYING/.test(t)) return 'routing';
	return 'default';
}

const STAGE_BADGE_CLASSES: Record<StageGroup, string> = {
	success: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40',
	error: 'bg-rose-500/20 text-rose-300 border-rose-500/40',
	building: 'bg-blue-500/20 text-blue-300 border-blue-500/40',
	health: 'bg-teal-500/20 text-teal-300 border-teal-500/40',
	git: 'bg-purple-500/20 text-purple-300 border-purple-500/40',
	docker: 'bg-sky-500/20 text-sky-300 border-sky-500/40',
	container: 'bg-indigo-500/20 text-indigo-300 border-indigo-500/40',
	routing: 'bg-violet-500/20 text-violet-300 border-violet-500/40',
	default: 'bg-zinc-700/40 text-zinc-300 border-zinc-600/40',
};

// Human-readable label for stage badge
function stageLabel(tag: string): string {
	return tag
		.replace(/_/g, ' ')
		.replace(/\b\w/g, (c) => c.toUpperCase());
}

function renderStageBadge(tag: string, key: string | number): React.ReactNode {
	const group = stageGroup(tag);
	const cls = STAGE_BADGE_CLASSES[group];
	return (
		<span
			key={key}
			className={`inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] font-mono font-bold tracking-wide border leading-none ${cls}`}
		>
			{stageLabel(tag)}
		</span>
	);
}

// ─── Public entry point ───────────────────────────────────────────────────────
export function renderRichLogText(text: string, searchQuery?: string): React.ReactNode {
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
					className="text-sky-400 underline hover:text-sky-300 font-medium cursor-pointer break-all transition-colors"
					onClick={(e) => e.stopPropagation()}
				>
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
function applySearchThenSyntax(text: string, searchQuery?: string): React.ReactNode {
	if (!text) return null;

	if (searchQuery?.trim()) {
		const q = searchQuery.trim();
		const escaped = q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
		const parts = text.split(new RegExp(`(${escaped})`, 'gi'));
		return parts.map((p, i) =>
			p.toLowerCase() === q.toLowerCase() ? (
				<mark key={i} className="bg-amber-400/40 text-amber-100 rounded px-0.5 font-bold underline">
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
			} catch { /* not JSON */ }
		}
	}

	return runTokenRegex(text);
}

// ─── Master Token Regex ───────────────────────────────────────────────────────
// Groups:
//  g1  Deployment stage tag   [BUILDING] [HEALTH_CHECK] etc.
//  g2  HTTP verbs             GET POST PUT DELETE PATCH HEAD OPTIONS
//  g3  HTTP status (context)  HTTP/1.1 200 | status: 404
//  g4  IPv4+port / localhost
//  g5  UUID
//  g6  sha256 digest
//  g7  double-quoted string
//  g8  key=value
// NOTE: Timestamps are handled by LogDate (from log-parser) and NOT re-highlighted here
//       to avoid duplicate/broken rendering in the log text body.
// ─────────────────────────────────────────────────────────────────────────────
const TOKEN_RE = new RegExp(
	[
		// g1 – deployment stage bracket tag [BUILDING] etc.
		String.raw`(\[(?:QUEUED|PREPARING|GIT_CLONE|GIT_RETRY|GIT_SUCCESS|DOCKER_IMAGE_PULL|DOCKER_IMAGE_BUILD|DOCKER_BUILD_RETRY|DOCKER_BUILD_SUCCESS|DOCKER_COMPOSE_PULL|DOCKER_COMPOSE_DOWN|DOCKER_COMPOSE_UP|DOCKER_COMPOSE_RESTART|DOCKER_COMPOSE_RETRY|CONTAINER_CREATING|CONTAINER_STARTING|CONTAINER_RUNNING|CONTAINER_STOPPING|CONTAINER_STOPPED|CONTAINER_RESTARTING|BUILDING|BUILDING_RETRY|BUILD_SUCCESS|DEPLOYING|HEALTH_CHECK|WAITING_FOR_HEALTHY|DEPLOYED|CANCELLED|STOPPED_BY_USER|ROLLING_BACK|ROLLBACK_SUCCESS|ROLLBACK_FAILED|CLEANING_UP|CLEANUP_COMPLETE|FAILED|RECOVER_AFTER_RESTART|SOURCE_READY|ROUTING)\])`,
		// g2 – HTTP verbs
		String.raw`\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT)\b`,
		// g3 – HTTP status in context only
		String.raw`(?:HTTP\/[\d.]+ |(?:status|code|response)[=:\s]+)(\d{3})\b`,
		// g4 – IPv4+port or localhost
		String.raw`\b((?:\d{1,3}\.){3}\d{1,3}(?::\d{1,5})?|localhost(?::\d{1,5})?)\b`,
		// g5 – UUID
		String.raw`\b([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\b`,
		// g6 – sha256 digest
		String.raw`(sha256:[a-f0-9]{8,64})`,
		// g7 – double-quoted string
		String.raw`("(?:[^"\\]|\\.)*")`,
		// g8 – key=value
		String.raw`\b([a-zA-Z_][a-zA-Z0-9_\-.]*=(?:[^\s,;'"[\]{}()]{1,60}))`,
	].join('|'),
	'gi',
);

function httpStatusBadge(code: string) {
	const n = Number(code);
	if (n >= 500) return 'bg-rose-500/20 text-rose-300 border-rose-500/30';
	if (n >= 400) return 'bg-amber-500/20 text-amber-300 border-amber-500/30';
	if (n >= 300) return 'bg-cyan-500/20 text-cyan-300 border-cyan-500/30';
	return 'bg-emerald-500/20 text-emerald-300 border-emerald-500/30';
}

const METHOD_COLORS: Record<string, string> = {
	GET: 'bg-emerald-500/15 text-emerald-300 border-emerald-600/30',
	POST: 'bg-blue-500/15 text-blue-300 border-blue-600/30',
	PUT: 'bg-amber-500/15 text-amber-300 border-amber-600/30',
	PATCH: 'bg-orange-500/15 text-orange-300 border-orange-600/30',
	DELETE: 'bg-rose-500/15 text-rose-300 border-rose-600/30',
	HEAD: 'bg-purple-500/15 text-purple-300 border-purple-600/30',
	OPTIONS: 'bg-zinc-500/15 text-zinc-400 border-zinc-600/30',
	CONNECT: 'bg-zinc-500/15 text-zinc-400 border-zinc-600/30',
};

function runTokenRegex(text: string): React.ReactNode {
	const parts: React.ReactNode[] = [];
	let lastIndex = 0;
	TOKEN_RE.lastIndex = 0;

	let m: RegExpExecArray | null;
	while ((m = TOKEN_RE.exec(text)) !== null) {
		if (m.index > lastIndex) parts.push(text.slice(lastIndex, m.index));

		const [full, g1, g2, g3, g4, g5, g6, g7, g8] = m;
		const key = `${m.index}`;

		if (g1) {
			// ── Deployment stage badge ──
			const tag = g1.replace(/^\[|\]$/g, '').toUpperCase();
			parts.push(renderStageBadge(tag, key));
		} else if (g2) {
			// ── HTTP verb ──
			const upper = g2.toUpperCase();
			parts.push(
				<span key={key} className={`inline-block px-1 py-0 rounded text-[10px] font-mono font-bold border leading-5 ${METHOD_COLORS[upper] ?? 'bg-zinc-500/15 text-zinc-300 border-zinc-600/30'}`}>
					{upper}
				</span>,
			);
		} else if (g3) {
			// ── HTTP status ──
			const prefixLen = full.length - g3.length;
			if (prefixLen > 0) parts.push(text.slice(m.index, m.index + prefixLen));
			parts.push(
				<span key={key + 'sc'} className={`inline-block px-1 py-0 rounded text-[10px] font-mono font-bold border leading-5 ${httpStatusBadge(g3)}`}>
					{g3}
				</span>,
			);
		} else if (g4) {
			// ── IP / localhost ──
			parts.push(
				<span key={key} className="text-cyan-300 font-mono font-medium bg-cyan-950/25 px-0.5 rounded">
					{g4}
				</span>,
			);
		} else if (g5) {
			// ── UUID ──
			parts.push(
				<span key={key} className="text-violet-400/70 font-mono text-[11px]">
					{g5}
				</span>,
			);
		} else if (g6) {
			// ── SHA hash ──
			parts.push(
				<span key={key} className="text-zinc-500 font-mono text-[10px] bg-zinc-800/50 px-0.5 rounded border border-zinc-700/40">
					{g6}
				</span>,
			);
		} else if (g7) {
			// ── Quoted string ──
			parts.push(
				<span key={key} className="text-emerald-300/90 font-mono">
					{g7}
				</span>,
			);
		} else if (g8) {
			// ── key=value ──
			const eq = g8.indexOf('=');
			const kk = g8.slice(0, eq);
			const vv = g8.slice(eq + 1);
			parts.push(
				<span key={key} className="font-mono text-xs inline-flex items-baseline">
					<span className="text-zinc-500">{kk}</span>
					<span className="text-zinc-600">=</span>
					<span className="text-amber-200/90 font-medium">{vv}</span>
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

// ─── Pretty JSON block ────────────────────────────────────────────────────────
function renderPrettyJson(obj: unknown): React.ReactNode {
	const lines = JSON.stringify(obj, null, 2).split('\n');
	return (
		<span className="inline-flex flex-col bg-zinc-950/70 border border-zinc-800/70 rounded px-3 py-2 my-0.5 font-mono text-xs w-full overflow-x-auto">
			{lines.map((line, i) => {
				const km = line.match(/^(\s*)("([^"]+)")\s*:\s*(.*)/);
				if (km) {
					const [, indent, , key, rest] = km;
					const val = rest.replace(/,$/, '').trim();
					const hasComma = rest.trimEnd().endsWith(',');
					return (
						<div key={i} className="whitespace-pre leading-relaxed">
							{indent}
							<span className="text-purple-300">"{key}"</span>
							<span className="text-zinc-600">: </span>
							{jsonValueNode(val)}
							{hasComma && <span className="text-zinc-600">,</span>}
						</div>
					);
				}
				return (
					<div key={i} className="whitespace-pre text-zinc-600 leading-relaxed">
						{line}
					</div>
				);
			})}
		</span>
	);
}

function jsonValueNode(val: string): React.ReactNode {
	if (val.startsWith('"')) return <span className="text-emerald-300">{val}</span>;
	if (val === 'true') return <span className="text-amber-300 font-bold">true</span>;
	if (val === 'false') return <span className="text-amber-300 font-bold">false</span>;
	if (val === 'null') return <span className="text-rose-400 italic">null</span>;
	if (/^-?\d+(\.\d+)?$/.test(val)) return <span className="text-sky-300">{val}</span>;
	return <span className="text-zinc-300">{val}</span>;
}

// Export for external use
export { DEPLOY_STAGE_BARE_RE, DEPLOY_STAGE_RE };
