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
		<div className="flex items-center justify-between gap-3 pb-3 border-b border-border/50">
			{/* Left: Icon, Title & Host Engine Selector Tightly Grouped */}
			<div className="flex items-center gap-3">
				<div className="p-2 rounded-xl bg-primary/10 text-primary shrink-0">
					<Globe2 className="w-5 h-5" />
				</div>

				<div className="flex items-center gap-2.5">
					<h1 className="text-base font-bold tracking-tight text-foreground whitespace-nowrap">
						Docker Swarm
					</h1>

					{/* Host Engine Dropdown Select Tightly Attached to Title */}
					<div className="w-40">
						<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
							<SelectTrigger className="h-8 text-xs bg-card border-border/80 px-2.5">
								<div className="flex items-center gap-1.5 truncate">
									<Server className="w-3.5 h-3.5 text-primary shrink-0" />
									<SelectValue placeholder="Select Host" />
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
				</div>
			</div>

			{/* Right: Refresh & Direct Join Tokens Toggle Button */}
			<div className="flex items-center gap-2 shrink-0">
				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Swarm Status"
					className="h-8 w-8 shrink-0"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
				</Button>

				{isSwarmActive && (
					<Button
						variant={isTokensExpanded ? 'default' : 'outline'}
						size="sm"
						onClick={onToggleTokens}
						className="h-8 text-xs font-semibold gap-1.5 px-3"
					>
						<KeyRound className="w-3.5 h-3.5" />
						<span>Join Tokens</span>
					</Button>
				)}
			</div>
		</div>
	);
}
