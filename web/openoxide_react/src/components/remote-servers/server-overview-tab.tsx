import {
	Activity,
	Boxes,
	CalendarDays,
	Clock,
	Cpu,
	Gauge,
	Globe,
	HardDrive,
	Layers,
	MemoryStick,
	Server,
	Shield,
	User,
	Wrench,
} from 'lucide-react';
import {Skeleton} from '#/components/ui/skeleton';
import {
	ListCard,
	ListEmpty,
	RowTile,
	SectionLabel,
} from '#/components/settings/list-card';
import {cn} from '#/api/utils';
import {describeCron} from '#/utils/cron';
import type {ServerMetricPayload} from '#/api/live/monitoring';
import {OsGlyph} from './os-icon';
import {
	ConnectionRow,
	TrendCell,
	VitalCell,
	formatAdded,
	formatGb,
	formatUptime,
	type ServerRecord,
} from './server-detail-parts';

/** "avg 34% · peak 71%" over the sample window. */
function windowNote(values: number[]): string | undefined {
	if (values.length < 2) return undefined;
	const avg = values.reduce((a, b) => a + b, 0) / values.length;
	return `avg ${Math.round(avg)}% · peak ${Math.round(Math.max(...values))}%`;
}

/** UPPER_SNAKE wire enum → "Upper snake". */
function humanEnum(value?: string): string {
	if (!value) return '-';
	return value
		.toLowerCase()
		.replace(/_/g, ' ')
		.replace(/^./, c => c.toUpperCase());
}

