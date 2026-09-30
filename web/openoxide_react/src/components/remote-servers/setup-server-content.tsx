import {useState} from 'react';
import type {LucideIcon} from 'lucide-react';
import {CheckCircle2, Cpu, Plug, RefreshCw, Terminal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {SetupLogsSurface} from './setup-logs-surface';
import {useServerSetup} from './use-server-setup';

import type {RemoteServerResponse} from '#/types/api-helpers';

/** One row in the shared ListCard-style stack: tile, text, right-aligned action. */
function ActionRow({
	icon: Icon,
	title,
	description,
	note,
	busy,
	busyLabel,
	idleLabel,
	primary,
	disabled,
	onClick,
}: {
	icon: LucideIcon;
	title: string;
	description: string;
	note?: string;
	busy: boolean;
	busyLabel: string;
	idleLabel: string;
	primary?: boolean;
	disabled?: boolean;
	onClick: () => void;
}) {
	return (
		<div className="flex items-center gap-3.5 px-4 py-3">
			<span className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
				<Icon className="size-4 text-muted-foreground" />
			</span>
			<div className="min-w-0 flex-1">
				<p className="truncate text-sm font-medium text-foreground">
					{title}
				</p>
				<p className="truncate text-xs text-muted-foreground">
					{description}
				</p>
				{note && (
					<p className="truncate text-xs text-amber-600 dark:text-amber-500">
						{note}
					</p>
				)}
			</div>
			<Button
				variant={primary ? 'default' : 'outline'}
				onClick={onClick}
				disabled={busy || disabled}
				className="h-8 shrink-0 gap-1.5 text-xs font-semibold">
				{busy ? (
					<RefreshCw className="size-3.5 animate-spin" />
				) : (
					<Icon className="size-3.5" />
				)}
				{busy ? busyLabel : idleLabel}
			</Button>
		</div>
	);
}

/**
 * Test → audit → provision flow for one server, as a plain content block
 * so it can live inside the server settings panel.
 */
export function SetupServerContent({
	server,
}: {
	server: RemoteServerResponse;
}) {
	const [advertiseAddr, setAdvertiseAddr] = useState('');
	const [sudoPassword, setSudoPassword] = useState('');
	const {
		testingConn,
		testConnection,
		auditing,
		audit,
		settingUp,
		setupState,
		connOk,
		setupLogs,
		setup,
		auditResult,
	} = useServerSetup(server);

	return (
		<div className="flex min-w-0 flex-col gap-4">
			<div className="w-full shrink-0 divide-y overflow-hidden rounded-xl border bg-card">
				<ActionRow
					icon={Plug}
					title="SSH Connection Test"
					description={`Verify port ${server.port || 22} reachability`}
					busy={testingConn}
					busyLabel="Testing…"
					idleLabel="Test Connection"
					onClick={() => void testConnection()}
				/>
				<ActionRow
					icon={Cpu}
					title="System Specs Audit"
					description="Check Docker engine, OS, CPU, and RAM"
					busy={auditing}
					busyLabel="Auditing…"
					idleLabel="Run Audit"
					onClick={() => void audit()}
				/>
				<ActionRow
					icon={Terminal}
					title="Setup Server"
					description="Install Docker, Swarm, Traefik, and the agent"
					note={
						connOk === false
							? 'SSH test failed — fix access and re-test before running setup.'
							: undefined
					}
					busy={settingUp}
					busyLabel="Setting up…"
					idleLabel={setupState === 'failed' ? 'Retry Setup' : 'Run Setup'}
					primary
					disabled={connOk === false}
					onClick={() => void setup(advertiseAddr, sudoPassword)}
				/>
			</div>

			<div className="grid shrink-0 grid-cols-1 gap-3 sm:grid-cols-2">
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
			<p className="shrink-0 text-[11px] text-muted-foreground">
				Advertise address auto-detects VPN interfaces when left blank; sudo
				password is only needed when the SSH user isn&rsquo;t root or lacks
				passwordless sudo.
			</p>

			{(settingUp || setupLogs.length > 0) && (
				<SetupLogsSurface
					logs={setupLogs}
					isLive={settingUp}
					state={setupState}
					className="min-h-[45vh] w-full shrink grow-0 rounded-lg"
				/>
			)}

			{auditResult && (
				<div className="shrink-0 rounded-xl border bg-card p-4">
					<div className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
						<CheckCircle2 className="size-3.5 text-emerald-500" />
						Audit outcome
					</div>
					<div className="mt-2 grid grid-cols-2 gap-x-4 gap-y-1 font-mono text-[11px] text-muted-foreground">
						<span>
							Docker:{' '}
							{auditResult.docker_installed ? 'Installed' : 'Missing'}
						</span>
						<span>Arch: {String(auditResult.arch || 'x86_64')}</span>
						<span>OS: {String(auditResult.os || 'Linux')}</span>
						<span>CPU cores: {String(auditResult.cpus || 1)}</span>
					</div>
				</div>
			)}
		</div>
	);
}
