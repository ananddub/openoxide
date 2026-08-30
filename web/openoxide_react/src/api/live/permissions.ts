import {createServiceClient, useResourceListQuery} from './query';
import {PermissionService} from '#/gen/connect/openoxide/api/v1/permission_pb';

/**
 * Permission payloads mirror the legacy live shape (snake_case JSON payloads
 * of the identity DTOs). The organization context travels in the
 * `X-Organization-Id` header attached by the transport.
 */
export type OrganizationMemberPayload = {
	id: bigint | null;
	user_id: bigint;
	email: string;
	avatar: string | null;
	organization_id: bigint;
	role: string | null;
	created_at: bigint;
	updated_at: bigint;
};

export type OrganizationInvitePayload = {
	id: bigint | null;
	email: string;
	role: string | null;
	group_id: bigint;
	organization_id: bigint;
	invited_by: bigint;
	expired_at: bigint;
	created_at: bigint;
};

export type PermissionGroupPayload = {
	id: bigint;
	name: string;
	is_system: boolean;
	organization_id: bigint | null;
	actions: string[];
	created_at: bigint;
	updated_at: bigint;
};

const permissionClient = createServiceClient(PermissionService);

/** Organization members (legacy live: usePermissionGroupMembers). */
export function usePermissionGroupMembers() {
	return useResourceListQuery<OrganizationMemberPayload>(
		['permission-members', 'list'],
		() => permissionClient.listMembers({}),
	);
}

/** Organization invites (legacy live: usePermissionGroupInvites). */
export function usePermissionGroupInvites() {
	return useResourceListQuery<OrganizationInvitePayload>(
		['permission-invites', 'list'],
		() => permissionClient.listInvites({}),
	);
}

/** Permission groups (legacy live: usePermissionGroupList). */
export function usePermissionGroupList() {
	return useResourceListQuery<PermissionGroupPayload>(
		['permission-groups', 'list'],
		() => permissionClient.listGroups({}),
	);
}
