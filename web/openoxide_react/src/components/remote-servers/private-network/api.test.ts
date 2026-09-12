import {describe, expect, it, vi} from 'vitest';

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

describe('private network ConnectRPC client', () => {
	it('unwraps the config JSON payload from ResourceResponse', async () => {
		rpc.getNetwork.mockResolvedValueOnce({
			id: 13n,
			kind: 'server_private_network',
			payload: JSON.stringify({server_id: 13, status: 'ACTIVE'}),
		});

		await expect(privateNetworkApi.get(13)).resolves.toMatchObject({
			server_id: 13,
			status: 'ACTIVE',
		});
		expect(rpc.getNetwork).toHaveBeenCalledWith({id: 13n});
	});

	it('maps an empty payload to null (no network configured)', async () => {
		rpc.getNetwork.mockResolvedValueOnce({id: 13n, kind: '', payload: ''});
		await expect(privateNetworkApi.get(13)).resolves.toBeNull();
	});

	it('sends the update input as JSON payload', async () => {
		rpc.updateNetwork.mockResolvedValueOnce({
			id: 7n,
			kind: '',
			payload: JSON.stringify({server_id: 7, status: 'CONFIGURING'}),
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
			payload: JSON.stringify(input),
		});
		expect(config.status).toBe('CONFIGURING');
	});

	it('reduces the health operation payload to status and error', async () => {
		rpc.health.mockResolvedValueOnce({
			success: true,
			operation: 'health',
			payload: JSON.stringify({status: 'HEALTHY', error: null}),
		});

		await expect(privateNetworkApi.health(3)).resolves.toEqual({
			status: 'HEALTHY',
			error: null,
		});
	});

	it('teardown awaits the RPC and returns void', async () => {
		rpc.teardown.mockResolvedValueOnce({
			success: true,
			operation: 'teardown',
		});
		await expect(privateNetworkApi.teardown(9)).resolves.toBeUndefined();
		expect(rpc.teardown).toHaveBeenCalledWith({id: 9n});
	});
});
