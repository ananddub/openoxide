import {Link} from '@tanstack/react-router';
import {Logo} from '#/components/shared/logo';
import {buttonVariants} from '#/components/ui/button';
import {ChevronLeft, AlertCircle} from 'lucide-react';
import {useAuthSession} from '#/stores/session-context';
import {SidebarProvider} from '#/components/ui/sidebar';
import {AppSidebar} from '#/components/layouts/sidebar';

export const NotFound = () => {
	const isAuth = useAuthSession().isAuth;

	if (isAuth) {
		// Same shell as the dashboard: floating rail + single scrolling main.
		return (
			<SidebarProvider className="h-dvh">
				<AppSidebar />
				<main className="flex min-w-0 flex-1 animate-in flex-col items-center justify-center p-6 text-center duration-200 fade-in">
					<div className="mb-3 flex size-14 items-center justify-center rounded-2xl bg-destructive/10 text-destructive shadow-sm">
						<AlertCircle className="size-7" />
					</div>
					<h2 className="mb-1 text-lg font-bold tracking-tight text-foreground">
						Page Not Found
					</h2>
					<p className="mb-4 max-w-sm text-xs text-muted-foreground">
						The page or resource you are looking for does not exist or
						has been moved.
					</p>
					<Link
						to="/"
						className="rounded-lg bg-primary px-4 py-2 text-xs font-semibold text-primary-foreground shadow-sm transition-all hover:bg-primary/90">
						Return to Overview
					</Link>
				</main>
			</SidebarProvider>
		);
	}

	return (
		<div className="flex h-screen flex-col bg-background text-foreground">
			<div className="mx-auto flex size-full max-w-200 flex-col">
				{/* Header */}
				<header className="z-50 mb-auto flex w-full justify-center py-6">
					<Link
						to="/"
						className="flex items-center gap-2 transition-opacity hover:opacity-80">
						<Logo className="size-8" />
						<span className="text-lg font-bold tracking-tight text-foreground">
							OpenOxide
						</span>
					</Link>
				</header>
				{/* Main Content */}
				<main id="content">
					<div className="mx-auto flex max-w-md flex-col items-center text-center">
						<div className="mb-4 flex size-16 items-center justify-center rounded-2xl bg-destructive/10 text-destructive">
							<AlertCircle className="size-8" />
						</div>
						<h1 className="mb-2 text-3xl font-bold tracking-tight text-foreground">
							404
						</h1>
						<p className="mb-6 text-sm text-muted-foreground">
							The page you are looking for was not found on this
							server.
						</p>
						<Link
							to="/"
							className={buttonVariants({className: 'mb-2'})}>
							Back to Dashboard
						</Link>
					</div>
				</main>
				{/* Footer */}
				<footer className="mt-auto py-6 text-center">
					<div className="flex items-center justify-center gap-4 text-xs text-muted-foreground">
						<a
							href="https://github.com/vajra-labs/goploy"
							target="_blank"
							rel="noopener noreferrer"
							className="transition-colors hover:text-foreground">
							GitHub
						</a>
						<span className="flex items-center gap-1">
							<ChevronLeft className="size-3" />
							OpenOxide Self-Hosted
						</span>
					</div>
				</footer>
			</div>
		</div>
	);
};
