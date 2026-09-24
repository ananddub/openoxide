import {createServiceClient} from '#/api/live/query';
import {
	ServerConnectionMode,
	ServerPrivateNetworkHealthStatus,
	ServerPrivateNetworkProvider,
	ServerPrivateNetworkService,
	ServerPrivateNetworkStatus,
	type ServerPrivateNetwork,
} from '#/gen/connect/openoxide/api/v1/server_private_network_pb';

import type {
	ConnectionMode,
	PrivateNetworkConfig,
	PrivateNetworkProvider,
	UpdatePrivateNetworkInput,
} from './types';

const networkClient = createServiceClient(ServerPrivateNetworkService);

const modeByName = ServerConnectionMode as unknown as Record<
	string,
	ServerConnectionMode
>;
const providerByName = ServerPrivateNetworkProvider as unknown as Record<
	string,
	ServerPrivateNetworkProvider
>;

/** Maps the typed `ServerPrivateNetwork` message to the snake_case config DTO the modal consumes; enums become their name strings and undefined optionals become null. */
function toConfig(
	network: ServerPrivateNetwork | undefined,
): PrivateNetworkConfig | null {
	if (!network) return null;
	return {
		server_id: Number(network.serverId),
		connection_mode: ServerConnectionMode[
			network.connectionMode
		] as unknown as ConnectionMode,
		provider:
			network.provider !== undefined
				? (ServerPrivateNetworkProvider[
						network.provider
					] as unknown as PrivateNetworkProvider)
				: null,
		private_host: network.privateHost ?? null,
		tunnel_address: network.tunnelAddress ?? null,
		public_key: network.publicKey ?? null,
		endpoint: network.endpoint ?? null,
		listen_port: network.listenPort ?? null,
		persistent_keepalive: network.persistentKeepalive ?? null,
		status: ServerPrivateNetworkStatus[
			network.status
		] as unknown as PrivateNetworkConfig['status'],
		dns_name: network.dnsName ?? null,
		routes: network.routes,
		health_status: ServerPrivateNetworkHealthStatus[
			network.healthStatus
		] as unknown as PrivateNetworkConfig['health_status'],
		health_error: network.healthError ?? null,
	};
}

/** Private-network operations over ConnectRPC. Requests/responses are fully typed messages; this module maps them to the snake_case config shape the modal consumes. */
export const privateNetworkApi = {
	get: async (serverId: number): Promise<PrivateNetworkConfig | null> => {
		const response = await networkClient.getNetwork({
			id: BigInt(serverId),
		});
		return toConfig(response.network);
	},
	update: async (
		serverId: number,
		input: UpdatePrivateNetworkInput,
	): Promise<PrivateNetworkConfig> => {
		const response = await networkClient.updateNetwork({
			id: BigInt(serverId),
			connectionMode: modeByName[input.connection_mode],
			provider: input.provider
				? providerByName[input.provider]
				: undefined,
			privateHost: input.private_host ?? undefined,
			tunnelAddress: input.tunnel_address ?? undefined,
			publicKey: input.public_key ?? undefined,
			endpoint: input.endpoint ?? undefined,
			listenPort: input.listen_port ?? undefined,
			persistentKeepalive: input.persistent_keepalive ?? undefined,
			dnsName: input.dns_name ?? undefined,
			routes: input.routes,
		});
		return toConfig(response.network) as PrivateNetworkConfig;
	},
	setup: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.setup({id: BigInt(serverId)});
		return toConfig(result.network) as PrivateNetworkConfig;
	},
	health: async (
		serverId: number,
	): Promise<{status: string; error: string | null}> => {
		const result = await networkClient.health({id: BigInt(serverId)});
		return {
			status: result.health
				? (ServerPrivateNetworkHealthStatus[result.health.status] ??
					'UNKNOWN')
				: 'UNKNOWN',
			error: result.health?.error ?? null,
		};
	},
	repair: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.repair({id: BigInt(serverId)});
		return toConfig(result.network) as PrivateNetworkConfig;
	},
	reSetup: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.reSetup({id: BigInt(serverId)});
		return toConfig(result.network) as PrivateNetworkConfig;
	},
	rotateKeys: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.rotateKeys({id: BigInt(serverId)});
		return toConfig(result.network) as PrivateNetworkConfig;
	},
	teardown: async (serverId: number): Promise<void> => {
		await networkClient.teardown({id: BigInt(serverId)});
	},
};
