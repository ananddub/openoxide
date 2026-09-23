import {useState, useMemo, useEffect} from 'react';
import {useUrlTab} from '#/hooks/use-url-tab';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	useComposeGet,
	useComposeList,
	useDeploymentItems,
	useDomainItems,
	useOverviewServices,
	useScheduleListByOrganization,
	useBackupItems,
} from '#/api/live';
import {useOrganizations} from '#/stores/session-context';
import {useComposeActions} from './use-compose-detail.actions';

export function useComposeDetail(composeId: number) {
	const [activeTab, setActiveTab] = useUrlTab('General');
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);
	const {activeOrg} = useOrganizations();
	const orgId = BigInt(activeOrg?.id || 1);

	// Org-wide compose lookup as live queries (table-watch keeps them
	// fresh) — same fallback chain the legacy store selector used.
	const composes = useComposeList().data ?? [];
	const overviewServices = useOverviewServices().data ?? [];
	const storeCompose = useMemo(
		() =>
			composes.find((c: any) => String(c.id) === String(composeId)) ||
			overviewServices.find(
				(s: any) =>
					String(s.id) === String(composeId) &&
					(s.service_type === 'COMPOSE' || s.type === 'compose'),
			) ||
			null,
		[composes, overviewServices, composeId],
	);
	const {data: liveCompose} = useComposeGet(BigInt(composeId));
	const rawCompose = liveCompose || storeCompose;

	const compose = rawCompose
		? {
				...rawCompose,
				status:
					localStatusOverride ||
					(rawCompose as any).compose_status ||
					(rawCompose as any).status ||
					'STOPPED',
				compose_status:
					localStatusOverride ||
					(rawCompose as any).compose_status ||
					(rawCompose as any).status ||
					'STOPPED',
			}
		: null;

	const composeStatus =
		(rawCompose as any)?.compose_status || (rawCompose as any)?.status;

	// Auto-clear localStatusOverride when Zustand status updates (using primitive string to prevent infinite loop)
	useEffect(() => {
		if (localStatusOverride && composeStatus) {
			const fetchedStatus = String(composeStatus).toUpperCase();
			const overrideUpper = localStatusOverride.toUpperCase();
			if (
				fetchedStatus === overrideUpper ||
				(overrideUpper === 'STARTING' &&
					(fetchedStatus === 'RUNNING' || fetchedStatus === 'HEALTHY')) ||
				(overrideUpper === 'STOPPING' &&
					(fetchedStatus === 'STOPPED' ||
						fetchedStatus === 'IDLE' ||
						fetchedStatus === 'CANCELLED')) ||
				(overrideUpper === 'CANCELLING' &&
					(fetchedStatus === 'CANCELLED' ||
						fetchedStatus === 'STOPPED' ||
						fetchedStatus === 'IDLE'))
			) {
				setLocalStatusOverride(null);
			}
		}
	}, [composeStatus, localStatusOverride]);

	// Live queries; the table-watch stream (domains / schedules / backups
	// tables) invalidates them while mounted, so no zustand slices involved.
	const {data: storeDomains = []} = useDomainItems(orgId);
	const {data: storeSchedules = []} = useScheduleListByOrganization(orgId);
	const {data: storeBackups = []} = useBackupItems(orgId);
	// Live deployments query; the table-watch stream (DEPLOYMENTS table)
	// invalidates ['deployments'] while mounted.
	const {data: storeDeployments = []} = useDeploymentItems();

	const domains = useMemo(
		() =>
			(storeDomains || []).filter(
				(d: any) => Number(d.compose_id) === Number(composeId),
			),
		[storeDomains, composeId],
	);
	const schedules = useMemo(
		() =>
			(storeSchedules || []).filter(
				(s: any) => Number(s.compose_id) === Number(composeId),
			),
		[storeSchedules, composeId],
	);
	const backups = useMemo(
		() =>
			(storeBackups || []).filter(
				(b: any) => Number(b.compose_id) === Number(composeId),
			),
		[storeBackups, composeId],
	);
	const deployments = useMemo(
		() =>
			(storeDeployments || []).filter(
				(d: any) => Number(d.compose_id) === Number(composeId),
			),
		[storeDeployments, composeId],
	);
	const isLoadingDomains = false;
	const isLoadingSchedules = false;
	const isLoadingBackups = false;
	const isLoadingDeployments = false;

	// 6. Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(
		composeId,
		'compose',
		activeTab === 'Monitoring',
	);

	// Live hooks auto-push updates — only trigger monitoring refresh
	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const {handleAction, handleUpdate} = useComposeActions({
		composeId,
		compose,
		setLocalStatusOverride,
		refetchAll,
	});

	return {
		compose,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: !compose,
		isLoadingDomains,
		isLoadingSchedules,
		isLoadingBackups,
		isLoadingDeployments,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
	};
}
