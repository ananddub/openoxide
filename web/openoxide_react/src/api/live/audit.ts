import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
	useActiveOrgId,
} from './query';
import {
	AuditService,
	type AuditLog,
} from '#/gen/connect/openoxide/api/v1/audit_pb';

/** Audit log rows as emitted by the panel's audit writers (`audit_logs` table). `action` is an UPPER_SNAKE verb (`PERMISSION_GROUP_CREATED`), `metadata` is a JSON string with resource details. */
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

/** Typed `AuditLog` → legacy payload (int64s narrow to numbers). */
function toAuditLogPayload(log: AuditLog): AuditLogPayload {
	return {
		id: Number(log.id),
		user_email: log.userEmail,
		user_role: log.userRole,
		action: log.action,
		resource_type: log.resourceType,
		resource_id: log.resourceId,
		resource_name: log.resourceName,
		metadata: log.metadata,
		organization_id:
			log.organizationId === undefined ? null : Number(log.organizationId),
		user_id: log.userId === undefined ? null : Number(log.userId),
		created_at: Number(log.createdAt),
	};
}

/** Audit log list scoped to the active organization, keyed under the `audit_logs` query prefix so the table-watch stream (AUDIT_LOGS) invalidates it while mounted. `query` filters by exact action. */
export function useAuditLogItems(options?: {
	action?: string;
	limit?: number;
}) {
	const orgId = useActiveOrgId();
	const action = options?.action;
	const limit = options?.limit ?? 200;
	return useConnectQuery<AuditLogPayload[]>(
		['audit_logs', orgId.toString(), action ?? '', limit],
		async () => {
			const response = await auditClient.listAuditLogs({
				organizationId: orgId,
				limit: BigInt(limit),
				action: action || undefined,
			});
			return response.logs.map(toAuditLogPayload);
		},
	);
}

/** CSV export of the active org's audit log (resolved value is raw CSV text). */
export function useAuditExport() {
	return useConnectMutation(async (action?: string) => {
		const response = await auditClient.exportAuditLogs({
			action: action || undefined,
		});
		return response.csv;
	});
}
