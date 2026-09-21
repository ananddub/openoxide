export type ActionType =
	| 'deploy'
	| 'reload'
	| 'rebuild'
	| 'start'
	| 'cancel'
	| 'stop';

export const ACTION_TITLES: Record<ActionType, string> = {
	deploy: 'Deploy Application',
	reload: 'Reload Application',
	rebuild: 'Rebuild Application',
	start: 'Start Application',
	stop: 'Stop Application',
	cancel: 'Cancel Application Build',
};

export const ACTION_DESCRIPTIONS: Record<ActionType, string> = {
	deploy:
		'Are you sure you want to deploy this application? This will pull latest code and trigger a build.',
	reload:
		'Are you sure you want to reload this application without rebuilding?',
	rebuild: 'Are you sure you want to rebuild this application?',
	start: 'Are you sure you want to start this application container?',
	stop: 'Are you sure you want to stop this running application container?',
	cancel: 'Are you sure you want to cancel the active deployment build?',
};

/** Deployment statuses that mean a build is still in flight. */
export const ACTIVE_DEPLOYMENT_STATUSES = [
	'QUEUED',
	'BUILDING',
	'STARTING',
	'DEPLOYING',
	'PREPARING',
	'PENDING',
];

/** App statuses that mean a stop/cancel request is being processed. */
export const isAppProcessing = (app: any): boolean => {
	const s = (app?.app_status || '').toUpperCase();
	return s === 'STOPPING' || s === 'CANCELLING';
};

/** Normalized lowercase application status from any field spelling. */
export const rawAppStatus = (app: any): string =>
	(
		app?.app_status ||
		app?.applicationStatus ||
		app?.application_status ||
		app?.status ||
		''
	).toLowerCase();
