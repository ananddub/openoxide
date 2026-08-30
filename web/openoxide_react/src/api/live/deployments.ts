import {createServiceClient, useConnectQuery} from './query';
import {DeploymentService, type Deployment} from '#/gen/connect/openoxide/api/v1/deployment_pb';

export type DeploymentPayload = {
	id: bigint;
	resource_type: string;
	resource_id: bigint;
	status: string;
	error?: string;
	logs?: string;
	created_at: bigint;
	updated_at: bigint;
};

const deploymentClient = createServiceClient(DeploymentService);

function toDeploymentPayload(deployment: Deployment): DeploymentPayload {
	return {
		id: deployment.id,
		resource_type: deployment.resourceType,
		resource_id: deployment.resourceId,
		status: deployment.status,
		error: deployment.error,
		logs: deployment.logs,
		created_at: deployment.createdAt,
		updated_at: deployment.updatedAt,
	};
}

/** Recent deployments across resources (legacy live: useDeploymentList). */
export function useDeploymentList() {
	return useConnectQuery<DeploymentPayload[]>(
		['deployments', 'list'],
		async () => {
			const response = await deploymentClient.listDeployments({});
			return response.deployments.map(toDeploymentPayload);
		},
	);
}

/** Currently running deployments (legacy live: useDeploymentRunning). */
export function useDeploymentRunning() {
	return useConnectQuery<DeploymentPayload[]>(
		['deployments', 'running'],
		async () => {
			const response = await deploymentClient.listRunningDeployments({});
			return response.deployments.map(toDeploymentPayload);
		},
	);
}
