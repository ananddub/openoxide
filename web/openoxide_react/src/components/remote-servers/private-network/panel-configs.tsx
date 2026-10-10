import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Textarea} from '#/components/ui/textarea';
import {serverHostLabel} from '#/api/utils';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {PrivateNetworkProvider} from './types';
import {EXTERNAL_PROVIDERS, Field} from './panel-parts';

/** Managed WireGuard tunnel parameters. */
export function WireGuardFields({
	server,
	listenPort,
	tunnelAddress,
	managedHost,
	keepalive,
	onListenPort,
	onTunnelAddress,
	onKeepalive,
}: {
	server: RemoteServerResponse;
	listenPort: string;
	tunnelAddress: string;
	managedHost: string | null;
	keepalive: string;
	onListenPort: (v: string) => void;
	onTunnelAddress: (v: string) => void;
	onKeepalive: (v: string) => void;
}) {
	return (
		<div className="grid gap-3">
			<div className="rounded-lg bg-muted/30 p-3 text-xs leading-relaxed text-muted-foreground">
				OpenOxide installs WireGuard on the remote server using its
				configured IP ({serverHostLabel(server.ip_address)}) and the UDP
				listen port below. No STUN discovery or mesh VPN — allow the UDP
				port in the server&apos;s firewall.
			</div>
			<div className="grid gap-3 sm:grid-cols-2">
				<Field label="UDP listen port">
					<Input
						type="number"
						value={listenPort}
						onChange={event => onListenPort(event.target.value)}
						placeholder="51820"
					/>
				</Field>
				<Field label="Tunnel network">
					<Input
						value={tunnelAddress}
						onChange={event => onTunnelAddress(event.target.value)}
						placeholder="10.77.2.0/24"
					/>
				</Field>
				<Field label="Remote tunnel IP">
					<Input value={managedHost || 'Invalid /24 network'} disabled />
				</Field>
				<Field label="Persistent keepalive (s)">
					<Input
						type="number"
						value={keepalive}
						onChange={event => onKeepalive(event.target.value)}
					/>
				</Field>
			</div>
		</div>
	);
}

/** Existing-VPN routing details (provider + private address). */
export function ExternalVpnFields({
	provider,
	privateHost,
	dnsName,
	routes,
	busy,
	onProvider,
	onPrivateHost,
	onDnsName,
	onRoutes,
}: {
	provider: PrivateNetworkProvider;
	privateHost: string;
	dnsName: string;
	routes: string;
	busy: boolean;
	onProvider: (v: PrivateNetworkProvider) => void;
	onPrivateHost: (v: string) => void;
	onDnsName: (v: string) => void;
	onRoutes: (v: string) => void;
}) {
	return (
		<div className="grid gap-3 sm:grid-cols-2">
			<Field label="VPN provider">
				<Select
					value={provider}
					onValueChange={value =>
						onProvider(value as PrivateNetworkProvider)
					}
					disabled={busy}>
					<SelectTrigger>
						<SelectValue>
							{EXTERNAL_PROVIDERS.find(p => p.value === provider)?.label ??
								provider}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{EXTERNAL_PROVIDERS.map(item => (
							<SelectItem key={item.value} value={item.value}>
								{item.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Field>
			<Field label="Private IP or hostname">
				<Input
					value={privateHost}
					onChange={event => onPrivateHost(event.target.value)}
					placeholder="100.80.10.20"
				/>
			</Field>
			<Field label="Private DNS name (optional)">
				<Input
					value={dnsName}
					onChange={event => onDnsName(event.target.value)}
					placeholder="node-1.openoxide.internal"
				/>
			</Field>
			<Field label="Private routes (optional, one CIDR per line)">
				<Textarea
					value={routes}
					onChange={event => onRoutes(event.target.value)}
					placeholder={'10.90.0.0/16\n172.30.0.0/16'}
				/>
			</Field>
		</div>
	);
}

/** Direct-mode explainer. */
export function DirectSshNote({server}: {server: RemoteServerResponse}) {
	return (
		<p className="text-xs leading-relaxed text-muted-foreground">
			Deployments and SSH connect to{' '}
			<span className="font-mono text-foreground/80">
				{serverHostLabel(server.ip_address)}
			</span>{' '}
			directly. Pick a tunnel mode above to route traffic over a private
			network instead.
		</p>
	);
}
