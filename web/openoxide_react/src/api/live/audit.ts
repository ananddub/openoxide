import {createServiceClient, useConnectMutation, useResourceListQuery, useActiveOrgId} from './query';
import {AuditService} from '#/gen/connect/openoxide/api/v1/audit_pb';

/**
 * Audit log rows as emitted by the panel's audit writers (`audit_logs`
 * table). `action` is an UPPER_SNAKE verb (`PERMISSION_GROUP_CREATED`),
 * `metadata` is a JSON string with resource details.
 */
export type AuditLogPayload = {
	id: number;
	user_email: string;
	user_role: string;
	action: string;
	resource_type: string;
	resource_id?: string | null;
	resource_name?: string | null;
	metadata?: string | null;
	organization_id?: number | null;
	user_id?: number | null;
	created_at: number;
};

const auditClient = createServiceClient(AuditService);

/**
 * Audit log list scoped to the active organization, keyed under the
 * `audit_logs` query prefix so the table-watch stream (AUDIT_LOGS)
 * invalidates it while mounted. `query` filters by exact action.
 */
export function useAuditLogItems(options?: {action?: string; limit?: number}) {
	const orgId = useActiveOrgId();
	const action = options?.action;
	const limit = options?.limit ?? 200;
	return useResourceListQuery<AuditLogPayload>(
		['audit_logs', orgId.toString(), action ?? '', limit],
		() =>
			auditClient.listAuditLogs({
				organizationId: orgId,
				limit: BigInt(limit),
				query: action,
			}),
	);
}

/** CSV export of the org's audit log (payload is raw CSV text). */
export function useAuditExport() {
	const orgId = useActiveOrgId();
	return useConnectMutation(
		async (action?: string) => {
			const response = await auditClient.exportAuditLogs({
				organizationId: orgId,
				query: action,
			});
			return response.payload;
		},
	);
}