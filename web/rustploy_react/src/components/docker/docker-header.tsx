import {Package, RefreshCw, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';

interface DockerHeaderProps {
	totalContainers: number;
	runningContainers: number;
	onRefresh: () => void;
	isRefreshing: boolean;
	servers?: any[];
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
	const SelectComponent = Select as any;
	const availableServers = [
		{ id: 'local', name: 'Local Server', ip_address: '127.0.0.1' },
		...(servers || []).filter((s: any) => String(s.id) !== 'local' && !String(s.name).toLowerCase().includes('local')),
	];

	const selectedServer = availableServers.find((s: any) => String(s.id) === String(selectedServerId));

	return (
		<section className="bg-card border border-border/80 rounded-2xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
			<div>
				<h3 className="text-sm font-bold text-foreground flex items-center gap-2 tracking-tight">
					<Package className="w-4 h-4 text-primary" /> Docker Engine Containers
				</h3>
				<p className="text-xs text-muted-foreground mt-0.5">Manage and inspect system-wide Docker containers running on the selected host server</p>
			</div>

			<div className="flex items-center gap-3 flex-wrap">
				{/* Server Switcher Dropdown */}
				<div className="flex items-center gap-2">
					<Server className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
					<SelectComponent value={selectedServerId} onValueChange={onSelectServer}>
						<SelectTrigger className="w-[180px] h-8 text-xs font-semibold bg-card border-border shadow-xs">
							<SelectValue>{selectedServer?.name || 'Local Server'}</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							{availableServers.map((s: any) => (
								<SelectItem key={String(s.id)} value={String(s.id)} className="text-xs font-semibold">
									{s.name} <span className="text-[10px] text-muted-foreground font-mono ml-1">({s.ip_address || 'local'})</span>
								</SelectItem>
							))}
						</SelectContent>
					</SelectComponent>
				</div>

				<Badge variant="outline" className="text-xs font-mono px-3 py-1 h-8 flex items-center gap-1 border-border">
					Running: <span className="text-emerald-400 font-bold ml-1">{runningContainers}</span> / {totalContainers}
				</Badge>

				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 text-xs font-semibold flex items-center gap-1.5 border-border"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin' : ''}`} /> Refresh
				</Button>
			</div>
		</section>
	);
}
