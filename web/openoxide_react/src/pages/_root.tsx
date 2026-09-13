import {
	createRootRoute,
	Outlet,
	HeadContent,
} from '@tanstack/react-router';
import {Code, ConnectError} from '@connectrpc/connect';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {RootPending} from '#/components/pending';
import {
	SessionProvider,
	setPendingUser,
	getSessionActions,
} from '#/stores/session-context';
import {TooltipProvider} from '#/components/ui/tooltip';
import {QueryClient, QueryClientProvider} from '@tanstack/react-query';
import {Toaster} from '#/components/ui/sonner';
import '#/styles/index.css';

const authClient = createServiceClient(AuthService);

// The root `beforeLoad` re-runs on EVERY client-side navigation, so the
// session bootstrap is memoized: exactly one `whoAmI` round-trip per page
// load instead of a blocking auth call on every route change.
let sessionBootstrap: Promise<void> | null = null;

async function bootstrapSession(): Promise<void> {
	// First page load: the provider has not mounted yet, so push the
	// whoAmI result into the pending handoff that SessionProvider
	// consumes as its initial state. On client-side navigations the
	// provider is live and imperative actions exist.
	const live = getSessionActions();

	const applyUser = (user: {
		id: bigint;
		email?: string;
		firstName?: string;
		lastName?: string;
	}) => {
		const mapped = {
			id: Number(user.id),
			email: user.email || '',
			firstName: user.firstName,
			lastName: user.lastName,
		};
		if (live) live.setAuth(mapped);
		else setPendingUser(mapped);
	};

	const clearUser = () => {
		if (live) live.logout();
		else setPendingUser(null);
	};

	const session = localStorage.getItem('openoxide-auth-session');
	if (!session || session === 'undefined') {
		clearUser();
		return;
	}

	try {
		// An expired access token is refreshed and the call replayed by
		// the auth interceptor — never retry manually on top of it.
		const user = await authClient.whoAmI({});
		applyUser(user);
	} catch (error) {
		if (error instanceof ConnectError && error.code === Code.Unauthenticated) {
			// The interceptor already tried refresh-and-replay and the
			// backend still rejected us: the refresh token is missing or
			// dead. Drop the session now — keeping it would re-run this
			// whole whoAmI → refresh → whoAmI chain on every navigation.
			clearUser();
			return;
		}
		// Transport/unknown failure (backend briefly down): keep the
		// stored session and let the next navigation bootstrap again.
		console.error('Failed to authenticate session:', error);
		sessionBootstrap = null;
	}
}

export const Route = createRootRoute({
	beforeLoad: async () => {
		sessionBootstrap ??= bootstrapSession();
		await sessionBootstrap;
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
