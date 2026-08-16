import { Link } from '@tanstack/react-router';
import { FolderOpen, Database, ChevronRight, RefreshCw, Trash2 } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { StatusBadge } from '#/components/shared/status-badge';
import type { DatabaseResponse } from '#/types/api-helpers';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

import { toast } from 'sonner';

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
	onAction?: (action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel') => Promise<void>;
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
	const kind = (detectedKind || 'postgres').toLowerCase();
	const statusStr = (database?.status || database?.app_status || '').toUpperCase();
	const isRunning = ['RUNNING', 'DONE', 'HEALTHY', 'SUCCESS', 'ACTIVE', 'OK'].includes(statusStr);

	const getIcon = () => {
		if (kind.includes('postgres')) return <PostgresqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mysql')) return <MysqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mariadb')) return <MariadbIcon className="size-6 shrink-0" />;
		if (kind.includes('mongo')) return <MongodbIcon className="size-6 shrink-0" />;
		if (kind.includes('redis')) return <RedisIcon className="size-6 shrink-0" />;
		if (kind.includes('libsql')) return <LibsqlIcon className="size-6 shrink-0" />;
		return <Database className="size-5 text-muted-foreground" />;
	};

	return (
		<header className="border-b border-border/40 pb-0">
			{/* Breadcrumb */}
			<div className="flex items-center gap-1.5 text-xs text-muted-foreground mb-3.5 font-semibold">
				<Link to="/projects" className="hover:text-foreground flex items-center gap-1 transition-colors">
					<FolderOpen className="w-3.5 h-3.5" /> Projects
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<Link 
					to={`/projects/${id}` as any} 
					search={(database as any)?.environment_id ? { env: Number((database as any).environment_id) } : undefined}
					className="hover:text-foreground transition-colors">
					Project Details
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<span className="text-foreground font-bold">{database?.name || database?.app_name}</span>
			</div>

			{/* Title row */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				<div className="flex items-center gap-3">
					<div className="size-11 rounded-xl bg-card border border-border/80 flex items-center justify-center shrink-0 shadow-xs">
						{getIcon()}
					</div>
					<div>
						<h1 className="text-2xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text capitalize">
							{database?.name || database?.app_name}
						</h1>
						<p className="text-xs text-muted-foreground font-mono mt-0.5">{database?.app_name || database?.name}</p>
					</div>
				</div>

				<div className="flex items-center gap-2">
					<Button variant="outline" size="icon" onClick={() => { refetch(); toast.success('Database status refreshed'); }} className="w-8 h-8 border-border rounded-lg" title="Refresh">
						<RefreshCw className="w-3.5 h-3.5" />
					</Button>
					<Button
						variant="outline"
						size="icon"
						onClick={onOpenDeleteDialog}
						title="Delete Database"
						className="w-8 h-8 border-destructive/40 text-destructive hover:bg-destructive/10 rounded-lg">
						<Trash2 className="w-3.5 h-3.5" />
					</Button>

					{/* Quick Lifecycle Control Buttons */}
					{isRunning ? (
						<Button
							size="sm"
							variant="destructive"
							onClick={() => onAction?.('stop')}
							disabled={actionLoading !== null}
							className="h-8 text-xs font-semibold px-3 rounded-lg flex items-center gap-1.5 cursor-pointer disabled:opacity-50"
						>
							{actionLoading === 'stop' ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : null}
							{actionLoading === 'stop' ? 'Stopping...' : 'Stop'}
						</Button>
					) : (
						<Button
							size="sm"
							variant="outline"
							onClick={() => onAction?.('start')}
							disabled={actionLoading !== null}
							className="h-8 text-xs font-semibold px-3 border-border hover:bg-muted text-foreground rounded-lg flex items-center gap-1.5 cursor-pointer disabled:opacity-50"
						>
							{actionLoading === 'start' ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : null}
							{actionLoading === 'start' ? 'Starting...' : 'Start'}
						</Button>
					)}

					<StatusBadge status={database?.status || database?.app_status || 'STOPPED'} isBuilding={isBuilding} actionLoading={actionLoading} />
					<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border border-border text-muted-foreground bg-muted/20 font-semibold select-none capitalize">
						{kind} Database
					</span>
				</div>
			</div>

			{/* Tabs Navigation Bar */}
			<div className="flex overflow-x-auto mt-6 scrollbar-none gap-2 border-b border-border/40 w-full -mb-[1px]">
				{tabs.map(tab => {
					const isActive = activeTab === tab;
					return (
						<button
							key={tab}
							onClick={() => setActiveTab(tab)}
							className={`px-4 pb-2.5 pt-2 text-xs font-bold whitespace-nowrap border-b-2 transition-all duration-150 -mb-[1px] cursor-pointer ${
								isActive
									? 'border-foreground text-foreground font-extrabold'
									: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border/40'
							}`}
						>
							{tab}
						</button>
					);
				})}
			</div>
		</header>
	);
}
