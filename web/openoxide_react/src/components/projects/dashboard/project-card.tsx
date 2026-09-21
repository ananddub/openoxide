import React, {useState} from 'react';
import {Link} from '@tanstack/react-router';

import {
	Folder,
	MoreHorizontal,
	SquarePen,
	FileText,
	Trash2,
	Server,
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

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({
	project,
	onDelete,
}) => {
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
			<div className="group relative flex min-h-[180px] w-full flex-col justify-between gap-3.5 rounded-xl border bg-card p-4 text-left shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)] transition-colors hover:bg-accent">
				{/* Clickable Card Overlay */}
				<Link
					to="/projects/$id"
					params={{id: String(project.id)}}
					preload="intent"
					className="absolute inset-0 z-0 rounded-xl"
					aria-label={`Open project ${project.name}`}
				/>

				{/* Top Body Section */}
				<div className="pointer-events-none relative z-10 flex min-w-0 flex-col gap-3.5">
					{/* Card Header */}
					<div className="flex items-start justify-between gap-2.5">
						<div className="flex min-w-0 items-center gap-3">
							<div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground transition-colors group-hover:bg-foreground/[0.08]">
								<Folder className="size-5" />
							</div>
							<div className="flex min-w-0 flex-col">
								<h3 className="line-clamp-1 text-sm font-medium text-foreground">
									{project.name}
								</h3>
								<span className="mt-0.5 text-xs text-muted-foreground">
									Created {formatDate(project.created_at)}
								</span>
							</div>
						</div>

						{/* Action Dropdown Menu */}
						<div className="pointer-events-auto shrink-0">
							<DropdownMenu>
								<DropdownMenuTrigger
									render={
										<Button
											variant="ghost"
											size="icon"
											className="size-7 rounded-lg text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0">
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
									<DropdownMenuItem
										onClick={() => {
											const projectData = `Project: ${project.name}\nCreated: ${formatDate(project.created_at)}\nServices: ${totalServices}\nEnvironments: ${totalEnvironments}`;
											navigator.clipboard.writeText(projectData);
										}}
										className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-foreground">
										<FileText className="size-4 text-muted-foreground" />
										<span>Copy Specs</span>
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

					{/* Tag Badges List */}
					{projectTagNames.length > 0 && (
						<div className="pointer-events-auto flex flex-wrap gap-1.5 pt-0.5">
							{projectTagNames.map(tag => (
								<TagBadge key={tag} name={tag} color={getTagColor(tag)} />
							))}
						</div>
					)}
				</div>

				{/* Bottom Stats Footer */}
				<div className="pointer-events-none relative z-10 mt-1 flex flex-col gap-2 border-t pt-3 text-xs text-muted-foreground">
					<div
						className="flex min-w-0 items-center gap-1.5"
						title="Deployment servers">
						<Server className="size-3.5 shrink-0 text-muted-foreground/80" />
						<span className="shrink-0">Server</span>
						<span className="truncate font-medium text-foreground">
							{projectServerNames.length > 0
								? projectServerNames.join(', ')
								: 'No services deployed'}
						</span>
					</div>
					<div className="flex items-center justify-between">
						<div className="flex items-center gap-1.5 font-mono">
							<span className="font-semibold text-foreground">
								{totalServices}
							</span>
							<span>{totalServices === 1 ? 'service' : 'services'}</span>
						</div>
						<div className="flex items-center gap-1.5 font-mono">
							<span className="font-semibold text-foreground">
								{totalEnvironments}
							</span>
							<span>{totalEnvironments === 1 ? 'env' : 'envs'}</span>
						</div>
					</div>
				</div>
			</div>

			{/* Edit/Update Dialog */}
			<HandleProjectDialog
				project={project}
				isOpen={isUpdateOpen}
				onOpenChange={setIsUpdateOpen}
			/>
		</>
	);
};
