import {
	Outlet,
	redirect,
	Navigate,
	createFileRoute,
} from '@tanstack/react-router';
import {client} from '#/api/client';
import {useAuthStore} from '#/stores/auth-store';
import {AuthLeftPanel} from '#/components/auth/auth-left-panel';

export const Route = createFileRoute('/_auth')({
	beforeLoad: async ({location}) => {
		try {
			// Verify setup status
			const {data: res} = await (client.GET(
				'/auth/setup' as any,
				{},
			) as Promise<{
				data?: {isOwnerPresent: boolean};
				error?: any;
			}>);
			const isOwnerPresent = res?.isOwnerPresent ?? true;
			// Owner not present & user is not on signup
			if (!isOwnerPresent && location.pathname !== '/singup') {
				throw redirect({
					to: '/singup',
					replace: true,
				});
			}
		} catch (error: any) {
			// Rethrow router redirect objects
			if (error && (error.isRedirect || 'to' in error || 'href' in error)) {
				throw error;
			}
			console.error('Failed to verify auth setup status:', error);
		}
	},
	component: AuthLayout,
});

function AuthLayout() {
	const isAuth = useAuthStore(state => state.isAuth);
	if (isAuth) return <Navigate to="/" replace />;
	return (
		<div className="flex min-h-svh w-screen bg-background">
			{/* Left Side */}
			<AuthLeftPanel />
			{/* Right Side */}
			<div className="flex w-full items-center justify-center bg-background p-6 md:p-10 lg:w-1/2">
				<Outlet />
			</div>
		</div>
	);
}
