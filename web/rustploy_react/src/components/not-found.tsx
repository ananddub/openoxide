import {Link} from '@tanstack/react-router';
import {Logo} from '#/components/shared/logo';
import {buttonVariants} from '#/components/ui/button';
import {ChevronLeft} from 'lucide-react';

export const NotFound = () => {
	const statusCode = 404;
	const appName = 'Goploy';
	const errorTitle = "Sorry, we couldn't find your page.";

	return (
		<div className="flex h-screen flex-col bg-background text-foreground">
			<div className="mx-auto flex size-full max-w-200 flex-col">
				{/* Header */}
				<header className="z-50 mb-auto flex w-full justify-center py-6">
					<nav className="px-4 sm:px-6 lg:px-8" aria-label="Global">
						<Link to="/" className="flex flex-row items-center gap-2">
							<Logo className="size-8" />
							<span className="text-base font-semibold tracking-tight">
								{appName}
							</span>
						</Link>
					</nav>
				</header>
				{/* Main Content */}
				<main id="content">
					<div className="px-4 py-10 text-center sm:px-6 lg:px-8">
						<h1 className="block text-7xl font-bold tracking-tight text-primary select-none sm:text-9xl">
							{statusCode}
						</h1>
						<p className="mt-3 text-sm font-medium text-muted-foreground sm:text-base">
							{errorTitle}
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
