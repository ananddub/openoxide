import type {ConnectionMode} from './types';

interface PrivateNetworkForm {
	mode: ConnectionMode;
	managedHost: string | null;
	endpoint: string;
	listenPort: string;
	privateHost: string;
}

export function validatePrivateNetworkForm(
	form: PrivateNetworkForm,
): string | null {
	if (form.mode === 'MANAGED_WIREGUARD' && !form.managedHost) {
		return 'Use a valid /24 IPv4 tunnel network, for example 10.77.2.0/24';
	}
	if (form.mode === 'MANAGED_WIREGUARD') {
		const listenPort = Number(form.listenPort);
		if (
			!Number.isInteger(listenPort) ||
			listenPort < 1 ||
			listenPort > 65535
		) {
			return 'Remote WireGuard UDP listen port must be between 1 and 65535';
		}
		if (form.endpoint.trim() && form.endpoint.trim().includes(':')) {
			const endpointPort = Number(form.endpoint.trim().split(':').at(-1));
			if (Number.isInteger(endpointPort) && endpointPort !== listenPort) {
				return 'Remote endpoint port must match the remote WireGuard UDP listen port';
			}
		}
	}
	if (
		form.mode === 'EXTERNAL_PRIVATE_NETWORK' &&
		!form.privateHost.trim()
	) {
		return 'Private IP or hostname is required for an existing VPN';
	}
	return null;
}
