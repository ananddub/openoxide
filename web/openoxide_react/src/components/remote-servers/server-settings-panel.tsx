import {Server, RefreshCw, RotateCw} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';
import {Sheet, SheetContent, SheetHeader} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from '#/components/ui/tabs';
import {SheetGroup} from '#/components/shared/field';
import {cn, serverHostLabel} from '#/api/utils';
import {
	useServerMetrics,
	useReinstallAgent,
	useRestartAgent,
} from '#/api/live/monitoring';
import {OsGlyph} from './os-icon';
import {LiveVitals, useLiveMetrics} from './live-vitals';
import {
	GpuSetupContent,
	SetupRunContent,
	SetupValidationContent,
} from './setup-server-content';
import {PrivateNetworkCard} from './private-network/network-card';
import {UsageRing} from '#/components/projects/compose/detail/cards/architecture-usage-rings';

import type {RemoteServerResponse} from '#/types/api-helpers';

/**
 * Server sheet opened by clicking a row: live vitals from the agent on the
 * Vitals tab, validate / re-setup / GPU configuration on the Setup tab.
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
	// Live graphs sample the agent from the panel itself — the stored
	// history is too sparse for anything drawn on a 15s cadence.
	const {samples: liveSamples, latest: liveLatest} =
		useLiveMetrics(serverId);
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
	// Rings read the live sample first, stored history until the first one lands.
	const latest = liveLatest ?? (metricsQuery.data ?? [])[0];

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent className="sm:data-[side=right]:w-[720px]">
				<SheetHeader className="border-b">
					<div className="flex items-center gap-3">
						<span className="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
							{latest ? (
								<OsGlyph
									distro={latest.distro}
									os={latest.os}
									kernel={latest.kernel}
									className="size-5"
								/>
							) : (
								<Server className="size-5" />
							)}
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

				<div className="flex min-h-0 flex-1 flex-col px-6 py-6">
					<Tabs defaultValue="vitals" className="min-h-0 flex-1 gap-4">
						<TabsList className="w-full">
							<TabsTrigger value="vitals">Vitals</TabsTrigger>
							<TabsTrigger value="network">Private Network</TabsTrigger>
							<TabsTrigger value="setup">Setup</TabsTrigger>
							<TabsTrigger value="validation">Validation</TabsTrigger>
							<TabsTrigger value="gpu">GPU</TabsTrigger>
						</TabsList>
						<TabsContent
							value="vitals"
							className="flex min-h-0 flex-1 flex-col gap-7 overflow-y-auto">
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
								title="Live"
								description="Sampled by this panel every 2s — the graph starts drawing from the first two points.">
								<LiveVitals samples={liveSamples} />
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
											serverId !== null &&
											runAgentAction('update', serverId)
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
											serverId !== null &&
											runAgentAction('restart', serverId)
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
						</TabsContent>

						{/* Only mounted once opened, so the panel does not
						    fetch a network config nobody asked for. */}
						<TabsContent
							value="network"
							className="flex min-h-0 flex-1 flex-col">
							<PrivateNetworkCard server={server} />
						</TabsContent>

						<TabsContent
							value="setup"
							className="flex min-h-0 flex-1 flex-col overflow-y-auto">
							<SetupRunContent server={server} />
						</TabsContent>

						<TabsContent
							value="validation"
							className="flex min-h-0 flex-1 flex-col overflow-y-auto">
							<SetupValidationContent server={server} />
						</TabsContent>

						<TabsContent
							value="gpu"
							className="flex min-h-0 flex-1 flex-col overflow-y-auto">
							<GpuSetupContent server={server} />
						</TabsContent>
					</Tabs>
				</div>
			</SheetContent>
		</Sheet>
	);
}
