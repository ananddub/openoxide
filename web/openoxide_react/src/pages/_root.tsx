import {
	createRootRoute,
	Outlet,
	HeadContent,
} from '@tanstack/react-router';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {RootPending} from '#/components/pending';
import {SessionProvider, setPendingUser, getSessionActions} from '#/stores/session-context';
import {refreshAccessToken} from '#/api/client';
import {TooltipProvider} from '#/components/ui/tooltip';
import {QueryClient, QueryClientProvider} from '@tanstack/react-query';
import {Toaster} from '#/components/ui/sonner';
import '#/styles/index.css';

const authClient = createServiceClient(AuthService);

export const Route = createRootRoute({
	beforeLoad: async () => {
		// First page load: the provider has not mounted yet, so push the
		// whoAmI result into the pending handoff that SessionProvider
		// consumes as its initial state. On client-side navigations the
		// provider is live and imperative actions exist.
		const live = getSessionActions();

		const session = localStorage.getItem('openoxide-auth-session');
		if (!session || session === 'undefined') {
			if (live) live.logout();
			else setPendingUser(null);
			return;
		}
		try {
			const user = await authClient.whoAmI({});
			if (user) {
				const mapped = {
					id: Number(user.id),
					email: user.email || '',
					firstName: user.firstName,
					lastName: user.lastName,
				};
				if (live) live.setAuth(mapped);
				else setPendingUser(mapped);
			}
		} catch (error) {
			// An expired/invalid session surfaces as a Connect error here;
			// keep the stored session and let the interceptor attempt a
			// refresh on the next authenticated call.
			console.error('Failed to authenticate session:', error);
		// The refresh endpoint is public, so silently rotate the token
		// and retry once — a page reload must not kick the user to login
		// just because 15 minutes passed since the last token mint.
		try {
			const newToken = await refreshAccessToken();
			if (!newToken) return;
			const retryUser = await authClient.whoAmI({});
			if (retryUser) {
				const mapped = {
					id: Number(retryUser.id),
					email: retryUser.email || '',
					firstName: retryUser.firstName,
					lastName: retryUser.lastName,
				};
				if (live) live.setAuth(mapped);
				else setPendingUser(mapped);
			}
		} catch (retryError) {
			console.error('Session refresh failed:', retryError);
		}
		}
	},
	component: RootComponent,
	pendingComponent: RootPending,
	errorComponent: RootError,
});

function RootError({error}: {error: unknown}) {
	// The boundary swallows the actual failure unless it is logged; without
	// this the screen gives no hint whether the cause was a render crash,
	// a route validation error, or an auth failure.
	console.error('[root-error]', error);
	if (error instanceof Error && error.stack) {
		console.error('[root-error] stack:', error.stack);
	}
	return (
		<div className="flex min-h-screen items-center justify-center bg-background p-6 text-sm text-muted-foreground">
			<div className="rounded-lg border border-border bg-card px-5 py-4 shadow-sm">
				Unable to load this view. Please refresh the page.
			</div>
		</div>
	);
}

// Create a client with 5-minute caching & mutation invalidation pattern
const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			staleTime: 1000 * 60 * 5, // 5 Minutes Caching
			gcTime: 1000 * 60 * 15, // 15 Minutes Cache Retention
			refetchOnWindowFocus: false,
			refetchOnReconnect: false,
			retry: 1,
		},
	},
});

function RootComponent() {
	return (
		<>
			<HeadContent />
			<QueryClientProvider client={queryClient}>
				<SessionProvider>
					<TooltipProvider>
						<Outlet />
						<Toaster />
					</TooltipProvider>
				</SessionProvider>
			</QueryClientProvider>
		</>
	);
}
