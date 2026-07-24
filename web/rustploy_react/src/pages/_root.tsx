import {
	Outlet,
	HeadContent,
	createRootRoute,
} from '@tanstack/react-router';
import {client} from '#/api/client';
import {RootPending} from '#/components/pending';
import {useAuthStore} from '#/stores/auth-store';
import {TooltipProvider} from '#/components/ui/tooltip';
import {QueryClient, QueryClientProvider} from '@tanstack/react-query';
import {Toaster} from '#/components/ui/sonner';
import '#/styles/index.css';

export const Route = createRootRoute({
	beforeLoad: async () => {
		try {
			const {data: res} = await client.GET('/auth/whoami');
			if (res) {
				useAuthStore.getState().setAuth({
					id: res.user_id,
					email: res.email || '',
					firstName: res.first_name,
					lastName: res.last_name,
				});
			} else {
				useAuthStore.getState().logout();
			}
		} catch (error) {
			console.error('Failed to authenticate session:', error);
			useAuthStore.getState().logout();
		}
	},
	component: RootComponent,
	pendingComponent: RootPending,
});

// Create a client
const queryClient = new QueryClient();

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
