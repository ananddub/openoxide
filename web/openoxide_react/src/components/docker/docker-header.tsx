import {Server} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Titlebar} from '#/components/shared/titlebar';
import {RefreshCw} from 'lucide-react';
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
		<Titlebar
			breadcrumbs={[{label: 'Docker Containers'}]}
			icon={
				<span className="relative flex items-center justify-center">
					<span
						className={`absolute -top-0.5 -right-0.5 size-2 rounded-full ${
							runningContainers > 0
								? 'animate-pulse bg-emerald-500'
								: 'bg-muted-foreground/40'
						}`}
					/>
					<span className="text-foreground/70">
						<Server className="size-5" strokeWidth={1.7} />
					</span>
				</span>
			}
			title="Docker Containers"
			description="Manage and inspect system-wide Docker containers running on the selected host"
			titleSuffix={
				<Badge
					variant="secondary"
					className="h-6 gap-1 px-2 font-mono text-[11px] font-medium">
					Running:
					<span className="font-semibold text-emerald-500">
						{runningContainers}
					</span>
					/ {totalContainers}
				</Badge>
			}
			actions={
				<>
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}>
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Select
						value={selectedServerId}
						onValueChange={v => v && onSelectServer(v)}>
						<SelectTrigger className="h-8 w-[170px] gap-2 bg-card text-xs font-medium">
							<Server className="size-3.5 shrink-0 text-muted-foreground" />
							<SelectValue>
								{selectedServer?.name || 'Local Server'}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="w-[190px] bg-card p-1 text-xs">
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
				</>
			}
		/>
	);
}
