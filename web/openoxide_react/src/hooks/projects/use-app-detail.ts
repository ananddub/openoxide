import {useState, useEffect, useMemo, useRef} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import type {ApplicationResponse} from '#/types/api-helpers';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {useAppStore, selectApplicationById} from '#/stores/app-store';
import {useApplicationGet} from 'virtual:openoxide-live';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');
	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);

	// 100% Pure Centralized Zustand Store Resolution
	const storeApp = useAppStore(state => selectApplicationById(state, appId));
	const storeDeployments = useAppStore(state => state.deployments);
	const {data: liveApp} = useApplicationGet(BigInt(appId));
	// Once the realtime store has the service, prefer it so live detail cache
	// cannot overwrite deployment lifecycle updates with an older snapshot.
	const rawApp = storeApp || liveApp;
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

	// Read raw arrays directly from Zustand store (Strict Reference Preservation for React 19 useSyncExternalStore)
	const storeDomains = useAppStore(state => state.domains);
	const storeSchedules = useAppStore(state => state.schedules);
	const storeBackups = useAppStore(state => state.backups);

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

	const deployMutation = $api.useMutation(
		'post',
		'/applications/{id}/deploy',
	);
	const reloadMutation = $api.useMutation(
		'post',
		'/applications/{id}/reload',
	);
	const rebuildMutation = $api.useMutation(
		'post',
		'/applications/{id}/rebuild',
	);
	const startMutation = $api.useMutation(
		'post',
		'/applications/{id}/start',
	);
	const stopMutation = $api.useMutation('post', '/applications/{id}/stop');
	const cancelMutation = $api.useMutation(
		'post',
		'/applications/{id}/cancel',
	);
	const patchMutation = $api.useMutation('patch', '/applications/{id}');

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
		(useAppStore.getState() as any).updateServiceStatus?.(
			appId,
			intermediateStatus,
		);

		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Deployment triggered successfully');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application reloaded successfully');
			} else if (action === 'rebuild') {
				await rebuildMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Rebuild triggered successfully');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application started successfully');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Application stopped successfully');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: appId}}});
				toast.success('Action cancelled successfully');
			}
			refetchAll();
		} catch (err) {
			setLocalStatusOverride(null);
			actionInFlight.current = false;
			statusBeforeAction.current = null;
			toast.error(`Action failed: ${action}`);
		}
	};

	const handleUpdate = async (patchData: Partial<ApplicationResponse>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: appId}},
				body: patchData as any,
			});
			toast.success('Application settings updated');
			refetchAll();
		} catch (err) {
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
