import {useEffect, useMemo, useState} from 'react';
import {
	Activity,
	Check,
	Globe,
	KeyRound,
	Network,
	RefreshCw,
	Save,
	Shield,
	Wrench,
} from 'lucide-react';
import {toast} from 'sonner';

import {cn, formatApiError, serverHostLabel} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Textarea} from '#/components/ui/textarea';
import type {RemoteServerResponse} from '#/types/api-helpers';

import {privateNetworkApi} from './api';
import type {
	ConnectionMode,
	PrivateNetworkConfig,
	PrivateNetworkProvider,
	UpdatePrivateNetworkInput,
} from './types';
import {validatePrivateNetworkForm} from './validation';

interface PrivateNetworkPanelProps {
	server: RemoteServerResponse;
}

const EXTERNAL_PROVIDERS: Array<{
	value: PrivateNetworkProvider;
	label: string;
}> = [
	{value: 'TAILSCALE', label: 'Tailscale'},
	{value: 'NETBIRD', label: 'NetBird'},
	{value: 'ZEROTIER', label: 'ZeroTier'},
	{value: 'CUSTOM', label: 'Other / Custom VPN'},
];

/** The three connection modes as selectable tiles — never a bare dropdown. */
const MODE_TILES: Array<{
	value: ConnectionMode;
	label: string;
	desc: string;
	icon: typeof Globe;
}> = [
	{
		value: 'DIRECT_SSH',
		label: 'Direct SSH',
		desc: 'Reach the host over its public IP — no tunnel.',
		icon: Globe,
	},
	{
		value: 'MANAGED_WIREGUARD',
		label: 'Managed WireGuard',
		desc: 'OpenOxide installs and owns the tunnel.',
		icon: Shield,
	},
	{
		value: 'EXTERNAL_PRIVATE_NETWORK',
		label: 'Existing VPN',
		desc: 'Route via Tailscale, NetBird, or similar.',
		icon: Network,
	},
];

const MODE_LABELS: Record<ConnectionMode, string> = {
	DIRECT_SSH: 'Direct SSH',
	MANAGED_WIREGUARD: 'Managed WireGuard',
	EXTERNAL_PRIVATE_NETWORK: 'Existing VPN',
};

function statusDot(status: PrivateNetworkConfig['status']): string {
	switch (status) {
		case 'ACTIVE':
			return 'bg-emerald-500';
		case 'FAILED':
			return 'bg-rose-500';
		case 'CONFIGURING':
			return 'bg-amber-500 animate-pulse';
		default:
			return 'bg-muted-foreground/40';
	}
}

function remoteHostFromTunnel(cidr: string): string | null {
	const [address, prefix] = cidr.trim().split('/');
	const octets = address?.split('.').map(Number);
	if (
		prefix !== '24' ||
		octets?.length !== 4 ||
		octets.some(
			value => !Number.isInteger(value) || value < 0 || value > 255,
		)
	)
		return null;
	return `${octets[0]}.${octets[1]}.${octets[2]}.2`;
}

/** Label/value row in the right-rail cards (OpenShip Connection-card language). */
function FactRow({
	icon: Icon,
	label,
	value,
	mono = true,
}: {
	icon: typeof Globe;
	label: string;
	value: React.ReactNode;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center justify-between gap-3 py-2.5">
			<div className="flex min-w-0 items-center gap-2 text-sm text-muted-foreground">
				<Icon className="size-4 shrink-0" strokeWidth={2} />
				{label}
			</div>
			<div
				className={cn(
					'min-w-0 truncate text-sm text-foreground',
					mono && 'font-mono text-[13px]',
				)}>
				{value}
			</div>
		</div>
	);
}

/**
 * Private Network tab on the server detail page. OpenShip layout: primary
 * card on the left (mode tiles + contextual config + actions), a sticky
 * right rail that always shows the live connection facts and a short
 * guide — so the page never reads as an empty form.
 */
