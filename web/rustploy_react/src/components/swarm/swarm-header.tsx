import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Globe2, RefreshCw, KeyRound, Server} from 'lucide-react';

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
		<div className="flex flex-col gap-3 pb-4 border-b border-border/50">
			{/* Top Row: Title & Cluster Action Controls */}
			<div className="flex items-center justify-between gap-4">
				<div className="flex items-center gap-3">
					<div className="p-2.5 rounded-xl bg-primary/10 text-primary shrink-0">
						<Globe2 className="w-6 h-6" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">Docker Swarm Cluster</h1>
						<p className="text-xs text-muted-foreground">
							Inspect Swarm cluster state, manager tokens, and node availability
						</p>
					</div>
				</div>

				<div className="flex items-center gap-2 shrink-0">
					{isSwarmActive && (
						<Button
							variant="outline"
							size="sm"
							onClick={onOpenTokens}
							className="h-8 text-xs font-semibold gap-1.5"
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
						className="h-8 w-8"
					>
						<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
					</Button>
				</div>
			</div>

			{/* Prominent Target Node Selector Bar */}
			<div className="flex items-center gap-2.5 bg-muted/40 p-2 rounded-xl border border-border/50 text-xs">
				<span className="text-muted-foreground font-semibold px-1.5 flex items-center gap-1.5 shrink-0">
					<Server className="w-3.5 h-3.5 text-primary" />
					Target Node:
				</span>
				<div className="w-72 max-w-full">
					<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
						<SelectTrigger className="h-8 text-xs bg-card border-border/80">
							<SelectValue placeholder="Select Target Host Node" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border text-xs z-50">
							<SelectItem value="local">🖥️ Local Server (Engine)</SelectItem>
							{servers.map((srv: any) => (
								<SelectItem key={srv.id} value={String(srv.id)}>
									🌐 {srv.name} ({srv.username || 'root'}@{srv.ip_address})
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
