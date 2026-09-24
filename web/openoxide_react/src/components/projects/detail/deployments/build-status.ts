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

/** True when a deployment event finished successfully. */
export const isBuildSucceeded = (e: any): boolean => {
	if (!e) return false;
	const s = ((e.status || e.state || '') as string).toUpperCase();
	return (
		s.includes('DONE') ||
		s.includes('HEALTHY') ||
		s.includes('SUCCESS') ||
		s.includes('DEPLOYED')
	);
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

/** Minimal status colors: a dot + matching text tone. */
export const getStatusTone = (
	e: any,
): {dot: string; text: string; pulse: boolean} => {
	const s = ((e.status || e.state || '') as string).toUpperCase();
	if (s === 'CANCELLING')
		return {dot: 'bg-amber-500', text: 'text-amber-500', pulse: true};
	if (s === 'CANCELLED' || s === 'STOPPEDBYUSER')
		return {
			dot: 'bg-zinc-500',
			text: 'text-muted-foreground',
			pulse: false,
		};
	if (
		s.includes('DONE') ||
		s.includes('HEALTHY') ||
		s.includes('SUCCESS') ||
		s.includes('DEPLOYED')
	)
		return {
			dot: 'bg-emerald-500',
			text: 'text-emerald-500',
			pulse: false,
		};
	if (s.includes('ERROR') || s.includes('FAIL') || s.includes('CRASH'))
		return {dot: 'bg-rose-500', text: 'text-rose-500', pulse: false};
	if (isBuildActive(e))
		return {dot: 'bg-amber-500', text: 'text-amber-500', pulse: true};
	return {
		dot: 'bg-neutral-500',
		text: 'text-muted-foreground',
		pulse: false,
	};
};

/** Relative "2h ago" style time for list rows. */
export const formatRelative = (raw: any): string => {
	const num = Number(raw);
	if (!num || isNaN(num)) return 'N/A';
	const ms = num < 1e11 ? num * 1000 : num;
	const secs = Math.max(0, (Date.now() - ms) / 1000);
	if (secs < 60) return 'just now';
	if (secs < 3600) return `${Math.floor(secs / 60)}m ago`;
	if (secs < 86400) return `${Math.floor(secs / 3600)}h ago`;
	if (secs < 86400 * 7) return `${Math.floor(secs / 86400)}d ago`;
	return new Date(ms).toLocaleDateString();
};

/** Render an epoch (seconds or millis) or raw string as a locale date. */
export const formatTimestamp = (raw: any): string => {
	if (!raw) return 'N/A';
	const num = Number(raw);
	if (isNaN(num)) return String(raw);
	const ms = num < 1e11 ? num * 1000 : num;
	return new Date(ms).toLocaleDateString();
};
