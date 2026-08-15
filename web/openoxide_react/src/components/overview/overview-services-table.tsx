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
	if (type === 'APP') return <Box className="size-5 text-foreground shrink-0" />;
	if (type === 'COMPOSE') return <Layers2 className="size-5 text-foreground shrink-0" />;
	const kind = (dbKind || '').toLowerCase();
	if (kind.includes('postgres')) return <PostgresqlIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mysql')) return <MysqlIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mariadb')) return <MariadbIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mongo')) return <MongodbIcon className="size-5.5 shrink-0" />;
	if (kind.includes('redis')) return <RedisIcon className="size-5.5 shrink-0" />;
	if (kind.includes('libsql')) return <LibsqlIcon className="size-5.5 shrink-0" />;

	return <DbIcon className="size-5 text-foreground shrink-0" />;
};

const renderStatusBadge = (statusStr: string) => {
	const s = (statusStr || 'done').toLowerCase();

	if (s.includes('stopping') || s.includes('cancelling')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-orange-500/10 text-xs font-semibold text-orange-500">
				<span className="size-2 rounded-full bg-orange-500 animate-pulse shrink-0" />
				<span>Stopping</span>
			</div>
		);
	}

	if (s.includes('starting') || s.includes('deploying') || s.includes('building') || s.includes('loading')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-amber-500/10 text-xs font-semibold text-amber-500">
				<span className="size-2 rounded-full bg-amber-500 animate-pulse shrink-0" />
				<span>Deploying</span>
			</div>
		);
	}

	if (s.includes('running') || s.includes('active') || s.includes('healthy') || s.includes('up') || s === 'done') {
		return (
			<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/10 text-xs font-semibold text-emerald-500">
				<span className="size-2 rounded-full bg-emerald-500 shrink-0" />
				<span>Running</span>
			</div>
		);
	}

	if (s.includes('err') || s.includes('fail') || s.includes('unhealthy') || s.includes('crash')) {
		return (
			<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-rose-500/10 text-xs font-semibold text-rose-500">
				<span className="size-2 rounded-full bg-rose-500 shrink-0" />
				<span>Error</span>
			</div>
		);
	}

	return (
		<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-muted/60 text-xs font-semibold text-muted-foreground">
			<span className="size-2 rounded-full bg-muted-foreground/40 shrink-0" />
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
		<div className="rounded-xl border border-border/60 bg-card overflow-y-auto max-h-[calc(100vh-340px)] min-h-[280px] shadow-xs">
			<Table>
				<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
					<TableRow className="border-b border-border/60 hover:bg-transparent">
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Service</TableHead>
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Type</TableHead>
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Status</TableHead>
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Server</TableHead>
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Created</TableHead>
						<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Last Deploy</TableHead>
						<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Actions</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{services.map((svc) => (
						<TableRow key={svc.key} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
							<TableCell className="py-3.5 px-4">
								<Link
									to="/projects/$id"
									params={{ id: String(svc.projectId) }}
									preload="intent"
									className="flex items-center gap-3 group"
								>
									{renderIcon(svc.type, svc.dbKind)}
									<div className="flex flex-col min-w-0">
										<span className="font-bold text-sm text-foreground group-hover:text-primary transition-colors truncate">
											{svc.name}
										</span>
										<span className="text-xs text-muted-foreground truncate">
											{svc.projectName} / {svc.environmentName}
										</span>
									</div>
								</Link>
							</TableCell>
							<TableCell className="py-3.5 px-4 text-xs font-semibold text-foreground">
								{svc.dbKind ? TYPE_DISPLAY_NAMES[svc.dbKind] || svc.dbKind : TYPE_DISPLAY_NAMES[svc.type] || svc.type}
							</TableCell>
							<TableCell className="py-3.5 px-4">{renderStatusBadge(svc.status)}</TableCell>
							<TableCell className="py-3.5 px-4">
								<div className="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
									<ServerIcon className="size-4 text-muted-foreground/70 shrink-0" />
									<span className="truncate">{svc.serverName || 'Rustploy Server'}</span>
								</div>
							</TableCell>
							<TableCell className="py-3.5 px-4">{formatDate(svc.createdAt)}</TableCell>
							<TableCell className="py-3.5 px-4">{formatDate(svc.lastDeployAt || svc.createdAt)}</TableCell>
							<TableCell className="py-3.5 px-4 text-right">
								<DropdownMenu>
									<DropdownMenuTrigger className="size-8 inline-flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer outline-none">
										<MoreHorizontal className="size-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuLabel className="truncate text-xs font-bold px-2 py-1.5 border-b border-border/40 mb-1">
											{svc.name}
										</DropdownMenuLabel>
										<DropdownMenuItem
											onClick={() => onDeploy?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold"
										>
											<RefreshCw className="size-4 shrink-0 text-muted-foreground" />
											<span>Deploy</span>
										</DropdownMenuItem>
										<DropdownMenuItem
											onClick={() => onStart?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold text-emerald-500 focus:text-emerald-500"
										>
											<Play className="size-4 shrink-0 fill-current" />
											<span>Start</span>
										</DropdownMenuItem>
										<DropdownMenuItem
											onClick={() => onStop?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold text-orange-500 focus:text-orange-500"
										>
											<Square className="size-4 shrink-0 fill-current" />
											<span>Stop</span>
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</TableCell>
						</TableRow>
					))}
				</TableBody>
			</Table>
		</div>
	);
};
