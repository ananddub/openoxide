import {useState} from 'react';
import {Card, CardContent} from '#/components/ui/card';
import {Button} from '#/components/ui/button';
import {
	ShieldCheck,
	Server,
	Network,
	ShieldAlert,
	LogOut,
	KeyRound,
	Copy,
	Check,
} from 'lucide-react';
import {toast} from 'sonner';

interface SwarmInfoCardProps {
	info?: {
		node_id?: string;
		node_addr?: string;
		local_node_state?: string;
		control_available?: boolean;
		nodes?: number;
		managers?: number;
	} | null;
	tokens?: {worker?: string; manager?: string} | null;
	isTokensExpanded?: boolean;
	isLoading: boolean;
	onLeaveSwarm: () => void;
}

export function SwarmInfoCard({
	info,
	tokens,
	isTokensExpanded = false,
	isLoading,
	onLeaveSwarm,
}: SwarmInfoCardProps) {
	const [copiedKey, setCopiedKey] = useState<string | null>(null);

	const handleCopy = (type: 'worker' | 'manager', token?: string) => {
		if (!token) {
			toast.error('Token not available');
			return;
		}
		const cmd = `docker swarm join --token ${token}`;
		navigator.clipboard.writeText(cmd);
		setCopiedKey(type);
		toast.success(`Copied ${type} join command`);
		setTimeout(() => setCopiedKey(null), 2000);
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3 py-4">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-20 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	const isActive = (info?.local_node_state || '').toLowerCase() === 'active';

	if (!isActive) {
		return (
			<Card className="bg-card border-border shadow-sm p-8 text-center flex flex-col items-center justify-center rounded-xl my-4">
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
		<div className="flex flex-col gap-3 py-3">
			{/* Top Overview Cards Grid */}
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

			{/* Expandable Join Tokens Section (Toggled from 3-dots header menu) */}
			{isTokensExpanded && (
				<Card className="bg-card border-border/80 rounded-xl p-4 shadow-sm flex flex-col gap-3">
					<div className="flex items-center justify-between border-b border-border/40 pb-2">
						<span className="text-xs font-bold text-foreground flex items-center gap-2">
							<KeyRound className="w-4 h-4 text-primary" />
							Swarm Join Tokens
						</span>
					</div>

					<div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
						{/* Worker Token */}
						<div className="flex flex-col gap-1.5 bg-muted/30 p-3 rounded-lg border border-border/50">
							<div className="flex items-center justify-between">
								<span className="font-semibold text-foreground">Worker Join Command</span>
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleCopy('worker', tokens?.worker)}
									className="h-6 text-[11px] font-medium gap-1 px-2"
								>
									{copiedKey === 'worker' ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
									{copiedKey === 'worker' ? 'Copied' : 'Copy'}
								</Button>
							</div>
							<code className="font-mono text-[11px] break-all select-all text-muted-foreground bg-background p-2 rounded border border-border/60">
								{tokens?.worker ? `docker swarm join --token ${tokens.worker}` : 'Token loading or unavailable'}
							</code>
						</div>

						{/* Manager Token */}
						<div className="flex flex-col gap-1.5 bg-muted/30 p-3 rounded-lg border border-border/50">
							<div className="flex items-center justify-between">
								<span className="font-semibold text-foreground">Manager Join Command</span>
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleCopy('manager', tokens?.manager)}
									className="h-6 text-[11px] font-medium gap-1 px-2"
								>
									{copiedKey === 'manager' ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3" />}
									{copiedKey === 'manager' ? 'Copied' : 'Copy'}
								</Button>
							</div>
							<code className="font-mono text-[11px] break-all select-all text-muted-foreground bg-background p-2 rounded border border-border/60">
								{tokens?.manager ? `docker swarm join --token ${tokens.manager}` : 'Token loading or unavailable'}
							</code>
						</div>
					</div>
				</Card>
			)}
		</div>
	);
}
