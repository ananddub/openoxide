import {Link} from '@tanstack/react-router';
import {ArrowRight} from 'lucide-react';
import {ProjectRow} from '#/components/home/project-row';

interface ProjectsCardProps {
	isLoading: boolean;
	projects: any[];
	servicesByProject: Map<number, any[]>;
	domains: any[];
}

/* ── Projects card: header + loading / empty / list states ─────────── */
export function ProjectsCard({
	isLoading,
	projects,
	servicesByProject,
	domains,
}: ProjectsCardProps) {
	return (
		<div className="overflow-hidden rounded-lg border bg-card shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)]">
			<div className="flex items-center justify-between border-b px-5 py-4">
				<h2 className="text-[15px] font-semibold text-foreground">
					Your Projects
				</h2>
				<Link
					to="/projects"
					className="flex items-center gap-1 text-sm text-muted-foreground transition-colors hover:text-foreground">
					View all
					<ArrowRight className="size-3.5" />
				</Link>
			</div>

			{isLoading ? (
				<div className="divide-y divide-border/60">
					{[...Array(3)].map((_, i) => (
						<div
							key={i}
							className="flex animate-pulse items-center gap-4 px-5 py-4">
							<div className="size-10 rounded-lg bg-muted" />
							<div className="flex-1 space-y-2">
								<div className="h-4 w-32 rounded bg-muted" />
								<div className="h-3 w-48 rounded bg-muted" />
							</div>
						</div>
					))}
				</div>
			) : projects.length === 0 ? (
				<div className="flex flex-1 flex-col items-center justify-center px-6 py-14 text-center">
					<h3 className="mb-1 text-[15px] font-semibold text-foreground">
						No projects yet
					</h3>
					<p className="mx-auto mb-5 max-w-sm text-sm leading-relaxed text-muted-foreground">
						Deploy applications, compose stacks, and databases to see them
						here.
					</p>
					<Link
						to="/projects"
						search={{create: 1} as never}
						className="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90">
						New Project
					</Link>
				</div>
			) : (
				<div className="divide-y divide-border/60">
					{projects.slice(0, 6).map(p => (
						<ProjectRow
							key={p.id}
							project={p}
							services={servicesByProject.get(p.id) ?? []}
							domains={domains}
						/>
					))}
					{projects.length > 6 && (
						<Link
							to="/projects"
							className="block px-5 py-3 text-center text-sm text-muted-foreground transition-colors hover:bg-muted/40 hover:text-foreground">
							View all {projects.length} projects
						</Link>
					)}
				</div>
			)}
		</div>
	);
}
