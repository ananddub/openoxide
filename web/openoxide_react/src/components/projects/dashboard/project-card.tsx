import React from 'react';
import { useNavigate } from '@tanstack/react-router';
import type { components } from '#/types/api.d.ts';
import { Folder, MoreHorizontal, SquarePen, FileText, Trash2 } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import { useAppStore } from '#/stores/app-store';

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({ project, onDelete }) => {
	const navigate = useNavigate();

	// Read real-time overview services from Zustand store for exact service & environment counts
	const overviewServices = useAppStore((state) => state.overviewServices || []);

	const handleCardClick = () => {
		navigate({ to: '/projects/$id', params: { id: String(project.id) } });
	};

	const formatDate = (timestamp?: number) => {
		if (!timestamp) return 'recently';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString();
	};

	// Calculate accurate total services & environments for this project from Zustand RAM store
	const projectServices = overviewServices.filter((s) => Number(s.project_id) === Number(project.id));
	const totalServices = projectServices.length;

	const envIds = new Set(projectServices.map((s) => s.environment_id).filter(Boolean));
	const totalEnvironments = envIds.size > 0 ? envIds.size : 1;

	return (
		<div
			onClick={handleCardClick}
			className="group flex cursor-pointer flex-col justify-between rounded-xl border border-border/80 bg-card p-4.5 shadow-xs transition-all hover:border-primary/50 hover:shadow-md min-h-[175px] max-w-[340px] w-full"
		>
			{/* Top Body Section */}
			<div className="flex flex-col gap-3 min-w-0">
				{/* Card Header */}
				<div className="flex items-start justify-between gap-2.5">
					<div className="flex items-center gap-2.5 min-w-0">
						<div className="flex size-9 items-center justify-center rounded-lg border border-border/60 bg-muted/80 text-foreground transition-colors group-hover:bg-primary/10 group-hover:text-primary shrink-0">
							<Folder className="size-4" />
						</div>
						<div className="flex flex-col min-w-0">
							<h3 className="line-clamp-1 text-sm font-bold text-foreground group-hover:text-primary transition-colors">
								{project.name}
							</h3>
							<span className="text-[11px] text-muted-foreground font-mono mt-0.5">
								Created {formatDate(project.created_at)}
							</span>
						</div>
					</div>

					{/* Action Dropdown Menu */}
					<div onClick={(e) => e.stopPropagation()} className="shrink-0">
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<Button
										variant="ghost"
										size="icon"
										onClick={(e) => e.stopPropagation()}
										className="size-7 text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded-lg focus-visible:ring-0"
									>
										<MoreHorizontal className="size-4" />
									</Button>
								}
							/>
							<DropdownMenuContent align="end" className="w-48 border border-border bg-popover/95 backdrop-blur-md shadow-lg">
								<DropdownMenuItem
									onClick={handleCardClick}
									className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-foreground"
								>
									<SquarePen className="size-4 text-muted-foreground" />
									<span>Update</span>
								</DropdownMenuItem>
								<DropdownMenuItem
									onClick={handleCardClick}
									className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-foreground"
								>
									<FileText className="size-4 text-muted-foreground" />
									<span>Project Environment</span>
								</DropdownMenuItem>
								<DropdownMenuSeparator className="bg-border/60" />
								<DropdownMenuItem
									onClick={() => onDelete(project.id)}
									className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-destructive focus:bg-destructive/10 focus:text-destructive"
								>
									<Trash2 className="size-4 text-destructive" />
									<span>Delete</span>
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>
					</div>
				</div>

				{/* Description */}
				{project.description && (
					<p className="line-clamp-2 text-xs leading-relaxed text-muted-foreground/90">
						{project.description}
					</p>
				)}

				{/* Tag Badges */}
				{(project as any).tags && (project as any).tags.length > 0 && (
					<div className="flex flex-wrap gap-1.5 pt-0.5">
						{(project as any).tags.map((t: string) => (
							<Badge key={t} variant="secondary" className="text-[10px] font-semibold px-2 py-0.5 rounded-md border border-border/40">
								{t}
							</Badge>
						))}
					</div>
				)}
			</div>

			{/* Card Footer Summary */}
			<div className="mt-4 flex items-center justify-between border-t border-border/40 pt-3 text-xs text-muted-foreground font-medium">
				<div className="flex items-center gap-2.5">
					<span>
						<strong className="font-bold text-foreground">{totalEnvironments}</strong>{' '}
						{totalEnvironments === 1 ? 'Environment' : 'Environments'}
					</span>
					<span className="text-muted-foreground/40">•</span>
					<span>
						<strong className="font-bold text-foreground">{totalServices}</strong>{' '}
						{totalServices === 1 ? 'Service' : 'Services'}
					</span>
				</div>
			</div>
		</div>
	);
};
