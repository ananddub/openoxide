import * as React from 'react';
import {useAuthStore} from '#/stores/auth-store';
import {createFileRoute, Navigate, Outlet} from '@tanstack/react-router';
import {
	SidebarInset,
	SidebarProvider,
	SidebarTrigger,
} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';

export const Route = createFileRoute('/_app')({
	component: AppLayout,
});

function AppLayout() {
	const isAuth = useAuthStore(state => state.isAuth);

	if (!isAuth) return <Navigate to="/singin" replace />;

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
				{/* Sticky Header */}
				<header className="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-4 border-b border-border/40 bg-background/80 px-6 backdrop-blur-md transition-all duration-200">
					<div className="flex w-full items-center justify-between">
						<div className="flex items-center gap-2">
							<SidebarTrigger className="-ml-1" />
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
