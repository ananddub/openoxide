import {Link} from '@tanstack/react-router';
import {FolderOpen, Layers2, ChevronRight, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import type {ComposeResponse} from '#/types/api-helpers';

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
						(compose as any)?.environment_id
							? {env: Number((compose as any).environment_id)}
							: undefined
					}
					className="transition-colors hover:text-foreground">
					Project Details
				</Link>
				<ChevronRight className="h-3 w-3 opacity-40" />
				<span className="font-bold text-foreground">
					{compose?.name || compose?.app_name}
				</span>
			</div>

			{/* Title row */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div>
					<h1 className="bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text text-2xl font-extrabold tracking-tight text-foreground">
						{compose?.name || compose?.app_name}
					</h1>
					<p className="mt-1 font-mono text-xs text-muted-foreground">
						{compose?.app_name || compose?.name}
					</p>
				</div>

				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="icon"
						onClick={() => {
							refetch();
							toast.success('Compose stack refreshed');
						}}
						className="h-8 w-8 rounded-lg border-border">
						<RefreshCw className="h-3.5 w-3.5" />
					</Button>
					<StatusBadge status={compose?.compose_status || ''} />
					<span className="inline-flex items-center gap-1.5 rounded-lg border border-border bg-muted/20 px-2.5 py-1 text-xs font-semibold text-muted-foreground select-none">
						<Layers2 className="h-3.5 w-3.5" /> Docker Compose
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
