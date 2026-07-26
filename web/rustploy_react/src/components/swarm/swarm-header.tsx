import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Globe2, RefreshCw, MoreVertical, Server} from 'lucide-react';

interface SwarmHeaderProps {
	servers: any[];
	selectedServerId: string;
	onSelectServer: (id: string) => void;
	onRefresh: () => void;
	onToggleTokens: () => void;
	isRefreshing: boolean;
	isSwarmActive: boolean;
	isTokensExpanded: boolean;
}

export function SwarmHeader({
	servers,
	selectedServerId,
	onSelectServer,
	onRefresh,
	onToggleTokens,
	isRefreshing,
	isSwarmActive,
	isTokensExpanded,
}: SwarmHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border/50">
			{/* Title & Subtitle */}
			<div className="flex items-center gap-3">
				<div className="p-2 rounded-xl bg-primary/10 text-primary shrink-0">
					<Globe2 className="w-5 h-5" />
				</div>
				<div>
					<h1 className="text-lg font-bold tracking-tight text-foreground">Docker Swarm Cluster</h1>
					<p className="text-xs text-muted-foreground">
						Inspect Swarm cluster state, manager tokens, and node availability
					</p>
				</div>
			</div>

			{/* Host Engine Dropdown Selector, Refresh & 3-Dots Menu */}
			<div className="flex items-center gap-1.5 shrink-0">
				{/* Host Engine Select Dropdown */}
				<div className="w-44 sm:w-48">
					<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
						<SelectTrigger className="h-8 text-xs bg-card border-border/80">
							<div className="flex items-center gap-1.5 truncate">
								<Server className="w-3.5 h-3.5 text-primary shrink-0" />
								<SelectValue placeholder="Select Host Engine" />
							</div>
						</SelectTrigger>
						<SelectContent className="bg-card border-border text-xs z-50">
							<SelectItem value="local">Local Server (Engine)</SelectItem>
							{servers.map((srv: any) => (
								<SelectItem key={srv.id} value={String(srv.id)}>
									{srv.name} ({srv.ip_address})
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>

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

				{/* Header 3-Dots Dropdown containing ONLY Join Tokens toggle */}
				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button variant="outline" size="icon" className="h-8 w-8 text-muted-foreground hover:text-foreground shrink-0" />
						}
					>
						<MoreVertical className="w-4 h-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-48 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
						<DropdownMenuItem
							disabled={!isSwarmActive}
							className="flex cursor-pointer items-center justify-between px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
							onClick={onToggleTokens}
						>
							<span>Join Tokens</span>
							<span className="text-[10px] text-muted-foreground font-mono">
								{isTokensExpanded ? 'Hide' : 'Show'}
							</span>
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>
		</div>
	);
}
