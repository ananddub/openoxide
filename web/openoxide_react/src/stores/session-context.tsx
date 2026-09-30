import {
	createContext,
	useContext,
	useMemo,
	useState,
	type ReactNode,
} from 'react';

import type {User} from '#/types/user';
import {
	getActiveOrganizationId,
	persistActiveOrganizationId,
} from './organization-context';

// Session state (auth user + active organization); server data lives in the Query cache.

type AuthState = {
	user: User | null;
	isAuth: boolean;
	setAuth: (user: User) => void;
	logout: () => void;
};

type OrganizationState = {
	organizations: Record<string, any>[];
	activeOrg: Record<string, any> | null;
	setOrganizations: (orgs: Record<string, any>[]) => void;
	setActiveOrg: (org: Record<string, any> | null) => void;
};

type SessionState = AuthState & OrganizationState;

const SessionContext = createContext<SessionState | null>(null);

// ── First-load handoff: beforeLoad stashes the user for the provider's first mount.
let pendingUser: User | null = null;

export function setPendingUser(user: User | null) {
	pendingUser = user;
}

export function SessionProvider({children}: {children: ReactNode}) {
	const [user, setUser] = useState<User | null>(() => pendingUser);
	const [organizations, setOrganizationsState] = useState<
		Record<string, any>[]
	>([]);
	const [activeOrg, setActiveOrgState] = useState<Record<
		string,
		any
	> | null>(() => {
		const preferredId = getActiveOrganizationId();
		return preferredId ? ({id: preferredId} as Record<string, any>) : null;
	});

	const session = useMemo<SessionState>(() => {
		const setAuth = (next: User) => setUser(next);
		const logout = () => {
			localStorage.removeItem('openoxide-auth-session');
			setUser(null);
		};

		const setOrganizations = (orgs: Record<string, any>[]) => {
			// Identical id+name lists keep the current state (and its identity) untouched.
			setOrganizationsState(current => {
				if (
					current.length === orgs.length &&
					current.every(
						(org, index) =>
							org.id === orgs[index]?.id && org.name === orgs[index]?.name,
					)
				) {
					return current;
				}

				const preferredId = activeOrg?.id ?? getActiveOrganizationId();
				// The server lists orgs newest-first; with no stored preference land on
				// the primary (oldest owned) workspace so a later-created empty org
				// doesn't become the default and hide the main data.
				const owned = orgs.filter(
					o => user?.id != null && Number(o.owner_id) === Number(user.id),
				);
				const pool = owned.length > 0 ? owned : orgs;
				const primary = pool.reduce<Record<string, any> | null>(
					(min, o) =>
						min == null || Number(o.id) < Number(min.id) ? o : min,
					null,
				);
				const nextActive =
					orgs.length > 0
						? orgs.find(o => o.id === preferredId) || primary
						: null;
				// {id}-only activeOrg on first load: refresh even when the id matches.
				const staleIdentity =
					nextActive != null &&
					(nextActive.id !== activeOrg?.id || !activeOrg?.name);
				if (staleIdentity || (nextActive == null && activeOrg != null)) {
					persistActiveOrganizationId(nextActive?.id ?? null);
					setActiveOrgState(nextActive);
				}
				return orgs;
			});
		};

		const setActiveOrg = (org: Record<string, any> | null) => {
			setActiveOrgState(org);
			persistActiveOrganizationId(org?.id ?? null);
		};

		return {
			user,
			isAuth: user !== null,
			setAuth,
			logout,
			organizations,
			activeOrg,
			setOrganizations,
			setActiveOrg,
		};
	}, [user, organizations, activeOrg]);

	setSessionRef({setAuth: session.setAuth, logout: session.logout});

	return (
		<SessionContext.Provider value={session}>
			{children}
		</SessionContext.Provider>
	);
}

function useSession(): SessionState {
	const context = useContext(SessionContext);
	if (!context) {
		throw new Error('useSession must be used within <SessionProvider>');
	}
	return context;
}

// ── Imperative bridge: a module-level ref serves code that runs before React renders.

let sessionRef: {
	setAuth: (user: User) => void;
	logout: () => void;
} | null = null;

/** Called by SessionProvider on every render to keep the ref fresh. */
function setSessionRef(ref: {
	setAuth: (user: User) => void;
	logout: () => void;
}) {
	sessionRef = ref;
}

// Null until the provider mounts — callers fall back to setPendingUser.
export function getSessionActions(): {
	setAuth: (user: User) => void;
	logout: () => void;
} | null {
	return sessionRef;
}

/** Auth session: `{user, isAuth, setAuth, logout}` — old auth-store shape. */
export function useAuthSession(): AuthState {
	const session = useSession();
	return {
		user: session.user,
		isAuth: session.isAuth,
		setAuth: session.setAuth,
		logout: session.logout,
	};
}

/** Active organization: old organization-store shape. */
export function useOrganizations(): OrganizationState {
	const session = useSession();
	return {
		organizations: session.organizations,
		activeOrg: session.activeOrg,
		setOrganizations: session.setOrganizations,
		setActiveOrg: session.setActiveOrg,
	};
}

/** Imperative access for non-hook code (router beforeLoad, api layer). */
export function useSessionActions() {
	const session = useSession();
	return {
		setAuth: session.setAuth,
		logout: session.logout,
	};
}
