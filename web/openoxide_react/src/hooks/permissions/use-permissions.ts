import {useMemo} from 'react';
import {useOrganizations, useAuthSession} from '#/stores/session-context';
import {useMyPermissions} from '#/api/live/permissions';

// Permission gating for the active org; `*` (owners/admins) short-circuits every check.
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
		// Outside every org (or after a failed grant fetch), hide write affordances.
		isMember: role !== 'NONE' && user !== null,
	};
}

export function useCan() {
	const {can} = usePermissions();
	return can;
}
