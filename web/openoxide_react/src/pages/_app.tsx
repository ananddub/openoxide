import {
	createFileRoute,
	Navigate,
	Outlet,
	useLocation,
} from '@tanstack/react-router';
import {SidebarProvider} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';
import {TopBar} from '#/components/layouts/topbar';
import {useTableWatch} from '#/hooks/use-table-watch';
import {useAuthSession} from '#/stores/session-context';

export const Route = createFileRoute('/_app')({
	component: AppLayout,
	notFoundComponent: AppNotFoundPlaceholder,
});

/**
 * Unknown project/app/database sub-paths bounce back to the owning project
 * (or the projects list) instead of rendering the shell's 404.
 */
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
/**
 * Dashboard shell, laid out like the Vercel dashboard: a fixed slim top bar
 * (workspace switcher · page title · search) spanning the full width, with
 * the nav rail starting *below* it beside a single scrolling main area.
 * Main content pads itself down by the bar's height.
 */
function AppLayout() {
	useTableWatch();
	const {isAuth} = useAuthSession();

	if (!isAuth) return <Navigate to="/singin" replace />;

	return (
		<SidebarProvider className="h-dvh">
			<div className="flex min-h-0 w-full flex-1 flex-col">
				{/* Fixed global top bar: scope switcher · title · search. The
				    rail starts below it; on mobile the SidebarTrigger in the
				    bar is the only way to open the drawer. */}
				<TopBar />
				<div className="flex min-h-0 flex-1 pt-12">
					<AppSidebar />
					{/* Main content */}
					<main className="min-w-0 flex-1 overflow-y-auto">
						<Outlet />
					</main>
				</div>
			</div>
		</SidebarProvider>
	);
}
