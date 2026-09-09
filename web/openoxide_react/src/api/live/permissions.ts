import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
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

/** Add a member to the active organization (email/role fields in the payload). */
export function useMemberAdd() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			permissionClient.addMember({payload: JSON.stringify(input)}),
		{invalidate: [['permission-members']]},
	);
}

/** Update an organization member's role by member id (role in the payload). */
export function useMemberUpdateRole() {
	return useConnectMutation(
		(input: {id: number | bigint; role: string}) =>
			permissionClient.updateMemberRole({
				id: BigInt(input.id),
				payload: JSON.stringify({role: input.role}),
			}),
		{invalidate: [['permission-members']]},
	);
}

/** Remove an organization member by member id (legacy REST: DELETE users/{id}/org-members). */
export function useMemberRemove() {
	return useConnectMutation(
		(id: number | bigint) =>
			permissionClient.removeMember({id: BigInt(id)}),
		{invalidate: [['permission-members']]},
	);
}

/** Create an invitation (email/role fields in the payload). */
export function useInviteCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			permissionClient.createInvite({payload: JSON.stringify(input)}),
		{invalidate: [['permission-invites']]},
	);
}

/**
 * Cancel an invitation by id. CancelInvite is a Resource-pattern request
 * whose target id travels in the payload JSON.
 */
export function useInviteCancel() {
	return useConnectMutation(
		(id: number | bigint | string) =>
			permissionClient.cancelInvite({payload: JSON.stringify({id})}),
		{invalidate: [['permission-invites']]},
	);
}

/** Legacy member shape the former zustand `members` slice carried. */
export type MemberItem = {
	id: string;
	user_id?: number;
	email: string;
	name?: string;
	role: string;
	avatar?: string;
	banned?: boolean;
	created_at?: number;
};

/** Legacy invite shape the former zustand `invites` slice carried. */
export type InviteItem = {
	id: string;
	email: string;
	role: string;
	created_at?: number;
};
