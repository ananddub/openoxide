import {Card, CardContent} from '#/components/ui/card';
import {Button} from '#/components/ui/button';
import {
	ShieldCheck,
	Server,
	Network,
	ShieldAlert,
	LogOut,
} from 'lucide-react';

import type {SwarmInfo} from '#/types/api-helpers';

interface SwarmInfoCardProps {
	info?: SwarmInfo | null;
	isLoading: boolean;
	onLeaveSwarm: () => void;
}

export function SwarmInfoCard({
	info,
	isLoading,
	onLeaveSwarm,
}: SwarmInfoCardProps) {
	if (isLoading) {
		return (
			<div className="grid grid-cols-1 gap-3 sm:grid-cols-2 md:grid-cols-4">
				{[1, 2, 3, 4].map(i => (
					<div
						key={i}
						className="h-16 animate-pulse rounded-xl border border-border/60 bg-muted/40"
					/>
				))}
			</div>
		);
	}

	const isActive =
		(info?.local_node_state || '').toLowerCase() === 'active';

	if (!isActive) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-border py-16 text-center">
				<div className="flex h-12 w-12 items-center justify-center rounded-full bg-amber-500/10 text-amber-500">
					<ShieldAlert className="h-5 w-5" />
				</div>
				<div>
					<p className="text-sm font-medium text-foreground">
						Docker Swarm Inactive
					</p>
					<p className="mt-0.5 text-xs text-muted-foreground">
						This host engine is running in standalone mode. Initialize
						Docker Swarm or join an existing manager node.
					</p>
				</div>
			</div>
		);
	}

	return (
		<div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
			{/* Cluster State */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Cluster State</p>
						<div className="mt-1 flex items-center gap-1.5">
							<span className="h-2 w-2 rounded-full bg-emerald-500" />
							<h4 className="text-base leading-tight font-bold text-foreground capitalize">
								{info?.local_node_state}
							</h4>
						</div>
					</div>
					<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-emerald-500/20 bg-emerald-500/10">
						<ShieldCheck className="h-3.5 w-3.5 text-emerald-500" />
					</div>
				</CardContent>
			</Card>

			{/* Total Swarm Nodes */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Swarm Nodes</p>
						<h4 className="mt-1 text-base leading-tight font-bold text-foreground">
							{info?.nodes || 0} Total
						</h4>
					</div>
					<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-primary/20 bg-primary/10">
						<Server className="h-3.5 w-3.5 text-primary" />
					</div>
				</CardContent>
			</Card>

			{/* Managers */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Managers</p>
						<h4 className="mt-1 text-base leading-tight font-bold text-foreground">
							{info?.managers || 0} Active
						</h4>
					</div>
					<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-sky-500/20 bg-sky-500/10">
						<Network className="h-3.5 w-3.5 text-sky-500" />
					</div>
				</CardContent>
			</Card>

			{/* Node IP & Role */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div className="min-w-0 flex-1">
						<p className="text-xs text-muted-foreground">Node IP / Role</p>
						<h4 className="mt-1 truncate font-mono text-xs leading-tight font-bold text-foreground">
							{info?.node_addr || '127.0.0.1'} (
							{info?.control_available ? 'Manager' : 'Worker'})
						</h4>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onLeaveSwarm}
						title="Leave Swarm Cluster"
						className="ml-2 h-8 w-8 shrink-0 cursor-pointer text-rose-500 hover:bg-rose-500/10 hover:text-rose-600">
						<LogOut className="h-3.5 w-3.5" />
					</Button>
				</CardContent>
			</Card>
		</div>
	);
}
