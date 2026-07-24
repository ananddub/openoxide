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

export const Route = createFileRoute('/_app')({
	component: AppLayout,
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
									const path = `/${pathSegments.slice(0, index + 1).join('/')}`;
									const isLast = index === pathSegments.length - 1;
									const label =
										segment.charAt(0).toUpperCase() + segment.slice(1);

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
