import {useNavigate} from '@tanstack/react-router';
import type {components} from '#/types/api.d.ts';
import {FolderOpen, Calendar, Terminal, MoreVertical, Trash2, Eye} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Card, CardTitle} from '#/components/ui/card';
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

export const ProjectCard: React.FC<ProjectCardProps> = ({project, onDelete}) => {
	const navigate = useNavigate();

	return (
		<Card 
			onClick={() => navigate({ to: '/projects/$id', params: { id: String(project.id) } })}
			className="group overflow-hidden border border-border bg-card shadow-2xs hover:border-border/80 transition-all duration-200 flex flex-col justify-between p-4 h-[160px] cursor-pointer rounded-xl"
		>
			{/* Top Row: Icon, Title, Description, Actions Switcher */}
			<div className="flex items-start justify-between gap-3 min-w-0">
				<div className="flex items-start gap-2.5 min-w-0">
					<div 
						className="p-2 rounded-lg bg-muted/40 text-primary border border-border/40 shrink-0 mt-0.5"
					>
						<FolderOpen className="size-4 text-primary" />
					</div>
					<div className="min-w-0">
						<CardTitle 
							className="text-xs font-bold tracking-tight text-foreground truncate group-hover:text-primary transition-colors"
						>
							{project.name}
						</CardTitle>
						<p className="text-[11px] text-muted-foreground line-clamp-3 mt-1 leading-snug">
							{project.description || 'No description provided.'}
						</p>
					</div>
				</div>

				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								onClick={e => e.stopPropagation()}
								className="size-7 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md shrink-0 focus-visible:ring-0">
								<MoreVertical className="size-3.5" />
							</Button>
						}
					/>
					<DropdownMenuContent align="end" className="w-36 border border-border bg-popover" onClick={e => e.stopPropagation()}>
						<DropdownMenuItem 
							onClick={() => navigate({ to: '/projects/$id', params: { id: String(project.id) } })}
							className="flex items-center gap-2 cursor-pointer text-xs font-semibold"
						>
							<Eye className="size-3.5 text-muted-foreground" />
							View details
						</DropdownMenuItem>
						<DropdownMenuSeparator className="bg-border/40" />
						<DropdownMenuItem
							onClick={() => onDelete(project.id)}
							className="flex items-center gap-2 cursor-pointer text-xs text-destructive font-semibold focus:text-destructive focus:bg-destructive/10">
							<Trash2 className="size-3.5" />
							Delete project
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>

			{/* Bottom Row: Metadata info */}
			<div className="flex items-center justify-between mt-auto pt-2.5 border-t border-border/40">
				<span className="text-[10px] text-muted-foreground/70 flex items-center gap-1.5 font-mono">
					<Calendar className="size-3" />
					{new Date(project.created_at * 1000).toLocaleDateString()}
				</span>

				{project.env_var ? (
					<span className="inline-flex items-center gap-1 text-[10px] font-mono font-bold text-primary bg-primary/10 border border-primary/20 px-2 py-0.5 rounded-md">
						<Terminal className="size-2.5" />
						Envs
					</span>
				) : null}
			</div>
		</Card>
	);
};
