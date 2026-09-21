import {
	Clock,
	Cpu,
	Globe,
	HardDrive,
	MemoryStick,
	Server,
	Shield,
} from 'lucide-react';
import {Skeleton} from '#/components/ui/skeleton';
import type {ServerMetricPayload} from '#/api/live/monitoring';
import {
	ConnectionRow,
	StatCard,
	formatGb,
	formatUptime,
} from './server-detail-parts';

/** Overview tab: CPU/Memory/Disk/Uptime stat cards + system facts. */
export function ServerOverviewTab({
	isLoading,
	isError,
	latest,
}: {
	isLoading: boolean;
	isError: boolean;
	latest: ServerMetricPayload | null;
}) {
	const memPct = latest ? Math.round(latest.mem_used) : null;
	const diskPct = latest ? Math.round(latest.disk_used) : null;
	const diskUsedGb = latest
		? (latest.total_disk * latest.disk_used) / 100
		: 0;

	return (
		<div className="flex flex-col gap-4">
			{/* Stat cards — CPU / Memory / Disk / Uptime */}
			<div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
				{isLoading ? (
					Array.from({length: 4}).map((_, i) => (
						<Skeleton key={i} className="h-[132px] rounded-xl" />
					))
				) : latest ? (
					<>
						<StatCard
							icon={Cpu}
							label="CPU"
							value={`${Math.round(latest.cpu)}%`}
							sub={
								latest.cpu_cores
									? `${latest.cpu_cores} cores · ${latest.cpu_model.slice(0, 24)}`
									: undefined
							}
							pct={Math.round(latest.cpu)}
						/>
						<StatCard
							icon={MemoryStick}
							label="Memory"
							value={`${memPct ?? 0}%`}
							sub={`${formatGb(latest.mem_used_gb)} of ${formatGb(latest.mem_total)}`}
							pct={memPct ?? undefined}
						/>
						<StatCard
							icon={HardDrive}
							label="Disk"
							value={`${diskPct ?? 0}%`}
							sub={`${formatGb(diskUsedGb)} of ${formatGb(latest.total_disk)}`}
							pct={diskPct ?? undefined}
						/>
						<StatCard
							icon={Clock}
							label="Uptime"
							value={formatUptime(latest.uptime)}
							sub="since last boot"
						/>
					</>
				) : (
					<div className="col-span-2 rounded-xl bg-card p-5 ring-1 ring-border/50 lg:col-span-4">
						<p className="text-sm font-medium text-foreground">
							No metrics yet
						</p>
						<p className="mt-1 text-xs text-muted-foreground">
							{isError
								? 'The monitoring agent on this server is unreachable. Check that the agent is installed and running (Remote Servers → Setup).'
								: 'Waiting for the first sample from the agent on this server.'}
						</p>
					</div>
				)}
			</div>

			{/* System facts card (only when we have a sample) */}
			{latest && (
				<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
					<div className="mb-2 flex items-center gap-2">
						<Globe
							className="size-4 text-muted-foreground"
							strokeWidth={2}
						/>
						<h2 className="text-sm font-semibold text-foreground">
							System
						</h2>
					</div>
					<ConnectionRow
						icon={Cpu}
						label="Processor"
						value={`${latest.cpu_model || '-'} · ${latest.cpu_cores} vCPU`}
					/>
					<ConnectionRow
						icon={Server}
						label="OS"
						value={latest.distro || latest.os || '-'}
					/>
					<ConnectionRow
						icon={Shield}
						label="Kernel"
						value={latest.kernel || '-'}
						mono
					/>
				</div>
			)}
		</div>
	);
}
