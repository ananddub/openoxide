import {useConnectMutation} from '../query';
import {permissionClient} from './shared';
import {PermissionUserRole} from '#/gen/connect/openoxide/api/v1/permission_pb';

/** Legacy role strings ("owner", "OWNER", "developer", "viewer", ...) → the membership enum. The server only stores OWNER/ADMIN/MEMBER; "developer"/"viewer" are UI shorthand for MEMBER plus a permission group. */
function parseRole(value: unknown): PermissionUserRole {
	switch (String(value ?? '').toUpperCase()) {
		case 'OWNER':
			return PermissionUserRole.OWNER;
		case 'ADMIN':
			return PermissionUserRole.ADMIN;
		case 'MEMBER':
		case 'DEVELOPER':
		case 'VIEWER':
			return PermissionUserRole.MEMBER;
		default:
			return PermissionUserRole.UNSPECIFIED;
	}
}

/** Add a member to the active organization (email/role/password/group fields). */
export function useMemberAdd() {
	return useConnectMutation(
		(input: {
			email?: string;
			role?: string;
			password?: string;
			group_id?: number | bigint;
			[key: string]: unknown;
		}) =>
			permissionClient.addMember({
				email: String(input.email ?? ''),
				password:
					typeof input.password === 'string' && input.password
						? input.password
						: undefined,
				role: parseRole(input.role),
				groupId:
					input.group_id === undefined || input.group_id === null
						? undefined
						: BigInt(input.group_id),
			}),
		{invalidate: [['permission-members']]},
	);
}

/** Update an organization member's role by member id. */
export function useMemberUpdateRole() {
	return useConnectMutation(
		(input: {id: number | bigint; role: string}) =>
			permissionClient.updateMemberRole({
				id: BigInt(input.id),
				role: parseRole(input.role),
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

/** Create an invitation (email/role/group_id fields); resolves to the opaque invite token the email link carries. group_id 0 means the invitee joins with role-only access. */
export function useInviteCreate() {
	return useConnectMutation(
		(input: {
			email?: string;
			role?: string;
			group_id?: number | bigint;
			[key: string]: unknown;
		}) =>
			permissionClient.createInvite({
				email: String(input.email ?? ''),
				role: parseRole(input.role),
				groupId: BigInt(input.group_id ?? 0),
			}),
		{invalidate: [['permission-invites']]},
	);
}

/** Cancel an invitation by id. */
export function useInviteCancel() {
	return useConnectMutation(
		(id: number | bigint | string) =>
			permissionClient.cancelInvite({id: BigInt(id)}),
		{invalidate: [['permission-invites']]},
	);
}

/** Create a permission group with its policy action ids. */
export function usePermissionGroupCreate() {
	return useConnectMutation(
		(input: {name: string; actions: string[]}) =>
			permissionClient.createGroup({
				name: input.name,
				actions: input.actions,
			}),
		{invalidate: [['permission-groups']]},
	);
}

/** Update a permission group (name and action ids are full replacements). */
export function usePermissionGroupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; name?: string; actions?: string[]}) => {
			const {id, name, actions} = input;
			return permissionClient.updateGroup({
				id: BigInt(id),
				name: name ?? '',
				actions: actions ?? [],
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

/** Attach a permission group to an organization member. `user_id` — the backend keys memberships by user id, not row id. */
export function useMemberGroupAssign() {
	return useConnectMutation(
		(input: {user_id: number | bigint; group_id: number | bigint}) =>
			permissionClient.assignMemberGroup({
				id: BigInt(input.user_id),
				groupId: BigInt(input.group_id),
			}),
		{invalidate: [['permission-members']]},
	);
}
