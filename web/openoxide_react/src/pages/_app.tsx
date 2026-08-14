import * as React from 'react';
import {useAuthStore} from '#/stores/auth-store';
import {createFileRoute, Navigate, Outlet, useLocation, Link} from '@tanstack/react-router';
import {
	SidebarInset,
	SidebarProvider,
	SidebarTrigger,
} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';
import {Separator} from '#/components/ui/separator';
import { useProjectGet, useApplicationGet, useComposeGet, usePostgresGet } from 'virtual:openoxide-live';

function ProjectNameBreadcrumb({ id }: { id: number }) {
	const { data: project } = useProjectGet(BigInt(id));
	return <>{project?.name || `Project #${id}`}</>;
}

function AppNameBreadcrumb({ id }: { id: number }) {
	const { data: app } = useApplicationGet(BigInt(id));
	return <>{app?.name || app?.app_name || `App #${id}`}</>;
}

function ComposeNameBreadcrumb({ id }: { id: number }) {
	const { data: compose } = useComposeGet(BigInt(id));
	return <>{compose?.name || compose?.app_name || `Compose #${id}`}</>;
}

function DatabaseNameBreadcrumb({ id }: { id: number }) {
	const { data: postgresDb } = usePostgresGet(BigInt(id));
	return <>{postgresDb?.name || postgresDb?.database_name || postgresDb?.app_name || `Database #${id}`}</>;
}


function AppNotFoundPlaceholder() {
	const location = useLocation();
	const pathSegments = location.pathname.split('/').filter(Boolean);
	const projectIndex = pathSegments.indexOf('projects');
	const projectId = projectIndex !== -1 && pathSegments[projectIndex + 1] && !isNaN(Number(pathSegments[projectIndex + 1])) ? pathSegments[projectIndex + 1] : null;

	if (projectId) {
		return <Navigate to="/projects/$id" params={{id: projectId}} replace />;
	}

	return <Navigate to="/projects" replace />;
}

export const Route = createFileRoute('/_app')({
	component: AppLayout,
	notFoundComponent: AppNotFoundPlaceholder,
});

function AppLayout() {
	const isAuth = useAuthStore(state => state.isAuth);
	const location = useLocation();

	if (!isAuth) return <Navigate to="/singin" replace />;

	const pathSegments = location.pathname.split('/').filter(Boolean);

	return (
		<SidebarProvider
			style={
				{
					'--sidebar-width': '16rem',
					'--sidebar-width-mobile': '16rem',
				} as React.CSSProperties
			}>
			<AppSidebar />
			<SidebarInset>
				{/* Sticky Header with breadcrumbs */}
				<header className="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-4 border-b border-border/40 bg-background/80 px-6 backdrop-blur-md transition-all duration-200">
					<div className="flex w-full items-center justify-between">
						<div className="flex items-center gap-2">
							<SidebarTrigger className="-ml-1" />
							<Separator orientation="vertical" className="mx-2 h-4" />
							<div className="flex items-center gap-2 text-xs font-semibold">
								<Link
									to="/projects"
									className="text-muted-foreground hover:text-foreground transition-colors">
									Dashboard
								</Link>
								{pathSegments.map((segment, index) => {
									let path = `/${pathSegments.slice(0, index + 1).join('/')}`;
									const isLast = index === pathSegments.length - 1;
									const isProjectParam = index > 0 && pathSegments[index - 1] === 'projects' && !isNaN(Number(segment));
									const isAppParam = index > 0 && pathSegments[index - 1] === 'app' && !isNaN(Number(segment));
									const isComposeParam = index > 0 && pathSegments[index - 1] === 'compose' && !isNaN(Number(segment));
									const isDatabaseParam = index > 0 && pathSegments[index - 1] === 'database' && !isNaN(Number(segment));

									if ((segment === 'app' || segment === 'database' || segment === 'compose') && index > 0 && !isNaN(Number(pathSegments[index - 1]))) {
										path = `/projects/${pathSegments[index - 1]}`;
									}

									const label = isProjectParam ? (
										<ProjectNameBreadcrumb id={Number(segment)} />
									) : isAppParam ? (
										<AppNameBreadcrumb id={Number(segment)} />
									) : isComposeParam ? (
										<ComposeNameBreadcrumb id={Number(segment)} />
									) : isDatabaseParam ? (
										<DatabaseNameBreadcrumb id={Number(segment)} />
									) : segment === 'app' ? (
										'Application'
									) : (
										segment.charAt(0).toUpperCase() + segment.slice(1)
									);

									return (
										<React.Fragment key={`${path}-${index}`}>
											<span className="text-muted-foreground/30 font-normal">/</span>
											{isLast ? (
												<span className="text-foreground font-bold">{label}</span>
											) : (
												<Link
													to={path as unknown as '.'}
													className="text-muted-foreground hover:text-foreground transition-colors">
													{label}
												</Link>
											)}
										</React.Fragment>
									);
								})}
							</div>
						</div>
					</div>
				</header>
				<main className="flex-1 p-6">
					<Outlet />
				</main>
			</SidebarInset>
		</SidebarProvider>
	);
}
