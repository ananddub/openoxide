import {useState} from 'react';
import {
	Download,
	Loader2,
	Play,
	Plus,
	RotateCw,
	Square,
	Trash2,
	UploadCloud,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {formatApiError} from '#/api/utils';
import {downloadTextFile} from '#/utils/download-text-file';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {
	useAgentStatus,
	useCreateMaintenanceWindow,
	useDeleteMaintenanceWindow,
	useExportServerMetrics,
	useMaintenanceWindows,
	usePrometheusMetrics,
	useReinstallAgent,
	useRestartAgent,
	useStartAgent,
	useStopAgent,
	type MaintenanceWindowPayload,
} from '#/api/live/monitoring';

import {
	DockerEmptyState,
	DockerTable,
	type DockerColumn,
} from '../docker/docker-shell';
import {Panel, PanelPill} from '../server-panel';
import {AgentTokenPanel} from './agent-token-panel';
import {
	AGENT_STATE_LABELS,
	defaultMaintenanceWindowDraft,
	fromLocalInputValue,
	formatLastSeen,
	maintenanceWindowProblem,
	maintenanceWindowState,
	type MaintenanceWindowDraft,
} from './maintenance-window';

/** Every agent lifecycle verb answers with the same payload, so one handler serves all four buttons. */
type AgentMutation = ReturnType<typeof useStartAgent>;

/** Epoch seconds → a short local stamp; the window table shows a start and an end per row. */
function formatWindowTime(epochSeconds: number): string {
	return new Date(epochSeconds * 1000).toLocaleString();
}

/** Agent tab: the monitoring agent's own health and controls, the metric exports the panel can produce, and the alert-suppression windows. */
export function ServerAgentPanel({serverId}: {serverId: bigint}) {
	const {can} = usePermissions();
	const canAct = can('server:create');
	const status = useAgentStatus(serverId);
	const start = useStartAgent();
	const stop = useStopAgent();
	const restart = useRestartAgent();
	const reinstall = useReinstallAgent();

	const runAction = (mutation: AgentMutation) => {
		mutation.mutate(serverId, {
			onSuccess: result =>
				toast[result.success ? 'success' : 'error'](result.message),
			onError: error => toast.error(formatApiError(error)),
		});
	};

	const state = status.data?.state ?? 'unregistered';
	const online = state === 'online';
	const busy =
		start.isPending ||
		stop.isPending ||
		restart.isPending ||
		reinstall.isPending;

	const lifecycle: {
		verb: string;
		label: string;
		busyLabel: string;
		mutation: AgentMutation;
		icon: typeof Play;
		disabled: boolean;
	}[] = [
		{
			verb: 'start',
			label: 'Start',
			busyLabel: 'Starting…',
			mutation: start,
			icon: Play,
			disabled: online,
		},
		{
			verb: 'stop',
			label: 'Stop',
			busyLabel: 'Stopping…',
			mutation: stop,
			icon: Square,
			disabled: !online,
		},
		{
			verb: 'restart',
			label: 'Restart',
			busyLabel: 'Restarting…',
			mutation: restart,
			icon: RotateCw,
			disabled: !online,
		},
		{
			verb: 'reinstall',
			label: 'Reinstall',
			busyLabel: 'Reinstalling…',
			mutation: reinstall,
			icon: UploadCloud,
			disabled: false,
		},
	];

	return (
		<div className="flex flex-col gap-4">
			<Panel
				title="Monitoring Agent"
				caption={
					status.isLoading
						? 'Reading the agent registry…'
						: AGENT_STATE_LABELS[state].hint
				}
				isRefreshing={status.isFetching}
				onRefresh={() => void status.refetch()}>
				{status.isError ? (
					<p className="py-8 text-center text-xs text-rose-500">
						{formatApiError(status.error)}
					</p>
				) : (
					<>
						<div className="flex flex-col gap-4 p-4">
							<div className="flex flex-wrap items-center gap-x-5 gap-y-2">
								<PanelPill
									ok={online}
									tone="warn"
									label={
										status.isLoading
											? 'Checking…'
											: AGENT_STATE_LABELS[state].label
									}
								/>
								<span className="text-xs text-muted-foreground">
									Version{' '}
									<span className="font-mono text-foreground">
										{status.data?.agent_version || '—'}
									</span>
								</span>
								<span className="text-xs text-muted-foreground">
									Last heartbeat{' '}
									<span className="text-foreground">
										{formatLastSeen(
											status.data?.last_seen_at ?? null,
											Math.floor(Date.now() / 1000),
										)}
									</span>
								</span>
							</div>
							<div className="flex flex-wrap gap-2">
								{lifecycle.map(item => {
									const Icon = item.icon;
									const pending = item.mutation.isPending;
									return (
										<Button
											key={item.verb}
											size="sm"
											variant="outline"
											disabled={!canAct || item.disabled || busy}
											onClick={() => runAction(item.mutation)}
											className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
											{pending ? (
												<Loader2 className="size-3.5 animate-spin" />
											) : (
												<Icon className="size-3.5" />
											)}
											{pending ? item.busyLabel : item.label}
										</Button>
									);
								})}
							</div>
							<p className="text-[11px] leading-snug text-muted-foreground">
								Start, stop and restart act on the agent container
								out-of-band — over the server&apos;s Docker socket locally,
								over SSH remotely — so they keep working while the agent
								itself is down. Reinstall pulls the current agent image and
								recreates the container.
							</p>
						</div>
						<AgentTokenPanel canAct={canAct} serverId={serverId} />
					</>
				)}
			</Panel>

			<MetricsExportPanel serverId={serverId} canAct={canAct} />
			<MaintenanceWindowsPanel serverId={serverId} canAct={canAct} />
		</div>
	);
}

interface MetricsExportPanelProps {
	serverId: bigint;
	canAct: boolean;
}

/** Metric dumps on demand: a stored-history CSV and the newest sample in Prometheus text. Nothing here polls — each click is one unary call. */
function MetricsExportPanel({serverId, canAct}: MetricsExportPanelProps) {
	const exportCsv = useExportServerMetrics();
	const exportProm = usePrometheusMetrics();
	const [prometheus, setPrometheus] = useState<string | null>(null);

	const downloadCsv = () => {
		exportCsv.mutate(serverId, {
			onSuccess: csv => {
				if (
					downloadTextFile(
						`server-${serverId}-metrics.csv`,
						csv,
						'text/csv;charset=utf-8',
					)
				) {
					toast.success('Metric history exported');
				}
			},
			onError: error => toast.error(formatApiError(error)),
		});
	};

	const loadPrometheus = () => {
		exportProm.mutate(serverId, {
			onSuccess: text => setPrometheus(text),
			onError: error => toast.error(formatApiError(error)),
		});
	};

	return (
		<Panel
			title="Metrics Export"
			caption="Pull the stored samples out for your own scraper or spreadsheet">
			<div className="flex flex-col gap-4 p-4">
				<div className="flex flex-wrap gap-2">
					<Button
						size="sm"
						variant="outline"
						disabled={!canAct || exportCsv.isPending}
						onClick={downloadCsv}
						className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
						{exportCsv.isPending ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Download className="size-3.5" />
						)}
						Download CSV
					</Button>
					<Button
						size="sm"
						variant="outline"
						disabled={!canAct || exportProm.isPending}
						onClick={loadPrometheus}
						className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
						{exportProm.isPending ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Download className="size-3.5" />
						)}
						Show Prometheus text
					</Button>
				</div>
				<p className="text-[11px] leading-snug text-muted-foreground">
					CSV carries up to the newest 1000 samples with one row per
					report. Prometheus text is the single latest sample, the shape a
					scraper would read from this server.
				</p>
				{prometheus !== null && (
					<>
						<pre className="max-h-64 scrollbar-thin scrollbar-thumb-border/40 overflow-auto rounded-lg bg-muted/40 p-4 font-mono text-[11px] leading-relaxed">
							{prometheus || 'No metrics reported yet.'}
						</pre>
						<Button
							size="sm"
							variant="outline"
							disabled={!canAct}
							onClick={() =>
								downloadTextFile(
									`server-${serverId}-metrics.prom`,
									prometheus,
								)
							}
							className="h-8 w-fit cursor-pointer gap-1.5 text-xs">
							<Download className="size-3.5" />
							Save Prometheus text
						</Button>
					</>
				)}
			</div>
		</Panel>
	);
}

