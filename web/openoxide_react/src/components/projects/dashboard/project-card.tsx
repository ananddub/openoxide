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

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({ project, onDelete }) => {
	const navigate = useNavigate();

	const handleCardClick = () => {
		navigate({ to: '/projects/$id', params: { id: String(project.id) } });
	};

	const formatDate = (timestamp?: number) => {
		if (!timestamp) return 'recently';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString();
	};

	const envs = (project as any).environments || (project as any).envs || [];
	const totalEnvironments = Array.isArray(envs) ? envs.length : 1;
	const totalServices = Array.isArray(envs)
		? envs.reduce((acc: number, env: any) => {
				const apps = env.applications?.length || env.apps?.length || 0;
				const comp = env.compose?.length || 0;
				const dbs = env.databases?.length || 0;
				return acc + apps + comp + dbs;
			}, 0)
		: 0;

	return (
		<div
			onClick={handleCardClick}
			className="group flex cursor-pointer flex-col justify-between rounded-xl border border-border/60 bg-background p-5 shadow-xs transition-all hover:border-primary/50 hover:shadow-md w-full"
		>
			<div className="flex flex-col gap-3">
				{/* Card Header */}
				<div className="flex items-start justify-between gap-3">
					<div className="flex items-center gap-2.5 min-w-0">
						<div className="flex size-9 items-center justify-center rounded-lg border border-border/40 bg-muted/60 text-muted-foreground transition-colors group-hover:bg-primary/10 group-hover:text-primary shrink-0">
							<Folder className="size-4" />
						</div>
						<div className="flex flex-col min-w-0">
							<h3 className="line-clamp-1 text-sm font-semibold text-foreground group-hover:text-primary transition-colors">
								{project.name}
							</h3>
							<span className="text-[11px] text-muted-foreground">
								Created {formatDate(project.created_at)}
							</span>
						</div>
					</div>

					{/* Action Dropdown Menu */}
					<div onClick={(e) => e.stopPropagation()}>
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<Button
										variant="ghost"
										size="icon"
										onClick={(e) => e.stopPropagation()}
										className="size-8 text-muted-foreground hover:text-foreground focus-visible:ring-0"
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
					<p className="line-clamp-2 text-xs leading-relaxed text-muted-foreground">
						{project.description}
					</p>
				)}

				{/* Tag Badges */}
				{(project as any).tags && (project as any).tags.length > 0 && (
					<div className="flex flex-wrap gap-1.5 pt-1">
						{(project as any).tags.map((t: string) => (
							<Badge key={t} variant="secondary" className="text-[11px] font-medium px-2 py-0.5">
								{t}
							</Badge>
						))}
					</div>
				)}
			</div>

			{/* Card Footer Summary */}
			<div className="mt-4 flex items-center justify-between border-t border-border/40 pt-3 text-xs text-muted-foreground">
				<div className="flex items-center gap-3">
					<span>
						<strong className="font-semibold text-foreground">{totalEnvironments}</strong>{' '}
						{totalEnvironments === 1 ? 'Environment' : 'Environments'}
					</span>
					<span>•</span>
					<span>
						<strong className="font-semibold text-foreground">{totalServices}</strong>{' '}
						{totalServices === 1 ? 'Service' : 'Services'}
					</span>
				</div>
			</div>
		</div>
	);
};
