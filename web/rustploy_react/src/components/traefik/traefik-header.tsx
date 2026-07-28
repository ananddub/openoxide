import {Globe, Server, RefreshCw, GitCompare, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import type {RemoteServerItem, TraefikHealthResponse} from './traefik-types';

interface TraefikHeaderProps {
	selectedServerId: string;
	onSelectServer: (id: string) => void;
	servers: RemoteServerItem[];
	health: TraefikHealthResponse | null;
	isCheckingHealth: boolean;
	onCheckHealth: () => void;
	onOpenDiff: () => void;
	onSave: () => void;
	isSaving: boolean;
	isDirty: boolean;
	isReadOnly: boolean;
}

export function TraefikHeader({
	selectedServerId,
	onSelectServer,
	servers,
	health,
	isCheckingHealth,
	onCheckHealth,
	onOpenDiff,
	onSave,
	isSaving,
	isDirty,
	isReadOnly,
}: TraefikHeaderProps) {
	const availableServers = [
		{id: 'local', name: 'Local Server', ip_address: '127.0.0.1'},
		...(servers || []).map((s) => ({
			id: s.id.toString(),
			name: s.name,
			ip_address: s.ip_address || 'Remote',
		})),
	];

	const selectedServer = availableServers.find((s) => s.id === selectedServerId);

	return (
		<div className="pb-3 border-b border-border/40 shrink-0">
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
				{/* Title Header */}
				<div className="flex items-center gap-3">
					<Globe className="size-6 text-muted-foreground shrink-0" />
					<div>
						<h1 className="text-base font-bold text-foreground tracking-tight">Traefik Engine</h1>
						<p className="text-xs text-muted-foreground mt-0.5">
							Manage dynamic configuration files and routing rules
						</p>
					</div>
				</div>

				{/* Toolbar Actions */}
				<div className="flex items-center gap-2.5 flex-wrap">
					{/* Health Check Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onCheckHealth}
						disabled={isCheckingHealth}
						title="Check Traefik health status"
						className="h-9 text-xs font-medium border-border/60 gap-2 cursor-pointer shadow-2xs">
						{isCheckingHealth ? (
							<RefreshCw className="size-3.5 animate-spin text-muted-foreground" />
						) : health?.is_healthy ? (
							<>
								<span className="size-2 rounded-full bg-emerald-500 animate-pulse shrink-0" />
								<span className="text-emerald-500 font-semibold">Healthy</span>
							</>
						) : (
							<>
								<span className="size-2 rounded-full bg-rose-500 shrink-0" />
								<span className="text-rose-500 font-semibold">Offline</span>
							</>
						)}
					</Button>

					{/* Config Diff Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onOpenDiff}
						title="View configuration diff"
						className="h-9 text-xs font-medium border-border/60 gap-1.5 cursor-pointer shadow-2xs">
						<GitCompare className="size-3.5 text-muted-foreground" /> Diff
					</Button>

					{/* Save Button */}
					<Button
						variant="default"
						size="sm"
						onClick={onSave}
						disabled={isSaving || !isDirty || isReadOnly}
						className="h-9 px-4 text-xs font-semibold gap-1.5 shadow-2xs cursor-pointer">
						{isSaving ? <RefreshCw className="size-3.5 animate-spin" /> : <Save className="size-3.5" />}
						Save File
					</Button>

					{/* Server Selector Dropdown */}
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
				</div>
			</div>
		</div>
	);
}
