import {useConnectQuery, useResourceListQuery} from '../query';
import {readResource} from '#/api/live/resource';
import {permissionClient} from './shared';
import type {
	MyPermissionsPayload,
	OrganizationInvitePayload,
	OrganizationMemberPayload,
	PermissionGroupPayload,
	PermissionPolicyPayload,
} from './types';

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

/**
 * The caller's role + effective actions inside the ACTIVE organization.
 * Keyed by organization id so switching organizations refetches; the
 * transport attaches X-Organization-Id from persisted active-org state.
 */
export function useMyPermissions(
	organizationId: number | bigint | undefined,
) {
	return useConnectQuery<MyPermissionsPayload>(
		['my-permissions', String(organizationId ?? 0)],
		async () =>
			readResource<MyPermissionsPayload>(
				await permissionClient.getMyPermissions({}),
			),
		{enabled: organizationId !== undefined && organizationId !== null},
	);
}

/** All policies (action strings like "app:create") for the group editor. */
export function usePolicyList() {
	return useResourceListQuery<PermissionPolicyPayload>(
		['policies', 'list'],
		() => permissionClient.listPolicies({}),
	);
}
