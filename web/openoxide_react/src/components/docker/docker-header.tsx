import {Package, RefreshCw, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
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
				String(s.id) !== 'local' &&
				!String(s.name).toLowerCase().includes('local'),
		),
	];

	const selectedServer = availableServers.find(
		s => String(s.id) === String(selectedServerId),
	);

	return (
		<div className="shrink-0 border-b border-border/40 pb-4">
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				{/* Title Section */}
				<div className="flex items-center gap-3">
					<Package className="size-6 shrink-0 self-center text-muted-foreground" />
					<div>
						<h1 className="text-base font-bold tracking-tight text-foreground">
							Docker Containers
						</h1>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Manage and inspect system-wide Docker containers running on
							the selected host
						</p>
					</div>
				</div>

				{/* Action Toolbar */}
				<div className="flex flex-wrap items-center gap-2.5">
					{/* Running Count Badge */}
					<Badge
						variant="secondary"
						className="h-9 border border-border/40 px-3.5 font-mono text-xs font-semibold">
						Running:{' '}
						<span className="ml-1 font-bold text-emerald-500">
							{runningContainers}
						</span>{' '}
						/ {totalContainers}
					</Badge>

					{/* Refresh Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}
						className="h-9 cursor-pointer gap-2 border-border/60 text-xs font-medium shadow-2xs">
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`}
						/>{' '}
						Refresh
					</Button>

					{/* Server Selector Dropdown */}
					<Select
						value={selectedServerId}
						onValueChange={v => v && onSelectServer(v)}>
						<SelectTrigger className="h-9 w-[170px] shrink-0 gap-2 border-border/60 bg-card text-xs font-medium shadow-2xs">
							<Server className="size-3.5 shrink-0 text-muted-foreground" />
							<SelectValue>
								{selectedServer?.name || 'Local Server'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="w-[190px] border-border bg-card p-1 text-xs shadow-md">
							{availableServers.map(s => (
								<SelectItem
									key={String(s.id)}
									value={String(s.id)}
									className="cursor-pointer text-xs font-medium">
									{s.name}{' '}
									<span className="font-mono text-[10px] text-muted-foreground">
										(
										{((s as Record<string, unknown>)
											.ip_address as string) || 'local'}
										)
									</span>
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
