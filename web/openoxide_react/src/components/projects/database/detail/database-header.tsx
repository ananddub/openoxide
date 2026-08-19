import {Link} from '@tanstack/react-router';
import {
	FolderOpen,
	Database,
	ChevronRight,
	RefreshCw,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import type {DatabaseResponse} from '#/types/api-helpers';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

import {toast} from 'sonner';

interface DatabaseHeaderProps {
	id: string;
	database: DatabaseResponse | null;
	detectedKind: string;
	actionLoading: string | null;
	isBuilding: boolean;
	activeTab: string;
	setActiveTab: (tab: string) => void;
	refetch: () => void;
	onOpenDeleteDialog: () => void;
	onAction?: (
		action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel',
	) => Promise<void>;
	tabs: readonly string[];
}

export function DatabaseHeader({
	id,
	database,
	detectedKind,
	actionLoading,
	isBuilding,
	activeTab,
	setActiveTab,
	refetch,
	onOpenDeleteDialog,
	onAction,
	tabs,
}: DatabaseHeaderProps) {
	const kind = (
		database?.kind ||
		(database as any)?.db_kind ||
		detectedKind ||
		''
	).toLowerCase();
	const statusStr = (
		database?.status ||
		database?.app_status ||
		''
	).toUpperCase();
	const isRunning = [
		'RUNNING',
		'DONE',
		'HEALTHY',
		'SUCCESS',
		'ACTIVE',
		'OK',
	].includes(statusStr);

	const getIcon = () => {
		if (kind.includes('postgres'))
			return <PostgresqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mysql'))
			return <MysqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mariadb'))
			return <MariadbIcon className="size-6 shrink-0" />;
		if (kind.includes('mongo'))
			return <MongodbIcon className="size-6 shrink-0" />;
		if (kind.includes('redis'))
			return <RedisIcon className="size-6 shrink-0" />;
		if (kind.includes('libsql'))
			return <LibsqlIcon className="size-6 shrink-0" />;
		return <Database className="size-5 text-muted-foreground" />;
	};

	return (
		<header className="border-b border-border/40 pb-0">
			{/* Breadcrumb */}
			<div className="mb-3.5 flex items-center gap-1.5 text-xs font-semibold text-muted-foreground">
				<Link
					to="/projects"
					className="flex items-center gap-1 transition-colors hover:text-foreground">
					<FolderOpen className="h-3.5 w-3.5" /> Projects
				</Link>
				<ChevronRight className="h-3 w-3 opacity-40" />
				<Link
					to={`/projects/${id}` as any}
					search={
						(database as any)?.environment_id
							? {env: Number((database as any).environment_id)}
							: undefined
					}
					className="transition-colors hover:text-foreground">
					Project Details
				</Link>
				<ChevronRight className="h-3 w-3 opacity-40" />
				<span className="font-bold text-foreground">
					{database?.name || database?.app_name}
				</span>
			</div>

			{/* Title row */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div className="flex items-center gap-3">
					<div className="flex size-11 shrink-0 items-center justify-center rounded-xl border border-border/80 bg-card shadow-xs">
						{getIcon()}
					</div>
					<div>
						<h1 className="bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text text-2xl font-extrabold tracking-tight text-foreground capitalize">
							{database?.name || database?.app_name}
						</h1>
						<p className="mt-0.5 font-mono text-xs text-muted-foreground">
							{database?.app_name || database?.name}
						</p>
					</div>
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="icon"
						onClick={() => {
							refetch();
							toast.success('Database status refreshed');
						}}
						className="h-8 w-8 rounded-lg border-border"
						title="Refresh">
						<RefreshCw className="h-3.5 w-3.5" />
					</Button>
					<Button
						variant="outline"
						size="icon"
						onClick={onOpenDeleteDialog}
						title="Delete Database"
						className="h-8 w-8 rounded-lg border-destructive/40 text-destructive hover:bg-destructive/10">
						<Trash2 className="h-3.5 w-3.5" />
					</Button>

					<StatusBadge
						status={database?.status || database?.app_status || 'STOPPED'}
						isBuilding={isBuilding}
						actionLoading={actionLoading}
					/>
					<span className="inline-flex items-center gap-1.5 rounded-lg border border-border bg-muted/20 px-2.5 py-1 text-xs font-semibold text-muted-foreground capitalize select-none">
						{kind} Database
					</span>
				</div>
			</div>

			{/* Tabs Navigation Bar */}
			<div className="mt-6 -mb-[1px] flex w-full scrollbar-none gap-2 overflow-x-auto border-b border-border/40">
				{tabs.map(tab => {
					const isActive = activeTab === tab;
					return (
						<button
							key={tab}
							onClick={() => setActiveTab(tab)}
							className={`-mb-[1px] cursor-pointer border-b-2 px-4 pt-2 pb-2.5 text-xs font-bold whitespace-nowrap transition-all duration-150 ${
								isActive
									? 'border-foreground font-extrabold text-foreground'
									: 'border-transparent text-muted-foreground hover:border-border/40 hover:text-foreground'
							}`}>
							{tab}
						</button>
					);
				})}
			</div>
		</header>
	);
}
