import {Activity, Gauge, Server, RefreshCw, RotateCw} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';
import {Sheet, SheetContent, SheetHeader} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {SheetGroup} from '#/components/shared/field';
import {cn, serverHostLabel} from '#/api/utils';
import {
	useServerMetrics,
	useReinstallAgent,
	useRestartAgent,
} from '#/api/live/monitoring';
import {TrendCell} from './server-detail-parts';
import {UsageRing} from '#/components/projects/compose/detail/cards/architecture-usage-rings';

import type {RemoteServerResponse} from '#/types/api-helpers';

/** "avg 34% · peak 71%" over the sample window. */
function windowNote(values: number[]): string | undefined {
	if (values.length < 2) return undefined;
	const avg = values.reduce((a, b) => a + b, 0) / values.length;
	return `avg ${Math.round(avg)}% · peak ${Math.round(Math.max(...values))}%`;
}

/**
 * Server monitoring panel: live vitals + recent trends from the agent.
 * Everything else about the server stays off this panel by design.
 */
export function ServerSettingsPanel({
	isOpen,
	server,
	onClose,
}: {
	isOpen: boolean;
	server: RemoteServerResponse | null;
	onClose: () => void;
}) {
	const serverId = isOpen && server ? BigInt(server.id) : null;
	const metricsQuery = useServerMetrics(serverId);
	const reinstall = useReinstallAgent();
	const restart = useRestartAgent();
	const [busy, setBusy] = useState<'update' | 'restart' | null>(null);

	const runAgentAction = async (
		kind: 'update' | 'restart',
		id: bigint,
	) => {
		setBusy(kind);
		const mutation = kind === 'update' ? reinstall : restart;
		try {
			const result = await mutation.mutateAsync(id);
			toast[result.success ? 'success' : 'error'](result.message);
		} catch (error) {
			toast.error(
				error instanceof Error ? error.message : 'Agent action failed',
			);
		} finally {
			setBusy(null);
		}
	};

	if (!server) return null;

	const isActive =
		(server.server_status || 'INACTIVE').toUpperCase() === 'ACTIVE';
	const host = serverHostLabel(server.ip_address);
	const samples = metricsQuery.data ?? [];
	const latest = samples[0];
	// Agent hands over newest-first; trends read left → right in time order.
	const chronological = [...samples].reverse();
	const cpuSeries = chronological.map(m => Number(m.cpu) || 0);
	const memSeries = chronological.map(m => Number(m.mem_used) || 0);

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent className="sm:data-[side=right]:w-[560px]">
				<SheetHeader className="border-b">
					<div className="flex items-center gap-3">
						<span className="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
							<Server className="size-5" />
						</span>
						<div className="min-w-0 flex-1 text-start">
							<div className="flex min-w-0 items-center gap-2">
								<h2 className="truncate text-[15px] font-semibold text-foreground">
									{server.name}
								</h2>
								<span
									className={cn(
										'inline-flex shrink-0 items-center gap-1.5 rounded-full px-2 py-0.5 text-[11px] font-medium',
										isActive
											? 'bg-emerald-500/10 text-emerald-500'
											: 'bg-rose-500/10 text-rose-500',
									)}>
									<span
										className={cn(
											'size-1.5 rounded-full',
											isActive ? 'bg-emerald-500' : 'bg-rose-500',
										)}
									/>
									{isActive ? 'Online' : 'Offline'}
								</span>
							</div>
							<p className="truncate font-mono text-xs text-muted-foreground">
								{server.username || 'root'}@{host}
							</p>
						</div>
					</div>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-7 overflow-y-auto px-6 py-6">
					<SheetGroup
						title="Vitals"
						description={
							latest
								? 'Live stats from the agent.'
								: 'Agent is not reporting metrics yet.'
						}>
						<div className="grid shrink-0 grid-cols-3 justify-items-center gap-2 rounded-xl border bg-card px-4 py-5">
							<UsageRing
								label="CPU"
								percent={latest?.cpu ?? 0}
								sub={latest ? `${latest.cpu_cores} cores` : 'no data'}
								ready={!!latest}
							/>
							<UsageRing
								label="Memory"
								percent={latest?.mem_used ?? 0}
								sub={
									latest
										? `${Math.round(latest.mem_used_gb)} / ${Math.round(latest.mem_total)} GB`
										: 'no data'
								}
								ready={!!latest}
							/>
							<UsageRing
								label="Disk"
								percent={latest?.disk_used ?? 0}
								sub={
									latest
										? `${Math.round((latest.disk_used / 100) * latest.total_disk)} / ${Math.round(latest.total_disk)} GB`
										: 'no data'
								}
								ready={!!latest}
							/>
						</div>
					</SheetGroup>

					<SheetGroup
						title="Trends"
						description="Recent history reported by the agent.">
						<div className="grid shrink-0 grid-cols-1 gap-px overflow-hidden rounded-xl border bg-border">
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
					</SheetGroup>

					<SheetGroup
						title="Agent"
						description="Update pulls the latest agent image and recreates the container; restart just cycles it.">
						<div className="flex flex-wrap gap-2 rounded-xl border bg-card px-4 py-4">
							<Button
								size="sm"
								variant="default"
								disabled={busy !== null || serverId === null}
								onClick={() =>
									serverId !== null && runAgentAction('update', serverId)
								}
								className="h-8 gap-1.5 text-xs">
								<RefreshCw
									className={cn(
										'size-3.5',
										busy === 'update' && 'animate-spin',
									)}
								/>
								{busy === 'update' ? 'Updating…' : 'Update Agent'}
							</Button>
							<Button
								size="sm"
								variant="outline"
								disabled={busy !== null || serverId === null}
								onClick={() =>
									serverId !== null && runAgentAction('restart', serverId)
								}
								className="h-8 gap-1.5 text-xs">
								<RotateCw
									className={cn(
										'size-3.5',
										busy === 'restart' && 'animate-spin',
									)}
								/>
								{busy === 'restart' ? 'Restarting…' : 'Restart'}
							</Button>
						</div>
					</SheetGroup>
				</div>
			</SheetContent>
		</Sheet>
	);
}
