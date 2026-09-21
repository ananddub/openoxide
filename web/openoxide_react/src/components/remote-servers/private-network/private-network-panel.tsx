import {RefreshCw, Shield} from 'lucide-react';
import {Button} from '#/components/ui/button';
import type {RemoteServerResponse} from '#/types/api-helpers';

import {ModeTiles} from './panel-parts';
import {
	DirectSshNote,
	ExternalVpnFields,
	WireGuardFields,
} from './panel-configs';
import {PanelActions} from './panel-actions';
import {PanelRail} from './panel-rail';
import {usePrivateNetwork} from './use-private-network';

interface PrivateNetworkPanelProps {
	server: RemoteServerResponse;
}

/**
 * Private Network tab on the server detail page. OpenShip layout: primary
 * card on the left (mode tiles + contextual config + actions), a sticky
 * right rail that always shows the live connection facts and a short
 * guide — so the page never reads as an empty form.
 */
export function PrivateNetworkPanel({server}: PrivateNetworkPanelProps) {
	const pn = usePrivateNetwork(server);
	const {
		mode,
		setMode,
		provider,
		setProvider,
		privateHost,
		setPrivateHost,
		tunnelAddress,
		setTunnelAddress,
		listenPort,
		setListenPort,
		keepalive,
		setKeepalive,
		dnsName,
		setDnsName,
		routes,
		setRoutes,
		config,
		loading,
		action,
		managedHost,
		busy,
		effectiveMode,
		run,
		save,
		refresh,
	} = pn;

	return (
		<div className="grid items-start gap-4 lg:grid-cols-[minmax(0,1fr)_340px]">
			{/* ── Left: configuration card ──────────────────────────── */}
			<div className="rounded-xl bg-card ring-1 ring-border/50">
				{/* Header */}
				<div className="flex flex-wrap items-center gap-3 border-b px-5 py-4">
					<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-foreground/[0.06]">
						<Shield className="size-[18px] text-foreground/80" />
					</div>
					<div className="min-w-0 flex-1">
						<h2 className="text-[15px] font-semibold text-foreground">
							Private Network
						</h2>
						<p className="text-xs text-muted-foreground">
							Choose how OpenOxide reaches{' '}
							<span className="font-mono text-foreground/80">
								{server.name || 'this host'}
							</span>{' '}
							— public IP, a managed tunnel, or your own VPN.
						</p>
					</div>
					<Button
						variant="ghost"
						size="icon-sm"
						onClick={refresh}
						disabled={busy}
						title="Refresh status"
						className="cursor-pointer">
						<RefreshCw
							className={`size-3.5 ${action === 'refresh' ? 'animate-spin' : ''}`}
						/>
					</Button>
				</div>

				{config?.health_error && (
					<div className="border-b bg-rose-500/5 px-5 py-2.5 text-xs text-rose-500">
						{config.health_error}
					</div>
				)}

				{/* Mode tiles */}
				<ModeTiles mode={mode} disabled={busy} onSelect={setMode} />

				{/* Contextual configuration */}
				{mode === 'MANAGED_WIREGUARD' && (
					<WireGuardFields
						server={server}
						listenPort={listenPort}
						tunnelAddress={tunnelAddress}
						managedHost={managedHost}
						keepalive={keepalive}
						onListenPort={setListenPort}
						onTunnelAddress={setTunnelAddress}
						onKeepalive={setKeepalive}
					/>
				)}

				{mode === 'EXTERNAL_PRIVATE_NETWORK' && (
					<ExternalVpnFields
						provider={provider}
						privateHost={privateHost}
						dnsName={dnsName}
						routes={routes}
						busy={busy}
						onProvider={setProvider}
						onPrivateHost={setPrivateHost}
						onDnsName={setDnsName}
						onRoutes={setRoutes}
					/>
				)}

				{mode === 'DIRECT_SSH' && <DirectSshNote server={server} />}

				{/* Footer actions */}
				<PanelActions
					serverId={server.id}
					hasConfig={!!config}
					mode={mode}
					busy={busy}
					onRun={(name, operation, message) =>
						void run(name, operation, message)
					}
					onSave={() => void save()}
				/>
			</div>

			{/* ── Right rail: live facts + guide ────────────────────── */}
			<PanelRail
				server={server}
				config={config}
				loading={loading}
				effectiveMode={effectiveMode}
			/>
		</div>
	);
}
