/** Shared minimalist log-line styling — one tone scale for every log surface. */

const ERROR_RE =
	/\b(error|errors|failed|failure|fatal|panic|exception)\b/i;
const WARN_RE = /\b(warn|warning|deprecated)\b/i;
const OK_RE = /(^|\s)(✔|✓|done|completed|success|is healthy|built in)/i;
/** Unique tone per deploy-pipeline stage tag, in lifecycle order. */
const STAGE_TONES: Array<[RegExp, string]> = [
	[
		/\[(?:QUEUED|PENDING|QUEUE)\]/,
		'font-medium text-slate-500 dark:text-slate-400',
	],
	[
		/\[(?:PREPARING|STARTING)\]/,
		'font-medium text-violet-600 dark:text-violet-400',
	],
	[/\[SOURCE_READY\]/, 'font-medium text-indigo-600 dark:text-indigo-400'],
	[/\[BUILDING\]/, 'font-medium text-amber-600 dark:text-amber-500'],
	[/\[IMAGE_READY\]/, 'font-medium text-teal-600 dark:text-teal-400'],
	[/\[DEPLOYING\]/, 'font-medium text-sky-600 dark:text-sky-400'],
	[/\[ROUTING\]/, 'font-medium text-fuchsia-600 dark:text-fuchsia-400'],
	[/\[HEALTH_CHECK\]/, 'font-medium text-orange-600 dark:text-orange-400'],
	[
		/\[(?:DEPLOYED|DONE|SUCCESS|COMPLETED|HEALTHY|RUNNING)\]/,
		'font-medium text-emerald-500 dark:text-emerald-400',
	],
];
/** npm-style command echo: `> app@1.0.0 build` */
const CMD_RE = /^\s*>/;
/** Asset lines: dist/index.html, foo-Ba1x.js */
const FILE_RE = /\S+\.(?:js|css|html|jpg|jpeg|png|svg|woff2?|ts|map)\b/;
const ANSI_RE = /\u001B\[[0-?]*[ -/]*[@-~]/g;
/** Leading clock timestamp — bare or bracketed (15:27:25.154 or [2026-09-22T15:27:25Z]). */
const TS_RE =
	/^(\s*\[?(?:\d{4}-\d{2}-\d{2}[T ])?\d{2}:\d{2}:\d{2}(?:[.,]\d+)?Z?\]?\s+)/;

export const stripAnsi = (text: string) => text.replace(ANSI_RE, '');

/** Theme-aware line tones: readable on the light surface AND the dark one. */
export function lineTone(line: string) {
	if (ERROR_RE.test(line)) return 'text-red-600 dark:text-red-400';
	if (WARN_RE.test(line)) return 'text-amber-600 dark:text-amber-500';
	for (const [re, tone] of STAGE_TONES) if (re.test(line)) return tone;
	if (OK_RE.test(line)) return 'text-emerald-500 dark:text-emerald-400';
	if (CMD_RE.test(line))
		return 'font-semibold text-neutral-900 dark:text-white';
	if (FILE_RE.test(line)) return 'text-emerald-600 dark:text-emerald-400';
	return 'text-neutral-800 dark:text-neutral-200';
}

/** Shared scrollable log surface — light by default, black in dark mode.
 *  Scrollbar follows the app theme (same tokens as the global rules). */
export const LOG_SURFACE_CLS =
	'overflow-y-auto bg-neutral-50 px-4 py-2.5 dark:bg-black [scrollbar-color:var(--border)_transparent] [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-[var(--border)] [&::-webkit-scrollbar-thumb]:hover:bg-[var(--muted-foreground)] [&::-webkit-scrollbar-track]:bg-transparent';

/** Dim leading timestamp — soft grey, recessed behind the log text. */
export const LOG_TS_CLS =
	'text-neutral-400/90 select-none dark:text-neutral-600';

/** Split a leading timestamp so it can render dim, like the reference. */
export function splitTimestamp(line: string): [string, string] {
	const m = line.match(TS_RE);
	return m ? [m[1], line.slice(m[1].length)] : ['', line];
}
