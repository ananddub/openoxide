import {useConnectMutation} from '../query';
import {applicationClient, idMutation} from './shared';

const DEPLOY_INVALIDATE = [['applications'], ['deployments']];

/** Deploy an application by id (legacy REST: POST /applications/{id}/deploy). */
export const useApplicationDeploy = idMutation(
	id => applicationClient.deploy({id}),
	DEPLOY_INVALIDATE,
);

/** Redeploy an application by id. */
export const useApplicationRedeploy = idMutation(
	id => applicationClient.redeploy({id}),
	DEPLOY_INVALIDATE,
);

/** Rebuild an application by id. */
export const useApplicationRebuild = idMutation(
	id => applicationClient.rebuild({id}),
	DEPLOY_INVALIDATE,
);

/** Reload an application's configuration by id. */
export const useApplicationReload = idMutation(id =>
	applicationClient.reload({id}),
);

/** Start a stopped application by id (legacy REST: POST /applications/{id}/start). */
export const useApplicationStart = idMutation(id =>
	applicationClient.start({id}),
);

/** Stop a running application by id (legacy REST: POST /applications/{id}/stop). */
export const useApplicationStop = idMutation(id =>
	applicationClient.stop({id}),
);

/** Cancel the running deployment of an application by id. */
export const useApplicationCancel = idMutation(
	id => applicationClient.cancel({id}),
	DEPLOY_INVALIDATE,
);

/** Clear this application's deployment history. */
export const useApplicationClearHistory = idMutation(
	id => applicationClient.clearDeploymentHistory({id}),
	[['deployments']],
);

/** Drop this application's pending queue entries. */
export const useApplicationCleanupQueue = idMutation(
	id => applicationClient.cleanupDeploymentQueue({id}),
	[['deployments']],
);

/** Force-kill the in-flight deployment of this application. */
export const useApplicationForceKill = idMutation(
	id => applicationClient.forceKillDeployment({id}),
	[['deployments']],
);

/** Rotate the application's webhook/refresh token; returns the new token. */
export function useApplicationRotateWebhook() {
	return useConnectMutation(
		async (id: number | bigint) => {
			const response = await applicationClient.rotateWebhookToken({
				id: BigInt(id),
			});
			return response.token;
		},
		{invalidate: [['applications']]},
	);
}
