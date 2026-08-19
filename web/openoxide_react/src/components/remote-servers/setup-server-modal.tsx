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
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	Terminal,
	ShieldCheck,
	Cpu,
	RefreshCw,
	CheckCircle2,
	Plug,
} from 'lucide-react';
import {LogViewer} from '#/components/shared/log-viewer';
import {extractLogLines} from '#/hooks/deployments/use-deployment-logs';

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
	const [testingConn, setTestingConn] = useState(false);
	const [auditing, setAuditing] = useState(false);
	const [settingUp, setSettingUp] = useState(false);
	const [setupLogs, setSetupLogs] = useState<string[]>([]);
	const [auditResult, setAuditResult] = useState<Record<
		string,
		unknown
	> | null>(null);
	const [advertiseAddr, setAdvertiseAddr] = useState('');

	const testConnMutation = $api.useMutation(
		'post',
		'/servers/{id}/test-connection',
	);
	const auditMutation = $api.useMutation('post', '/servers/{id}/audit');

	const handleTestConnection = async () => {
		if (!server?.id) return;
		setTestingConn(true);
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''},
			});
			toast.success(
				`SSH Connection to ${server.name} verified successfully!`,
			);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setTestingConn(false);
		}
	};

	const handleAudit = async () => {
		if (!server?.id) return;
		setAuditing(true);
		try {
			const res = await auditMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''},
			});
			setAuditResult(res as Record<string, unknown>);
			toast.success('Server audit completed!');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setAuditing(false);
		}
	};

	const handleSetup = async () => {
		if (!server?.id) return;
		setSettingUp(true);
		setSetupLogs([]);
		try {
			const sessionRaw = localStorage.getItem('openoxide-auth-session');
			let accessToken = '';
			if (sessionRaw) {
				try {
					const session = JSON.parse(sessionRaw);
					accessToken = session?.tokens?.access_token || '';
				} catch {}
			}

			const response = await fetch(
				`/api/servers/${server.id}/setup/logs`,
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: accessToken ? `Bearer ${accessToken}` : '',
					},
					body: JSON.stringify({
						host_key_fingerprint: '',
						install_dependencies: true,
						...(advertiseAddr.trim()
							? {advertise_addr: advertiseAddr.trim()}
							: {}),
					}),
				},
			);

			if (!response.ok) {
				throw new Error(await response.text());
			}

			const reader = response.body?.getReader();
			if (!reader) {
				throw new Error('Setup log stream is not readable');
			}

			const decoder = new TextDecoder();
			let buffer = '';
			let failedMessage = '';
			while (true) {
				const {done, value} = await reader.read();
				if (done) break;

				buffer += decoder.decode(value, {stream: true});
				const rawLines = buffer.split('\n');
				buffer = rawLines.pop() || '';

				for (const rawLine of rawLines) {
					const lines = extractLogLines(rawLine);
					if (lines.length > 0) {
						failedMessage ||=
							lines.find(line => line.includes('Setup Server failed')) ||
							'';
						setSetupLogs(prev => [...prev, ...lines]);
					}
				}
			}

			if (buffer.trim()) {
				const lines = extractLogLines(buffer);
				if (lines.length > 0) {
					failedMessage ||=
						lines.find(line => line.includes('Setup Server failed')) || '';
					setSetupLogs(prev => [...prev, ...lines]);
				}
			}

			if (failedMessage) {
				throw new Error(failedMessage);
			}

			toast.success(`Server ${server.name} setup completed successfully!`);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSettingUp(false);
		}
	};

	if (!server) return null;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="w-full rounded-xl border-border bg-card p-6 shadow-xl sm:max-w-xl md:max-w-2xl">
				<DialogHeader className="border-b border-border/40 pb-3">
					<DialogTitle className="flex items-center gap-2 text-base font-bold text-foreground">
						<Terminal className="h-5 w-5 text-primary" />
						Setup & Audit: {server.name}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Test SSH connectivity and audit host specs (
						{server.username || 'root'}@{server.ip_address})
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 py-3">
					<div className="flex items-center justify-between rounded-xl border border-border/50 bg-muted/20 p-3.5">
						<div className="flex items-center gap-2.5">
							<ShieldCheck className="h-5 w-5 text-primary" />
							<div>
								<h4 className="text-xs font-bold text-foreground">
									SSH Connection Test
								</h4>
								<p className="text-[11px] text-muted-foreground">
									Verify port {server.port || 22} reachability
								</p>
							</div>
						</div>
						<Button
							variant="outline"
							size="sm"
							onClick={handleTestConnection}
							disabled={testingConn}
							className="h-8 gap-1.5 text-xs font-semibold">
							{testingConn ? (
								<RefreshCw className="h-3.5 w-3.5 animate-spin" />
							) : (
								<Plug className="h-3.5 w-3.5 text-primary" />
							)}
							{testingConn ? 'Testing...' : 'Test Connection'}
						</Button>
					</div>

					<div className="flex items-center justify-between rounded-xl border border-border/50 bg-muted/20 p-3.5">
						<div className="flex items-center gap-2.5">
							<Cpu className="h-5 w-5 text-primary" />
							<div>
								<h4 className="text-xs font-bold text-foreground">
									System Specs Audit
								</h4>
								<p className="text-[11px] text-muted-foreground">
									Check Docker engine, OS, CPU, and RAM
								</p>
							</div>
						</div>
						<Button
							variant="outline"
							size="sm"
							onClick={handleAudit}
							disabled={auditing}
							className="h-8 gap-1.5 text-xs font-semibold">
							{auditing ? (
								<RefreshCw className="h-3.5 w-3.5 animate-spin" />
							) : (
								<Cpu className="h-3.5 w-3.5" />
							)}
							{auditing ? 'Auditing...' : 'Run Audit'}
						</Button>
					</div>

					<div className="flex flex-col gap-2.5 rounded-xl border border-primary/30 bg-primary/10 p-3.5">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-2.5">
								<Terminal className="h-5 w-5 text-primary" />
								<div>
									<h4 className="text-xs font-bold text-foreground">
										Setup Server
									</h4>
									<p className="text-[11px] text-muted-foreground">
										Install dependencies, Docker, Swarm, Traefik, and
										monitoring
									</p>
								</div>
							</div>
							<Button
								size="sm"
								onClick={handleSetup}
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
								className="h-8 rounded-md border-border bg-background px-3 text-xs"
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
						<div className="flex flex-col gap-2 rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-3.5 font-mono text-xs text-foreground">
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
