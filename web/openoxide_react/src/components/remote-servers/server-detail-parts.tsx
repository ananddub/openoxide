import {Cpu} from 'lucide-react';
import {ListRow, RowTile} from '#/components/settings/list-card';

/** What the detail page reads off the server record (wire DTO, snake_case). */
export interface ServerRecord {
	name?: string;
	description?: string;
	ip_address?: string;
	ip?: string;
	port?: number | string;
	username?: string;
	server_status?: string;
	server_type?: string;
	enable_docker_cleanup?: number | string | boolean;
	log_cleanup_cron?: string;
	created_at?: number | string;
}

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

/** Epoch seconds | milliseconds | ISO string → "12 Mar 2026". */
export function formatAdded(value?: number | string): string | null {
	if (value === undefined || value === null || value === '') return null;
	const ms =
		typeof value === 'string'
			? Date.parse(value)
			: value < 1e11
				? value * 1000
				: value;
	if (!Number.isFinite(ms)) return null;
	return new Date(ms).toLocaleDateString(undefined, {
		day: 'numeric',
		month: 'short',
		year: 'numeric',
	});
}

/** Usage bar. Neutral foreground tone by default — amber past 70%, red past 90%. Colour tracks the data, not per-metric branding. */
function UsageBar({pct}: {pct: number}) {
	const tone =
		pct >= 90
			? 'bg-rose-500'
			: pct >= 70
				? 'bg-amber-500'
				: 'bg-foreground/60';
	return (
		<div className="h-1 overflow-hidden rounded-full bg-muted">
			<div
				className={`h-full rounded-full transition-all duration-700 ease-out ${tone}`}
				style={{width: `${Math.min(Math.max(pct, 0), 100)}%`}}
			/>
		</div>
	);
}

/** One cell of the vitals band — cells tile a single hairline card. */
export function VitalCell({
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
		<div className="flex flex-col gap-2 bg-card p-4">
			<div className="flex items-center gap-2">
				<Icon
					className="size-3.5 shrink-0 text-muted-foreground"
					strokeWidth={2}
				/>
				<span className="truncate text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
					{label}
				</span>
			</div>
			<p className="text-2xl leading-none font-semibold tracking-tight text-foreground tabular-nums">
				{value}
			</p>
			{sub && (
				<p className="truncate text-[11px] text-muted-foreground tabular-nums">
					{sub}
				</p>
			)}
			{pct != null && <UsageBar pct={pct} />}
		</div>
	);
}

/** Percentage history as a full-width hairline line + faint fill. */
export function Sparkline({values}: {values: number[]}) {
	if (values.length < 2) return null;
	const height = 28;
	const step = 100 / (values.length - 1);
	const points = values.map((v, i) => {
		const pct = Math.min(Math.max(v, 0), 100);
		return `${(i * step).toFixed(2)},${(height - (pct / 100) * height).toFixed(2)}`;
	});
	const line = points.join(' ');
	return (
		<svg
			aria-hidden
			viewBox={`0 0 100 ${height}`}
			preserveAspectRatio="none"
			className="h-7 w-full">
			<polygon
				points={`0,${height} ${line} 100,${height}`}
				className="fill-foreground/[0.06]"
			/>
			<polyline
				points={line}
				className="fill-none stroke-foreground/50"
				strokeWidth={1.5}
				vectorEffect="non-scaling-stroke"
			/>
		</svg>
	);
}

/** One cell of the trends band: label + window note + sparkline. */
export function TrendCell({
	icon: Icon,
	label,
	values,
	meta,
}: {
	icon: typeof Cpu;
	label: string;
	values: number[];
	meta?: string;
}) {
	return (
		<div className="flex flex-col gap-3 bg-card p-4">
			<div className="flex items-center justify-between gap-2">
				<span className="flex min-w-0 items-center gap-2">
					<Icon
						className="size-3.5 shrink-0 text-muted-foreground"
						strokeWidth={2}
					/>
					<span className="truncate text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
						{label}
					</span>
				</span>
				{meta && (
					<span className="shrink-0 text-[11px] text-muted-foreground tabular-nums">
						{meta}
					</span>
				)}
			</div>
			{values.length > 1 ? (
				<Sparkline values={values} />
			) : (
				<p className="text-[11px] text-muted-foreground/60">
					Need at least two samples to draw a trend.
				</p>
			)}
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
		<ListRow
			icon={
				<RowTile>
					<Icon className="size-4" strokeWidth={2} />
				</RowTile>
			}
			title={
				<span className={mono ? 'font-mono text-[13px]' : undefined}>
					{value}
				</span>
			}
			subtitle={label}
		/>
	);
}
