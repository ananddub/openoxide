import {useConnectMutation} from '../query';
import {composeClient, idMutation} from './shared';

const DEPLOY_INVALIDATE = [['composes'], ['deployments']];
const DEPLOYMENTS_ONLY = [['deployments']];

/** Deploy a compose by id (legacy REST: POST /composes/{id}/deploy). */
export const useComposeDeploy = idMutation(
	id => composeClient.deploy({id}),
	DEPLOY_INVALIDATE,
);

/** Redeploy a compose by id. */
export const useComposeRedeploy = idMutation(
	id => composeClient.redeploy({id}),
	DEPLOY_INVALIDATE,
);

/** Reload a compose's configuration by id. */
export const useComposeReload = idMutation(id =>
	composeClient.reload({id}),
);

/** Start a stopped compose by id (legacy REST: POST /composes/{id}/start). */
export const useComposeStart = idMutation(id => composeClient.start({id}));

/** Stop a running compose by id (legacy REST: POST /composes/{id}/stop). */
export const useComposeStop = idMutation(id => composeClient.stop({id}));

/** Cancel the running deployment of a compose by id (legacy REST: POST /compose/{id}/cancel). */
export const useComposeCancel = idMutation(
	id => composeClient.cancel({id}),
	DEPLOY_INVALIDATE,
);

/** Rotate the compose project's webhook token; returns the new token. */
export function useComposeRotateWebhook() {
	return useConnectMutation(
		async (id: number | bigint) => {
			const response = await composeClient.rotateWebhookToken({
				id: BigInt(id),
			});
			return response.token;
		},
		{invalidate: [['composes']]},
	);
}

/** Clear this compose project's deployment history. */
export const useComposeClearHistory = idMutation(
	id => composeClient.clearDeploymentHistory({id}),
	DEPLOYMENTS_ONLY,
);

/** Drop this compose project's pending queue entries. */
export const useComposeCleanupQueue = idMutation(
	id => composeClient.cleanupDeploymentQueue({id}),
	DEPLOYMENTS_ONLY,
);

/** Force-kill the in-flight deployment of this compose project. */
export const useComposeForceKill = idMutation(
	id => composeClient.forceKillDeployment({id}),
	DEPLOYMENTS_ONLY,
);
