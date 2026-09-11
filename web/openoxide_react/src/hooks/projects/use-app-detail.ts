import {useState, useEffect, useMemo, useRef} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {useApplicationCancel, useApplicationDeploy, useApplicationGet, useApplicationList, useApplicationRebuild, useApplicationReload, useApplicationStart, useApplicationStop, useApplicationUpdate, useDeploymentItems, useDomainItems, useOverviewServices, useScheduleListByOrganization, useBackupItems} from '#/api/live';
import {useOrganizations} from '#/stores/session-context';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);
	const {activeOrg} = useOrganizations();
	const orgId = BigInt(activeOrg?.id || 1);

	// Org-wide application lookup as live queries (table-watch keeps them
	// fresh) — same fallback chain the legacy store selector used.
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
	const rawApp = liveApp ? { ...liveApp, ...(storeApp ?? {}) } : storeApp;
	const actionInFlight = useRef(false);
	const statusBeforeAction = useRef<string | null>(null);

	const app = rawApp
		? {
				...rawApp,
				status:
					localStatusOverride ||
					(rawApp as any).app_status ||
					(rawApp as any).status ||
					'STOPPED',
				app_status:
					localStatusOverride ||
					(rawApp as any).app_status ||
					(rawApp as any).status ||
					'STOPPED',
			}
		: null;

	const appStatus = (rawApp as any)?.app_status || (rawApp as any)?.status;

	// Auto-clear localStatusOverride when Zustand status updates (using primitive string to prevent infinite loop)
	useEffect(() => {
		if (localStatusOverride && appStatus) {
			const fetchedStatus = String(appStatus).toUpperCase();
			const overrideUpper = localStatusOverride.toUpperCase();
			const statusTransitioned =
				statusBeforeAction.current !== null &&
				fetchedStatus !== statusBeforeAction.current;
			if (
				statusTransitioned ||
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
				actionInFlight.current = false;
				statusBeforeAction.current = null;
			}
		}
	}, [appStatus, localStatusOverride]);

	useEffect(() => {
		if (!localStatusOverride) return;
		const timeout = window.setTimeout(() => {
			setLocalStatusOverride(null);
			actionInFlight.current = false;
			statusBeforeAction.current = null;
		}, 30_000);
		return () => window.clearTimeout(timeout);
	}, [localStatusOverride]);

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
	const deploymentStatus = useMemo(() => {
		const active = deployments
			.filter((deployment: any) => {
				if (deployment.finished_at && Number(deployment.finished_at) > 0)
					return false;
				const status = String(
					deployment.status || deployment.state || '',
				).toUpperCase();
				return [
					'QUEUED',
					'PENDING',
					'BUILDING',
					'PREPARING',
					'STARTING',
					'DEPLOYING',
				].includes(status);
			})
			.sort(
				(a: any, b: any) =>
					Number(b.created_at || 0) - Number(a.created_at || 0),
			);
		return active[0]
			? String(active[0].status || active[0].state).toUpperCase()
			: null;
	}, [deployments]);
	const displayApp = app
		? {
				...app,
				...(deploymentStatus
					? {app_status: deploymentStatus, status: deploymentStatus}
					: {}),
			}
		: app;

	// Central Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(
		appId,
		'application',
		activeTab === 'Monitoring',
	);

	// Live hooks auto-push updates — only trigger monitoring refresh
	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const deployMutation = useApplicationDeploy();
	const reloadMutation = useApplicationReload();
	const rebuildMutation = useApplicationRebuild();
	const startMutation = useApplicationStart();
	const stopMutation = useApplicationStop();
	const cancelMutation = useApplicationCancel();
	const patchMutation = useApplicationUpdate();

	const handleAction = async (
		action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'stop' | 'cancel',
	) => {
		if (actionInFlight.current && action !== 'cancel') return;
		actionInFlight.current = true;
		const currentSt = (app?.app_status || app?.status || '').toUpperCase();
		statusBeforeAction.current = currentSt;
		const isCurrentlyBuilding = [
			'QUEUED',
			'BUILDING',
			'STARTING',
			'PREPARING',
			'PENDING',
			'DEPLOYING',
		].includes(currentSt);
		const intermediateStatus =
			action === 'stop' || action === 'cancel'
				? isCurrentlyBuilding
					? 'CANCELLING'
					: 'STOPPING'
				: action === 'start'
					? 'STARTING'
					: 'DEPLOYING';
		setLocalStatusOverride(intermediateStatus);
		// Legacy updateServiceStatus store write removed — table-watch
		// invalidates ['applications'] when the action lands, refetching
		// fresh status through the live query.

		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync(appId);
				toast.success('Deployment triggered successfully');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync(appId);
				toast.success('Application reloaded successfully');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync(appId);
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				await startMutation.mutateAsync(appId);
				toast.success('Application started successfully');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync(appId);
				toast.success('Application stopped successfully');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync(appId);
				toast.success('Action cancelled successfully');
			}
			refetchAll();
		} catch (err) {
			setLocalStatusOverride(null);
			actionInFlight.current = false;
			statusBeforeAction.current = null;
			// Surface the RPC's own message — a generic "Action failed"
			// hid the builder's real error (e.g. a broken deployment
			// config query) and made failed deploys look like no-ops.
			toast.error(formatApiError(err));
		}
	};

	const handleUpdate = async (patchData: Partial<ApplicationResponse>) => {
		try {
			await patchMutation.mutateAsync({
				id: appId,
				...(patchData as any),
			});
			toast.success('Application settings updated');
			refetchAll();
		} catch {
			toast.error('Failed to update application settings');
		}
	};

	return {
		app: displayApp,
		domains,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading: !app,
		isLoadingDomains: false,
		isLoadingSchedules: false,
		isLoadingBackups: false,
		isLoadingDeployments: false,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
	};
}
