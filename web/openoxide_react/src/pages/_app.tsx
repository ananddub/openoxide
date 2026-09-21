import {
	createFileRoute,
	Navigate,
	Outlet,
	useLocation,
} from '@tanstack/react-router';
import {SidebarProvider, SidebarTrigger} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';
import {Logo} from '#/components/shared/logo';
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
 * Dashboard shell, laid out like the OpenShip dashboard: a full-height flex
 * column with the floating rounded rail beside a single scrolling main area.
 * There is no sticky breadcrumb header — each page owns its own title, and
 * the rail carries navigation, theme, and the workspace switcher.
 */
function AppLayout() {
	useTableWatch();
	const {isAuth} = useAuthSession();

	if (!isAuth) return <Navigate to="/singin" replace />;

	return (
		<SidebarProvider className="h-dvh">
			<div className="flex min-h-0 w-full flex-1 flex-col">
				{/* Mobile-only top bar: the floating rail is hidden below md,
				    and this trigger is the only way to open the drawer. */}
				<div className="flex h-12 shrink-0 items-center justify-between gap-2 border-b bg-background px-3 md:hidden">
					<div className="flex min-w-0 items-center gap-2.5">
						<Logo className="size-6 shrink-0" />
						<span className="truncate text-sm font-semibold tracking-tight">
							OpenOxide
						</span>
					</div>
					<SidebarTrigger className="size-9" />
				</div>
				<div className="flex min-h-0 flex-1">
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
