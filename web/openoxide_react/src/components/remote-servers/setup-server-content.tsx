import {useState} from 'react';
import {CheckCircle2, Loader2, RefreshCw, XCircle} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {formatApiError} from '#/api/utils';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {
	useAgentStatus,
	useMonitoringContainerStates,
	useServerMetrics,
} from '#/api/live/monitoring';
import {
	useServerManagement,
	useServerManagementAction,
	type ServerActionPayload,
} from '#/api/live/server-management';
import {SetupLogsSurface} from './setup-logs-surface';
import {useServerSetup} from './use-server-setup';

import type {RemoteServerResponse} from '#/types/api-helpers';

/**
 * One check row. Pass/fail form: label left, green/red verdict with icon
 * right. Info form (`isEnabled` unset): plain muted value right, no icon —
 * for host facts that are neither good nor bad.
 */
function StatusRow({
	label,
	isEnabled,
	description,
	value,
}: {
	label: string;
	isEnabled?: boolean;
	description?: string;
	value?: string;
}) {
	if (isEnabled === undefined) {
		return (
			<div className="flex items-center justify-between gap-3">
				<span className="truncate text-sm text-foreground">{label}</span>
				<span className="max-w-[60%] truncate text-xs text-muted-foreground">
					{value || '—'}
				</span>
			</div>
		);
	}
	return (
		<div className="flex items-center justify-between gap-3">
			<span className="truncate text-sm text-foreground">{label}</span>
			<span className="flex shrink-0 items-center gap-1.5">
				<span
					className={`text-xs ${
						isEnabled ? 'text-emerald-500' : 'text-rose-500'
					}`}>
					{description || (isEnabled ? 'OK' : 'Missing')}
				</span>
				{isEnabled ? (
					<CheckCircle2 className="size-3.5 text-emerald-500" />
				) : (
					<XCircle className="size-3.5 text-rose-500" />
				)}
			</span>
		</div>
	);
}

/** A bordered group of check rows with a heading, as one status category. */
function CheckSection({
	title,
	caption,
	children,
}: {
	title: string;
	caption: string;
	children: React.ReactNode;
}) {
	return (
		<div className="rounded-lg border border-border/60 bg-muted/10 p-4">
			<h5 className="text-sm font-semibold text-foreground">{title}</h5>
			<p className="mt-0.5 mb-3 text-xs text-muted-foreground">
				{caption}
			</p>
			<div className="grid gap-2.5">{children}</div>
		</div>
	);
}

/** Shared header row for one bordered setup section. */
function SectionHeader({
	title,
	caption,
	action,
}: {
	title: string;
	caption: string;
	action?: React.ReactNode;
}) {
	return (
		<div className="flex items-start justify-between gap-3">
			<div className="min-w-0">
				<h5 className="text-sm font-semibold text-foreground">{title}</h5>
				<p className="mt-0.5 text-xs text-muted-foreground">{caption}</p>
			</div>
			{action && <div className="shrink-0">{action}</div>}
		</div>
	);
}

/**
 * Agent-based setup section for one server: live validation from the agent
 * container's own reporting, re-run setup (confirm + live logs), and GPU
 * configuration. Lives inside the server sheet's Setup tab.
 */
