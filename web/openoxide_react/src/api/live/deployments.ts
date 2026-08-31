import {createServiceClient, useConnectMutation, useConnectQuery} from './query';
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

/** Delete a single deployment by id (legacy REST: DELETE /deployments/{id}). */
export function useDeploymentDelete() {
	return useConnectMutation((id: number | bigint) =>
		deploymentClient.deleteDeployment({id: BigInt(id)}),
	);
}

/** Delete all finished deployments of an application (legacy REST: DELETE /deployments/app/{id}). */
export function useDeploymentDeleteByApplication() {
	return useConnectMutation((id: number | bigint) =>
		deploymentClient.deleteApplicationDeployments({id: BigInt(id)}),
	);
}

/** Delete all finished deployments of a compose (legacy REST: DELETE /deployments/compose/{id}). */
export function useDeploymentDeleteByCompose() {
	return useConnectMutation((id: number | bigint) =>
		deploymentClient.deleteComposeDeployments({id: BigInt(id)}),
	);
}

/** Delete all finished deployments of a database (legacy REST: DELETE /deployments/database/{id}). */
export function useDeploymentDeleteByDatabase() {
	return useConnectMutation((id: number | bigint) =>
		deploymentClient.deleteDatabaseDeployments({id: BigInt(id)}),
	);
}

/** Cancel a running deployment by id (legacy REST: POST /deployments/{id}/cancel). */
export function useDeploymentCancel() {
	return useConnectMutation((id: number | bigint) =>
		deploymentClient.cancelDeployment({id: BigInt(id)}),
	);
}

/** Docker container snapshot (legacy REST: GET /deployments/docker/containers). */
export type DockerContainerPayload = {
	id: string;
	name: string;
	image: string;
	state: string;
	status?: string;
};

/**
 * Docker containers visible to the panel (legacy REST: GET
 * /deployments/docker/containers). ListContainers takes no server filter, so
 * unlike the REST endpoint this always lists the local panel server.
 */
export function useDeploymentListContainers() {
	return useConnectQuery<DockerContainerPayload[]>(
		['deployments', 'docker-containers'],
		async () => {
			const response = await deploymentClient.listContainers({});
			return response.containers.map((container) => ({
				id: container.id,
				name: container.name,
				image: container.image,
				state: container.state,
				status: container.status,
			}));
		},
	);
}