interface MaintenanceWindowsPanelProps {
	serverId: bigint;
	canAct: boolean;
}

/** Alert-suppression windows for this server, including the organization-wide ones that also cover it. */
function MaintenanceWindowsPanel({
	canAct,
	serverId,
}: MaintenanceWindowsPanelProps) {
	const windows = useMaintenanceWindows(serverId);
	const create = useCreateMaintenanceWindow();
	const remove = useDeleteMaintenanceWindow();
	const [draft, setDraft] = useState<MaintenanceWindowDraft>(
		defaultMaintenanceWindowDraft,
	);
	const nowSeconds = Math.floor(Date.now() / 1000);
	const problem = maintenanceWindowProblem(draft);

	const patch = (changes: Partial<MaintenanceWindowDraft>) =>
		setDraft(prev => ({...prev, ...changes}));

	const add = () => {
		const startsAt = fromLocalInputValue(draft.startsAt);
		const endsAt = fromLocalInputValue(draft.endsAt);
		if (startsAt === null || endsAt === null || problem !== null) return;
		create.mutate(
			{serverId, startsAt, endsAt, reason: draft.reason.trim()},
			{
				onSuccess: () => {
					toast.success('Maintenance window scheduled');
					setDraft(defaultMaintenanceWindowDraft());
				},
				onError: error => toast.error(formatApiError(error)),
			},
		);
	};

	const drop = (window: MaintenanceWindowPayload) => {
		remove.mutate(BigInt(window.id), {
			onSuccess: () => toast.success('Maintenance window removed'),
			onError: error => toast.error(formatApiError(error)),
		});
	};

	const columns: DockerColumn<MaintenanceWindowPayload>[] = [
		{
			label: 'Status',
			cell: window => {
				const windowState = maintenanceWindowState(window, nowSeconds);
				return (
					<PanelPill
						ok={windowState === 'active'}
						tone={
							windowState === 'scheduled'
								? 'warn'
								: windowState === 'ended'
									? 'muted'
									: 'default'
						}
						label={windowState}
					/>
				);
			},
		},
		{
			label: 'Window',
			cell: window => (
				<span className="text-muted-foreground">
					{formatWindowTime(window.starts_at)} →{' '}
					{formatWindowTime(window.ends_at)}
				</span>
			),
		},
		{
			label: 'Reason',
			cell: window => (
				<span className="text-foreground">{window.reason || '—'}</span>
			),
		},
		{
			label: 'Scope',
			cell: window => (
				<span className="text-muted-foreground">
					{window.server_id === null ? 'All servers' : 'This server'}
				</span>
			),
		},
		{
			label: '',
			align: 'right',
			cell: window => (
				<Button
					variant="ghost"
					size="sm"
					disabled={!canAct || remove.isPending}
					onClick={() => drop(window)}
					title="Remove window"
					className="size-7 cursor-pointer p-0 text-rose-500 disabled:cursor-default">
					<Trash2 className="size-3.5" />
				</Button>
			),
		},
	];

	return (
		<Panel
			title="Maintenance Windows"
			caption={`${windows.data?.length ?? 0} window(s) — alerting stays quiet while one covers now`}
			isRefreshing={windows.isFetching}
			onRefresh={() => void windows.refetch()}>
			{windows.isError ? (
				<p className="py-8 text-center text-xs text-rose-500">
					{formatApiError(windows.error)}
				</p>
			) : windows.isLoading ? (
				<div className="space-y-2 p-4">
					<Skeleton className="h-8 w-full" />
					<Skeleton className="h-8 w-full" />
				</div>
			) : (windows.data?.length ?? 0) === 0 ? (
				<DockerEmptyState label="No maintenance windows for this server." />
			) : (
				<DockerTable
					columns={columns}
					rows={windows.data ?? []}
					keyOf={window => String(window.id)}
				/>
			)}

			<div className="flex flex-col gap-3 border-t border-border/50 p-4">
				<span className="text-xs font-semibold text-foreground">
					Schedule a window
				</span>
				<div className="flex flex-wrap items-end gap-3">
					<label className="flex flex-col gap-1.5 text-xs">
						<span className="font-semibold text-foreground">From</span>
						<Input
							type="datetime-local"
							value={draft.startsAt}
							disabled={!canAct || create.isPending}
							onChange={event => patch({startsAt: event.target.value})}
							className="h-8 w-52 text-xs"
						/>
					</label>
					<label className="flex flex-col gap-1.5 text-xs">
						<span className="font-semibold text-foreground">Until</span>
						<Input
							type="datetime-local"
							value={draft.endsAt}
							disabled={!canAct || create.isPending}
							onChange={event => patch({endsAt: event.target.value})}
							className="h-8 w-52 text-xs"
						/>
					</label>
					<label className="flex min-w-48 flex-1 flex-col gap-1.5 text-xs">
						<span className="font-semibold text-foreground">Reason</span>
						<Input
							value={draft.reason}
							maxLength={500}
							placeholder="What is being done, for the on-call reader."
							disabled={!canAct || create.isPending}
							onChange={event => patch({reason: event.target.value})}
							className="h-8 text-xs"
						/>
					</label>
					<Button
						size="sm"
						disabled={
							!canAct ||
							create.isPending ||
							windows.isError ||
							problem !== null
						}
						onClick={add}
						title={problem ?? undefined}
						className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
						{create.isPending ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Plus className="size-3.5" />
						)}
						Add window
					</Button>
				</div>
				<p className="text-[11px] leading-snug text-muted-foreground">
					Times are read in your browser&apos;s zone. A window mutes agent
					alerting for this server; it does not stop deployments — use
					maintenance mode on the Actions tab for that.
				</p>
			</div>
		</Panel>
	);
}
