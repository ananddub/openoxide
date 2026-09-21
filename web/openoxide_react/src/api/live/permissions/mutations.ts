import {useConnectMutation} from '../query';
import {permissionClient} from './shared';

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
