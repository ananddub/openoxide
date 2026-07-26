import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Globe2, RefreshCw, KeyRound} from 'lucide-react';

interface SwarmHeaderProps {
	servers: any[];
	selectedServerId: string;
	onSelectServer: (id: string) => void;
	onRefresh: () => void;
	onOpenTokens: () => void;
	isRefreshing: boolean;
	isSwarmActive: boolean;
}

export function SwarmHeader({
	servers,
	selectedServerId,
	onSelectServer,
	onRefresh,
	onOpenTokens,
	isRefreshing,
	isSwarmActive,
}: SwarmHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border/50">
			<div className="flex items-center gap-3">
				<div className="p-2.5 rounded-xl bg-primary/10 text-primary">
					<Globe2 className="w-6 h-6" />
				</div>
				<div>
					<h1 className="text-xl font-bold tracking-tight text-foreground">Docker Swarm Cluster</h1>
					<p className="text-xs text-muted-foreground">
						Inspect Swarm cluster state, manager tokens, and node availability
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2">
				{/* Server Node Selector */}
				<div className="w-48 sm:w-56">
					<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
						<SelectTrigger className="h-9 text-xs">
							<SelectValue placeholder="Select Host Node" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border text-xs z-50">
							<SelectItem value="local">Local Node (Engine)</SelectItem>
							{servers.map((srv: any) => (
								<SelectItem key={srv.id} value={String(srv.id)}>
									{srv.name} ({srv.ip_address})
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>

				{isSwarmActive && (
					<Button
						variant="outline"
						size="sm"
						onClick={onOpenTokens}
						className="h-9 text-xs font-semibold gap-1.5"
					>
						<KeyRound className="w-3.5 h-3.5 text-primary" />
						Join Tokens
					</Button>
				)}

				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Swarm Status"
					className="h-9 w-9"
				>
					<RefreshCw className={`w-4 h-4 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
				</Button>
			</div>
		</div>
	);
}
