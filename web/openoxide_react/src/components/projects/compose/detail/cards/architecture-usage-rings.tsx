import {useEffect} from 'react';
import {ArrowDown, ArrowUp, ArrowDownUp, Gauge} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	useContainerMonitoring,
	type MonitoringEntityType,
} from '#/hooks/monitoring/use-container-monitoring';

function toneFor(percent: number) {
	if (percent >= 85) return 'text-rose-500';
	if (percent >= 60) return 'text-amber-500';
	return 'text-emerald-500';
}

/** Circular docker-stats gauge: % ring with tone thresholds + label/sub line. */
export function UsageRing({
	label,
	percent,
	sub,
	ready,
}: {
	label: string;
	percent: number;
	sub: string;
	ready: boolean;
}) {
	const R = 16;
	const C = 2 * Math.PI * R;
	const pct = Math.min(100, Math.max(0, percent));
	return (
		<div className="flex items-center gap-3">
			<div className="relative size-14 shrink-0">
				<svg viewBox="0 0 44 44" className="size-14 -rotate-90">
					<circle
						cx="22"
						cy="22"
						r={R}
						fill="none"
						strokeWidth="3.5"
						className="stroke-muted-foreground/20"
					/>
					<circle
						cx="22"
						cy="22"
						r={R}
						fill="none"
						strokeWidth="3.5"
						strokeLinecap="round"
						strokeDasharray={C}
						strokeDashoffset={ready ? C * (1 - pct / 100) : C}
						className={cn(
							'stroke-current transition-[stroke-dashoffset] duration-500',
							toneFor(pct),
							!ready && 'opacity-30',
						)}
					/>
				</svg>
				<span className="absolute inset-0 flex items-center justify-center font-mono text-[11px] font-bold text-foreground">
					{ready
						? `${pct >= 10 ? Math.round(pct) : pct.toFixed(1)}%`
						: '—'}
				</span>
			</div>
			<div className="flex min-w-0 flex-col">
				<span className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
					{label}
				</span>
				<span className="truncate font-mono text-[11.5px] text-foreground">
					{sub}
				</span>
			</div>
		</div>
	);
}

interface UsageRingsProps {
	id: number;
	entityType: MonitoringEntityType;
}

/** Live docker-stats gauges for the architecture card (CPU / memory / disk rings + net I/O) as an aligned 2×2 block. Same cadence as the project service cards: refreshed every 15s while mounted. */
export function UsageRingsStrip({id, entityType}: UsageRingsProps) {
	const {metrics, hasError, triggerRefresh} = useContainerMonitoring(
		id,
		entityType,
		id > 0,
		false,
	);
	useEffect(() => {
		const timer = setInterval(triggerRefresh, 15_000);
		return () => clearInterval(timer);
	}, [triggerRefresh]);

	// Container stopped/unreachable — keep the card clean.
	if (hasError && !metrics) return null;

	const diskPercent =
		metrics?.dockerDiskPercent || metrics?.diskSpacePercent || 0;
	const diskSub =
		metrics?.dockerDiskUsage || metrics?.diskSpaceUsed || '—';

	return (
		<div className="w-full min-w-0">
			<span className="flex items-center gap-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
				<Gauge className="size-3.5" />
				Resource usage
			</span>
			<div className="mt-3.5 grid grid-cols-2 gap-x-4 gap-y-4 sm:gap-x-8">
				<UsageRing
					label="CPU"
					percent={metrics?.cpuPercent ?? 0}
					sub={metrics ? `${metrics.pids} procs` : '—'}
					ready={!!metrics}
				/>
				<UsageRing
					label="Memory"
					percent={metrics?.memPercent ?? 0}
					sub={metrics ? `${metrics.memUsage} / ${metrics.memLimit}` : '—'}
					ready={!!metrics}
				/>
				<UsageRing
					label="Disk"
					percent={diskPercent}
					sub={diskSub}
					ready={!!metrics}
				/>
				<div className="flex min-w-0 items-center gap-3">
					<div className="flex size-14 shrink-0 items-center justify-center rounded-full bg-muted/50">
						<ArrowDownUp className="size-[18px] text-muted-foreground" />
					</div>
					<div className="flex min-w-0 flex-col">
						<span className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
							Network
						</span>
						<span className="flex items-center gap-1.5 truncate font-mono text-[11px] text-foreground">
							<span className="flex items-center gap-0.5">
								<ArrowDown className="size-2.5" />
								{metrics?.netRx ?? '—'}
							</span>
							<span className="flex items-center gap-0.5">
								<ArrowUp className="size-2.5" />
								{metrics?.netTx ?? '—'}
							</span>
						</span>
					</div>
				</div>
			</div>
		</div>
	);
}
