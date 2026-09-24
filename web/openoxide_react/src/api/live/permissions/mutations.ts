import {useConnectMutation} from '../query';
import {permissionClient} from './shared';
import {PermissionUserRole} from '#/gen/connect/openoxide/api/v1/permission_pb';

/** Legacy role strings ("owner", "OWNER", "developer", "viewer", ...) → the membership enum; only OWNER/ADMIN/MEMBER exist server-side, so any other value (or an absent role) maps to UNSPECIFIED. */
function parseRole(value: unknown): PermissionUserRole {
	switch (String(value ?? '').toUpperCase()) {
		case 'OWNER':
			return PermissionUserRole.OWNER;
		case 'ADMIN':
			return PermissionUserRole.ADMIN;
		case 'MEMBER':
			return PermissionUserRole.MEMBER;
		default:
			return PermissionUserRole.UNSPECIFIED;
	}
}

/** Add a member to the active organization (email/role/password fields). */
export function useMemberAdd() {
	return useConnectMutation(
		(input: {
			email?: string;
			role?: string;
			password?: string;
			[key: string]: unknown;
		}) =>
			permissionClient.addMember({
				email: String(input.email ?? ''),
				password:
					typeof input.password === 'string' && input.password
						? input.password
						: undefined,
				role: parseRole(input.role),
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

/** Create an invitation (email/role/group_id fields); resolves to the opaque invite token the email link carries. */
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
				groupId:
					input.group_id === undefined || input.group_id === null
						? 0n
						: BigInt(input.group_id),
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

/** Attach a permission group to an organization member. */
export function useMemberGroupAssign() {
	return useConnectMutation(
		(input: {member_id: number | bigint; group_id: number | bigint}) =>
			permissionClient.assignMemberGroup({
				id: BigInt(input.member_id),
				groupId: BigInt(input.group_id),
			}),
		{invalidate: [['permission-members']]},
	);
}
