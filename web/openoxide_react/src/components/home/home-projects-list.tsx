import {Folder, ChevronRight, Layers, ArrowUpRight, Plus} from 'lucide-react';
import {buttonVariants} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Link} from '@tanstack/react-router';

export interface ProjectItem {
	id: number | string;
	name: string;
	description?: string;
	environment?: string;
	created_at?: string;
	appsCount?: number;
}

interface HomeProjectsListProps {
	projects: ProjectItem[];
	isLoading?: boolean;
}

export function HomeProjectsList({projects = [], isLoading}: HomeProjectsListProps) {
	return (
		<div className="flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<h2 className="text-sm font-bold text-foreground tracking-tight flex items-center gap-2">
					<Folder className="size-4 text-primary" /> Active Projects
				</h2>
				<Link
					to="/projects"
					className={buttonVariants({
						variant: 'ghost',
						size: 'sm',
						className: 'h-7 text-xs font-semibold px-2 text-primary hover:bg-primary/10 cursor-pointer',
					})}>
					View All ({projects.length}) <ChevronRight className="size-3 ml-0.5" />
				</Link>
			</div>

			{isLoading && projects.length === 0 ? (
				<div className="border border-border/60 rounded-xl p-8 text-center text-xs text-muted-foreground font-mono bg-card">
					Loading projects...
				</div>
			) : projects.length === 0 ? (
				<div className="border border-border/60 rounded-xl p-8 text-center flex flex-col items-center justify-center gap-3 bg-card">
					<Layers className="size-8 text-muted-foreground/50" />
					<div className="space-y-1">
						<p className="text-xs font-bold text-foreground">No projects created yet</p>
						<p className="text-[11px] text-muted-foreground max-w-xs">
							Create your first project to organize applications, databases, and environments.
						</p>
					</div>
					<Link
						to="/projects"
						className={buttonVariants({
							variant: 'default',
							size: 'sm',
							className: 'h-8 px-3.5 text-xs font-semibold gap-1.5 mt-1 cursor-pointer',
						})}>
						<Plus className="size-3.5" /> Create Project
					</Link>
				</div>
			) : (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
					{projects.slice(0, 6).map((project) => (
						<div
							key={project.id}
							className="group border border-border/60 hover:border-primary/40 rounded-xl p-4 bg-card/60 hover:bg-card transition-all duration-200 shadow-2xs flex flex-col justify-between gap-3">
							<div className="space-y-1.5">
								<div className="flex items-start justify-between gap-2">
									<h3 className="text-xs font-bold text-foreground group-hover:text-primary transition-colors truncate">
										{project.name}
									</h3>
									<Badge variant="outline" className="text-[10px] font-mono px-1.5 py-0 bg-primary/5 text-primary border-primary/20 shrink-0">
										{project.environment || 'production'}
									</Badge>
								</div>
								<p className="text-[11px] text-muted-foreground line-clamp-2 leading-relaxed">
									{project.description || 'Application infrastructure & deployment configuration.'}
								</p>
							</div>

							<div className="flex items-center justify-between pt-2 border-t border-border/30 text-[11px]">
								<span className="text-muted-foreground font-mono">
									{project.appsCount ?? 1} service{(project.appsCount ?? 1) > 1 ? 's' : ''}
								</span>
								<Link
									to="/projects/$id"
									params={{id: String(project.id)}}
									preload="intent"
									className="text-xs font-semibold text-primary inline-flex items-center gap-1 group-hover:translate-x-0.5 transition-transform">
									Manage <ArrowUpRight className="size-3" />
								</Link>
							</div>
						</div>
					))}
				</div>
			)}
		</div>
	);
}
