import {Link} from '@tanstack/react-router';
import {FolderOpen, Box, ChevronRight, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import type {ApplicationResponse} from '#/types/api-helpers';
import {toast} from 'sonner';

interface AppHeaderProps {
	id: string;
	app: ApplicationResponse | null;
	activeTab: string;
	setActiveTab: (tab: string) => void;
	refetch: () => void;
	tabs: readonly string[];
}

export function AppHeader({
	id,
	app,
	activeTab,
	setActiveTab,
	refetch,
	tabs,
}: AppHeaderProps) {
	return (
		<header className="flex flex-col gap-4 border-b border-border/40 pb-0">
			{/* Breadcrumb Navigation */}
			<div className="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
				<Link
					to="/projects"
					className="flex items-center gap-1 transition-colors hover:text-foreground">
					<FolderOpen className="size-3.5" /> Projects
				</Link>
				<ChevronRight className="size-3 opacity-40" />
				<Link
					to={`/projects/${id}` as any}
					search={
						(app as any)?.environment_id
							? {env: Number((app as any).environment_id)}
							: undefined
					}
					className="transition-colors hover:text-foreground">
					Project Details
				</Link>
				<ChevronRight className="size-3 opacity-40" />
				<span className="font-bold text-foreground">
					{app?.name || app?.app_name}
				</span>
			</div>

			{/* App Title & Quick Actions Row */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div className="flex items-center gap-3">
					<div className="flex size-10 shrink-0 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
						<Box className="size-5" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">
							{app?.name || app?.app_name}
						</h1>
						<p className="mt-0.5 font-mono text-xs text-muted-foreground">
							{app?.app_name || app?.name}
						</p>
					</div>
				</div>

				<div className="flex items-center gap-2.5">
					<Button
						variant="outline"
						size="icon"
						onClick={() => {
							refetch();
							toast.success('Application status refreshed');
						}}
						className="size-8 rounded-lg border-border/60 bg-card shadow-2xs hover:bg-muted">
						<RefreshCw className="size-3.5" />
					</Button>
					<StatusBadge status={app?.app_status || ''} />
				</div>
			</div>

			{/* Dokploy Style Tab Bar */}
			<div className="-mb-[1px] flex w-full scrollbar-none gap-6 overflow-x-auto border-b border-border/50 pt-3">
				{tabs.map(tab => {
					const isActive = activeTab === tab;
					return (
						<button
							key={tab}
							type="button"
							onClick={() => setActiveTab(tab)}
							className={`-mb-[1px] cursor-pointer border-b-2 pb-3 text-xs font-semibold whitespace-nowrap transition-all duration-150 ${
								isActive
									? 'border-primary font-bold text-primary'
									: 'border-transparent text-muted-foreground hover:border-border/60 hover:text-foreground'
							}`}>
							{tab}
						</button>
					);
				})}
			</div>
		</header>
	);
}
