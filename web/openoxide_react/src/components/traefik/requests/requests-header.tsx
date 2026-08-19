import {ArrowDownUp, RefreshCw, Zap, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';
import type {RemoteServerItem} from '../traefik-types';

interface RequestsHeaderProps {
	isActive: boolean;
	isToggling: boolean;
	onToggle: (enable: boolean) => void;
	cronInput: string;
	onCronChange: (val: string) => void;
	selectedServerId: string;
	onSelectServer?: (id: string) => void;
	servers?: RemoteServerItem[];
}

export function RequestsHeader({
	isActive,
	isToggling,
	onToggle,
	cronInput,
	onCronChange,
	selectedServerId,
	onSelectServer,
	servers = [],
}: RequestsHeaderProps) {
	const availableServers = [
		{id: 'local', name: 'Local Server', ip_address: '127.0.0.1'},
		...servers.map(s => ({
			id: s.id.toString(),
			name: s.name,
			ip_address: s.ip_address || 'Remote',
		})),
	];

	const selectedServer = availableServers.find(
		s => s.id === selectedServerId,
	);

	return (
		<div className="shrink-0 border-b border-border/40 pb-3">
			<div className="flex flex-col justify-between gap-4 xl:flex-row xl:items-center">
				<div className="flex items-center gap-3">
					<ArrowDownUp className="size-6 shrink-0 self-center text-muted-foreground" />
					<div>
						<h1 className="text-base font-bold tracking-tight text-foreground">
							Requests
						</h1>
						<p className="mt-0.5 text-xs text-muted-foreground">
							See all the incoming HTTP requests that pass through Traefik
						</p>
					</div>
				</div>

				<div className="flex flex-wrap items-center gap-3">
					{/* Log Cleanup Schedule Group */}
					<div className="flex items-center gap-2">
						<Input
							value={cronInput}
							onChange={e => onCronChange(e.target.value)}
							placeholder="0 0 * * *"
							className="h-9 w-28 border-border/60 bg-card px-3 font-mono text-xs shadow-2xs"
						/>
						<Button
							variant="outline"
							size="sm"
							onClick={() => toast.success('Log cleanup schedule updated')}
							className="h-9 cursor-pointer border-border/60 text-xs font-medium shadow-2xs">
							Update Schedule
						</Button>
					</div>

					{/* Activate / Deactivate Button */}
					<Button
						variant={isActive ? 'destructive' : 'default'}
						size="sm"
						onClick={() => onToggle(!isActive)}
						disabled={isToggling}
						className="h-9 cursor-pointer gap-2 px-4 text-xs font-semibold shadow-xs">
						{isToggling ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : isActive ? (
							'Deactivate'
						) : (
							<>
								<Zap className="size-3.5 fill-primary-foreground" />{' '}
								Activate Requests
							</>
						)}
					</Button>

					{/* Server Selector Dropdown */}
					{onSelectServer && (
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
								{availableServers.map(srv => (
									<SelectItem
										key={srv.id}
										value={srv.id}
										className="cursor-pointer text-xs font-medium">
										{srv.name}{' '}
										<span className="font-mono text-[10px] text-muted-foreground">
											({srv.ip_address})
										</span>
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					)}
				</div>
			</div>
		</div>
	);
}
