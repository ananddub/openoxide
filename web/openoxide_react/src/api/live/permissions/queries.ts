import {useConnectQuery} from '../query';
import {permissionClient} from './shared';
import {
	PermissionUserRole,
	type PermissionGroup,
	type PermissionInvite,
	type PermissionMember,
	type PermissionPolicy,
} from '#/gen/connect/openoxide/api/v1/permission_pb';
import type {
	MyPermissionsPayload,
	OrganizationInvitePayload,
	OrganizationMemberPayload,
	PermissionGroupPayload,
	PermissionPolicyPayload,
} from './types';

/** Membership role enum → legacy payload string (null when unset). */
function roleName(role?: PermissionUserRole): string | null {
	switch (role) {
		case PermissionUserRole.OWNER:
			return 'OWNER';
		case PermissionUserRole.ADMIN:
			return 'ADMIN';
		case PermissionUserRole.MEMBER:
			return 'MEMBER';
		default:
			return null;
	}
}

function toMemberPayload(
	member: PermissionMember,
): OrganizationMemberPayload {
	return {
		id: member.id ?? null,
		user_id: member.userId,
		email: member.email,
		avatar: member.avatar ?? null,
		organization_id: member.organizationId,
		role: roleName(member.role),
		created_at: member.createdAt,
		updated_at: member.updatedAt,
		group_id: member.groupId,
		group_name: member.groupName ?? null,
	};
}

function toInvitePayload(
	invite: PermissionInvite,
): OrganizationInvitePayload {
	return {
		id: invite.id,
		email: invite.email,
		role: roleName(invite.role),
		group_id: invite.groupId,
		organization_id: invite.organizationId,
		invited_by: invite.invitedBy,
		expired_at: invite.expiredAt,
		created_at: invite.createdAt,
	};
}

function toGroupPayload(group: PermissionGroup): PermissionGroupPayload {
	return {
		id: group.id,
		name: group.name,
		is_system: group.isSystem,
		organization_id: group.organizationId ?? null,
		actions: group.actions,
		created_at: group.createdAt,
		updated_at: group.updatedAt,
	};
}

/** Organization members (legacy live: usePermissionGroupMembers). */
export function usePermissionGroupMembers() {
	return useConnectQuery<OrganizationMemberPayload[]>(
		['permission-members', 'list'],
		async () => {
			const response = await permissionClient.listMembers({});
			return response.members.map(toMemberPayload);
		},
	);
}

/** Organization invites (legacy live: usePermissionGroupInvites). */
export function usePermissionGroupInvites() {
	return useConnectQuery<OrganizationInvitePayload[]>(
		['permission-invites', 'list'],
		async () => {
			const response = await permissionClient.listInvites({});
			return response.invites.map(toInvitePayload);
		},
	);
}

/** Permission groups (legacy live: usePermissionGroupList). */
export function usePermissionGroupList() {
	return useConnectQuery<PermissionGroupPayload[]>(
		['permission-groups', 'list'],
		async () => {
			const response = await permissionClient.listGroups({});
			return response.groups.map(toGroupPayload);
		},
	);
}

/** The caller's role + effective actions inside the ACTIVE organization. Keyed by organization id so switching organizations refetches; the transport attaches X-Organization-Id from persisted active-org state. */
export function useMyPermissions(
	organizationId: number | bigint | undefined,
) {
	return useConnectQuery<MyPermissionsPayload>(
		['my-permissions', String(organizationId ?? 0)],
		async () => {
			const response = await permissionClient.getMyPermissions({});
			return {
				role: roleName(response.role) ?? 'NONE',
				organization_id: response.organizationId,
				actions: response.actions,
			};
		},
		{enabled: organizationId !== undefined && organizationId !== null},
	);
}

/** All policies (action strings like "app:create") for the group editor. */
export function usePolicyList() {
	return useConnectQuery<PermissionPolicyPayload[]>(
		['policies', 'list'],
		async () => {
			const response = await permissionClient.listPolicies({});
			return response.policies.map((policy: PermissionPolicy) => ({
				id: Number(policy.id),
				action: policy.action,
			}));
		},
	);
}
