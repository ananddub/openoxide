import {ArrowRight} from 'lucide-react';
import {buttonVariants} from '#/components/ui/button';
import {Link} from '@tanstack/react-router';

interface HomeHeaderProps {
	firstName?: string;
}

export function HomeHeader({firstName}: HomeHeaderProps) {
	const greeting = firstName ? `Welcome back, ${firstName}` : 'Welcome back';

	return (
		<div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between pb-4 border-b border-border/30">
			<div>
				<h1 className="text-3xl font-extrabold tracking-tight text-foreground">
					{greeting}
				</h1>
				<p className="text-xs text-muted-foreground mt-1 font-medium">
					Infrastructure overview & service status metrics
				</p>
			</div>

			<Link
				to="/projects"
				className={buttonVariants({
					variant: 'secondary',
					size: 'default',
					className: 'w-fit cursor-pointer gap-2 font-semibold border border-border/60 hover:bg-muted/80 shadow-2xs',
				})}>
				Go to projects
				<ArrowRight className="size-4" />
			</Link>
		</div>
	);
}
