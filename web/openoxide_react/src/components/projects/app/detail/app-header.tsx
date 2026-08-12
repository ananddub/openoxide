import { Link } from '@tanstack/react-router';
import { FolderOpen, Box, ChevronRight, RefreshCw } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { StatusBadge } from '#/components/shared/status-badge';
import type { ApplicationResponse } from '#/types/api-helpers';

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
		<header className="border-b border-border/30 pb-1">
			{/* Breadcrumb */}
			<div className="flex items-center gap-1.5 text-xs text-muted-foreground mb-3 font-medium">
				<Link to="/projects" className="hover:text-foreground flex items-center gap-1 transition-colors">
					<FolderOpen className="w-3.5 h-3.5" /> Projects
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<Link to={`/projects/${id}` as any} className="hover:text-foreground transition-colors">
					Project Details
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<span className="text-foreground font-bold">{app?.name || app?.app_name}</span>
			</div>

			{/* Title row */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				<div>
					<h1 className="text-2xl font-extrabold tracking-tight text-foreground">
						{app?.name || app?.app_name}
					</h1>
					<p className="text-xs text-muted-foreground font-mono mt-1">{app?.app_name || app?.name}</p>
				</div>

				<div className="flex items-center gap-2.5">
					<Button
						variant="outline"
						size="icon"
						onClick={() => { refetch(); toast.success('Application status refreshed'); }}
						className="w-8 h-8 border-border/40 bg-muted/20 dark:bg-muted/15 hover:bg-muted/30 rounded-lg shadow-2xs">
						<RefreshCw className="w-3.5 h-3.5" />
					</Button>
					<StatusBadge status={app?.app_status || ''} />
					<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border border-border/40 text-muted-foreground bg-muted/20 dark:bg-muted/15 font-mono select-none">
						<Box className="w-3.5 h-3.5 text-primary" /> Application
					</span>
				</div>
			</div>

			{/* Tabs Navigation Bar */}
			<div className="flex overflow-x-auto mt-6 scrollbar-none gap-2 border-b border-border/30 w-full -mb-[1px]">
				{tabs.map(tab => {
					const isActive = activeTab === tab;
					return (
						<button
							key={tab}
							onClick={() => setActiveTab(tab)}
							className={`px-3.5 pb-2.5 pt-2 text-xs font-semibold whitespace-nowrap border-b-2 transition-all duration-150 -mb-[1px] cursor-pointer ${
								isActive
									? 'border-primary text-primary font-bold'
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
