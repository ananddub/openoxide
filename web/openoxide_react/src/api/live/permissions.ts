import {createServiceClient, useConnectMutation, useConnectQuery, useResourceListQuery} from './query';
import {readResource} from '#/api/live/resource';
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

// ---------------------------------------------------------------------------
// Effective grants for the calling user (drives all UI permission gating).
// ---------------------------------------------------------------------------

export type MyPermissionsPayload = {
	role: string;
	organization_id: bigint;
	actions: string[];
};

/**
 * The caller's role + effective actions inside the ACTIVE organization.
 * Keyed by organization id so switching organizations refetches; the
 * transport attaches X-Organization-Id from persisted active-org state.
 */
export function useMyPermissions(organizationId: number | bigint | undefined) {
	return useConnectQuery<MyPermissionsPayload>(
		['my-permissions', String(organizationId ?? 0)],
		async () =>
			readResource<MyPermissionsPayload>(
				await permissionClient.getMyPermissions({}),
			),
		{enabled: organizationId !== undefined && organizationId !== null},
	);
}

// ---------------------------------------------------------------------------
// Permission groups (roles) and the policy catalog.
// ---------------------------------------------------------------------------

export type PermissionPolicyPayload = {
	id: number;
	action: string;
};

/** All policies (action strings like "app:create") for the group editor. */
export function usePolicyList() {
	return useResourceListQuery<PermissionPolicyPayload>(
		['policies', 'list'],
		() => permissionClient.listPolicies({}),
	);
}

/** Create a permission group with its policy action ids. */
export function usePermissionGroupCreate() {
	return useConnectMutation(
		(input: {name: string; actions: string[]}) =>
			permissionClient.createGroup({payload: JSON.stringify(input)}),
		{invalidate: [['permission-groups']]},
	);
}

/** Update a permission group (name and/or action ids). */
export function usePermissionGroupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; name?: string; actions?: string[]}) => {
			const {id, ...body} = input;
			return permissionClient.updateGroup({
				id: BigInt(id),
				payload: JSON.stringify(body),
			});
		},
		{invalidate: [['permission-groups']]},
	);
}

/** Delete a permission group. */
export function usePermissionGroupDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			permissionClient.deleteGroup({id: BigInt(id)}),
		{invalidate: [['permission-groups']]},
	);
}

/** Attach a permission group to an organization member. */
export function useMemberGroupAssign() {
	return useConnectMutation(
		(input: {member_id: number | bigint; group_id: number | bigint}) =>
			permissionClient.assignMemberGroup({
				id: BigInt(input.member_id),
				payload: JSON.stringify({group_id: input.group_id}),
			}),
		{invalidate: [['permission-members']]},
	);
}
