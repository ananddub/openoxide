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

const ACTIVE_DEPLOYMENT_STATUSES = [
	'QUEUED',
	'PENDING',
	'BUILDING',
	'PREPARING',
	'STARTING',
	'DEPLOYING',
];

/**
 * Org-wide live reads folded onto the active application: the store/overview
 * lookup, the GetApplication query and the per-application domain / schedule
 * / backup / deployment slices. 0ms instant reads kept fresh by table-watch.
 */
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

	// Live deployments query; the table-watch stream (DEPLOYMENTS table)
	// invalidates ['deployments'] while mounted.
	const {data: storeDeployments = []} = useDeploymentItems();
	const {data: liveApp} = useApplicationGet(BigInt(appId));

	// Merge, don't prefer: storeApp (overview list) is fresher for lifecycle
	// status, but its payload carries no source_type / docker_image /
	// build_type — using it alone made the source card fall back to 'GIT'
	// after every refetch. liveApp (GetApplication) owns those fields; the
	// store copy layers on top so only its shared keys (status etc.) win.
	const rawApp = liveApp ? {...liveApp, ...storeApp} : storeApp;
	const appStatus = (rawApp as any)?.app_status || (rawApp as any)?.status;

	// Live queries; the table-watch stream (domains / schedules / backups
	// tables) invalidates them while mounted, so no zustand slices involved.
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
