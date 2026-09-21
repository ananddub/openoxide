import {useActiveOrgId, useConnectQuery} from '../query';
import type {OverviewServiceItemPayload} from '../overview';
import {composeClient, overviewClient} from './shared';
import {
	toComposePayload,
	type ComposeItemPayload,
	type ComposePayload,
} from './types';

/**
 * Organization-wide compose list (legacy app-store `composes` slice).
 * There is no org-wide Compose RPC, so this derives from the overview
 * services stream and is keyed under the `composes` prefix so
 * SERVICE_COMPOSE table changes invalidate it.
 */
export function useComposeList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<ComposeItemPayload[]>(
		['composes', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.items
				.map(
					item => JSON.parse(item.payload) as OverviewServiceItemPayload,
				)
				.filter(service => service.service_type === 'COMPOSE')
				.map(
					service =>
						({
							id: Number(service.id),
							name: service.name,
							project_id: Number(service.project_id),
							environment_id: Number(service.environment_id),
							status: service.status,
							server_id:
								service.server_id === null
									? undefined
									: Number(service.server_id),
							server_name: service.server_name ?? undefined,
							created_at: Number(service.created_at),
							updated_at: Number(service.created_at),
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
