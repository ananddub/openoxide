import {
	useApplicationDeploy,
	useApplicationStart,
	useApplicationStop,
	useComposeDeploy,
	useComposeStart,
	useComposeStop,
	useDatabaseDeploy,
	useDatabaseStart,
	useDatabaseStop,
	parseDatabaseKind,
} from '#/api/live';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {OverviewServiceItem} from './overview-services-table';

type ServiceAction = 'deploy' | 'start' | 'stop';

/** Deploy/start/stop mutations for apps, compose stacks and databases.
 *  All three actions share one dispatch + toast flow. */
export function useServiceActions() {
	const app = {
		deploy: useApplicationDeploy(),
		start: useApplicationStart(),
		stop: useApplicationStop(),
	};
	const compose = {
		deploy: useComposeDeploy(),
		start: useComposeStart(),
		stop: useComposeStop(),
	};
	const db = {
		deploy: useDatabaseDeploy(),
		start: useDatabaseStart(),
		stop: useDatabaseStop(),
	};
	const tables = {
		deploy: {app, compose, db},
		start: {app, compose, db},
		stop: {app, compose, db},
	} as const;

	const run = async (action: ServiceAction, svc: OverviewServiceItem) => {
		try {
			const m = tables[action];
			if (svc.type === 'APP') {
				await m.app[action].mutateAsync(svc.id);
			} else if (svc.type === 'COMPOSE') {
				await m.compose[action].mutateAsync(svc.id);
			} else if (svc.dbKind) {
				await m.db[action].mutateAsync({
					kind: parseDatabaseKind(svc.dbKind),
					id: svc.id,
				});
			}
			if (action === 'deploy')
				toast.success(`Queued deployment for ${svc.name}`);
			else if (action === 'start')
				toast.success(`Starting ${svc.name}...`);
			else toast.info(`Stopping ${svc.name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return {
		handleDeploy: (svc: OverviewServiceItem) => run('deploy', svc),
		handleStart: (svc: OverviewServiceItem) => run('start', svc),
		handleStop: (svc: OverviewServiceItem) => run('stop', svc),
	};
}
