import React from 'react';
import { Link } from '@tanstack/react-router';
import {
	Box,
	Layers2,
	Database as DbIcon,
	ServerIcon,
	MoreHorizontal,
	RefreshCw,
	Play,
	Square,
} from 'lucide-react';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';
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
	onStart?: (service: OverviewServiceItem) => void;
	onStop?: (service: OverviewServiceItem) => void;
};

const TYPE_DISPLAY_NAMES: Record<string, string> = {
	APP: 'Application',
	COMPOSE: 'Compose Stacks',
	DATABASE: 'Database',
	postgres: 'PostgreSQL',
	mysql: 'MySQL',
	mariadb: 'MariaDB',
	mongo: 'MongoDB',
	redis: 'Redis',
	libsql: 'Libsql',
};

const renderIcon = (type: string, dbKind?: string) => {
	if (type === 'APP') return <Box className="size-4.5 text-foreground shrink-0" />;
	if (type === 'COMPOSE') return <Layers2 className="size-4.5 text-foreground shrink-0" />;
	const kind = (dbKind || '').toLowerCase();
	if (kind.includes('postgres')) return <PostgresqlIcon className="size-5 shrink-0" />;
	if (kind.includes('mysql')) return <MysqlIcon className="size-5 shrink-0" />;
	if (kind.includes('mariadb')) return <MariadbIcon className="size-5 shrink-0" />;
	if (kind.includes('mongo')) return <MongodbIcon className="size-5 shrink-0" />;
	if (kind.includes('redis')) return <RedisIcon className="size-5 shrink-0" />;
	if (kind.includes('libsql')) return <LibsqlIcon className="size-5 shrink-0" />;

	return <DbIcon className="size-4.5 text-foreground shrink-0" />;
};

const renderStatusBadge = (statusStr: string) => {
	const s = (statusStr || 'done').toLowerCase();

	if (s.includes('stopping') || s.includes('cancelling')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-orange-500/10 text-[11px] font-semibold text-orange-500">
				<span className="size-2 rounded-full bg-orange-500 animate-pulse" />
				<span>Stopping</span>
			</div>
		);
	}

	if (s.includes('starting') || s.includes('deploying') || s.includes('building') || s.includes('loading')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-amber-500/10 text-[11px] font-semibold text-amber-500">
				<span className="size-2 rounded-full bg-amber-500 animate-pulse" />
				<span>Deploying</span>
			</div>
		);
	}

	if (s.includes('running') || s.includes('active') || s.includes('healthy') || s.includes('up') || s === 'done') {
		return (
			<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-emerald-500/10 text-[11px] font-semibold text-emerald-500">
				<span className="size-2 rounded-full bg-emerald-500" />
				<span>Running</span>
			</div>
		);
	}

	if (s.includes('err') || s.includes('fail') || s.includes('unhealthy') || s.includes('crash')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-rose-500/10 text-[11px] font-semibold text-rose-500">
				<span className="size-2 rounded-full bg-rose-500" />
				<span>Error</span>
			</div>
		);
	}

	return (
		<div className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-muted/60 text-[11px] font-semibold text-muted-foreground">
			<span className="size-2 rounded-full bg-muted-foreground/40" />
			<span>Idle</span>
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

export const OverviewServicesTable: React.FC<Props> = ({ services, onDeploy, onStart, onStop }) => {
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
										onClick={() => onStart?.(svc)}
										className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-emerald-500 focus:text-emerald-500"
									>
										<Play className="size-3.5 fill-current" />
										Start
									</DropdownMenuItem>
									<DropdownMenuItem
										onClick={() => onStop?.(svc)}
										className="flex items-center gap-2 cursor-pointer text-xs font-medium py-1.5 text-orange-500 focus:text-orange-500"
									>
										<Square className="size-3.5 fill-current" />
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