export function PrivateNetworkPanel({server}: PrivateNetworkPanelProps) {
	const [mode, setMode] = useState<ConnectionMode>('DIRECT_SSH');
	const [provider, setProvider] =
		useState<PrivateNetworkProvider>('TAILSCALE');
	const [privateHost, setPrivateHost] = useState('');
	const [tunnelAddress, setTunnelAddress] = useState('10.77.1.0/24');
	const [listenPort, setListenPort] = useState('51820');
	const [keepalive, setKeepalive] = useState('25');
	const [dnsName, setDnsName] = useState('');
	const [routes, setRoutes] = useState('');
	const [config, setConfig] = useState<PrivateNetworkConfig | null>(null);
	const [loading, setLoading] = useState(false);
	const [action, setAction] = useState('');

	const managedHost = useMemo(
		() => remoteHostFromTunnel(tunnelAddress),
		[tunnelAddress],
	);

	useEffect(() => {
		let cancelled = false;
		setLoading(true);
		privateNetworkApi
			.get(server.id)
			.then(current => {
				if (cancelled) return;
				setConfig(current);
				if (!current) return;
				setMode(current.connection_mode);
				setProvider(
					current.provider === 'WIREGUARD' || !current.provider
						? 'TAILSCALE'
						: current.provider,
				);
				setPrivateHost(current.private_host || '');
				setTunnelAddress(
					current.tunnel_address || `10.77.${(server.id % 250) + 1}.0/24`,
				);
				setListenPort(String(current.listen_port || 51820));
				setKeepalive(String(current.persistent_keepalive || 25));
				setDnsName(current.dns_name || '');
				setRoutes(current.routes.join('\n'));
			})
			.catch(error => toast.error(formatApiError(error)))
			.finally(() => !cancelled && setLoading(false));
		return () => {
			cancelled = true;
		};
	}, [server.id]);

	const input = (): UpdatePrivateNetworkInput => ({
		connection_mode: mode,
		provider:
			mode === 'MANAGED_WIREGUARD'
				? 'WIREGUARD'
				: mode === 'EXTERNAL_PRIVATE_NETWORK'
					? provider
					: null,
		private_host:
			mode === 'MANAGED_WIREGUARD'
				? managedHost
				: mode === 'EXTERNAL_PRIVATE_NETWORK'
					? privateHost.trim()
					: null,
		tunnel_address:
			mode === 'MANAGED_WIREGUARD' ? tunnelAddress.trim() : null,
		public_key: config?.public_key || null,
		endpoint:
			mode === 'MANAGED_WIREGUARD'
				? `${server.ip_address}:${listenPort || '51820'}`
				: null,
		listen_port: mode === 'MANAGED_WIREGUARD' ? Number(listenPort) : null,
		persistent_keepalive:
			mode === 'MANAGED_WIREGUARD' ? Number(keepalive) : null,
		dns_name: dnsName.trim() || null,
		routes: routes
			.split(/[\n,]/)
			.map(route => route.trim())
			.filter(Boolean),
	});

	const run = async (
		name: string,
		operation: () => Promise<unknown>,
		message: string,
	) => {
		setAction(name);
		try {
			const result = await operation();
			if (
				result &&
				typeof result === 'object' &&
				'connection_mode' in result
			)
				setConfig(result as PrivateNetworkConfig);
			toast.success(message);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setAction('');
		}
	};

	const save = async (setup: boolean) => {
		const validationError = validatePrivateNetworkForm({
			mode,
			managedHost,
			endpoint: `${server.ip_address}:${listenPort || '51820'}`,
			listenPort,
			privateHost,
		});
		if (validationError) {
			toast.error(validationError);
			return;
		}
		await run(
			setup ? 'setup' : 'save',
			async () => {
				const updated = await privateNetworkApi.update(server.id, input());
				setConfig(updated);
				return setup ? privateNetworkApi.setup(server.id) : updated;
			},
			setup
				? 'WireGuard configured successfully'
				: 'Private network settings saved',
		);
	};

	const refresh = async () => {
		await run(
			'refresh',
			async () => {
				const current = await privateNetworkApi.get(server.id);
				setConfig(current);
				return current;
			},
			'Private network status refreshed',
		);
	};

	const busy = loading || !!action;
	const effectiveMode = config?.connection_mode ?? mode;
	const configured = !!config && config.connection_mode !== 'DIRECT_SSH';

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
				<div className="grid gap-2.5 p-5 sm:grid-cols-3">
					{MODE_TILES.map(tile => {
						const selected = mode === tile.value;
						return (
							<button
								key={tile.value}
								type="button"
								onClick={() => setMode(tile.value)}
								disabled={busy}
								className={cn(
									'flex cursor-pointer flex-col gap-1.5 rounded-xl border p-3.5 text-start transition-colors disabled:cursor-not-allowed disabled:opacity-60',
									selected
										? 'border-primary/60 bg-primary/5'
										: 'border-border/60 bg-muted/20 hover:bg-muted/40',
								)}>
								<span className="flex items-center gap-2 text-sm font-medium text-foreground">
									<tile.icon className="size-4 shrink-0" strokeWidth={2} />
									<span className="flex-1 truncate">{tile.label}</span>
									{selected && (
										<Check className="size-3.5 shrink-0 text-primary" />
									)}
								</span>
								<span className="text-xs leading-snug text-muted-foreground">
									{tile.desc}
								</span>
							</button>
						);
					})}
				</div>

				{/* Contextual configuration */}
				{mode === 'MANAGED_WIREGUARD' && (
					<div className="grid gap-3 px-5 pb-5">
						<div className="rounded-lg bg-muted/30 p-3 text-xs leading-relaxed text-muted-foreground">
							OpenOxide installs WireGuard on the remote server using its
							configured IP ({serverHostLabel(server.ip_address)}) and the
							UDP listen port below. No STUN discovery or mesh VPN — allow
							the UDP port in the server's firewall.
						</div>
						<div className="grid gap-3 sm:grid-cols-2">
							<Field label="UDP listen port">
								<Input
									type="number"
									value={listenPort}
									onChange={event => setListenPort(event.target.value)}
									placeholder="51820"
								/>
							</Field>
							<Field label="Tunnel network">
								<Input
									value={tunnelAddress}
									onChange={event => setTunnelAddress(event.target.value)}
									placeholder="10.77.2.0/24"
								/>
							</Field>
							<Field label="Remote tunnel IP">
								<Input
									value={managedHost || 'Invalid /24 network'}
									disabled
								/>
							</Field>
							<Field label="Persistent keepalive (s)">
								<Input
									type="number"
									value={keepalive}
									onChange={event => setKeepalive(event.target.value)}
								/>
							</Field>
						</div>
					</div>
				)}

				{mode === 'EXTERNAL_PRIVATE_NETWORK' && (
					<div className="grid gap-3 px-5 pb-5 sm:grid-cols-2">
						<Field label="VPN provider">
							<Select
								value={provider}
								onValueChange={value =>
									setProvider(value as PrivateNetworkProvider)
								}
								disabled={busy}>
								<SelectTrigger>
									<SelectValue>
										{EXTERNAL_PROVIDERS.find(p => p.value === provider)
											?.label ?? provider}
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
								onChange={event => setPrivateHost(event.target.value)}
								placeholder="100.80.10.20"
							/>
						</Field>
						<Field label="Private DNS name (optional)">
							<Input
								value={dnsName}
								onChange={event => setDnsName(event.target.value)}
								placeholder="node-1.openoxide.internal"
							/>
						</Field>
						<Field label="Private routes (optional, one CIDR per line)">
							<Textarea
								value={routes}
								onChange={event => setRoutes(event.target.value)}
								placeholder={'10.90.0.0/16\n172.30.0.0/16'}
							/>
						</Field>
					</div>
				)}

				{mode === 'DIRECT_SSH' && (
					<p className="px-5 pb-5 text-xs leading-relaxed text-muted-foreground">
						Deployments and SSH connect to{' '}
						<span className="font-mono text-foreground/80">
							{serverHostLabel(server.ip_address)}
						</span>{' '}
						directly. Pick a tunnel mode above to route traffic over a
						private network instead.
					</p>
				)}

				{/* Footer actions */}
				<div className="flex flex-wrap items-center gap-2 border-t px-5 py-3.5">
					{config && mode !== 'DIRECT_SSH' && (
						<>
							<Button
								variant="outline"
								size="sm"
								disabled={busy}
								onClick={() =>
									run(
										'health',
										() => privateNetworkApi.health(server.id),
										'Health check completed',
									)
								}
								className="cursor-pointer gap-1.5 text-xs">
								<Activity className="size-3.5" /> Health
							</Button>
							<Button
								variant="outline"
								size="sm"
								disabled={busy}
								onClick={() =>
									run(
										'repair',
										() => privateNetworkApi.repair(server.id),
										'Private network repaired',
									)
								}
								className="cursor-pointer gap-1.5 text-xs">
								<Wrench className="size-3.5" /> Repair
							</Button>
							{mode === 'MANAGED_WIREGUARD' && (
								<>
									<Button
										variant="outline"
										size="sm"
										disabled={busy}
										onClick={() =>
											run(
												're-setup',
												() => privateNetworkApi.reSetup(server.id),
												'WireGuard re-setup completed',
											)
										}
										className="cursor-pointer gap-1.5 text-xs">
										<RefreshCw className="size-3.5" /> Re-setup
									</Button>
									<Button
										variant="outline"
										size="sm"
										disabled={busy}
										onClick={() =>
											run(
												'rotate',
												() => privateNetworkApi.rotateKeys(server.id),
												'WireGuard keys rotated',
											)
										}
										className="cursor-pointer gap-1.5 text-xs">
										<KeyRound className="size-3.5" /> Rotate keys
									</Button>
								</>
							)}
						</>
					)}
					<Button
						size="sm"
						onClick={() => save(mode === 'MANAGED_WIREGUARD')}
						disabled={busy}
						className="ml-auto cursor-pointer gap-1.5 text-xs">
						{busy ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : mode === 'MANAGED_WIREGUARD' ? (
							<Shield className="size-3.5" />
						) : (
							<Save className="size-3.5" />
						)}
						{mode === 'MANAGED_WIREGUARD' ? 'Save & Setup' : 'Save'}
					</Button>
				</div>
			</div>

			{/* ── Right rail: live facts + guide ────────────────────── */}
			<div className="flex flex-col gap-4 lg:sticky lg:top-6">
				{/* Connection facts */}
				<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
					<div className="mb-2 flex items-center gap-2">
						<Network
							className="size-4 text-muted-foreground"
							strokeWidth={2}
						/>
						<h2 className="text-sm font-semibold text-foreground">
							Connection
						</h2>
					</div>
					<div className="divide-y divide-border/40">
						<FactRow
							icon={Shield}
							label="Mode"
							value={
								<span className="flex items-center gap-2">
									<span
										className={`size-2 shrink-0 rounded-full ${config ? statusDot(config.status) : 'bg-muted-foreground/40'}`}
									/>
									{MODE_LABELS[effectiveMode]}
								</span>
							}
							mono={false}
						/>
						<FactRow
							icon={Activity}
							label="Status"
							value={
								config ? (
									<span>{config.status}</span>
								) : (
									<span className="text-muted-foreground">
										{loading ? 'Loading…' : 'Not configured'}
									</span>
								)
							}
							mono={false}
						/>
						<FactRow
							icon={Globe}
							label="Public IP"
							value={serverHostLabel(server.ip_address)}
						/>
						<FactRow
							icon={Network}
							label="Private host"
							value={
								config?.private_host || (
									<span className="text-muted-foreground">—</span>
								)
							}
						/>
						{configured && config?.endpoint && (
							<FactRow
								icon={Activity}
								label="Endpoint"
								value={config.endpoint}
							/>
						)}
						{configured && config?.dns_name && (
							<FactRow
								icon={Globe}
								label="DNS name"
								value={config.dns_name}
							/>
						)}
						{configured && config && config.routes.length > 0 && (
							<FactRow
								icon={Network}
								label="Routes"
								value={config.routes.join(', ')}
							/>
						)}
					</div>
				</div>

				{/* Guide */}
				<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
					<div className="mb-3 flex items-center gap-2">
						<Shield
							className="size-4 text-muted-foreground"
							strokeWidth={2}
						/>
						<h2 className="text-sm font-semibold text-foreground">
							How it works
						</h2>
					</div>
					<ul className="space-y-2.5 text-xs leading-relaxed text-muted-foreground">
						<li className="flex gap-2">
							<Globe className="mt-0.5 size-3.5 shrink-0" />
							<span>
								<strong className="font-medium text-foreground">
									Direct SSH
								</strong>{' '}
								uses the public IP — simplest, but every deploy travels
								over the open internet.
							</span>
						</li>
						<li className="flex gap-2">
							<Shield className="mt-0.5 size-3.5 shrink-0" />
							<span>
								<strong className="font-medium text-foreground">
									Managed WireGuard
								</strong>{' '}
								installs a point-to-point tunnel; panel and host share one
								/24. Traffic stays encrypted.
							</span>
						</li>
						<li className="flex gap-2">
							<Network className="mt-0.5 size-3.5 shrink-0" />
							<span>
								<strong className="font-medium text-foreground">
									Existing VPN
								</strong>{' '}
								keeps your Tailscale/NetBird mesh — give the private
								address and OpenOxide routes through it.
							</span>
						</li>
					</ul>
				</div>
			</div>
		</div>
	);
}

function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<div className="grid gap-1.5">
			<label className="text-xs font-medium">{label}</label>
			{children}
		</div>
	);
}
