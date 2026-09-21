export const FINAL_STATES = [
	'DONE',
	'DEPLOYED',
	'SUCCESS',
	'FAILED',
	'ERROR',
	'CANCELLED',
	'STOPPEDBYUSER',
	'CRASHED',
];

const ACTIVE_KEYWORDS = [
	'RUNNING',
	'BUILDING',
	'PREPARING',
	'QUEUE',
	'QUEUED',
	'STARTING',
	'DEPLOYING',
	'PENDING',
	'GIT',
	'DOCKER',
];

/** True while a deployment event is still building/deploying. */
export const isBuildActive = (e: any): boolean => {
	if (!e) return false;
	if (e.finished_at && Number(e.finished_at) > 0) return false;
	const s = (e.status || '').toUpperCase();
	const st = (e.state || '').toUpperCase();
	if (FINAL_STATES.includes(s) || FINAL_STATES.includes(st)) return false;
	return ACTIVE_KEYWORDS.some(kw => s.includes(kw) || st.includes(kw));
};

/** Tailwind classes for the status pill of a deployment event. */
export const getStatusBadge = (e: any): string => {
	const s = ((e.status || e.state || '') as string).toUpperCase();
	if (s === 'CANCELLING')
		return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
	if (s === 'CANCELLED')
		return 'text-zinc-400 bg-zinc-500/10 border-zinc-500/20';
	if (
		s.includes('DONE') ||
		s.includes('HEALTHY') ||
		s.includes('SUCCESS') ||
		s.includes('DEPLOYED')
	)
		return 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20';
	if (s.includes('ERROR') || s.includes('FAIL') || s.includes('CRASH'))
		return 'text-rose-500 bg-rose-500/10 border-rose-500/20';
	if (isBuildActive(e))
		return 'text-amber-500 bg-amber-500/10 border-amber-500/30 animate-pulse';
	return 'text-muted-foreground bg-muted ';
};

/** Render an epoch (seconds or millis) or raw string as a locale date. */
export const formatTimestamp = (raw: any): string => {
	if (!raw) return 'N/A';
	const num = Number(raw);
	if (isNaN(num)) return String(raw);
	const ms = num < 1e11 ? num * 1000 : num;
	return new Date(ms).toLocaleDateString();
};
