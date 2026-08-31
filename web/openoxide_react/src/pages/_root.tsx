import {
	Outlet,
	HeadContent,
	createRootRoute,
} from '@tanstack/react-router';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {RootPending} from '#/components/pending';
import {useAuthStore} from '#/stores/auth-store';
import {TooltipProvider} from '#/components/ui/tooltip';
import {QueryClient, QueryClientProvider} from '@tanstack/react-query';
import {Toaster} from '#/components/ui/sonner';
import '#/styles/index.css';

const authClient = createServiceClient(AuthService);

export const Route = createRootRoute({
	beforeLoad: async () => {
		const alreadyUser = useAuthStore.getState().user;
		if (alreadyUser) return;

		const session = localStorage.getItem('openoxide-auth-session');
		if (!session || session === 'undefined') {
			useAuthStore.getState().logout();
			return;
		}
		try {
			const user = await authClient.whoAmI({});
			if (user) {
				useAuthStore.getState().setAuth({
					id: Number(user.id),
					email: user.email || '',
					firstName: user.firstName,
					lastName: user.lastName,
				});
			}
		} catch (error) {
			// An expired/invalid session surfaces as a Connect error here;
			// keep the stored session and let the interceptor attempt a
			// refresh on the next authenticated call.
			console.error('Failed to authenticate session:', error);
		}
	},
	component: RootComponent,
	pendingComponent: RootPending,
	errorComponent: RootError,
});

function RootError() {
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
				<TooltipProvider>
					<Outlet />
					<Toaster />
				</TooltipProvider>
			</QueryClientProvider>
		</>
	);
}
