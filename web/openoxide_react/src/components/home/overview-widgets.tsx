import {ArrowUpRight, ArrowDownRight} from 'lucide-react';
import {cn} from '#/api/utils';

/* ── Geist stat cell: value + label, separated by vertical hairlines ── */
export function StatCell({
	label,
	value,
	sub,
}: {
	label: string;
	value: string | number;
	sub?: string;
}) {
	return (
		<div className="flex min-w-0 flex-col gap-1 border-l border-border px-5 py-4 first:border-l-0 first:pl-0">
			<span className="truncate text-[13px] text-muted-foreground">
				{label}
			</span>
			<span className="text-2xl leading-none font-semibold tracking-[-0.02em] text-foreground">
				{value}
			</span>
			{sub && (
				<span className="truncate text-xs text-muted-foreground">
					{sub}
				</span>
			)}
		</div>
	);
}

/* Percentage bar: Geist "usage meter" — single flat fill on a track. */
export function MeterBar({
	segments,
}: {
	segments: {label: string; value: number; className: string}[];
}) {
	const total = segments.reduce((acc, s) => acc + s.value, 0) || 1;
	return (
		<div className="flex h-2 w-full gap-px overflow-hidden rounded-full bg-muted">
			{segments.map(
				s =>
					s.value > 0 && (
						<div
							key={s.label}
							title={`${s.label}: ${s.value}`}
							className={cn('h-full', s.className)}
							style={{width: `${(s.value / total) * 100}%`}}
						/>
					),
			)}
		</div>
	);
}

/* Deployment history: Git-graph style vertical bars per day (Vercel). */
export function DeployBars({
	days,
}: {
	days: {label: string; count: number}[];
}) {
	const max = Math.max(...days.map(d => d.count), 1);
	return (
		<div className="flex h-10 items-end gap-1">
			{days.map(d => (
				<div
					key={d.label}
					title={`${d.label}: ${d.count} deploy${d.count === 1 ? '' : 's'}`}
					className="min-w-2 flex-1 rounded-[2px] bg-foreground/25 transition-colors hover:bg-foreground/50"
					style={{height: `${Math.max((d.count / max) * 100, 8)}%`}}
				/>
			))}
		</div>
	);
}

export function HealthRow({
	label,
	count,
	dot,
}: {
	label: string;
	count: number;
	dot: string;
}) {
	return (
		<div className="flex items-center justify-between">
			<span className="flex items-center gap-2 text-[13px] text-muted-foreground">
				<span className={cn('size-2 rounded-full', dot)} />
				{label}
			</span>
			<span className="text-sm font-semibold text-foreground">
				{count}
			</span>
		</div>
	);
}

export function DeltaTag({delta}: {delta?: string}) {
	if (!delta) return null;
	const positive = delta.startsWith('+');
	const neutral = !positive && !delta.startsWith('-');
	const Icon = positive ? ArrowUpRight : ArrowDownRight;
	return (
		<span
			className={cn(
				'flex items-center gap-1 text-xs font-medium',
				neutral
					? 'text-muted-foreground'
					: positive
						? 'text-emerald-500'
						: 'text-red-500',
			)}>
			{!neutral && <Icon className="size-3" />}
			{delta}
		</span>
	);
}
