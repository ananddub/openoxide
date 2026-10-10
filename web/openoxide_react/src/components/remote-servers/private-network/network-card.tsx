import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {IconTooltip} from '#/components/shared/icon-tooltip';
import {OptionRow} from '#/components/shared/sheet-wizard';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {NetworkActionButtons} from './panel-actions';
import {
	DirectSshNote,
	ExternalVpnFields,
	WireGuardFields,
} from './panel-configs';
import {MODE_LABELS, MODE_TILES, statusDot} from './panel-parts';
import {usePrivateNetwork} from './use-private-network';

/**
 * Private network settings for an existing server, sized for a sheet: plain
 * sections instead of the detail page's bordered card (the pane already has
 * its own padding), and the action row pinned to the bottom of the sheet so
 * Save always sits where the other sheets put theirs. Used by the edit
 * sheet's Private Network tab and the server settings side panel.
 */
export function PrivateNetworkCard({
	server,
}: {
	server: RemoteServerResponse;
}) {
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
		managedHost,
		busy,
		run,
		save,
		refresh,
	} = usePrivateNetwork(server);

	return (
		<div className="flex min-h-0 flex-1 flex-col">
			{/* Scrollable content — the action bar below is a real footer, no
			    sticky/negative-margin tricks (those were spawning phantom scrollbars). */}
			<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto pb-5">
				{/* Live status strip — what is running, as opposed to what is being edited. */}
				<div className="flex items-center gap-3 rounded-xl border border-border/60 bg-muted/20 px-4 py-3">
					<span
						className={`size-2 shrink-0 rounded-full ${statusDot(config?.status ?? 'DISABLED')}`}
					/>
					<span className="min-w-0 flex-1 truncate text-[13px] text-muted-foreground">
						{config
							? `${MODE_LABELS[config.connection_mode]} · ${config.status}`
							: 'No private network configured — reached over its public IP'}
					</span>
					<IconTooltip label="Refresh status">
						<Button
							variant="ghost"
							size="icon-sm"
							onClick={refresh}
							disabled={busy}
							aria-label="Refresh status"
							className="shrink-0 cursor-pointer">
							<RefreshCw
								className={`size-3.5 ${busy ? 'animate-spin' : ''}`}
							/>
						</Button>
					</IconTooltip>
				</div>

				{/* Mode picker */}
				<div className="flex flex-col gap-2">
					{MODE_TILES.map(tile => (
						<OptionRow
							key={tile.value}
							selected={mode === tile.value}
							disabled={busy}
							onClick={() => setMode(tile.value)}
							icon={<tile.icon className="size-4.5" />}
							title={tile.label}
							subtitle={tile.desc}
						/>
					))}
				</div>

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
			</div>

			{/* Footer — same grammar as SheetFooter (border + card/40 surface). */}
			<div className="-mb-6 flex shrink-0 flex-wrap items-center gap-3 border-t border-border bg-card/40 py-4">
				<NetworkActionButtons
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
		</div>
	);
}
