import {Link} from '@tanstack/react-router';
import {Logo} from '#/components/shared/logo';
import {buttonVariants} from '#/components/ui/button';
import {ChevronLeft, Construction} from 'lucide-react';
import {useAuthStore} from '#/stores/auth-store';
import {
	SidebarInset,
	SidebarProvider,
	SidebarTrigger,
} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';
import {Separator} from '#/components/ui/separator';

export const NotFound = () => {
	const isAuth = useAuthStore(state => state.isAuth);

	if (isAuth) {
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
					<header className="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-4 border-b border-border/40 bg-background/80 px-6 backdrop-blur-md">
						<div className="flex items-center gap-2">
							<SidebarTrigger className="-ml-1" />
							<Separator orientation="vertical" className="mx-2 h-4" />
							<span className="text-xs font-bold text-foreground">Page Under Construction</span>
						</div>
					</header>
					<main className="flex-1 flex flex-col items-center justify-center p-6 text-center animate-in fade-in duration-200">
						<div className="size-14 rounded-2xl bg-primary/10 flex items-center justify-center text-primary shadow-sm border border-primary/20 mb-3">
							<Construction className="size-7" />
						</div>
						<h2 className="text-lg font-bold tracking-tight text-foreground mb-1">
							Page Under Construction
						</h2>
						<p className="text-xs text-muted-foreground max-w-sm mb-4">
							This feature is currently being crafted. Your sidebar remains fully active and accessible.
						</p>
						<Link
							to="/"
							className="px-4 py-2 text-xs font-semibold rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all shadow-sm">
							Return to Dashboard
						</Link>
					</main>
				</SidebarInset>
			</SidebarProvider>
		);
	}

	return (
		<div className="flex h-screen flex-col bg-background text-foreground">
			<div className="mx-auto flex size-full max-w-200 flex-col">
				{/* Header */}
				<header className="z-50 mb-auto flex w-full justify-center py-6">
					<nav className="px-4 sm:px-6 lg:px-8" aria-label="Global">
						<Link to="/" className="flex flex-row items-center gap-2">
							<Logo className="size-8" />
							<span className="text-base font-semibold tracking-tight">
								Goploy
							</span>
						</Link>
					</nav>
				</header>
				{/* Main Content */}
				<main id="content">
					<div className="px-4 py-10 text-center sm:px-6 lg:px-8">
						<h1 className="block text-7xl font-bold tracking-tight text-primary select-none sm:text-9xl">
							404
						</h1>
						<p className="mt-3 text-sm font-medium text-muted-foreground sm:text-base">
							Sorry, we couldn't find your page.
						</p>
						<div className="mt-6 flex flex-col items-center justify-center gap-2 sm:flex-row sm:gap-3">
							<Link
								to="/"
								className={buttonVariants({
									variant: 'secondary',
									className: 'flex flex-row gap-2 items-center',
								})}>
								<ChevronLeft className="size-4 shrink-0" />
								Go to homepage
							</Link>
						</div>
					</div>
				</main>
				{/* Footer */}
				<footer className="mt-auto py-6 text-center">
					<div className="max-w-8xl mx-auto px-4 sm:px-6 lg:px-8">
						<p className="text-xs text-muted-foreground/80">
							<a
								href="https://github.com/vajra-labs/goploy/issues"
								target="_blank"
								rel="noreferrer"
								className="underline transition-colors hover:text-primary">
								Submit issue on Github
							</a>
						</p>
					</div>
				</footer>
			</div>
		</div>
	);
};
