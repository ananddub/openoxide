export type ConnectionMode = 'DIRECT_SSH' | 'MANAGED_WIREGUARD' | 'EXTERNAL_PRIVATE_NETWORK';

export type PrivateNetworkProvider = 'WIREGUARD' | 'TAILSCALE' | 'ZEROTIER' | 'NETBIRD' | 'CUSTOM';

export interface PrivateNetworkConfig {
	server_id: number;
	connection_mode: ConnectionMode;
	provider: PrivateNetworkProvider | null;
	private_host: string | null;
	tunnel_address: string | null;
	public_key: string | null;
	endpoint: string | null;
	listen_port: number | null;
	persistent_keepalive: number | null;
	status: 'DISABLED' | 'CONFIGURING' | 'ACTIVE' | 'FAILED';
	dns_name: string | null;
	routes: string[];
	health_status: 'UNKNOWN' | 'HEALTHY' | 'DEGRADED' | 'UNREACHABLE' | 'CONFIG_DRIFT';
	health_error: string | null;
}

export interface UpdatePrivateNetworkInput {
	connection_mode: ConnectionMode;
	provider: PrivateNetworkProvider | null;
	private_host: string | null;
	tunnel_address: string | null;
	public_key: string | null;
	endpoint: string | null;
	listen_port: number | null;
	persistent_keepalive: number | null;
	dns_name: string | null;
	routes: string[];
}