/** Overview tab: vitals band, CPU/memory trends, then the fact cards. */
export function ServerOverviewTab({
	isLoading,
	isError,
	samples,
	server,
	host,
	isHealthy,
}: {
	isLoading: boolean;
	isError: boolean;
	/** Newest-first, straight off the monitoring hook. */
	samples: ServerMetricPayload[];
	server: ServerRecord;
	host: string;
	isHealthy: boolean;
}) {
	const latest = samples[0] ?? null;
	// Agent hands over newest-first; trends read left → right in time order.
	const chronological = [...samples].reverse();
	const cpuSeries = chronological.map(m => m.cpu);
	const memSeries = chronological.map(m => m.mem_used);
	const memPct = latest ? Math.round(latest.mem_used) : 0;
	const diskPct = latest ? Math.round(latest.disk_used) : 0;
	const diskUsedGb = latest
		? (latest.total_disk * latest.disk_used) / 100
		: 0;
	const cleanupEnabled = Number(server.enable_docker_cleanup ?? 0) > 0;
	const logCron = (server.log_cleanup_cron ?? '').trim();

	return (
		<div className="flex flex-col gap-4">
			{/* ── Vitals + trends ─────────────────────────────────────── */}
			{isLoading ? (
				<div className="grid grid-cols-1 gap-4">
					<div className="grid grid-cols-2 gap-4 md:grid-cols-4">
						{[...Array(4)].map((_, i) => (
							<Skeleton key={i} className="h-[112px] rounded-xl" />
						))}
					</div>
					<div className="grid gap-4 md:grid-cols-2">
						{[...Array(2)].map((_, i) => (
							<Skeleton key={i} className="h-[104px] rounded-xl" />
						))}
					</div>
				</div>
			) : latest ? (
				<>
					<div className="grid grid-cols-2 gap-px overflow-hidden rounded-xl border bg-border md:grid-cols-4">
						<VitalCell
							icon={Cpu}
							label="CPU"
							value={`${Math.round(latest.cpu)}%`}
							sub={
								latest.cpu_cores
									? `${latest.cpu_cores} vCPU · ${latest.cpu_model}`
									: undefined
							}
							pct={Math.round(latest.cpu)}
						/>
						<VitalCell
							icon={MemoryStick}
							label="Memory"
							value={`${memPct}%`}
							sub={`${formatGb(latest.mem_used_gb)} of ${formatGb(latest.mem_total)}`}
							pct={memPct}
						/>
						<VitalCell
							icon={HardDrive}
							label="Disk"
							value={`${diskPct}%`}
							sub={`${formatGb(diskUsedGb)} of ${formatGb(latest.total_disk)}`}
							pct={diskPct}
						/>
						<VitalCell
							icon={Clock}
							label="Uptime"
							value={formatUptime(latest.uptime)}
							sub="since last boot"
						/>
					</div>

					<div className="grid grid-cols-1 gap-px overflow-hidden rounded-xl border bg-border md:grid-cols-2">
						<TrendCell
							icon={Activity}
							label="CPU load"
							values={cpuSeries}
							meta={windowNote(cpuSeries)}
						/>
						<TrendCell
							icon={Gauge}
							label="Memory used"
							values={memSeries}
							meta={windowNote(memSeries)}
						/>
					</div>
				</>
			) : (
				<ListEmpty
					icon={Activity}
					title="No metrics yet"
					description={
						isError
							? 'The monitoring agent on this server is unreachable. Check that the agent is installed and running (Remote Servers → Setup).'
							: 'Waiting for the first sample from the agent on this server.'
					}
				/>
			)}

			{/* ── Facts: rendered whether or not metrics have arrived ──── */}
			<div className="grid grid-cols-1 gap-4 md:grid-cols-2">
				{latest && (
					<div className="flex flex-col gap-2">
						<SectionLabel>System</SectionLabel>
						<ListCard>
							<ConnectionRow
								icon={Cpu}
								label="Processor"
								value={`${latest.cpu_model || '-'} · ${latest.cpu_cores} vCPU`}
							/>
							<ConnectionRow
								tile={
									<RowTile>
										<OsGlyph
											distro={latest.distro}
											os={latest.os}
											kernel={latest.kernel}
											className="size-4"
										/>
									</RowTile>
								}
								label="Operating system"
								value={latest.distro || latest.os || '-'}
							/>
							<ConnectionRow
								icon={Shield}
								label="Kernel"
								value={latest.kernel || '-'}
								mono
							/>
							<ConnectionRow
								icon={Boxes}
								label="Architecture"
								value={latest.arch || '-'}
								mono
							/>
						</ListCard>
					</div>
				)}

				<div className="flex flex-col gap-2">
					<SectionLabel>Connection</SectionLabel>
					<ListCard>
						<ConnectionRow
							icon={Globe}
							label="Host"
							value={
								<span className="flex items-center gap-2">
									<span
										className={`size-2 shrink-0 rounded-full ${isHealthy ? 'bg-emerald-500' : 'bg-rose-500'}`}
									/>
									<span className="font-mono text-[13px]">{host}</span>
								</span>
							}
						/>
						<ConnectionRow
							icon={Server}
							label="SSH port"
							value={server.port ?? '-'}
							mono
						/>
						<ConnectionRow
							icon={User}
							label="User"
							value={server.username || 'root'}
							mono
						/>
					</ListCard>
				</div>

				<div
					className={cn('flex flex-col gap-2', latest && 'md:col-span-2')}>
					<SectionLabel>This server</SectionLabel>
					<ListCard>
						<ConnectionRow
							icon={Layers}
							label="Server type"
							value={humanEnum(server.server_type)}
						/>
						<ConnectionRow
							icon={Activity}
							label="Agent status"
							value={
								<span className="flex items-center gap-2">
									<span
										className={`size-2 shrink-0 rounded-full ${isHealthy ? 'bg-emerald-500' : 'bg-rose-500'}`}
									/>
									{humanEnum(server.server_status || 'INACTIVE')}
								</span>
							}
						/>
						<ConnectionRow
							icon={Wrench}
							label="Docker image cleanup"
							value={cleanupEnabled ? 'Enabled' : 'Disabled'}
						/>
						<ConnectionRow
							icon={Clock}
							label="Log cleanup"
							value={describeCron(logCron) || logCron || 'Not scheduled'}
						/>
						<ConnectionRow
							icon={CalendarDays}
							label="Added"
							value={formatAdded(server.created_at) ?? '-'}
						/>
					</ListCard>
				</div>
			</div>
		</div>
	);
}
