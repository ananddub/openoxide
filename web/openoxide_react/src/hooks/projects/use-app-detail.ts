import {useState} from 'react';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {useOrganizations} from '#/stores/session-context';
import {
	useAppDetailData,
	useDeploymentStatus,
} from './use-app-detail.data';
import {useAppStatusOverride} from './use-app-detail.status';
import {useAppActions} from './use-app-detail.actions';

export function useAppDetail(appId: number) {
	const [activeTab, setActiveTab] = useState('General');
	const {activeOrg} = useOrganizations();
	const orgId = BigInt(activeOrg?.id || 1);

	const {rawApp, appStatus, domains, schedules, backups, deployments} =
		useAppDetailData(appId, orgId);
	const {
		localStatusOverride,
		setLocalStatusOverride,
		actionInFlight,
		statusBeforeAction,
		clearOverride,
	} = useAppStatusOverride(appStatus);

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

	const deploymentStatus = useDeploymentStatus(deployments);
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

	const {handleAction, handleUpdate} = useAppActions({
		appId,
		app,
		setLocalStatusOverride,
		actionInFlight,
		statusBeforeAction,
		clearOverride,
		refetchAll,
	});

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
