import { Link } from '@tanstack/react-router';
import { FolderOpen, Box, ChevronRight, RefreshCw } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { StatusBadge } from '#/components/shared/status-badge';
import type { ApplicationResponse } from '#/types/api-helpers';
import { toast } from 'sonner';

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
			<div className="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
				<Link to="/projects" className="hover:text-foreground flex items-center gap-1 transition-colors">
					<FolderOpen className="size-3.5" /> Projects
				</Link>
				<ChevronRight className="size-3 opacity-40" />
				<Link 
					to={`/projects/${id}` as any} 
					search={(app as any)?.environment_id ? { env: Number((app as any).environment_id) } : undefined}
					className="hover:text-foreground transition-colors">
					Project Details
				</Link>
				<ChevronRight className="size-3 opacity-40" />
				<span className="text-foreground font-bold">{app?.name || app?.app_name}</span>
			</div>

			{/* App Title & Quick Actions Row */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				<div className="flex items-center gap-3">
					<div className="size-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center border border-primary/20 shrink-0">
						<Box className="size-5" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">
							{app?.name || app?.app_name}
						</h1>
						<p className="text-xs text-muted-foreground font-mono mt-0.5">{app?.app_name || app?.name}</p>
					</div>
				</div>

				<div className="flex items-center gap-2.5">
					<Button
						variant="outline"
						size="icon"
						onClick={() => { refetch(); toast.success('Application status refreshed'); }}
						className="size-8 border-border/60 bg-card hover:bg-muted rounded-lg shadow-2xs"
					>
						<RefreshCw className="size-3.5" />
					</Button>
					<StatusBadge status={app?.app_status || ''} />
				</div>
			</div>

			{/* Dokploy Style Tab Bar */}
			<div className="flex overflow-x-auto scrollbar-none gap-6 pt-3 border-b border-border/50 w-full -mb-[1px]">
				{tabs.map((tab) => {
					const isActive = activeTab === tab;
					return (
						<button
							key={tab}
							type="button"
							onClick={() => setActiveTab(tab)}
							className={`pb-3 text-xs font-semibold whitespace-nowrap border-b-2 transition-all duration-150 -mb-[1px] cursor-pointer ${
								isActive
									? 'border-primary text-primary font-bold'
									: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border/60'
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
