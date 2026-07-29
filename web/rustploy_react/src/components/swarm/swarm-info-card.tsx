import {Card, CardContent} from '#/components/ui/card';
import {Button} from '#/components/ui/button';
import {
	ShieldCheck,
	Server,
	Network,
	ShieldAlert,
	LogOut,
} from 'lucide-react';

interface SwarmInfoCardProps {
	info?: {
		node_id?: string;
		node_addr?: string;
		local_node_state?: string;
		control_available?: boolean;
		nodes?: number;
		managers?: number;
	} | null;
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
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-16 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	const isActive = (info?.local_node_state || '').toLowerCase() === 'active';

	if (!isActive) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 py-16 text-center border border-dashed border-border rounded-lg">
				<div className="w-12 h-12 rounded-full bg-amber-500/10 flex items-center justify-center text-amber-500">
					<ShieldAlert className="w-5 h-5" />
				</div>
				<div>
					<p className="text-sm font-medium text-foreground">Docker Swarm Inactive</p>
					<p className="text-xs text-muted-foreground mt-0.5">
						This host engine is running in standalone mode. Initialize Docker Swarm or join an existing manager node.
					</p>
				</div>
			</div>
		);
	}

	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
			{/* Cluster State */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Cluster State</p>
						<div className="flex items-center gap-1.5 mt-1">
							<span className="w-2 h-2 rounded-full bg-emerald-500" />
							<h4 className="text-base font-bold text-foreground capitalize leading-tight">
								{info?.local_node_state}
							</h4>
						</div>
					</div>
					<div className="w-8 h-8 rounded-md bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center shrink-0">
						<ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />
					</div>
				</CardContent>
			</Card>

			{/* Total Swarm Nodes */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Swarm Nodes</p>
						<h4 className="text-base font-bold text-foreground mt-1 leading-tight">
							{info?.nodes || 0} Total
						</h4>
					</div>
					<div className="w-8 h-8 rounded-md bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
						<Server className="w-3.5 h-3.5 text-primary" />
					</div>
				</CardContent>
			</Card>

			{/* Managers */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div>
						<p className="text-xs text-muted-foreground">Managers</p>
						<h4 className="text-base font-bold text-foreground mt-1 leading-tight">
							{info?.managers || 0} Active
						</h4>
					</div>
					<div className="w-8 h-8 rounded-md bg-sky-500/10 border border-sky-500/20 flex items-center justify-center shrink-0">
						<Network className="w-3.5 h-3.5 text-sky-500" />
					</div>
				</CardContent>
			</Card>

			{/* Node IP & Role */}
			<Card className="shadow-none">
				<CardContent className="flex items-center justify-between p-4">
					<div className="min-w-0 flex-1">
						<p className="text-xs text-muted-foreground">Node IP / Role</p>
						<h4 className="text-xs font-bold font-mono text-foreground mt-1 truncate leading-tight">
							{info?.node_addr || '127.0.0.1'} ({info?.control_available ? 'Manager' : 'Worker'})
						</h4>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onLeaveSwarm}
						title="Leave Swarm Cluster"
						className="h-8 w-8 text-rose-500 hover:text-rose-600 hover:bg-rose-500/10 shrink-0 ml-2 cursor-pointer"
					>
						<LogOut className="w-3.5 h-3.5" />
					</Button>
				</CardContent>
			</Card>
		</div>
	);
}
