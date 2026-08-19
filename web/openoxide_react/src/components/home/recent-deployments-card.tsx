import {Rocket, Server} from 'lucide-react';
import {Link} from '@tanstack/react-router';

export type DeploymentStatus = 'idle' | 'running' | 'done' | 'error';

export interface RecentDeploymentItem {
	deploymentId: string;
	name: string;
	projectName: string;
	environment: string;
	serverName: string;
	status: DeploymentStatus;
	createdAt: string;
	href?: string;
}

const statusDotClass: Record<string, string> = {
	done: 'bg-emerald-500',
	running: 'bg-amber-500 animate-pulse',
	error: 'bg-rose-500',
	idle: 'bg-muted-foreground/40',
};

function formatDistanceToNow(dateStr: string): string {
	try {
		const date = new Date(dateStr);
		const diffSec = Math.floor((Date.now() - date.getTime()) / 1000);
		if (isNaN(diffSec) || diffSec < 0) return 'just now';
		if (diffSec < 60) return `${diffSec}s ago`;
		const diffMin = Math.floor(diffSec / 60);
		if (diffMin < 60) return `${diffMin}m ago`;
		const diffHour = Math.floor(diffMin / 60);
		if (diffHour < 24) return `${diffHour}h ago`;
		const diffDays = Math.floor(diffHour / 24);
		return `${diffDays}d ago`;
	} catch {
		return 'recently';
	}
}

interface RecentDeploymentsCardProps {
	deployments: RecentDeploymentItem[];
	isLoading?: boolean;
	canReadDeployments?: boolean;
}

export function RecentDeploymentsCard({
	deployments = [],
	isLoading,
	canReadDeployments = true,
}: RecentDeploymentsCardProps) {
	return (
		<div className="flex shrink-0 flex-col overflow-hidden rounded-xl border border-border bg-card">
			{/* Header */}
			<div className="flex shrink-0 items-center justify-between border-b border-border/40 px-5 py-4">
				<div className="flex items-center gap-2">
					<Rocket className="size-4 text-muted-foreground" />
					<h2 className="text-sm font-semibold text-foreground">
						Recent deployments
					</h2>
				</div>
				{canReadDeployments && (
					<Link
						to={'/deployments' as any}
						className="text-xs font-medium text-muted-foreground transition-colors hover:text-foreground">
						view all →
					</Link>
				)}
			</div>

			{/* Scrollable Deployments List */}
			{!canReadDeployments ? (
				<div className="flex flex-col items-center justify-center gap-3 p-10 py-12 text-center text-sm text-muted-foreground">
					<Rocket className="size-8 opacity-40" />
					<span>You do not have permission to view deployments.</span>
				</div>
			) : isLoading && deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 p-10 py-12 text-center text-sm text-muted-foreground">
					<Rocket className="size-8 animate-pulse opacity-40" />
					<span>Loading recent deployments...</span>
				</div>
			) : deployments.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 p-10 py-12 text-center text-sm text-muted-foreground">
					<Rocket className="size-8 opacity-40" />
					<span>No deployments yet.</span>
				</div>
			) : (
				<div className="max-h-[550px] min-h-[380px] overflow-y-auto">
					<ul className="divide-y divide-border/30">
						{deployments.map(d => {
							const status = d.status ?? 'idle';
							return (
								<li key={d.deploymentId}>
									<Link
										to={'/deployments' as any}
										className="flex items-center gap-4 px-5 py-3.5 transition-colors hover:bg-muted/30">
										<span
											className={`size-2 shrink-0 rounded-full ${statusDotClass[status] ?? statusDotClass.idle}`}
											aria-hidden
										/>
										<div className="flex min-w-0 flex-1 flex-col">
											<span className="truncate text-xs font-bold text-foreground">
												{d.name}
											</span>
											<span className="truncate font-mono text-[11px] text-muted-foreground">
												{d.projectName} · {d.environment}
											</span>
										</div>
										<span className="hidden w-32 items-center justify-end gap-1.5 truncate font-mono text-xs text-muted-foreground lg:flex">
											<Server className="size-3 shrink-0 text-muted-foreground/70" />
											<span className="truncate">{d.serverName}</span>
										</span>
										<span className="hidden w-20 text-right font-mono text-xs text-muted-foreground capitalize sm:inline">
											{status}
										</span>
										<span className="hidden w-24 text-right font-mono text-xs text-muted-foreground md:inline">
											{formatDistanceToNow(d.createdAt)}
										</span>
										<span className="shrink-0 text-xs font-semibold text-primary transition-colors hover:underline">
											logs →
										</span>
									</Link>
								</li>
							);
						})}
					</ul>
				</div>
			)}
		</div>
	);
}
