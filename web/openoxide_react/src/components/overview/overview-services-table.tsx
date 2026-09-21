import React from 'react';
import {Link} from '@tanstack/react-router';
import {
	ServerIcon,
	MoreHorizontal,
	RefreshCw,
	Play,
	Square,
} from 'lucide-react';
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
import {ServiceStatusBadge} from './service-status-badge';
import {ServiceTypeIcon, TYPE_DISPLAY_NAMES} from './service-type-icon';

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

const COLUMNS = [
	'Service',
	'Type',
	'Status',
	'Server',
	'Created',
	'Last Deploy',
	'Actions',
];

const HEAD_CLS =
	'sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md';

const formatDate = (timestamp?: number | bigint) => {
	// Proto int64 timestamps arrive as bigint — coerce before multiplying.
	const seconds = Number(timestamp ?? 0);
	if (!seconds) return <span className="text-muted-foreground">—</span>;
	const d = new Date(seconds * 1000);
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
		<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border bg-card">
			<Table>
				<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
					<TableRow className="border-b  hover:bg-transparent">
						{COLUMNS.map((c, i) => (
							<TableHead
								key={c}
								className={
									HEAD_CLS +
									(i === COLUMNS.length - 1 ? ' text-right' : '')
								}>
								{c}
							</TableHead>
						))}
					</TableRow>
				</TableHeader>
				<TableBody>
					{services.map(svc => (
						<TableRow
							key={svc.key}
							className="border-b  transition-colors hover:bg-muted/40">
							<TableCell className="px-4 py-3.5">
								<Link
									to="/projects/$id"
									params={{id: String(svc.projectId)}}
									preload="intent"
									className="group flex items-center gap-3">
									<ServiceTypeIcon type={svc.type} dbKind={svc.dbKind} />
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
								<ServiceStatusBadge status={svc.status} />
							</TableCell>
							<TableCell className="px-4 py-3.5">
								<div className="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
									<ServerIcon className="size-4 shrink-0 text-muted-foreground/70" />
									<span className="truncate">
										{svc.serverName || 'Localhost (Main server)'}
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
										<DropdownMenuLabel className="mb-1 truncate border-b  px-2 py-1.5 text-xs font-bold">
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
