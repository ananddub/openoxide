import {
	Folder,
	ChevronRight,
	Layers,
	ArrowUpRight,
	Plus,
} from 'lucide-react';
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

export function HomeProjectsList({
	projects = [],
	isLoading,
}: HomeProjectsListProps) {
	return (
		<div className="flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<h2 className="flex items-center gap-2 text-sm font-bold tracking-tight text-foreground">
					<Folder className="size-4 text-primary" /> Active Projects
				</h2>
				<Link
					to="/projects"
					className={buttonVariants({
						variant: 'ghost',
						size: 'sm',
						className:
							'h-7 text-xs font-semibold px-2 text-primary hover:bg-primary/10 cursor-pointer',
					})}>
					View All ({projects.length}){' '}
					<ChevronRight className="ml-0.5 size-3" />
				</Link>
			</div>

			{isLoading && projects.length === 0 ? (
				<div className="rounded-xl border border-border/60 bg-card p-8 text-center font-mono text-xs text-muted-foreground">
					Loading projects...
				</div>
			) : projects.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-xl border border-border/60 bg-card p-8 text-center">
					<Layers className="size-8 text-muted-foreground/50" />
					<div className="space-y-1">
						<p className="text-xs font-bold text-foreground">
							No projects created yet
						</p>
						<p className="max-w-xs text-[11px] text-muted-foreground">
							Create your first project to organize applications,
							databases, and environments.
						</p>
					</div>
					<Link
						to="/projects"
						className={buttonVariants({
							variant: 'default',
							size: 'sm',
							className:
								'h-8 px-3.5 text-xs font-semibold gap-1.5 mt-1 cursor-pointer',
						})}>
						<Plus className="size-3.5" /> Create Project
					</Link>
				</div>
			) : (
				<div className="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-3">
					{projects.slice(0, 6).map(project => (
						<div
							key={project.id}
							className="group flex flex-col justify-between gap-3 rounded-xl border border-border/60 bg-card/60 p-4 shadow-2xs transition-all duration-200 hover:border-primary/40 hover:bg-card">
							<div className="space-y-1.5">
								<div className="flex items-start justify-between gap-2">
									<h3 className="truncate text-xs font-bold text-foreground transition-colors group-hover:text-primary">
										{project.name}
									</h3>
									<Badge
										variant="outline"
										className="shrink-0 border-primary/20 bg-primary/5 px-1.5 py-0 font-mono text-[10px] text-primary">
										{project.environment || 'production'}
									</Badge>
								</div>
								<p className="line-clamp-2 text-[11px] leading-relaxed text-muted-foreground">
									{project.description ||
										'Application infrastructure & deployment configuration.'}
								</p>
							</div>

							<div className="flex items-center justify-between border-t border-border/30 pt-2 text-[11px]">
								<span className="font-mono text-muted-foreground">
									{project.appsCount ?? 1} service
									{(project.appsCount ?? 1) > 1 ? 's' : ''}
								</span>
								<Link
									to="/projects/$id"
									params={{id: String(project.id)}}
									preload="intent"
									className="inline-flex items-center gap-1 text-xs font-semibold text-primary transition-transform group-hover:translate-x-0.5">
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
