import {Link} from '@tanstack/react-router';
import {ArrowRight, Zap} from 'lucide-react';

/** "Auto-deploy" / "Get started" hint card at the bottom of the rail. */
export function DeploymentsQuickTip({total}: {total: number}) {
	const hasAny = total > 0;
	return (
		<div className="rounded-xl border bg-muted/30 p-5">
			<div className="mb-3 flex items-center gap-2">
				<Zap className="size-4 text-foreground" />
				<h3 className="text-sm font-semibold text-foreground">
					{hasAny ? 'Auto-deploy' : 'Get started'}
				</h3>
			</div>
			<p className="text-sm leading-relaxed text-muted-foreground">
				{hasAny
					? 'Enable Autodeploy on an application and every git push ships automatically — watch the builds land here in real time.'
					: 'Deploy your first application or database and its build stream will show up here, line by line.'}
			</p>
			<Link
				to="/projects"
				className="mt-3 inline-flex items-center gap-1 text-sm font-medium text-primary transition-colors hover:text-primary/80">
				{hasAny ? 'Go to projects' : 'Browse projects'}
				<ArrowRight className="size-3.5" />
			</Link>
		</div>
	);
}
