import {createServiceClient} from '#/api/live/query';
import {ServerPrivateNetworkService} from '#/gen/connect/openoxide/api/v1/server_private_network_pb';

import type {
	PrivateNetworkConfig,
	UpdatePrivateNetworkInput,
} from './types';

const networkClient = createServiceClient(ServerPrivateNetworkService);

function parseConfig(payload: string): PrivateNetworkConfig | null {
	if (!payload) return null;
	try {
		return JSON.parse(payload) as PrivateNetworkConfig;
	} catch {
		return null;
	}
}

/**
 * Private-network operations over ConnectRPC. The panel handlers carry the
 * config DTO as a JSON string in `ResourceResponse.payload` /
 * `OperationResult.payload`; this module unwraps it back to the typed shape
 * the modal consumes.
 */
export const privateNetworkApi = {
	get: async (serverId: number): Promise<PrivateNetworkConfig | null> => {
		const response = await networkClient.getNetwork({id: BigInt(serverId)});
		return parseConfig(response.payload);
	},
	update: async (
		serverId: number,
		input: UpdatePrivateNetworkInput,
	): Promise<PrivateNetworkConfig> => {
		const response = await networkClient.updateNetwork({
			id: BigInt(serverId),
			payload: JSON.stringify(input),
		});
		return parseConfig(response.payload) as PrivateNetworkConfig;
	},
	setup: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.setup({id: BigInt(serverId)});
		return parseConfig(result.payload ?? '') as PrivateNetworkConfig;
	},
	health: async (
		serverId: number,
	): Promise<{status: string; error: string | null}> => {
		const result = await networkClient.health({id: BigInt(serverId)});
		const parsed = result.payload ? JSON.parse(result.payload) : null;
		return {
			status: String(parsed?.status ?? 'UNKNOWN'),
			error: parsed?.error ?? null,
		};
	},
	repair: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.repair({id: BigInt(serverId)});
		return parseConfig(result.payload ?? '') as PrivateNetworkConfig;
	},
	reSetup: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.reSetup({id: BigInt(serverId)});
		return parseConfig(result.payload ?? '') as PrivateNetworkConfig;
	},
	rotateKeys: async (serverId: number): Promise<PrivateNetworkConfig> => {
		const result = await networkClient.rotateKeys({id: BigInt(serverId)});
		return parseConfig(result.payload ?? '') as PrivateNetworkConfig;
	},
	teardown: async (serverId: number): Promise<void> => {
		await networkClient.teardown({id: BigInt(serverId)});
	},
};
