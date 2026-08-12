import { Link } from '@tanstack/react-router';
import { FolderOpen, Layers2, ChevronRight, RefreshCw } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { StatusBadge } from '#/components/shared/status-badge';
import type { ComposeResponse } from '#/types/api-helpers';

import {toast} from 'sonner';

interface ComposeHeaderProps {
	id: string;
	compose: ComposeResponse | null;
	activeTab: string;
	setActiveTab: (tab: string) => void;
	refetch: () => void;
	tabs: readonly string[];
}

export function ComposeHeader({
	id,
	compose,
	activeTab,
	setActiveTab,
	refetch,
	tabs,
}: ComposeHeaderProps) {
	return (
		<header className="border-b border-border/40">
			{/* Breadcrumb */}
			<div className="flex items-center gap-1.5 text-xs text-muted-foreground mb-3.5 font-semibold">
				<Link to="/projects" className="hover:text-foreground flex items-center gap-1 transition-colors">
					<FolderOpen className="w-3.5 h-3.5" /> Projects
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<Link to={`/projects/${id}` as any} className="hover:text-foreground transition-colors">
					Project Details
				</Link>
				<ChevronRight className="w-3 h-3 opacity-40" />
				<span className="text-foreground font-bold">{compose?.name || compose?.app_name}</span>
			</div>

			{/* Title row */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
				<div>
					<h1 className="text-2xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text">
						{compose?.name || compose?.app_name}
					</h1>
					<p className="text-xs text-muted-foreground font-mono mt-1">{compose?.app_name || compose?.name}</p>
				</div>

				<div className="flex items-center gap-2">
					<Button variant="outline" size="icon" onClick={() => { refetch(); toast.success('Compose stack refreshed'); }} className="w-8 h-8 border-border rounded-lg">
						<RefreshCw className="w-3.5 h-3.5" />
					</Button>
					<StatusBadge status={compose?.compose_status || ''} />
					<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border border-border text-muted-foreground bg-muted/20 font-semibold select-none">
						<Layers2 className="w-3.5 h-3.5" /> Docker Compose
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
