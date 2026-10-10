import {useEffect, useState} from 'react';
import {Cpu, MemoryStick} from 'lucide-react';
import {
	useServerMetrics,
	type ServerMetricPayload,
} from '#/api/live/monitoring';

/** 2s per sample, ~3 minutes of history in view. */
const POLL_MS = 2_000;
const MAX_POINTS = 90;

/**
 * Rolling window of agent metric samples collected client-side while the
 * panel is open — the backend's stored history is too sparse for a live
 * graph, so the panel samples the agent itself and starts drawing after the
 * first two points.
 */
export function useLiveMetrics(serverId: bigint | null) {
	const query = useServerMetrics(serverId, 1, POLL_MS);
	const [samples, setSamples] = useState<ServerMetricPayload[]>([]);

	useEffect(() => {
		const sample = query.data?.[0];
		if (!sample) return;
		setSamples(prev => {
			// Polls can return the same stored sample twice — dedupe on it.
			if (prev[prev.length - 1]?.timestamp === sample.timestamp)
				return prev;
			return [...prev, sample].slice(-MAX_POINTS);
		});
	}, [query.data]);

	return {samples, latest: samples[samples.length - 1]};
}

/** Percentage history as a hairline line + faint fill — live-sized Sparkline. */
function LiveLine({values}: {values: number[]}) {
	const height = 56;
	if (values.length < 2) {
		return (
			<div className="flex h-14 items-center justify-center text-[11px] text-muted-foreground/60">
				Collecting samples…
			</div>
		);
	}
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
			className="h-14 w-full">
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

/** One live chart cell: label + pulsing live dot + current value + graph. */
function LiveChart({
	icon: Icon,
	label,
	values,
	current,
}: {
	icon: typeof Cpu;
	label: string;
	values: number[];
	current?: string;
}) {
	return (
		<div className="flex flex-col gap-3 bg-card p-4">
			<div className="flex items-center gap-2">
				<Icon
					className="size-3.5 shrink-0 text-muted-foreground"
					strokeWidth={2}
				/>
				<span className="truncate text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
					{label}
				</span>
				<span
					className="size-1.5 shrink-0 animate-pulse rounded-full bg-emerald-500"
					title="Live"
				/>
				<span className="ml-auto shrink-0 text-sm font-semibold text-foreground tabular-nums">
					{current ?? '—'}
				</span>
			</div>
			<LiveLine values={values} />
		</div>
	);
}

/**
 * Live vitals band for the server side panel: the frontend samples the agent
 * every 2s and draws CPU / memory graphs from its own rolling window — no
 * waiting on the sparse stored history. `samples` come from the caller's
 * `useLiveMetrics`, so the rings above and these graphs share one stream.
 */
export function LiveVitals({samples}: {samples: ServerMetricPayload[]}) {
	const latest = samples[samples.length - 1];
	const cpuValues = samples.map(s => Number(s.cpu) || 0);
	const memValues = samples.map(s => Number(s.mem_used) || 0);

	return (
		<div className="grid shrink-0 grid-cols-1 gap-px overflow-hidden rounded-xl border bg-border sm:grid-cols-2">
			<LiveChart
				icon={Cpu}
				label="CPU load"
				values={cpuValues}
				current={latest ? `${Math.round(Number(latest.cpu))}%` : undefined}
			/>
			<LiveChart
				icon={MemoryStick}
				label="Memory used"
				values={memValues}
				current={
					latest
						? `${latest.mem_used_gb.toFixed(1)} / ${Math.round(latest.mem_total)} GB`
						: undefined
				}
			/>
		</div>
	);
}
