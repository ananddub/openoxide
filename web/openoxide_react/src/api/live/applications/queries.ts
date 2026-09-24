import {useActiveOrgId, useConnectQuery} from '../query';
import {OverviewServiceType} from '#/gen/connect/openoxide/api/v1/overview_pb';
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

/** Organization-wide application list (legacy app-store `applications` slice). There is no org-wide Application RPC, so this derives from the overview services stream — the same source the legacy realtime sync fed the store from — and is keyed under the `applications` prefix so SERVICE_APPLICATION table changes invalidate it. */
export function useApplicationList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<ApplicationItemPayload[]>(
		['applications', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.services
				.filter(service => service.serviceType === OverviewServiceType.APP)
				.map(
					service =>
						({
							id: Number(service.id),
							name: service.name,
							app_name: service.name,
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
						}) satisfies ApplicationItemPayload,
				);
		},
	);
}
