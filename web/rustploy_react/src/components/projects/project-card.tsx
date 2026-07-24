import * as React from 'react';
import type {components} from '#/types/api.d.ts';
import {FolderOpen, Calendar, Terminal, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Card, CardTitle} from '#/components/ui/card';

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({project, onDelete}) => {
	return (
		<Card className="group overflow-hidden border-border bg-card/40 shadow-sm backdrop-blur-sm transition-all duration-300 hover:shadow-md hover:border-primary/20 hover:bg-card/70 flex flex-col justify-between p-4 h-[145px]">
			{/* Top Row: Icon, Title, Description, Delete Button */}
			<div className="flex items-start justify-between gap-3 min-w-0">
				<div className="flex items-start gap-2.5 min-w-0">
					<div className="p-2 rounded-lg bg-primary/5 text-primary border border-primary/10 group-hover:bg-primary/10 transition-all shrink-0 mt-0.5">
						<FolderOpen className="size-4" />
					</div>
					<div className="min-w-0">
						<CardTitle className="text-sm font-semibold tracking-tight text-foreground truncate group-hover:text-primary transition-colors">
							{project.name}
						</CardTitle>
						<p className="text-[12px] text-muted-foreground line-clamp-2 mt-1 leading-snug">
							{project.description || 'No description provided.'}
						</p>
					</div>
				</div>

				<Button
					variant="ghost"
					size="icon"
					onClick={() => onDelete(project.id)}
					className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/5 rounded-md shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
					<Trash2 className="size-3.5" />
				</Button>
			</div>

			{/* Bottom Row: Metadata info */}
			<div className="flex items-center justify-between mt-auto pt-2 border-t border-border/10">
				<span className="text-[10px] text-muted-foreground flex items-center gap-1.5 font-medium">
					<Calendar className="size-3" />
					{new Date(project.created_at * 1000).toLocaleDateString()}
				</span>

				{project.env_var ? (
					<span className="inline-flex items-center gap-1 text-[9px] font-semibold text-primary bg-primary/5 border border-primary/10 px-1.5 py-0.5 rounded">
						<Terminal className="size-2.5" />
						Envs
					</span>
				) : null}
			</div>
		</Card>
	);
};
