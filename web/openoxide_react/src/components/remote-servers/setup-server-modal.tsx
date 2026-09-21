import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {serverHostLabel} from '#/api/utils';
import {
	Terminal,
	ShieldCheck,
	Cpu,
	RefreshCw,
	CheckCircle2,
	Plug,
} from 'lucide-react';
import {LogViewer} from '#/components/shared/log-viewer';
import {SetupActionRow} from './setup-action-row';
import {useServerSetup} from './use-server-setup';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface SetupServerModalProps {
	isOpen: boolean;
	server: RemoteServerResponse | null;
	onClose: () => void;
}

export function SetupServerModal({
	isOpen,
	server,
	onClose,
}: SetupServerModalProps) {
	const [advertiseAddr, setAdvertiseAddr] = useState('');
	const {
		testingConn,
		testConnection,
		auditing,
		audit,
		settingUp,
		setupLogs,
		setup,
		auditResult,
	} = useServerSetup(server);

	if (!server) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="border- w-full rounded-xl bg-card p-6 shadow-md sm:max-w-xl md:max-w-2xl">
				<DialogHeader className="border-b  pb-3">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
						<Terminal className="h-5 w-5 text-primary" />
						Setup & Audit: {server.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Test SSH connectivity and audit host specs (
						{server.username || 'root'}@
						{serverHostLabel(server.ip_address)})
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 py-3">
					<SetupActionRow
						icon={ShieldCheck}
						title="SSH Connection Test"
						description={`Verify port ${server.port || 22} reachability`}
						busy={testingConn}
						busyLabel="Testing..."
						idleLabel="Test Connection"
						idleIcon={Plug}
						onClick={() => void testConnection()}
					/>

					<SetupActionRow
						icon={Cpu}
						title="System Specs Audit"
						description="Check Docker engine, OS, CPU, and RAM"
						busy={auditing}
						busyLabel="Auditing..."
						idleLabel="Run Audit"
						idleIcon={Cpu}
						onClick={() => void audit()}
					/>

					<div className="flex flex-col gap-2.5 rounded-xl bg-primary/10 p-3.5">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-2.5">
								<Terminal className="h-5 w-5 text-primary" />
								<div>
									<h4 className="text-xs font-bold text-foreground">
										Setup Server
									</h4>
									<p className="text-[11px] text-muted-foreground">
										Install dependencies, Docker, Swarm, Traefik, and the
										OpenOxide Agent
									</p>
								</div>
							</div>
							<Button
								size="sm"
								onClick={() => void setup(advertiseAddr)}
								disabled={settingUp}
								className="h-8 gap-1.5 text-xs font-semibold">
								{settingUp ? (
									<RefreshCw className="h-3.5 w-3.5 animate-spin" />
								) : (
									<Terminal className="h-3.5 w-3.5" />
								)}
								{settingUp ? 'Setting up...' : 'Run Setup'}
							</Button>
						</div>
						<div className="flex flex-col gap-1">
							<label className="text-[11px] font-semibold text-foreground">
								Advertise Address (optional)
							</label>
							<Input
								value={advertiseAddr}
								onChange={e => setAdvertiseAddr(e.target.value)}
								disabled={settingUp}
								placeholder="e.g. 100.x.x.x — Tailscale/Netbird/VPN IP"
								className="border- h-8 rounded-md bg-background px-3 text-xs"
							/>
							<p className="text-[10px] text-muted-foreground">
								Used by Docker Swarm to advertise this node. Leave blank to
								auto-detect (prefers Tailscale/WireGuard/VPN interfaces);
								set this if nodes should talk over a specific VPN mesh.
							</p>
						</div>
					</div>

					{(settingUp || setupLogs.length > 0) && (
						<LogViewer
							logs={setupLogs}
							isLoading={settingUp}
							loadingText="Running server setup..."
							emptyText="Waiting for setup output..."
							heightClass="h-[220px]"
							isLive={settingUp}
							showFilter={false}
						/>
					)}

					{auditResult && (
						<div className="flex flex-col gap-2 rounded-xl bg-emerald-500/10 p-3.5 font-mono text-xs text-foreground">
							<div className="flex items-center gap-1.5 font-sans font-bold text-emerald-500">
								<CheckCircle2 className="h-4 w-4" /> Audit Outcome
							</div>
							<div className="grid grid-cols-2 gap-2 text-[11px] text-muted-foreground">
								<span>
									Docker:{' '}
									{auditResult.docker_installed ? 'Installed' : 'Missing'}
								</span>
								<span>Arch: {String(auditResult.arch || 'x86_64')}</span>
								<span>OS: {String(auditResult.os || 'Linux')}</span>
								<span>CPU Cores: {String(auditResult.cpus || 1)}</span>
							</div>
						</div>
					)}
				</div>
			</DialogContent>
		</Dialog>
	);
}
