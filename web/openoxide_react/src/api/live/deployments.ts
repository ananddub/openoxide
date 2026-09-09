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
	// Legacy consumers filter on flat resource ids; the proto carries them
	// as resourceType ("Deploying Database"/"Database" etc.) + resourceId.
	// Map back so database_id/application_id/compose_id filters keep working.
	const rid = Number(deployment.resourceId);
	const type = (deployment.resourceType || '').toLowerCase();
	return {
		id: deployment.id,
		resource_type: deployment.resourceType,
		resource_id: deployment.resourceId,
		database_id: type.includes('database') ? rid : null,
		application_id: type.includes('application') ? rid : null,
		compose_id: type.includes('compose') ? rid : null,
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

/**
 * Live deployment list under the table-watch-managed `deployments` key
 * prefix (DEPLOYMENTS table → refetch while the page is mounted). The
 * query cache replaces the former zustand `deployments` slice; payload
 * shape is the legacy snake_case one (id, application_id/compose_id/
 * database_id, status, created_at, …) so consumers keep working
 * unchanged.
 */
export type DeploymentItemPayload = {
	id: number;
	status: string;
	application_id?: number | null;
	compose_id?: number | null;
	database_id?: number | null;
	finished_at?: number | null;
	state?: string;
	last_state_at?: number | null;
	error_message?: string | null;
	log_content?: string | null;
	title?: string;
	description?: string | null;
	created_at?: number | null;
	[key: string]: unknown;
};

export function useDeploymentItems() {
	return useConnectQuery<DeploymentItemPayload[]>(
		['deployments'],
		async () => {
			const response = await deploymentClient.listDeployments({});
			return response.deployments.map(
				(deployment) =>
					JSON.parse(
						JSON.stringify(toDeploymentPayload(deployment)),
					) as DeploymentItemPayload,
			);
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
	return useConnectMutation(
		(id: number | bigint) =>
			deploymentClient.deleteDeployment({id: BigInt(id)}),
		{invalidate: [['deployments']]},
	);
}

/** Delete all finished deployments of an application (legacy REST: DELETE /deployments/app/{id}). */
export function useDeploymentDeleteByApplication() {
	return useConnectMutation(
		(id: number | bigint) =>
			deploymentClient.deleteApplicationDeployments({id: BigInt(id)}),
		{invalidate: [['deployments']]},
	);
}

/** Delete all finished deployments of a compose (legacy REST: DELETE /deployments/compose/{id}). */
export function useDeploymentDeleteByCompose() {
	return useConnectMutation(
		(id: number | bigint) =>
			deploymentClient.deleteComposeDeployments({id: BigInt(id)}),
		{invalidate: [['deployments']]},
	);
}

/** Delete all finished deployments of a database (legacy REST: DELETE /deployments/database/{id}). */
export function useDeploymentDeleteByDatabase() {
	return useConnectMutation(
		(id: number | bigint) =>
			deploymentClient.deleteDatabaseDeployments({id: BigInt(id)}),
		{invalidate: [['deployments']]},
	);
}

/** Cancel a running deployment by id (legacy REST: POST /deployments/{id}/cancel). */
export function useDeploymentCancel() {
	return useConnectMutation(
		(id: number | bigint) =>
			deploymentClient.cancelDeployment({id: BigInt(id)}),
		{invalidate: [['deployments']]},
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
 * /deployments/docker/containers?server_id=...). ListContainers has no
 * server filter on the wire; the serverId only keys the query cache so
 * callers can switch contexts (local server for any null/undefined id).
 */
export function useDeploymentListContainers(serverId?: number | bigint | null) {
	return useConnectQuery<DockerContainerPayload[]>(
		['deployments', 'docker-containers', String(serverId ?? 'local')],
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
