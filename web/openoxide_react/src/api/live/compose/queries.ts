import {useActiveOrgId, useConnectQuery} from '../query';
import {OverviewServiceType} from '#/gen/connect/openoxide/api/v1/overview_pb';
import {composeClient, overviewClient} from './shared';
import {
	toComposePayload,
	type ComposeItemPayload,
	type ComposePayload,
} from './types';

/** Organization-wide compose list (legacy app-store `composes` slice). There is no org-wide Compose RPC, so this derives from the overview services stream and is keyed under the `composes` prefix so SERVICE_COMPOSE table changes invalidate it. */
export function useComposeList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<ComposeItemPayload[]>(
		['composes', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.services
				.filter(
					service => service.serviceType === OverviewServiceType.COMPOSE,
				)
				.map(
					service =>
						({
							id: Number(service.id),
							name: service.name,
							project_id: Number(service.projectId),
							environment_id: Number(service.environmentId),
							status: service.status,
							server_id:
								service.serverId === undefined
									? undefined
									: Number(service.serverId),
							server_name: service.serverName,
							created_at: Number(service.createdAt),
							updated_at: Number(service.createdAt),
						}) satisfies ComposeItemPayload,
				);
		},
	);
}

/** Single compose (legacy live: useComposeGet). */
export function useComposeGet(id: bigint) {
	return useConnectQuery<ComposePayload>(
		['composes', 'get', id.toString()],
		async () => toComposePayload(await composeClient.getCompose({id})),
		{enabled: id > 0n},
	);
}

/** Composes for an environment (legacy live: useComposeListByEnvironment). */
export function useComposeListByEnvironment(environmentId: bigint) {
	return useConnectQuery<ComposePayload[]>(
		['composes', 'list', environmentId.toString()],
		async () => {
			const response = await composeClient.listComposes({environmentId});
			return response.composes.map(toComposePayload);
		},
		{enabled: environmentId > 0n},
	);
}
