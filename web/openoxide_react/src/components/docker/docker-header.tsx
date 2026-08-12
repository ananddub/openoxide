import {Package, RefreshCw, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import type {RemoteServerResponse} from '#/types/api-helpers';

interface DockerHeaderProps {
	totalContainers: number;
	runningContainers: number;
	onRefresh: () => void;
	isRefreshing: boolean;
	servers?: RemoteServerResponse[];
	selectedServerId: string;
	onSelectServer: (id: string) => void;
}

export function DockerHeader({
	totalContainers,
	runningContainers,
	onRefresh,
	isRefreshing,
	servers = [],
	selectedServerId,
	onSelectServer,
}: DockerHeaderProps) {
	const availableServers = [
		{id: 'local', name: 'Local Server', ip_address: '127.0.0.1'},
		...(servers || []).filter(
			(s: RemoteServerResponse) =>
				String(s.id) !== 'local' && !String(s.name).toLowerCase().includes('local')
		),
	];

	const selectedServer = availableServers.find((s) => String(s.id) === String(selectedServerId));

	return (
		<div className="pb-4 border-b border-border/40 shrink-0">
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				{/* Title Section */}
				<div className="flex items-center gap-3">
					<Package className="size-6 text-muted-foreground self-center shrink-0" />
					<div>
						<h1 className="text-base font-bold text-foreground tracking-tight">Docker Containers</h1>
						<p className="text-xs text-muted-foreground mt-0.5">
							Manage and inspect system-wide Docker containers running on the selected host
						</p>
					</div>
				</div>

				{/* Action Toolbar */}
				<div className="flex items-center gap-2.5 flex-wrap">
					{/* Running Count Badge */}
					<Badge variant="secondary" className="h-9 px-3.5 text-xs font-mono border border-border/40 font-semibold">
						Running: <span className="text-emerald-500 font-bold ml-1">{runningContainers}</span> / {totalContainers}
					</Badge>

					{/* Refresh Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}
						className="h-9 text-xs font-medium border-border/60 gap-2 cursor-pointer shadow-2xs">
						<RefreshCw className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`} /> Refresh
					</Button>

					{/* Server Selector Dropdown */}
					<Select value={selectedServerId} onValueChange={(v) => v && onSelectServer(v)}>
						<SelectTrigger className="w-[170px] h-9 text-xs font-medium bg-card border-border/60 gap-2 shrink-0 shadow-2xs">
							<Server className="size-3.5 text-muted-foreground shrink-0" />
							<SelectValue>{selectedServer?.name || 'Local Server'}</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card border-border text-xs w-[190px] p-1 shadow-md">
							{availableServers.map((s) => (
								<SelectItem key={String(s.id)} value={String(s.id)} className="text-xs font-medium cursor-pointer">
									{s.name} <span className="text-[10px] text-muted-foreground font-mono">({(s as Record<string, unknown>).ip_address as string || 'local'})</span>
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
