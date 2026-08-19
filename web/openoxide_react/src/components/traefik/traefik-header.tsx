import {Globe, Server, RefreshCw, GitCompare, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {
	RemoteServerItem,
	TraefikHealthResponse,
} from './traefik-types';

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
		...(servers || []).map(s => ({
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
			<div className="flex flex-col justify-between gap-3 sm:flex-row sm:items-center">
				{/* Title Header */}
				<div className="flex items-center gap-3">
					<Globe className="size-6 shrink-0 text-muted-foreground" />
					<div>
						<h1 className="text-base font-bold tracking-tight text-foreground">
							Traefik Engine
						</h1>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Manage dynamic configuration files and routing rules
						</p>
					</div>
				</div>

				{/* Toolbar Actions */}
				<div className="flex flex-wrap items-center gap-2.5">
					{/* Health Check Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onCheckHealth}
						disabled={isCheckingHealth}
						title="Check Traefik health status"
						className="h-9 cursor-pointer gap-2 border-border/60 text-xs font-medium shadow-2xs">
						{isCheckingHealth ? (
							<RefreshCw className="size-3.5 animate-spin text-muted-foreground" />
						) : health?.is_healthy ? (
							<>
								<span className="size-2 shrink-0 animate-pulse rounded-full bg-emerald-500" />
								<span className="font-semibold text-emerald-500">
									Healthy
								</span>
							</>
						) : (
							<>
								<span className="size-2 shrink-0 rounded-full bg-rose-500" />
								<span className="font-semibold text-rose-500">
									Offline
								</span>
							</>
						)}
					</Button>

					{/* Config Diff Button */}
					<Button
						variant="outline"
						size="sm"
						onClick={onOpenDiff}
						title="View configuration diff"
						className="h-9 cursor-pointer gap-1.5 border-border/60 text-xs font-medium shadow-2xs">
						<GitCompare className="size-3.5 text-muted-foreground" /> Diff
					</Button>

					{/* Save Button */}
					<Button
						variant="default"
						size="sm"
						onClick={onSave}
						disabled={isSaving || !isDirty || isReadOnly}
						className="h-9 cursor-pointer gap-1.5 px-4 text-xs font-semibold shadow-2xs">
						{isSaving ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : (
							<Save className="size-3.5" />
						)}
						Save File
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
				</div>
			</div>
		</div>
	);
}