export function SetupServerContent({
	server,
}: {
	server: RemoteServerResponse;
}) {
	const {can} = usePermissions();
	const canAct = can('server:create');
	const serverId = BigInt(server.id);

	// Validate: everything reads what the agent itself reports over gRPC —
	// no SSH round-trip. The agent running as a container IS the check.
	const agentStatus = useAgentStatus(serverId);
	const containers = useMonitoringContainerStates(serverId);
	const metrics = useServerMetrics(serverId, 1);
	const latest = metrics.data?.[0];
	const refreshing =
		agentStatus.isFetching || containers.isFetching || metrics.isFetching;
	const refresh = () => {
		void agentStatus.refetch();
		void containers.refetch();
		void metrics.refetch();
	};

	// Re-setup: streamed over SSH; a failed SSH test gates the run.
	const {
		testingConn,
		testConnection,
		settingUp,
		setupState,
		connOk,
		setupLogs,
		setup,
	} = useServerSetup(server);
	const [advertiseAddr, setAdvertiseAddr] = useState('');
	const [sudoPassword, setSudoPassword] = useState('');
	const [setupConfirmOpen, setSetupConfirmOpen] = useState(false);

	// GPU: read current support state, run the toolkit configuration on demand.
	const management = useServerManagement(serverId);
	const gpuAction = useServerManagementAction(serverId);
	const [gpuConfirmOpen, setGpuConfirmOpen] = useState(false);
	const [gpuResult, setGpuResult] = useState<ServerActionPayload | null>(
		null,
	);

	const runGpu = () => {
		setGpuConfirmOpen(false);
		gpuAction.mutate('configureGpu', {
			onSuccess: payload => {
				setGpuResult(payload);
				if (payload.success) {
					toast.success('GPU support configured');
				} else {
					toast.error(payload.stderr || 'GPU configuration failed');
				}
			},
			onError: error => toast.error(formatApiError(error)),
		});
	};

	// The agent's own state is the verdict: ONLINE means the container is up
	// and its gRPC channel answers; everything else reads as not reporting.
	const agentRow = (() => {
		const status = agentStatus.data;
		if (!status) {
			return {ok: undefined, text: 'checking…'} as const;
		}
		if (status.state === 'online') {
			return {
				ok: true,
				text: status.agent_version
					? `reporting · v${status.agent_version}`
					: 'reporting',
			} as const;
		}
		if (status.state === 'offline') {
			return {
				ok: false,
				text: status.last_seen_at
					? `offline · last seen ${new Date(status.last_seen_at * 1000).toLocaleString()}`
					: 'offline',
			} as const;
		}
		if (status.state === 'unregistered') {
			return {ok: false, text: 'agent not installed'} as const;
		}
		return {ok: false, text: 'never reported'} as const;
	})();

	// The agent reads container state through the host's Docker socket, so a
	// successful states fetch is direct proof the engine is reachable.
	const dockerRow = containers.isError
		? {ok: false, text: 'unreachable'}
		: containers.isSuccess
			? {
					ok: true,
					text: `reachable · ${(containers.data?.length ?? 0).toString()} container(s)`,
				}
			: {ok: undefined, text: 'checking…'};

	return (
		<div className="flex flex-col gap-4">
			{/* ── Validate (agent-reported, live) ── */}
			<div className="rounded-lg border border-border/60 bg-card p-4">
				<SectionHeader
					title="Setup Validation"
					caption="Live state reported by the agent container on this server — no SSH involved."
					action={
						<Button
							size="sm"
							variant="outline"
							disabled={refreshing}
							onClick={refresh}
							className="h-8 gap-1.5 text-xs">
							{refreshing ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<RefreshCw className="size-3.5" />
							)}
							Refresh
						</Button>
					}
				/>
				<div className="mt-3 flex flex-col gap-3">
					<CheckSection
						title="Agent"
						caption="The agent container the panel talks to over gRPC.">
						<StatusRow
							label="Agent reporting"
							isEnabled={agentRow.ok}
							description={agentRow.text}
						/>
					</CheckSection>
					<CheckSection
						title="Engine"
						caption="Docker state as seen through the agent's own socket.">
						<StatusRow
							label="Docker engine"
							isEnabled={dockerRow.ok}
							description={dockerRow.text}
						/>
					</CheckSection>
					<CheckSection
						title="Host"
						caption="Host facts the agent reads directly from the machine.">
						<StatusRow
							label="Operating system"
							value={latest?.distro || latest?.os}
						/>
						<StatusRow label="Architecture" value={latest?.arch} />
						<StatusRow label="Kernel" value={latest?.kernel} />
						<StatusRow
							label="CPU"
							value={
								latest
									? `${latest.cpu_model} · ${latest.cpu_cores} cores`
									: undefined
							}
						/>
					</CheckSection>
				</div>
			</div>

			{/* ── Re-setup ── */}
			<div className="rounded-lg border border-border/60 bg-card p-4">
				<SectionHeader
					title="Setup Server"
					caption="Installs Docker, Swarm, Traefik, and the agent. Safe to re-run on a host that is already set up."
				/>
				<div className="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-2">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Advertise address
						</Label>
						<Input
							value={advertiseAddr}
							onChange={e => setAdvertiseAddr(e.target.value)}
							disabled={settingUp}
							placeholder="100.x.x.x (VPN IP, optional)"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Sudo password</Label>
						<Input
							type="password"
							value={sudoPassword}
							onChange={e => setSudoPassword(e.target.value)}
							disabled={settingUp}
							placeholder="If sudo is required"
						/>
					</div>
				</div>
				<div className="mt-3 flex flex-wrap items-center gap-2">
					<Button
						size="sm"
						variant="outline"
						disabled={testingConn || settingUp}
						onClick={() => void testConnection()}
						className="h-8 gap-1.5 text-xs">
						{testingConn ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<RefreshCw className="size-3.5" />
						)}
						{testingConn ? 'Testing…' : 'Test Connection'}
					</Button>
					<Button
						size="sm"
						disabled={settingUp || connOk === false}
						onClick={() => setSetupConfirmOpen(true)}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						{settingUp ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<RefreshCw className="size-3.5" />
						)}
						{setupState === 'failed' ? 'Retry Setup' : 'Setup Server'}
					</Button>
					{connOk === false && (
						<p className="text-xs text-amber-600 dark:text-amber-500">
							SSH test failed — fix access and re-test before running
							setup.
						</p>
					)}
				</div>
			</div>

			{(settingUp || setupLogs.length > 0) && (
				<SetupLogsSurface
					logs={setupLogs}
					isLive={settingUp}
					state={setupState}
					className="min-h-[30vh] w-full rounded-lg"
				/>
			)}

			{/* ── GPU ── */}
			<div className="rounded-lg border border-border/60 bg-card p-4">
				<SectionHeader
					title="GPU Setup"
					caption="Installs the NVIDIA container toolkit so services on this server can request GPU resources."
					action={
						<Button
							size="sm"
							variant="outline"
							disabled={!canAct || gpuAction.isPending}
							onClick={() => setGpuConfirmOpen(true)}
							className="h-8 gap-1.5 text-xs">
							{gpuAction.isPending ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<RefreshCw className="size-3.5" />
							)}
							{gpuAction.isPending ? 'Configuring…' : 'Configure GPU'}
						</Button>
					}
				/>
				<div className="mt-3 grid gap-2.5">
					<StatusRow
						label="GPU support enabled"
						isEnabled={management.data?.gpuEnabled === true}
						description={
							management.data?.gpuEnabled
								? 'enabled'
								: 'not enabled — toggle it from the Actions tab after configuring'
						}
					/>
				</div>
				{gpuResult && (
					<div className="mt-3 border-t border-border/50 pt-3">
						<span
							className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
								gpuResult.success
									? 'bg-emerald-500/10 text-emerald-500'
									: 'bg-rose-500/10 text-rose-500'
							}`}>
							{gpuResult.success ? 'success' : 'failed'}
						</span>
						<pre className="mt-2 max-h-48 overflow-auto rounded-lg bg-muted/40 p-3 font-mono text-[11px] leading-relaxed">
							{[gpuResult.stdout, gpuResult.stderr]
								.filter(Boolean)
								.join('\n\n') || 'No output.'}
						</pre>
					</div>
				)}
			</div>

			{/* ── Confirms ── */}
			<AlertDialog
				open={setupConfirmOpen}
				onOpenChange={setSetupConfirmOpen}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>
							Run setup on {server.name}?
						</AlertDialogTitle>
						<AlertDialogDescription>
							This re-runs the full setup over SSH — installing
							dependencies and configuring Docker Swarm, Traefik, and the
							agent. It can take a few minutes.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel>Cancel</AlertDialogCancel>
						<AlertDialogAction
							onClick={() => {
								setSetupConfirmOpen(false);
								void setup(advertiseAddr, sudoPassword);
							}}>
							Run Setup
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>

			<AlertDialog open={gpuConfirmOpen} onOpenChange={setGpuConfirmOpen}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>
							Configure GPU on {server.name}?
						</AlertDialogTitle>
						<AlertDialogDescription>
							This installs the NVIDIA container toolkit on the host
							through the agent and can take a few minutes. Docker is
							restarted as part of it.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel>Cancel</AlertDialogCancel>
						<AlertDialogAction onClick={runGpu}>
							Configure GPU
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
