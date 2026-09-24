import {useState} from 'react';
import {Link} from '@tanstack/react-router';
import {
	Folder,
	MoreHorizontal,
	Server,
	SquarePen,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TagBadge} from '#/components/shared/tag-badge';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {useProjectCardData} from './use-project-card-data';
import {HandleProjectDialog} from './handle-project-dialog';

type Project = Record<string, any>;

type ProjectListRowProps = {
	project: Project;
	onDelete: (id: number) => void;
};

/** Compact list-view counterpart to ProjectCard — one row per project, stretched-link navigation, kebab carries the same update/delete actions. */
export function ProjectListRow({project, onDelete}: ProjectListRowProps) {
	const [isUpdateOpen, setIsUpdateOpen] = useState(false);

	const {
		formatDate,
		totalServices,
		totalEnvironments,
		projectServerNames,
		projectTagNames,
		getTagColor,
	} = useProjectCardData(project);

	return (
		<>
			<div className="group relative flex items-center gap-3.5 px-4 py-3 transition-colors hover:bg-accent/60">
				{/* Stretched navigation */}
				<Link
					to="/projects/$id"
					params={{id: String(project.id)}}
					preload="intent"
					className="absolute inset-0 z-0"
					aria-label={`Open project ${project.name}`}
				/>

				<div className="pointer-events-none relative z-10 flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground">
					<Folder className="size-4.5" />
				</div>

				<div className="pointer-events-none relative z-10 w-44 min-w-0 flex-shrink-0 lg:w-64">
					<p className="truncate text-sm font-medium text-foreground">
						{project.name}
					</p>
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						Created {formatDate(project.created_at)}
					</p>
				</div>

				{projectTagNames.length > 0 && (
					<div className="pointer-events-none relative z-10 hidden min-w-0 flex-1 flex-wrap gap-1.5 md:flex">
						{projectTagNames.map(tag => (
							<TagBadge
								key={tag}
								name={tag}
								color={getTagColor(tag)}
								className="px-2 py-0.5 text-[11px]"
							/>
						))}
					</div>
				)}

				<div className="pointer-events-none relative z-10 ml-auto flex shrink-0 items-center gap-4 text-xs text-muted-foreground">
					<span className="hidden font-mono sm:block">
						<span className="font-semibold text-foreground">
							{totalServices}
						</span>{' '}
						{totalServices === 1 ? 'service' : 'services'}
					</span>
					<span className="hidden font-mono lg:block">
						<span className="font-semibold text-foreground">
							{totalEnvironments}
						</span>{' '}
						{totalEnvironments === 1 ? 'env' : 'envs'}
					</span>
					<span
						className="hidden max-w-[160px] items-center gap-1.5 truncate xl:flex"
						title="Deployment servers">
						<Server className="size-3.5 shrink-0 text-muted-foreground/80" />
						<span className="truncate">
							{projectServerNames.length > 0
								? projectServerNames.join(', ')
								: 'No services deployed'}
						</span>
					</span>

					<DropdownMenu>
						<DropdownMenuTrigger
							render={
								<Button
									variant="ghost"
									size="icon"
									className="pointer-events-auto size-7 rounded-lg text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
									title={`${project.name} actions`}
									aria-label={`Project ${project.name} actions`}>
									<MoreHorizontal className="size-4" />
								</Button>
							}
						/>
						<DropdownMenuContent align="end" className="w-48">
							<DropdownMenuItem
								onClick={() => setIsUpdateOpen(true)}
								className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-foreground">
								<SquarePen className="size-4 text-muted-foreground" />
								<span>Update</span>
							</DropdownMenuItem>
							<DropdownMenuSeparator />
							<DropdownMenuItem
								onClick={() => onDelete(project.id)}
								className="flex cursor-pointer items-center gap-2 text-xs font-medium text-danger focus:text-danger">
								<Trash2 className="size-4" />
								<span>Delete</span>
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>
			</div>

			<HandleProjectDialog
				project={project}
				isOpen={isUpdateOpen}
				onOpenChange={setIsUpdateOpen}
			/>
		</>
	);
}
