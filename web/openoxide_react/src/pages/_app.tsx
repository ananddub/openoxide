import {useEffect} from 'react';
import {
	createFileRoute,
	Navigate,
	Outlet,
	redirect,
	useLocation,
	useNavigate,
} from '@tanstack/react-router';
import {SidebarProvider} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';
import {TopBar} from '#/components/layouts/topbar';
import {useTableWatch} from '#/hooks/table/use-table-watch';
import {useSessionExpiry} from '#/hooks/auth/use-session-expiry';
import {getSessionUser, useAuthSession} from '#/stores/session-context';

export const Route = createFileRoute('/_app')({
	beforeLoad: ({location}) => {
		// Keep the invite link (or any deep link) so sign-in can return there.
		// Router-level redirect: rendering <Navigate> with a location-derived
		// search re-fires on every render and loops until React gives up.
		if (!getSessionUser()) {
			throw redirect({
				to: '/singin',
				search: {redirect: location.href},
				replace: true,
			});
		}
	},
	component: AppLayout,
	notFoundComponent: AppNotFoundPlaceholder,
});

/** Unknown project/app/database sub-paths bounce back to the owning project (or the projects list) instead of rendering the shell's 404. */
function AppNotFoundPlaceholder() {
	const location = useLocation();
	const pathSegments = location.pathname.split('/').filter(Boolean);
	const projectIndex = pathSegments.indexOf('projects');
	const projectId =
		projectIndex !== -1 &&
		pathSegments[projectIndex + 1] &&
		!isNaN(Number(pathSegments[projectIndex + 1]))
			? pathSegments[projectIndex + 1]
			: null;

	if (projectId) {
		return (
			<Navigate to="/projects/$id" params={{id: projectId}} replace />
		);
	}

	return <Navigate to="/projects" replace />;
}
/** Dashboard shell, laid out like the Vercel dashboard: the nav rail runs the FULL viewport height on the left (top to bottom, no offsets), and the slim top bar (workspace switcher · page title · search) sits only over the content column, pinned above a single scrolling main area. */
function AppLayout() {
	useTableWatch();
	useSessionExpiry();
	const navigate = useNavigate();
	const {isAuth} = useAuthSession();

	// The router guard only runs on navigation. When the session dies while the
	// user is sitting on a page (refresh token expired), logout() clears the
	// user without a route change — without this the shell stays on screen.
	useEffect(() => {
		if (!isAuth) void navigate({to: '/singin', replace: true});
	}, [isAuth, navigate]);

	return (
		<SidebarProvider className="h-dvh">
			<div className="flex min-h-0 w-full flex-1">
				<AppSidebar />
				{/* Content column: the top bar overlays ONLY this column; the
				    rail beside it is untouched full-height. On mobile the
				    SidebarTrigger in the bar is the only drawer opener. */}
				<main className="flex min-h-0 min-w-0 flex-1 flex-col">
					<TopBar />
					<div className="min-h-0 flex-1 overflow-y-auto">
						<Outlet />
					</div>
				</main>
			</div>
		</SidebarProvider>
	);
}
