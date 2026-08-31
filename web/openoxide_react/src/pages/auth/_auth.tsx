import {
	Outlet,
	redirect,
	Navigate,
	createFileRoute,
} from '@tanstack/react-router';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useAuthStore} from '#/stores/auth-store';
import {AuthLeftPanel} from '#/components/auth/auth-left-panel';

const authClient = createServiceClient(AuthService);

export const Route = createFileRoute('/_auth')({
	beforeLoad: async ({location}) => {
		try {
			// Verify setup status
			const status = await authClient.getSetupStatus({});
			const isOwnerPresent = status.ownerPresent ?? true;
			// Owner not present & user is not on signup
			if (!isOwnerPresent && location.pathname !== '/singup') {
				throw redirect({
					to: '/singup',
					replace: true,
				});
			}
		} catch (error: unknown) {
			// Rethrow router redirect objects
			const err = error as Record<string, unknown> | null;
			if (err && (err.isRedirect || 'to' in err || 'href' in err)) {
				throw error;
			}
			// Silently fallback if backend server is unreachable
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
