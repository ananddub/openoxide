import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Globe2, RefreshCw, Plus, Server} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface SwarmHeaderProps {
	servers: RemoteServerResponse[];
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
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
			{/* Left: Icon, Title & Description */}
			<div className="flex items-center gap-3">
				<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
					<Globe2 className="w-4 h-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base font-semibold text-foreground leading-none">Docker Swarm</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Manage Swarm cluster nodes, managers, workers, and join tokens
					</p>
				</div>
			</div>

			{/* Right: Engine Selector + Refresh + Add Node Button */}
			<div className="flex items-center gap-2 sm:ml-auto flex-wrap sm:flex-nowrap">
				{/* Host Engine Dropdown Select */}
				<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
					<SelectTrigger size="sm" className="h-8 w-52 text-xs">
						<div className="flex items-center gap-1.5 truncate">
							<Server className="w-3.5 h-3.5 text-primary shrink-0" />
							<SelectValue placeholder="Select Engine" />
						</div>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="all">All Clusters &amp; Remote Servers</SelectItem>
						<SelectItem value="local">Local Server (Docker Engine)</SelectItem>
						{servers.map((srv: RemoteServerResponse) => (
							<SelectItem key={srv.id} value={String(srv.id)}>
								Server: {srv.name} ({srv.ip_address})
							</SelectItem>
						))}
					</SelectContent>
				</Select>

				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 text-xs gap-1.5 cursor-pointer"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin' : ''}`} />
					Refresh
				</Button>

				{isSwarmActive && (
					<Button
						size="sm"
						onClick={onOpenTokens}
						className="h-8 text-xs gap-1.5 cursor-pointer"
					>
						<Plus className="w-3.5 h-3.5" />
						Add Node
					</Button>
				)}
			</div>
		</div>
	);
}
