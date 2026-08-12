import {Activity, CheckCircle2, Clock, Terminal} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {buttonVariants} from '#/components/ui/button';
import {Link} from '@tanstack/react-router';

export interface ActivityLogItem {
	id: string;
	title: string;
	subtitle: string;
	timestamp: string;
	status: 'success' | 'pending' | 'failed';
}

interface HomeActivityFeedProps {
	activities?: ActivityLogItem[];
}

export function HomeActivityFeed({activities}: HomeActivityFeedProps) {
	const items: ActivityLogItem[] = activities && activities.length > 0 ? activities : [
		{
			id: 'act-1',
			title: 'System Deployment Completed',
			subtitle: 'Traefik reverse proxy configuration active',
			timestamp: '2 mins ago',
			status: 'success',
		},
		{
			id: 'act-2',
			title: 'Container Ingest Metrics Streamed',
			subtitle: 'Host Docker Engine container metrics synchronized',
			timestamp: '15 mins ago',
			status: 'success',
		},
		{
			id: 'act-3',
			title: 'Health Check Verification',
			subtitle: 'Traefik core health check passed (HTTP 200 OK)',
			timestamp: '1 hour ago',
			status: 'success',
		},
	];

	return (
		<div className="flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<h2 className="text-sm font-bold text-foreground tracking-tight flex items-center gap-2">
					<Activity className="size-4 text-primary" /> Platform Activity & System Stream
				</h2>
				<Link
					to="/monitoring"
					className={buttonVariants({
						variant: 'ghost',
						size: 'sm',
						className: 'h-7 text-xs font-semibold px-2 text-primary hover:bg-primary/10 cursor-pointer',
					})}>
					System Logs <Terminal className="size-3 ml-1" />
				</Link>
			</div>

			<div className="border border-border/60 rounded-xl overflow-hidden bg-card/60 backdrop-blur-sm divide-y divide-border/30">
				{items.map((act) => (
					<div key={act.id} className="p-3.5 flex items-center justify-between gap-3 hover:bg-muted/20 transition-colors">
						<div className="flex items-center gap-3">
							<div className="p-2 rounded-lg bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 shrink-0">
								<CheckCircle2 className="size-4" />
							</div>
							<div className="space-y-0.5">
								<h4 className="text-xs font-bold text-foreground">{act.title}</h4>
								<p className="text-[11px] text-muted-foreground font-mono truncate max-w-md">
									{act.subtitle}
								</p>
							</div>
						</div>

						<div className="flex items-center gap-2 shrink-0">
							<Badge variant="outline" className="text-[10px] font-mono text-muted-foreground border-border/40 gap-1 font-medium">
								<Clock className="size-2.5" /> {act.timestamp}
							</Badge>
						</div>
					</div>
				))}
			</div>
		</div>
	);
}
