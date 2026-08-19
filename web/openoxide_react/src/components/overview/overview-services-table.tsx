import React from 'react';
import {Link} from '@tanstack/react-router';
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
	if (type === 'APP')
		return <Box className="size-5 shrink-0 text-foreground" />;
	if (type === 'COMPOSE')
		return <Layers2 className="size-5 shrink-0 text-foreground" />;
	const kind = (dbKind || '').toLowerCase();
	if (kind.includes('postgres'))
		return <PostgresqlIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mysql'))
		return <MysqlIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mariadb'))
		return <MariadbIcon className="size-5.5 shrink-0" />;
	if (kind.includes('mongo'))
		return <MongodbIcon className="size-5.5 shrink-0" />;
	if (kind.includes('redis'))
		return <RedisIcon className="size-5.5 shrink-0" />;
	if (kind.includes('libsql'))
		return <LibsqlIcon className="size-5.5 shrink-0" />;

	return <DbIcon className="size-5 shrink-0 text-foreground" />;
};

const renderStatusBadge = (statusStr: string) => {
	const s = (statusStr || 'done').toLowerCase();

	if (s.includes('queued')) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-amber-500/10 px-2.5 py-1 text-xs font-semibold text-amber-500">
				<span className="size-2 shrink-0 animate-pulse rounded-full bg-amber-500" />
				<span>Queued</span>
			</div>
		);
	}

	if (s.includes('cancelling')) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-amber-500/10 px-2.5 py-1 text-xs font-semibold text-amber-500">
				<span className="size-2 shrink-0 animate-pulse rounded-full bg-amber-500" />
				<span>Cancelling</span>
			</div>
		);
	}

	if (s.includes('stopping')) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-orange-500/10 px-2.5 py-1 text-xs font-semibold text-orange-500">
				<span className="size-2 shrink-0 animate-pulse rounded-full bg-orange-500" />
				<span>Stopping</span>
			</div>
		);
	}

	if (s.includes('starting')) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-amber-500/10 px-2.5 py-1 text-xs font-semibold text-amber-500">
				<span className="size-2 shrink-0 animate-pulse rounded-full bg-amber-500" />
				<span>Starting</span>
			</div>
		);
	}

	if (
		s.includes('deploying') ||
		s.includes('building') ||
		s.includes('loading')
	) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-amber-500/10 px-2.5 py-1 text-xs font-semibold text-amber-500">
				<span className="size-2 shrink-0 animate-pulse rounded-full bg-amber-500" />
				<span>Deploying</span>
			</div>
		);
	}

	if (s.includes('cancelled')) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-zinc-500/10 px-2.5 py-1 text-xs font-semibold text-zinc-400">
				<span className="size-2 shrink-0 rounded-full bg-zinc-400" />
				<span>Cancelled</span>
			</div>
		);
	}

	if (
		s.includes('running') ||
		s.includes('active') ||
		s.includes('healthy') ||
		s.includes('up') ||
		s === 'done'
	) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-emerald-500/10 px-2.5 py-1 text-xs font-semibold text-emerald-500">
				<span className="size-2 shrink-0 rounded-full bg-emerald-500" />
				<span>Running</span>
			</div>
		);
	}

	if (
		s.includes('err') ||
		s.includes('fail') ||
		s.includes('unhealthy') ||
		s.includes('crash')
	) {
		return (
			<div className="inline-flex items-center gap-1.5 rounded-full bg-rose-500/10 px-2.5 py-1 text-xs font-semibold text-rose-500">
				<span className="size-2 shrink-0 rounded-full bg-rose-500" />
				<span>Error</span>
			</div>
		);
	}

	return (
		<div className="inline-flex items-center gap-1.5 rounded-full bg-zinc-500/10 px-2.5 py-1 text-xs font-semibold text-zinc-400">
			<span className="size-2 shrink-0 rounded-full bg-zinc-400" />
			<span>Stopped</span>
		</div>
	);
};

const formatDate = (timestamp?: number) => {
	if (!timestamp) return <span className="text-muted-foreground">—</span>;
	const d = new Date(timestamp * 1000);
	return (
		<span className="font-mono text-xs text-muted-foreground">
			{d.toLocaleDateString(undefined, {
				month: 'short',
				day: 'numeric',
				year: 'numeric',
			})}
		</span>
	);
};

export const OverviewServicesTable: React.FC<Props> = ({
	services,
	onDeploy,
	onStart,
	onStop,
}) => {
	return (
		<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border border-border/60 bg-card shadow-xs">
			<Table>
				<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
					<TableRow className="border-b border-border/60 hover:bg-transparent">
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Service
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Type
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Status
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Server
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Created
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Last Deploy
						</TableHead>
						<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
							Actions
						</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{services.map(svc => (
						<TableRow
							key={svc.key}
							className="border-b border-border/40 transition-colors hover:bg-muted/40">
							<TableCell className="px-4 py-3.5">
								<Link
									to="/projects/$id"
									params={{id: String(svc.projectId)}}
									preload="intent"
									className="group flex items-center gap-3">
									{renderIcon(svc.type, svc.dbKind)}
									<div className="flex min-w-0 flex-col">
										<span className="truncate text-sm font-bold text-foreground transition-colors group-hover:text-primary">
											{svc.name}
										</span>
										<span className="truncate text-xs text-muted-foreground">
											{svc.projectName} / {svc.environmentName}
										</span>
									</div>
								</Link>
							</TableCell>
							<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
								{svc.dbKind
									? TYPE_DISPLAY_NAMES[svc.dbKind] || svc.dbKind
									: TYPE_DISPLAY_NAMES[svc.type] || svc.type}
							</TableCell>
							<TableCell className="px-4 py-3.5">
								{renderStatusBadge(svc.status)}
							</TableCell>
							<TableCell className="px-4 py-3.5">
								<div className="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
									<ServerIcon className="size-4 shrink-0 text-muted-foreground/70" />
									<span className="truncate">
										{svc.serverName || 'Rustploy Server'}
									</span>
								</div>
							</TableCell>
							<TableCell className="px-4 py-3.5">
								{formatDate(svc.createdAt)}
							</TableCell>
							<TableCell className="px-4 py-3.5">
								{formatDate(svc.lastDeployAt || svc.createdAt)}
							</TableCell>
							<TableCell className="px-4 py-3.5 text-right">
								<DropdownMenu>
									<DropdownMenuTrigger className="inline-flex size-8 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors outline-none hover:bg-muted hover:text-foreground">
										<MoreHorizontal className="size-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuLabel className="mb-1 truncate border-b border-border/40 px-2 py-1.5 text-xs font-bold">
											{svc.name}
										</DropdownMenuLabel>
										<DropdownMenuItem
											onClick={() => onDeploy?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold">
											<RefreshCw className="size-4 shrink-0 text-muted-foreground" />
											<span>Deploy</span>
										</DropdownMenuItem>
										<DropdownMenuItem
											onClick={() => onStart?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold text-emerald-500 focus:text-emerald-500">
											<Play className="size-4 shrink-0 fill-current" />
											<span>Start</span>
										</DropdownMenuItem>
										<DropdownMenuItem
											onClick={() => onStop?.(svc)}
											className="cursor-pointer gap-2.5 text-xs font-semibold text-orange-500 focus:text-orange-500">
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
