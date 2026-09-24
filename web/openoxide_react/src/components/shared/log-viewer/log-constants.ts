// ─── URL regex ────────────────────────────────────────────────────────────────
export const URL_REGEX = /(https?:\/\/[^\s<>'"]+)/gi;

// ─── Deployment Stage Badges ────────────────────────────────────────────────── Matches [BUILDING], [HEALTH_CHECK], [DEPLOYING], etc. — from DeployState enum
export const DEPLOY_STAGE_RE =
	/\[(QUEUED|PREPARING|GIT_CLONE|GIT_RETRY|GIT_SUCCESS|DOCKER_IMAGE_PULL|DOCKER_IMAGE_BUILD|DOCKER_BUILD_RETRY|DOCKER_BUILD_SUCCESS|DOCKER_COMPOSE_PULL|DOCKER_COMPOSE_DOWN|DOCKER_COMPOSE_UP|DOCKER_COMPOSE_RESTART|DOCKER_COMPOSE_RETRY|CONTAINER_CREATING|CONTAINER_STARTING|CONTAINER_RUNNING|CONTAINER_STOPPING|CONTAINER_STOPPED|CONTAINER_RESTARTING|BUILDING|BUILDING_RETRY|BUILD_SUCCESS|IMAGE_READY|DEPLOYING|HEALTH_CHECK|WAITING_FOR_HEALTHY|DEPLOYED|CANCELLED|STOPPED_BY_USER|ROLLING_BACK|ROLLBACK_SUCCESS|ROLLBACK_FAILED|CLEANING_UP|CLEANUP_COMPLETE|FAILED|RECOVER_AFTER_RESTART|SOURCE_READY|ROUTING)\]/gi;

// Also match unbracketed standalone stage labels at line start
export const DEPLOY_STAGE_BARE_RE =
	/^(QUEUED|PREPARING|BUILDING|DEPLOYING|HEALTH_CHECK|DEPLOYED|CANCELLED|FAILED|SOURCE_READY|ROUTING)\s*$/i;

export type StageGroup =
	| 'success'
	| 'error'
	| 'building'
	| 'health'
	| 'git'
	| 'docker'
	| 'container'
	| 'routing'
	| 'default';

export function stageGroup(tag: string): StageGroup {
	const t = tag.toUpperCase();
	if (
		/DEPLOYED|BUILD_SUCCESS|GIT_SUCCESS|ROLLBACK_SUCCESS|CLEANUP_COMPLETE/.test(
			t,
		)
	)
		return 'success';
	if (/FAILED|CANCELLED|ROLLBACK_FAILED|STOPPED_BY_USER/.test(t))
		return 'error';
	if (
		/BUILDING|BUILD|PREPARING|QUEUED|SOURCE_READY|IMAGE_READY|RECOVER/.test(
			t,
		)
	)
		return 'building';
	if (/HEALTH_CHECK|WAITING_FOR_HEALTHY/.test(t)) return 'health';
	if (/GIT/.test(t)) return 'git';
	if (/DOCKER/.test(t)) return 'docker';
	if (/CONTAINER/.test(t)) return 'container';
	if (/ROUTING|DEPLOYING/.test(t)) return 'routing';
	return 'default';
}

export const STAGE_BADGE_CLASSES: Record<StageGroup, string> = {
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
export function stageLabel(tag: string): string {
	return tag.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}

export function httpStatusBadge(code: string) {
	const n = Number(code);
	if (n >= 500) return 'bg-rose-500/20 text-rose-300 border-rose-500/30';
	if (n >= 400)
		return 'bg-amber-500/20 text-amber-300 border-amber-500/30';
	if (n >= 300) return 'bg-cyan-500/20 text-cyan-300 border-cyan-500/30';
	return 'bg-emerald-500/20 text-emerald-300 border-emerald-500/30';
}

export const METHOD_COLORS: Record<string, string> = {
	GET: 'bg-emerald-500/15 text-emerald-300 border-emerald-600/30',
	POST: 'bg-blue-500/15 text-blue-300 border-blue-600/30',
	PUT: 'bg-amber-500/15 text-amber-300 border-amber-600/30',
	PATCH: 'bg-orange-500/15 text-orange-300 border-orange-600/30',
	DELETE: 'bg-rose-500/15 text-rose-300 border-rose-600/30',
	HEAD: 'bg-purple-500/15 text-purple-300 border-purple-600/30',
	OPTIONS: 'bg-zinc-500/15 text-zinc-400 border-zinc-600/30',
	CONNECT: 'bg-zinc-500/15 text-zinc-400 border-zinc-600/30',
};

// ─── Master Token Regex ─────────────────────────────────────────────────────── Groups:  g1  Deployment stage tag   [BUILDING] [HEALTH_CHECK] etc.  g2  HTTP verbs             GET POST PUT DELETE PATCH HEAD OPTIONS  g3  HTTP status (context)  HTTP/1.1 200 | status: 404  g4  IPv4+port / localhost  g5  UUID  g6  sha256 digest  g7  double-quoted string  g8  key=value NOTE: Timestamps are handled by LogDate (from log-parser) and NOT re-highlighted here       to avoid duplicate/broken rendering in the log text body.
export const TOKEN_RE = new RegExp(
	[
		String.raw`(\[(?:QUEUED|PREPARING|GIT_CLONE|GIT_RETRY|GIT_SUCCESS|DOCKER_IMAGE_PULL|DOCKER_IMAGE_BUILD|DOCKER_BUILD_RETRY|DOCKER_BUILD_SUCCESS|DOCKER_COMPOSE_PULL|DOCKER_COMPOSE_DOWN|DOCKER_COMPOSE_UP|DOCKER_COMPOSE_RESTART|DOCKER_COMPOSE_RETRY|CONTAINER_CREATING|CONTAINER_STARTING|CONTAINER_RUNNING|CONTAINER_STOPPING|CONTAINER_STOPPED|CONTAINER_RESTARTING|BUILDING|BUILDING_RETRY|BUILD_SUCCESS|IMAGE_READY|DEPLOYING|HEALTH_CHECK|WAITING_FOR_HEALTHY|DEPLOYED|CANCELLED|STOPPED_BY_USER|ROLLING_BACK|ROLLBACK_SUCCESS|ROLLBACK_FAILED|CLEANING_UP|CLEANUP_COMPLETE|FAILED|RECOVER_AFTER_RESTART|SOURCE_READY|ROUTING)\])`,
		String.raw`\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT)\b`,
		String.raw`(?:HTTP\/[\d.]+ |(?:status|code|response)[=:\s]+)(\d{3})\b`,
		String.raw`\b((?:\d{1,3}\.){3}\d{1,3}(?::\d{1,5})?|localhost(?::\d{1,5})?)\b`,
		String.raw`\b([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\b`,
		String.raw`(sha256:[a-f0-9]{8,64})`,
		String.raw`("(?:[^"\\]|\\.)*")`,
		// g8 – key=value. The (?<!-) guard keeps CLI flags (--detach=false)
		// intact: splitting them into key/=/value spans broke the word across
		// lines ("Since --\ndetach\n=\nfalse") in the wrapped log body.
		String.raw`(?<![-\w])\b([a-zA-Z_][a-zA-Z0-9_\-.]*=(?:[^\s,;'"[\]{}()]{1,60}))`,
	].join('|'),
	'gi',
);
