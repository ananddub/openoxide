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

/**
 * Session state (auth user + active organization) as plain React context.
 * Server data lives in the TanStack Query cache — this context holds only
 * the two pieces of state that are genuinely session-scoped.
 *
 * Consumers use the `useAuthSession` / `useOrganizations` hooks.
 */

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

// ── First-load handoff ──────────────────────────────────────────────────────
// The router's root beforeLoad runs BEFORE the provider mounts, so it
// cannot push the whoAmI result into context. It stashes the resolved
// user here instead; SessionProvider reads it as its initial state.
let pendingUser: User | null = null;

/** Stashes the user resolved in beforeLoad for the provider's first mount. */
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
			// Same de-dupe as the legacy store: identical id+name lists keep
			// the current state (and its identity) untouched.
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
				const nextActive =
					orgs.length > 0
						? orgs.find(o => o.id === preferredId) || orgs[0]
						: null;
				// On first load activeOrg is only `{id}` (restored from
				// localStorage) — no name. Refresh it with the full entry even
				// when the id already matches, or the switcher shows the
				// "Workspace" fallback until the user manually switches.
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

// ── Imperative bridge for non-hook code ─────────────────────────────────────
// The router's beforeLoad runs before React renders, so it cannot consume
// context. A module-level ref (kept in sync by the provider) covers that
// window without any global store.

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

/**
 * Imperative auth actions for non-hook code. Returns null when the
 * provider has not mounted yet (root beforeLoad on first page load) —
 * callers should fall back to setPendingUser in that window.
 */
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
