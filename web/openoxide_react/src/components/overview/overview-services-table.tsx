import React from 'react';
import { Link } from '@tanstack/react-router';
import {
	GlobeIcon,
	CircuitBoard,
	ServerIcon,
	MoreHorizontal,
	RefreshCw,
	Ban,
	Database,
} from 'lucide-react';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import { Button } from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
} from '#/components/ui/dropdown';

export type OverviewServiceItem = {
	key: string;
	id: number;
	name: string;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	status: string;
	createdAt: number;
	projectId: number;
	projectName: string;
	environmentId: number;
	environmentName: string;
	dbKind?: string;
	serverName?: string;
	lastDeployAt?: number;
};

type Props = {
	services: OverviewServiceItem[];
	onDeploy?: (service: OverviewServiceItem) => void;
	onStop?: (service: OverviewServiceItem) => void;
};

const TYPE_DISPLAY_NAMES: Record<string, string> = {
	APP: 'Application',
	COMPOSE: 'Compose',
	DATABASE: 'Database',
	postgres: 'PostgreSQL',
	mysql: 'MySQL',
	mariadb: 'MariaDB',
	mongo: 'MongoDB',
	redis: 'Redis',
	libsql: 'Libsql',
};

const renderIcon = (type: string, dbKind?: string) => {
	if (type === 'COMPOSE') return <CircuitBoard className="size-5 text-primary" />;
	if (type === 'DATABASE' || dbKind) return <Database className="size-5 text-amber-500" />;
	return <GlobeIcon className="size-5 text-blue-500" />;
};

const renderStatusBadge = (statusStr: string) => {
	const s = (statusStr || 'done').toLowerCase();
	let dotColor = 'bg-emerald-500';
	let label = 'Running';

	if (s.includes('run') || s.includes('build') || s.includes('deploy')) {
		dotColor = 'bg-amber-500 animate-pulse';
		label = 'Deploying';
	} else if (s.includes('err') || s.includes('fail') || s.includes('crash')) {
		dotColor = 'bg-destructive';
		label = 'Error';
	} else if (s.includes('idle') || s.includes('stop')) {
		dotColor = 'bg-muted-foreground/40';
		label = 'Idle';
	}

	return (
		<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-muted/60 text-[11px] font-semibold text-foreground">
			<span className={`size-2 rounded-full ${dotColor}`} />
			<span>{label}</span>
		</div>
	);
};

const formatDate = (timestamp?: number) => {
	if (!timestamp) return <span className="text-muted-foreground">—</span>;
	const d = new Date(timestamp * 1000);
	return (
		<span className="text-xs text-muted-foreground font-mono">
			{d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })}
		</span>
	);
};

export const OverviewServicesTable: React.FC<Props> = ({ services, onDeploy, onStop }) => {
	return (
		<Table>
			<TableHeader>
				<TableRow className="border-b border-border/60 hover:bg-transparent">
					<TableHead className="font-bold text-foreground">Service</TableHead>
					<TableHead className="font-bold text-foreground">Type</TableHead>
					<TableHead className="font-bold text-foreground">Status</TableHead>
					<TableHead className="font-bold text-foreground">Server</TableHead>
					<TableHead className="font-bold text-foreground">Created</TableHead>
					<TableHead className="font-bold text-foreground">Last Deploy</TableHead>
					<TableHead className="text-right font-bold text-foreground">Actions</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				{services.map((svc) => (
					<TableRow key={svc.key} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
						<TableCell>
							<Link
								to="/projects/$id"
								params={{ id: String(svc.projectId) }}
								className="flex items-center gap-2.5 group"
							>
								{renderIcon(svc.type, svc.dbKind)}
								<div className="flex flex-col min-w-0">
									<span className="font-semibold text-xs text-foreground group-hover:text-primary transition-colors truncate">
										{svc.name}
									</span>
									<span className="text-[11px] text-muted-foreground truncate">
										{svc.projectName} / {svc.environmentName}
									</span>
								</div>
							</Link>
						</TableCell>
						<TableCell className="text-xs font-medium text-foreground">
							{svc.dbKind ? TYPE_DISPLAY_NAMES[svc.dbKind] || svc.dbKind : TYPE_DISPLAY_NAMES[svc.type] || svc.type}
						</TableCell>
						<TableCell>{renderStatusBadge(svc.status)}</TableCell>
						<TableCell>
							<div className="flex items-center gap-1.5 text-xs text-muted-foreground">
								<ServerIcon className="size-3.5" />
								<span className="truncate">{svc.serverName || 'Rustploy Server'}</span>
							</div>
						</TableCell>
						<TableCell>{formatDate(svc.createdAt)}</TableCell>
						<TableCell>{formatDate(svc.lastDeployAt || svc.createdAt)}</TableCell>
						<TableCell className="text-right">
							<DropdownMenu>
								<DropdownMenuTrigger
									render={
										<Button variant="ghost" size="icon" className="size-8 p-0 text-muted-foreground hover:text-foreground">
											<MoreHorizontal className="size-4 text-foreground" />
										</Button>
									}
								/>
								<DropdownMenuContent align="end" className="w-40 border border-border bg-popover/95 shadow-md">
									<DropdownMenuLabel className="truncate text-xs font-bold">{svc.name}</DropdownMenuLabel>
									<DropdownMenuItem
										onClick={() => onDeploy?.(svc)}
										className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5"
									>
										<RefreshCw className="size-3.5" />
										Deploy
									</DropdownMenuItem>
									<DropdownMenuItem
										onClick={() => onStop?.(svc)}
										className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-orange-500 focus:text-orange-500"
									>
										<Ban className="size-3.5" />
										Stop
									</DropdownMenuItem>
								</DropdownMenuContent>
							</DropdownMenu>
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	);
};
