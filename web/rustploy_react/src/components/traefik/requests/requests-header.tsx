import {ArrowDownUp, RefreshCw, Zap, Server} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
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
		...servers.map((s) => ({
			id: s.id.toString(),
			name: s.name,
			ip_address: s.ip_address || 'Remote',
		})),
	];

	const selectedServer = availableServers.find((s) => s.id === selectedServerId);

	return (
		<div className="pb-3 border-b border-border/40 shrink-0">
			<div className="flex flex-col xl:flex-row xl:items-center justify-between gap-4">
				<div className="flex items-center gap-3">
					<ArrowDownUp className="size-6 text-muted-foreground self-center shrink-0" />
					<div>
						<h1 className="text-base font-bold text-foreground tracking-tight">Requests</h1>
						<p className="text-xs text-muted-foreground mt-0.5">
							See all the incoming HTTP requests that pass through Traefik
						</p>
					</div>
				</div>

				<div className="flex items-center gap-3 flex-wrap">
					{/* Log Cleanup Schedule Group */}
					<div className="flex items-center gap-2">
						<Input
							value={cronInput}
							onChange={(e) => onCronChange(e.target.value)}
							placeholder="0 0 * * *"
							className="w-28 h-9 text-xs font-mono px-3 bg-card border-border/60 shadow-2xs"
						/>
						<Button
							variant="outline"
							size="sm"
							onClick={() => toast.success('Log cleanup schedule updated')}
							className="h-9 text-xs font-medium border-border/60 shadow-2xs cursor-pointer">
							Update Schedule
						</Button>
					</div>

					{/* Activate / Deactivate Button */}
					<Button
						variant={isActive ? 'destructive' : 'default'}
						size="sm"
						onClick={() => onToggle(!isActive)}
						disabled={isToggling}
						className="h-9 px-4 text-xs font-semibold gap-2 shadow-xs cursor-pointer">
						{isToggling ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : isActive ? (
							'Deactivate'
						) : (
							<>
								<Zap className="size-3.5 fill-primary-foreground" /> Activate Requests
							</>
						)}
					</Button>

					{/* Server Selector Dropdown */}
					{onSelectServer && (
						<Select value={selectedServerId} onValueChange={(v) => v && onSelectServer(v)}>
							<SelectTrigger className="w-[170px] h-9 text-xs font-medium bg-card border-border/60 gap-2 shrink-0 shadow-2xs">
								<Server className="size-3.5 text-muted-foreground shrink-0" />
								<SelectValue>{selectedServer?.name || 'Local Server'}</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card border-border text-xs w-[190px] p-1 shadow-md">
								{availableServers.map((srv) => (
									<SelectItem key={srv.id} value={srv.id} className="text-xs font-medium cursor-pointer">
										{srv.name} <span className="text-[10px] text-muted-foreground font-mono">({srv.ip_address})</span>
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
