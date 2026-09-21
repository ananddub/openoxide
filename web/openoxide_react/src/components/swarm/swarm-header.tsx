import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {RefreshCw, Plus, Server} from 'lucide-react';
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
		<div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			{/* Right: Engine Selector + Refresh + Add Node Button */}
			<div className="flex flex-wrap items-center gap-2 sm:ml-auto sm:flex-nowrap">
				{/* Host Engine Dropdown Select */}
				<Select
					value={selectedServerId}
					onValueChange={val => val && onSelectServer(val)}>
					<SelectTrigger size="sm" className="h-8 w-52 text-xs">
						<div className="flex items-center gap-1.5 truncate">
							<Server className="h-3.5 w-3.5 shrink-0 text-primary" />
							<SelectValue placeholder="Select Engine" />
						</div>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="all">
							All Clusters &amp; Remote Servers
						</SelectItem>
						<SelectItem value="local">
							Local Server (Docker Engine)
						</SelectItem>
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
					className="h-8 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`h-3.5 w-3.5 ${isRefreshing ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>

				{isSwarmActive && (
					<Button
						size="sm"
						onClick={onOpenTokens}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						<Plus className="h-3.5 w-3.5" />
						Add Node
					</Button>
				)}
			</div>
		</div>
	);
}
