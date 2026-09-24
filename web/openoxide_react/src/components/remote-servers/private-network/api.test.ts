import {describe, expect, it, vi} from 'vitest';

import {
	ServerConnectionMode,
	ServerPrivateNetworkHealthStatus,
	ServerPrivateNetworkProvider,
	ServerPrivateNetworkStatus,
} from '#/gen/connect/openoxide/api/v1/server_private_network_pb';

import type {UpdatePrivateNetworkInput} from './types';

const rpc = vi.hoisted(() => ({
	getNetwork: vi.fn(),
	updateNetwork: vi.fn(),
	setup: vi.fn(),
	health: vi.fn(),
	repair: vi.fn(),
	reSetup: vi.fn(),
	rotateKeys: vi.fn(),
	teardown: vi.fn(),
}));

vi.mock('#/api/live/query', () => ({
	createServiceClient: () => rpc,
}));

import {privateNetworkApi} from './api';

/** Minimal typed network message with sensible defaults. */
function networkMessage(
	overrides: Partial<{
		serverId: bigint;
		connectionMode: ServerConnectionMode;
		provider: ServerPrivateNetworkProvider | undefined;
		status: ServerPrivateNetworkStatus;
		healthStatus: ServerPrivateNetworkHealthStatus;
	}> = {},
) {
	return {
		serverId: overrides.serverId ?? 13n,
		connectionMode:
			overrides.connectionMode ?? ServerConnectionMode.DIRECT_SSH,
		provider: overrides.provider,
		status: overrides.status ?? ServerPrivateNetworkStatus.ACTIVE,
		healthStatus:
			overrides.healthStatus ?? ServerPrivateNetworkHealthStatus.UNKNOWN,
		routes: [],
		configVersion: 0n,
		consecutiveFailures: 0n,
		rotationState: 0,
		updatedAt: 0n,
	};
}

describe('private network ConnectRPC client', () => {
	it('maps the typed network message to the snake_case config', async () => {
		rpc.getNetwork.mockResolvedValueOnce({
			network: networkMessage({serverId: 13n, status: 3 /* ACTIVE */}),
		});

		await expect(privateNetworkApi.get(13)).resolves.toMatchObject({
			server_id: 13,
			status: 'ACTIVE',
			connection_mode: 'DIRECT_SSH',
		});
		expect(rpc.getNetwork).toHaveBeenCalledWith({id: 13n});
	});

	it('maps an absent network to null (no configuration)', async () => {
		rpc.getNetwork.mockResolvedValueOnce({network: undefined});
		await expect(privateNetworkApi.get(13)).resolves.toBeNull();
	});

	it('sends the update input as typed enum fields', async () => {
		rpc.updateNetwork.mockResolvedValueOnce({
			network: networkMessage({
				serverId: 7n,
				connectionMode: ServerConnectionMode.MANAGED_WIREGUARD,
				provider: ServerPrivateNetworkProvider.WIREGUARD,
				status: ServerPrivateNetworkStatus.CONFIGURING,
			}),
		});

		const input = {
			connection_mode: 'MANAGED_WIREGUARD',
			provider: 'WIREGUARD',
			private_host: null,
			tunnel_address: null,
			public_key: null,
			endpoint: null,
			listen_port: null,
			persistent_keepalive: null,
			dns_name: null,
			routes: [],
		} satisfies UpdatePrivateNetworkInput;
		const config = await privateNetworkApi.update(7, input);

		expect(rpc.updateNetwork).toHaveBeenCalledWith({
			id: 7n,
			connectionMode: ServerConnectionMode.MANAGED_WIREGUARD,
			provider: ServerPrivateNetworkProvider.WIREGUARD,
			routes: [],
		});
		expect(config.status).toBe('CONFIGURING');
		expect(config.provider).toBe('WIREGUARD');
	});

	it('reduces the health message to status and error', async () => {
		rpc.health.mockResolvedValueOnce({
			health: {
				status: ServerPrivateNetworkHealthStatus.HEALTHY,
				interfaceExists: true,
				privateSshReachable: true,
				checkedAt: 0n,
			},
		});

		await expect(privateNetworkApi.health(3)).resolves.toEqual({
			status: 'HEALTHY',
			error: null,
		});
	});

	it('teardown awaits the RPC and returns void', async () => {
		rpc.teardown.mockResolvedValueOnce({});
		await expect(privateNetworkApi.teardown(9)).resolves.toBeUndefined();
		expect(rpc.teardown).toHaveBeenCalledWith({id: 9n});
	});
});
