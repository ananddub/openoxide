import {useMemo} from 'react';
import {
	useApplicationGet,
	useApplicationList,
	useOverviewServices,
	useDeploymentItems,
	useDomainItems,
	useScheduleListByOrganization,
	useBackupItems,
} from '#/api/live';
import {ACTIVE_DEPLOYMENT_STATUSES} from '../shared/resource-status';

// Live reads folded onto the active application; table-watch keeps them all fresh.
export function useAppDetailData(appId: number, orgId: bigint) {
	const applications = useApplicationList().data ?? [];
	const overviewServices = useOverviewServices().data ?? [];
	const storeApp = useMemo(
		() =>
			applications.find((a: any) => String(a.id) === String(appId)) ||
			overviewServices.find(
				(s: any) =>
					String(s.id) === String(appId) &&
					(s.service_type === 'APP' || s.type === 'application'),
			) ||
			null,
		[applications, overviewServices, appId],
	);

	const {data: storeDeployments = []} = useDeploymentItems();
	const {data: liveApp} = useApplicationGet(BigInt(appId));

	// Only liveApp carries source_type / docker_image / build_type; storeApp is fresher for status.
	const rawApp = liveApp ? {...liveApp, ...storeApp} : storeApp;
	const appStatus = (rawApp as any)?.app_status || (rawApp as any)?.status;

	const {data: storeDomains = []} = useDomainItems(orgId);
	const {data: storeSchedules = []} = useScheduleListByOrganization(orgId);
	const {data: storeBackups = []} = useBackupItems(orgId);

	const domains = useMemo(
		() =>
			(storeDomains || []).filter(
				(d: any) => Number(d.application_id) === Number(appId),
			),
		[storeDomains, appId],
	);
	const schedules = useMemo(
		() =>
			(storeSchedules || []).filter(
				(s: any) => Number(s.application_id) === Number(appId),
			),
		[storeSchedules, appId],
	);
	const backups = useMemo(
		() =>
			(storeBackups || []).filter(
				(b: any) => Number(b.application_id) === Number(appId),
			),
		[storeBackups, appId],
	);
	const deployments = useMemo(
		() =>
			(storeDeployments || []).filter(
				(d: any) => Number(d.application_id) === Number(appId),
			),
		[storeDeployments, appId],
	);

	return {rawApp, appStatus, domains, schedules, backups, deployments};
}

/** Latest in-flight deployment status (ignores finished rows). */
export function useDeploymentStatus(deployments: any[]) {
	return useMemo(() => {
		const active = deployments
			.filter((deployment: any) => {
				if (deployment.finished_at && Number(deployment.finished_at) > 0)
					return false;
				const status = String(
					deployment.status || deployment.state || '',
				).toUpperCase();
				return ACTIVE_DEPLOYMENT_STATUSES.includes(status);
			})
			.sort(
				(a: any, b: any) =>
					Number(b.created_at || 0) - Number(a.created_at || 0),
			);
		return active[0]
			? String(active[0].status || active[0].state).toUpperCase()
			: null;
	}, [deployments]);
}
