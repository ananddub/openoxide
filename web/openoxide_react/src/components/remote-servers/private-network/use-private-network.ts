import {useEffect, useMemo, useState} from 'react';
import {toast} from 'sonner';

import {formatApiError} from '#/api/utils';
import type {RemoteServerResponse} from '#/types/api-helpers';

import {privateNetworkApi} from './api';
import type {
	ConnectionMode,
	PrivateNetworkConfig,
	PrivateNetworkProvider,
	UpdatePrivateNetworkInput,
} from './types';
import {validatePrivateNetworkForm} from './validation';
import {remoteHostFromTunnel} from './panel-parts';

/** All state + server operations behind the private-network panel. */
export function usePrivateNetwork(server: RemoteServerResponse) {
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

	const save = async () => {
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
		const setup = mode === 'MANAGED_WIREGUARD';
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

	return {
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
		busy: loading || !!action,
		effectiveMode: config?.connection_mode ?? mode,
		run,
		save,
		refresh,
	};
}
