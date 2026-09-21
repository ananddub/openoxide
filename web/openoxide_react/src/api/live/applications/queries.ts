import {useActiveOrgId, useConnectQuery} from '../query';
import type {OverviewServiceItemPayload} from '../overview';
import {applicationClient, overviewClient} from './shared';
import {
	toApplicationPayload,
	type ApplicationItemPayload,
	type ApplicationPayload,
} from './types';

/** Single application (legacy live: useApplicationGet). */
export function useApplicationGet(id: bigint) {
	return useConnectQuery<ApplicationPayload>(
		['applications', 'get', id.toString()],
		async () =>
			toApplicationPayload(await applicationClient.getApplication({id})),
		{enabled: id > 0n},
	);
}

/** Applications for an environment (legacy live: useApplicationListByEnvironment). */
export function useApplicationListByEnvironment(environmentId: bigint) {
	return useConnectQuery<ApplicationPayload[]>(
		['applications', 'list', environmentId.toString()],
		async () => {
			const response = await applicationClient.listApplications({
				environmentId,
			});
			return response.applications.map(toApplicationPayload);
		},
		{enabled: environmentId > 0n},
	);
}

/**
 * Organization-wide application list (legacy app-store `applications`
 * slice). There is no org-wide Application RPC, so this derives from the
 * overview services stream — the same source the legacy realtime sync fed
 * the store from — and is keyed under the `applications` prefix so
 * SERVICE_APPLICATION table changes invalidate it.
 */
export function useApplicationList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<ApplicationItemPayload[]>(
		['applications', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.items
				.map(
					item => JSON.parse(item.payload) as OverviewServiceItemPayload,
				)
				.filter(service => service.service_type === 'APP')
				.map(
					service =>
						({
							id: Number(service.id),
							name: service.name,
							app_name: service.name,
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
						}) satisfies ApplicationItemPayload,
				);
		},
	);
}
