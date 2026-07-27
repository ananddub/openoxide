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
import {$api} from '#/api/query';

function ProjectNameBreadcrumb({ id }: { id: number }) {
	const { data: project } = $api.useQuery(
		'get',
		'/projects/{id}',
		{ params: { path: { id } } }
	);
	return <>{project?.name || 'Loading...'}</>;
}

function AppNameBreadcrumb({ id }: { id: number }) {
	const { data: app } = $api.useQuery(
		'get',
		'/applications/{id}',
		{ params: { path: { id } } }
	);
	return <>{app?.name || app?.app_name || 'Loading...'}</>;
}

function ComposeNameBreadcrumb({ id }: { id: number }) {
	const { data: compose } = $api.useQuery(
		'get',
		'/compose/{id}',
		{ params: { path: { id } } }
	);
	return <>{compose?.name || compose?.app_name || 'Loading...'}</>;
}

import {Construction} from 'lucide-react';

function AppNotFoundPlaceholder() {
	return (
		<div className="flex h-[calc(100vh-10rem)] w-full flex-col items-center justify-center gap-4 text-center p-6 animate-in fade-in duration-200">
			<div className="size-14 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shadow-sm border border-primary/20">
				<Construction className="size-7" />
			</div>
			<div className="max-w-md space-y-1.5">
				<h2 className="text-lg font-bold tracking-tight text-foreground">
					Page Under Construction
				</h2>
				<p className="text-xs text-muted-foreground leading-relaxed">
					This feature is currently being crafted. Your sidebar remains fully active for seamless navigation across all platform tools.
				</p>
			</div>
			<div className="flex items-center gap-3 pt-2">
				<Link
					to="/"
					className="px-4 py-2 text-xs font-semibold rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all shadow-sm">
					Return to Dashboard
				</Link>
			</div>
		</div>
	);
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
									to="/"
									className="text-muted-foreground hover:text-foreground transition-colors">
									Dashboard
								</Link>
								{pathSegments.map((segment, index) => {
									if (segment === 'app') return null;

									const path = `/${pathSegments.slice(0, index + 1).join('/')}`;
									const isLast = index === pathSegments.length - 1;
									const isProjectParam = index > 0 && pathSegments[index - 1] === 'projects' && !isNaN(Number(segment));
									const isAppParam = index > 0 && (pathSegments[index - 1] === 'app' || (index > 1 && pathSegments[index - 2] === 'app')) && !isNaN(Number(segment));
									const isComposeParam = index > 0 && pathSegments[index - 1] === 'compose' && !isNaN(Number(segment));

									const label = isProjectParam ? (
										<ProjectNameBreadcrumb id={Number(segment)} />
									) : isAppParam ? (
										<AppNameBreadcrumb id={Number(segment)} />
									) : isComposeParam ? (
										<ComposeNameBreadcrumb id={Number(segment)} />
									) : (
										segment.charAt(0).toUpperCase() + segment.slice(1)
									);

									return (
										<React.Fragment key={path}>
											<span className="text-muted-foreground/30 font-normal">/</span>
											{isLast ? (
												<span className="text-foreground font-bold">{label}</span>
											) : (
												<Link
													to={path as any}
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
