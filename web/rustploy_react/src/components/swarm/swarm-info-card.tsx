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
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3 py-3">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-20 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	const isActive = (info?.local_node_state || '').toLowerCase() === 'active';

	if (!isActive) {
		return (
			<Card className="bg-card border-border shadow-sm p-8 text-center flex flex-col items-center justify-center rounded-xl my-3">
				<div className="w-12 h-12 rounded-xl bg-amber-500/10 flex items-center justify-center text-amber-500 mb-3">
					<ShieldAlert className="w-6 h-6" />
				</div>
				<h3 className="text-sm font-bold text-foreground">Docker Swarm Inactive</h3>
				<p className="text-xs text-muted-foreground max-w-md mt-1 mb-4">
					This host engine is running in standalone mode. Initialize Docker Swarm or join an existing manager node.
				</p>
			</Card>
		);
	}

	return (
		<div className="py-3">
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
				{/* State */}
				<Card className="bg-card border-border rounded-xl p-3.5 shadow-sm">
					<CardContent className="p-0 flex items-center justify-between">
						<div>
							<p className="text-[11px] font-medium text-muted-foreground">Cluster State</p>
							<div className="flex items-center gap-1.5 mt-1">
								<span className="w-2 h-2 rounded-full bg-emerald-500 shadow-sm shadow-emerald-500/50" />
								<h4 className="text-sm font-bold text-foreground capitalize">{info?.local_node_state}</h4>
							</div>
						</div>
						<div className="p-2 rounded-lg bg-emerald-500/10 text-emerald-500">
							<ShieldCheck className="w-4 h-4" />
						</div>
					</CardContent>
				</Card>

				{/* Total Nodes */}
				<Card className="bg-card border-border rounded-xl p-3.5 shadow-sm">
					<CardContent className="p-0 flex items-center justify-between">
						<div>
							<p className="text-[11px] font-medium text-muted-foreground">Swarm Nodes</p>
							<h4 className="text-sm font-bold text-foreground mt-1">{info?.nodes || 0} Total</h4>
						</div>
						<div className="p-2 rounded-lg bg-primary/10 text-primary">
							<Server className="w-4 h-4" />
						</div>
					</CardContent>
				</Card>

				{/* Managers */}
				<Card className="bg-card border-border rounded-xl p-3.5 shadow-sm">
					<CardContent className="p-0 flex items-center justify-between">
						<div>
							<p className="text-[11px] font-medium text-muted-foreground">Managers</p>
							<h4 className="text-sm font-bold text-foreground mt-1">{info?.managers || 0} Active</h4>
						</div>
						<div className="p-2 rounded-lg bg-sky-500/10 text-sky-500">
							<Network className="w-4 h-4" />
						</div>
					</CardContent>
				</Card>

				{/* Clean Node IP & Role Card */}
				<Card className="bg-card border-border rounded-xl p-3.5 shadow-sm">
					<CardContent className="p-0 flex items-center justify-between">
						<div className="min-w-0 flex-1">
							<p className="text-[11px] font-medium text-muted-foreground">Node IP / Role</p>
							<h4 className="text-xs font-bold font-mono text-foreground mt-1 truncate">
								{info?.node_addr || '127.0.0.1'} ({info?.control_available ? 'Manager' : 'Worker'})
							</h4>
						</div>
						<Button
							variant="ghost"
							size="icon"
							onClick={onLeaveSwarm}
							title="Leave Swarm Cluster"
							className="h-8 w-8 text-rose-500 hover:text-rose-600 hover:bg-rose-500/10 shrink-0 ml-2"
						>
							<LogOut className="w-4 h-4" />
						</Button>
					</CardContent>
				</Card>
			</div>
		</div>
	);
}
