import React, {useState, useMemo} from 'react';
import {Link} from '@tanstack/react-router';
import type {components} from '#/types/api.d.ts';
import {
	Folder,
	MoreHorizontal,
	SquarePen,
	FileText,
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
import {useAppStore} from '#/stores/app-store';
import {getTagsFromDescription} from '#/hooks/projects/use-projects-list';
import {HandleProjectDialog} from './handle-project-dialog';

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({
	project,
	onDelete,
}) => {
	const [isUpdateOpen, setIsUpdateOpen] = useState(false);

	// Read real-time overview services and tags directly from Zustand RAM Store
	const overviewServices = useAppStore(
		state => state.overviewServices || [],
	);
	const availableTags = useAppStore(state => state.tags || []);

	const formatDate = (timestamp?: number) => {
		if (!timestamp) return 'recently';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString();
	};

	// Calculate accurate total services & environments for this project from Zustand RAM store
	const projectServices = overviewServices.filter(
		s => Number(s.project_id) === Number(project.id),
	);
	const totalServices = projectServices.length;

	const envIds = new Set(
		projectServices.map(s => s.environment_id).filter(Boolean),
	);
	const totalEnvironments = envIds.size > 0 ? envIds.size : 1;

	// Extract project tags from description (hashtag tags)
	const projectTagNames = getTagsFromDescription(
		project.description || '',
	);

	// Match tag with exact custom color from Settings Tags
	const getTagColor = (tagName: string) => {
		const clean = tagName.replace(/^#/, '').trim().toLowerCase();
		const found = availableTags.find(
			(at: any) => (at.name || '').trim().toLowerCase() === clean,
		);
		if (found && found.color) return found.color;

		// Fallback deterministic badge color palette
		const palette = [
			'#3b82f6',
			'#10b981',
			'#f59e0b',
			'#8b5cf6',
			'#ec4899',
			'#06b6d4',
		];
		let hash = 0;
		for (let i = 0; i < clean.length; i++)
			hash = clean.charCodeAt(i) + ((hash << 5) - hash);
		return palette[Math.abs(hash) % palette.length];
	};

	return (
		<>
			<div className="group relative flex min-h-[200px] w-full max-w-[340px] flex-col justify-between rounded-xl border border-border/80 bg-card p-5 text-left shadow-xs transition-all hover:border-primary/50 hover:shadow-md">
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
							<div className="flex size-9.5 shrink-0 items-center justify-center rounded-lg border border-border/60 bg-muted/80 text-foreground transition-colors group-hover:bg-primary/10 group-hover:text-primary">
								<Folder className="size-4.5" />
							</div>
							<div className="flex min-w-0 flex-col">
								<h3 className="line-clamp-1 text-sm font-bold text-foreground transition-colors group-hover:text-primary">
									{project.name}
								</h3>
								<span className="mt-0.5 font-mono text-[11px] text-muted-foreground">
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
								<DropdownMenuContent
									align="end"
									className="w-48 border border-border bg-popover/95 shadow-lg backdrop-blur-md">
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
										className="flex cursor-pointer items-center gap-2 text-xs font-semibold text-rose-500 focus:text-rose-500">
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
								<TagBadge
									key={tag}
									name={tag}
									color={getTagColor(tag)}
									size="sm"
								/>
							))}
						</div>
					)}
				</div>

				{/* Bottom Stats Footer */}
				<div className="pointer-events-none relative z-10 mt-4 flex items-center justify-between border-t border-border/40 pt-3 text-[11px] font-medium text-muted-foreground">
					<div className="flex items-center gap-1.5 font-mono">
						<span className="font-bold text-foreground">
							{totalServices}
						</span>
						<span>{totalServices === 1 ? 'service' : 'services'}</span>
					</div>
					<div className="flex items-center gap-1.5 font-mono">
						<span className="font-bold text-foreground">
							{totalEnvironments}
						</span>
						<span>{totalEnvironments === 1 ? 'env' : 'envs'}</span>
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
