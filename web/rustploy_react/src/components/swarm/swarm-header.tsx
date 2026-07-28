import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
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
	const selectedServer = selectedServerId === 'local' || selectedServerId === 'all' ? null : servers.find(s => String(s.id) === selectedServerId);
	const selectedName = selectedServerId === 'all' ? 'All Clusters & Servers' : selectedServerId === 'local' ? 'Local Server (Engine)' : selectedServer ? `${selectedServer.name} (${selectedServer.ip_address})` : `Server #${selectedServerId}`;

	return (
		<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3 border-b border-border/50">
			{/* Left: Icon, Title & Full-Width Host Engine Selector */}
			<div className="flex items-center gap-3 w-full sm:w-auto">
				<div className="p-2 rounded-xl bg-primary/10 text-primary shrink-0">
					<Globe2 className="w-5 h-5" />
				</div>

				<div className="flex items-center gap-2.5 w-full sm:w-auto">
					<h1 className="text-base font-bold tracking-tight text-foreground whitespace-nowrap">
						Docker Swarm
					</h1>

					{/* Full Width Host Engine Dropdown Select */}
					<div className="w-full sm:w-64">
						<Select value={selectedServerId} onValueChange={val => val && onSelectServer(val)}>
							<SelectTrigger className="!h-8 text-xs bg-card border-border/80 w-full px-3 flex items-center justify-between">
								<div className="flex items-center gap-1.5 truncate">
									<Server className="w-3.5 h-3.5 text-primary shrink-0" />
									<span className="truncate font-semibold text-foreground">{selectedName}</span>
								</div>
							</SelectTrigger>
							<SelectContent className="bg-card border-border text-xs z-50">
								<SelectItem value="all" className="font-bold text-primary">All Clusters & Servers</SelectItem>
								<SelectItem value="local">Local Server (Engine)</SelectItem>
								{servers.map((srv: RemoteServerResponse) => (
									<SelectItem key={srv.id} value={String(srv.id)}>
										{srv.name} ({srv.ip_address})
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>
				</div>
			</div>

			{/* Right: Refresh & Add Node to Cluster Button */}
			<div className="flex items-center gap-2 shrink-0 ml-auto sm:ml-0">
				<Button
					variant="outline"
					size="icon"
					onClick={onRefresh}
					disabled={isRefreshing}
					title="Refresh Swarm Status"
					className="h-8 w-8 shrink-0 cursor-pointer"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin text-primary' : ''}`} />
				</Button>

				{isSwarmActive && (
					<Button
						variant="default"
						size="sm"
						onClick={onOpenTokens}
						className="h-8 text-xs font-bold gap-1.5 px-3.5 bg-primary text-primary-foreground shadow-xs cursor-pointer"
					>
						<Plus className="w-4 h-4" />
						<span>Add Node to Cluster</span>
					</Button>
				)}
			</div>
		</div>
	);
}
