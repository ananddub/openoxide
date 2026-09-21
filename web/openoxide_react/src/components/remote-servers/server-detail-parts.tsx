import {Cpu} from 'lucide-react';

/** Compact GB formatter — mirrors the panel's other byte readouts. */
export function formatGb(gb: number): string {
	if (!Number.isFinite(gb) || gb <= 0) return '0 GB';
	if (gb >= 1024) return `${(gb / 1024).toFixed(1)} TB`;
	return `${gb.toFixed(1)} GB`;
}

/** Seconds since boot → "241d 21h" / "3h 12m" / "7m". */
export function formatUptime(seconds: number): string {
	const s = Math.max(0, Math.floor(seconds || 0));
	const days = Math.floor(s / 86400);
	const hours = Math.floor((s % 86400) / 3600);
	const mins = Math.floor((s % 3600) / 60);
	if (days > 0) return `${days}d ${hours}h`;
	if (hours > 0) return `${hours}h ${mins}m`;
	return `${mins}m`;
}

/**
 * Usage bar. Neutral foreground tone by default — amber past 70%, red past
 * 90%. Colour tracks the data, not per-metric branding (OpenShip pattern).
 */
function UsageBar({pct}: {pct: number}) {
	const tone =
		pct >= 90
			? 'bg-rose-500'
			: pct >= 70
				? 'bg-amber-500'
				: 'bg-foreground/60';
	return (
		<div className="mt-3 h-1.5 overflow-hidden rounded-full bg-muted">
			<div
				className={`h-full rounded-full transition-all duration-700 ease-out ${tone}`}
				style={{width: `${Math.min(Math.max(pct, 0), 100)}%`}}
			/>
		</div>
	);
}

export function StatCard({
	icon: Icon,
	label,
	value,
	sub,
	pct,
}: {
	icon: typeof Cpu;
	label: string;
	value: string;
	sub?: string;
	pct?: number;
}) {
	return (
		<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
			<div className="mb-3 flex items-center gap-2">
				<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				<span className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
					{label}
				</span>
			</div>
			<p className="text-2xl font-semibold tracking-tight text-foreground tabular-nums">
				{value}
			</p>
			{sub && (
				<p className="mt-1 text-xs text-muted-foreground tabular-nums">
					{sub}
				</p>
			)}
			{pct != null && <UsageBar pct={pct} />}
		</div>
	);
}

export function ConnectionRow({
	icon: Icon,
	label,
	value,
	mono = false,
}: {
	icon: typeof Cpu;
	label: string;
	value: React.ReactNode;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center justify-between gap-3 py-1.5">
			<div className="flex min-w-0 items-center gap-2">
				<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/60">
					<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				</div>
				<span className="text-sm text-muted-foreground">{label}</span>
			</div>
			<div
				className={`min-w-0 truncate text-sm font-medium text-foreground ${mono ? 'font-mono text-[13px]' : ''}`}>
				{value}
			</div>
		</div>
	);
}
