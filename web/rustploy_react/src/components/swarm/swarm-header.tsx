import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Globe2, RefreshCw, MoreVertical, Check} from 'lucide-react';

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

			{/* Top Right Controls: Refresh & 3-Dots Menu */}
			<div className="flex items-center gap-2 shrink-0">
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

				{/* Header 3-Dots Dropdown containing Engine Selection & Swarm Actions */}
				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button variant="outline" size="icon" className="h-8 w-8 text-muted-foreground hover:text-foreground shrink-0" />
						}
					>
						<MoreVertical className="w-4 h-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-56 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
						<div className="px-2.5 py-1 text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
							Select Host Engine
						</div>

						<DropdownMenuItem
							className={`flex cursor-pointer items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium ${
								selectedServerId === 'local' ? 'bg-primary/10 text-primary font-bold' : 'hover:bg-muted'
							}`}
							onClick={() => onSelectServer('local')}
						>
							<span>Local Server (Engine)</span>
							{selectedServerId === 'local' && <Check className="w-3.5 h-3.5 text-primary shrink-0" />}
						</DropdownMenuItem>

						{servers.map((srv: any) => {
							const srvIdStr = String(srv.id);
							const isSelected = selectedServerId === srvIdStr;

							return (
								<DropdownMenuItem
									key={srv.id}
									className={`flex cursor-pointer items-center justify-between px-2.5 py-1.5 rounded-lg text-xs font-medium ${
										isSelected ? 'bg-primary/10 text-primary font-bold' : 'hover:bg-muted'
									}`}
									onClick={() => onSelectServer(srvIdStr)}
								>
									<span className="truncate">{srv.name} ({srv.ip_address})</span>
									{isSelected && <Check className="w-3.5 h-3.5 text-primary shrink-0" />}
								</DropdownMenuItem>
							);
						})}

						<DropdownMenuSeparator className="my-1 border-border/50" />

						<DropdownMenuItem
							disabled={!isSwarmActive}
							className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
							onClick={onOpenTokens}
						>
							Join Tokens
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>
		</div>
	);
}
