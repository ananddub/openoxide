import * as React from 'react';
import type {components} from '#/types/api.d.ts';
import {FolderOpen, Calendar, Terminal, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Card,
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({project, onDelete}) => {
	return (
		<Card className="group overflow-hidden border-border bg-card/40 shadow-sm backdrop-blur-sm transition-all duration-300 hover:shadow-lg hover:border-primary/20 hover:bg-card/75">
			<CardHeader className="pb-3">
				<div className="flex items-start justify-between">
					<div className="flex items-center gap-3">
						<div className="p-2.5 rounded-lg bg-primary/5 text-primary border border-primary/10 group-hover:bg-primary/10 group-hover:border-primary/20 transition-all">
							<FolderOpen className="size-5" />
						</div>
						<div>
							<CardTitle className="text-lg font-bold group-hover:text-primary transition-colors">
								{project.name}
							</CardTitle>
							<span className="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold flex items-center gap-1.5 mt-1">
								<Calendar className="size-3" />
								{new Date(project.created_at * 1000).toLocaleDateString()}
							</span>
						</div>
					</div>
				</div>
			</CardHeader>
			<CardContent className="pb-4 min-h-[72px]">
				<p className="text-sm text-muted-foreground line-clamp-3 leading-relaxed">
					{project.description || 'No description provided.'}
				</p>
			</CardContent>
			<CardFooter className="pt-3 border-t border-border/30 bg-muted/15 flex items-center justify-between">
				<div className="flex items-center gap-2">
					{project.env_var && (
						<span className="inline-flex items-center gap-1 text-[11px] font-semibold text-primary/95 bg-primary/5 border border-primary/10 px-2 py-0.5 rounded">
							<Terminal className="size-3" />
							Envs Configured
						</span>
					)}
				</div>

				<div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onDelete(project.id)}
						className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/5 rounded-md">
						<Trash2 className="size-4" />
					</Button>
				</div>
			</CardFooter>
		</Card>
	);
};
