import {useEffect, useMemo, useState} from 'react';
import {
	Activity,
	KeyRound,
	Network,
	RefreshCw,
	Save,
	Shield,
	Wrench,
} from 'lucide-react';
import {toast} from 'sonner';

import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
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

interface PrivateNetworkModalProps {
	open: boolean;
	server: RemoteServerResponse | null;
	onClose: () => void;
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

export function PrivateNetworkModal({
	open,
	server,
	onClose,
}: PrivateNetworkModalProps) {
	const [mode, setMode] = useState<ConnectionMode>('DIRECT_SSH');
	const [provider, setProvider] =
		useState<PrivateNetworkProvider>('TAILSCALE');
	const [privateHost, setPrivateHost] = useState('');
	const [tunnelAddress, setTunnelAddress] = useState('10.77.1.0/24');
	const [endpoint, setEndpoint] = useState('');
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
		if (!open || !server) return;
		setMode('DIRECT_SSH');
		setProvider('TAILSCALE');
		setPrivateHost('');
		setEndpoint(`${server.ip_address}:51820`);
		setTunnelAddress(`10.77.${(server.id % 250) + 1}.0/24`);
		setListenPort('51820');
		setKeepalive('25');
		setDnsName('');
		setRoutes('');
		setConfig(null);
		setLoading(true);
		privateNetworkApi
			.get(server.id)
			.then(current => {
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
				setEndpoint(current.endpoint || `${server.ip_address}:51820`);
				setListenPort(String(current.listen_port || 51820));
				setKeepalive(String(current.persistent_keepalive || 25));
				setDnsName(current.dns_name || '');
				setRoutes(current.routes.join('\n'));
			})
			.catch(error => toast.error(formatApiError(error)))
			.finally(() => setLoading(false));
	}, [open, server]);

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
				? `${server?.ip_address}:${listenPort || '51820'}`
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
		if (!server) return;
		const validationError = validatePrivateNetworkForm({
			mode,
			managedHost,
			endpoint,
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
		if (!server) return;
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

	if (!server) return null;
	const busy = loading || !!action;

	return (
		<Dialog open={open} onOpenChange={next => !next && onClose()}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2">
						<Shield className="size-4 text-primary" /> Private Network:{' '}
						{server.name}
					</DialogTitle>
					<DialogDescription>
						Use managed WireGuard or connect through an existing private
						VPN.
					</DialogDescription>
				</DialogHeader>

				<div className="grid gap-4 py-2">
					<div className="grid gap-1.5">
						<label className="text-xs font-medium">Connection mode</label>
						<Select
							value={mode}
							onValueChange={value => setMode(value as ConnectionMode)}
							disabled={busy}>
							<SelectTrigger>
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="DIRECT_SSH">
									Direct SSH (no VPN)
								</SelectItem>
								<SelectItem value="MANAGED_WIREGUARD">
									Managed WireGuard
								</SelectItem>
								<SelectItem value="EXTERNAL_PRIVATE_NETWORK">
									Existing VPN / private network
								</SelectItem>
							</SelectContent>
						</Select>
					</div>

					{mode === 'MANAGED_WIREGUARD' && (
						<>
							<div className="rounded-lg border bg-muted/30 p-3 text-xs text-muted-foreground">
								OpenOxide installs WireGuard on the remote server using its configured IP ({server.ip_address}) and the UDP listen port below. No STUN server discovery or complex mesh VPN is used. Ensure the UDP port is allowed in your server's firewall.
							</div>
							<div className="grid gap-3 sm:grid-cols-2">
								<Field label="Remote server UDP listen port">
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
										onChange={event =>
											setTunnelAddress(event.target.value)
										}
										placeholder="10.77.2.0/24"
									/>
								</Field>
								<Field label="Remote tunnel IP">
									<Input
										value={managedHost || 'Invalid /24 network'}
										disabled
									/>
								</Field>
								<Field label="Persistent keepalive">
									<Input
										type="number"
										value={keepalive}
										onChange={event => setKeepalive(event.target.value)}
									/>
								</Field>
							</div>
						</>
					)}

					{mode === 'EXTERNAL_PRIVATE_NETWORK' && (
						<div className="grid gap-3 sm:grid-cols-2">
							<Field label="VPN provider">
								<Select
									value={provider}
									onValueChange={value =>
										setProvider(value as PrivateNetworkProvider)
									}
									disabled={busy}>
									<SelectTrigger>
										<SelectValue />
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
						</div>
					)}

					{mode !== 'DIRECT_SSH' && (
						<>
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
						</>
					)}

					{config && (
						<div className="flex flex-wrap items-center gap-2 rounded-lg border px-3 py-2 text-xs">
							<Network className="size-3.5" /> Status:{' '}
							<strong>{config.status}</strong>
							<span className="text-muted-foreground">
								Health: {config.health_status}
							</span>
							{config.health_error && (
								<span className="text-destructive">
									{config.health_error}
								</span>
							)}
						</div>
					)}
				</div>

				<DialogFooter className="flex-wrap">
					<Button
						variant="outline"
						size="sm"
						disabled={busy}
						onClick={refresh}>
						<RefreshCw
							className={action === 'refresh' ? 'animate-spin' : ''}
						/>{' '}
						Refresh
					</Button>
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
								}>
								<Activity /> Health
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
								}>
								<Wrench /> Repair
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
										}>
										<RefreshCw /> Re-setup
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
										}>
										<KeyRound /> Rotate keys
									</Button>
								</>
							)}
						</>
					)}
					<Button
						size="sm"
						onClick={() => save(mode === 'MANAGED_WIREGUARD')}
						disabled={busy}>
						{busy ? (
							<RefreshCw className="animate-spin" />
						) : mode === 'MANAGED_WIREGUARD' ? (
							<Shield />
						) : (
							<Save />
						)}
						{mode === 'MANAGED_WIREGUARD' ? 'Save & Setup' : 'Save'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
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
