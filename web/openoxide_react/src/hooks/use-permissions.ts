import {useMemo} from 'react';
import {useOrganizations, useAuthSession} from '#/stores/session-context';
import {useMyPermissions} from '#/api/live/permissions';

/**
 * Permission gating for the active organization.
 *
 * `can("app:create")` checks the effective action list from GetMyPermissions;
 * owners/admins receive the `*` wildcard from the backend, so a single
 * wildcard-or-exact check covers every role. Actions are `resource:operation`
 * (resource names are the backend's short forms: app, env, org, ssh_key, ...).
 *
 * While the grant query is in flight, `can` reports false — buttons flicker
 * off-then-on rather than exposing an action a member may not have.
 */
export function usePermissions() {
	const {activeOrg} = useOrganizations();
	const {user} = useAuthSession();
	const query = useMyPermissions(activeOrg?.id);

	const {role, actions, wildcard} = useMemo(() => {
		const data = query.data;
		return {
			role: data?.role ?? 'NONE',
			actions: data?.actions ?? [],
			wildcard: (data?.actions ?? []).includes('*'),
		};
	}, [query.data]);

	return {
		role,
		actions,
		isOwner: role === 'OWNER',
		isAdmin: role === 'ADMIN' || role === 'OWNER',
		can: (action: string) => wildcard || actions.includes(action),
		canAny: (...actionsToCheck: string[]) =>
			wildcard || actionsToCheck.some(action => actions.includes(action)),
		isLoading: query.isLoading,
		// Users outside every organization (or a failed grant fetch) must not
		// see write affordances at all.
		isMember: role !== 'NONE' && user !== null,
	};
}

export function useCan() {
	const {can} = usePermissions();
	return can;
}
