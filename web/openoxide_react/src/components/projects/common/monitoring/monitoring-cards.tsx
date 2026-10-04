import {
	Card,
	CardContent,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {HardDrive, MemoryStick, Network, Timer, Zap} from 'lucide-react';

// Live snapshot cards — current docker-stats values only. No history buffer,
// no time-series charts: the panel deliberately avoids keeping sample
// windows in the browser (RAM churn) or the Agent (5-min deque).

interface MonitoringCardsProps {
	metrics: {
		cpuPercent: number;
		memPercent: number;
		memUsage: string;
		memLimit: string;
		diskSpaceUsed: string;
		diskSpaceTotal: string;
		diskSpacePercent: number;
		netRx: string;
		netTx: string;
		blockRead: string;
		blockWrite: string;
		pids: string;
	};
}

function UsageBar({pct, color}: {pct: number; color: string}) {
	return (
		<div className="mt-3 h-1.5 w-full overflow-hidden rounded-full bg-muted">
			<div
				className={`h-full rounded-full transition-[width] duration-500 ${color}`}
				style={{width: `${Math.min(100, Math.max(0, pct))}%`}}
			/>
		</div>
	);
}

function MetricCard({
	icon: Icon,
	title,
	value,
	sub,
	pct,
	barColor,
}: {
	icon: React.ComponentType<{className?: string}>;
	title: string;
	value: string;
	sub?: string;
	pct?: number;
	barColor?: string;
}) {
	return (
		<Card className="border- bg-card shadow-xs">
			<CardHeader className="flex flex-row items-center justify-between pb-2">
				<CardTitle className="flex items-center gap-2 text-sm font-bold text-foreground">
					<Icon className="size-4 text-muted-foreground" />
					{title}
				</CardTitle>
				{pct !== undefined && (
					<span className="font-mono text-xs font-bold text-foreground">
						{pct.toFixed(1)}%
					</span>
				)}
			</CardHeader>
			<CardContent className="pt-2">
				<p className="font-mono text-lg font-semibold text-foreground">
					{value}
				</p>
				{sub && (
					<p className="mt-0.5 font-mono text-xs text-muted-foreground">
						{sub}
					</p>
				)}
				{pct !== undefined && (
					<UsageBar pct={pct} color={barColor ?? 'bg-primary'} />
				)}
			</CardContent>
		</Card>
	);
}

export function MonitoringCards({metrics}: MonitoringCardsProps) {
	return (
		<div className="grid grid-cols-1 gap-6 md:grid-cols-2">
			<MetricCard
				icon={Zap}
				title="CPU Usage"
				value={`${metrics?.cpuPercent?.toFixed(2) ?? '0.00'}%`}
				pct={metrics?.cpuPercent ?? 0}
				barColor="bg-blue-500"
			/>
			<MetricCard
				icon={MemoryStick}
				title="Memory Usage"
				value={metrics?.memUsage || '0 B'}
				sub={`Limit: ${metrics?.memLimit || '0 B'}`}
				pct={metrics?.memPercent ?? 0}
				barColor="bg-emerald-500"
			/>
			<MetricCard
				icon={HardDrive}
				title="Disk Space"
				value={metrics?.diskSpaceUsed || '0 GB'}
				sub={`Total: ${metrics?.diskSpaceTotal || '0 GB'}`}
				pct={metrics?.diskSpacePercent ?? 0}
				barColor="bg-purple-500"
			/>
			<MetricCard
				icon={Timer}
				title="Block I/O"
				value={metrics?.blockRead || '0 B'}
				sub={`Write: ${metrics?.blockWrite || '0 B'}`}
			/>
			<MetricCard
				icon={Network}
				title="Network I/O"
				value={metrics?.netRx || '0 B'}
				sub={`Out: ${metrics?.netTx || '0 B'} · PIDs: ${metrics?.pids || '0'}`}
			/>
		</div>
	);
}
