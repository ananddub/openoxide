import React from 'react';
import { useNavigate } from '@tanstack/react-router';
import type { components } from '#/types/api.d.ts';
import { Book, MoreHorizontal, Trash2, Eye } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Card, CardHeader, CardTitle, CardFooter } from '#/components/ui/card';
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

	const handleNavigate = () => {
		navigate({ to: '/projects/$id', params: { id: String(project.id) } });
	};

	const formatDate = (timestamp?: number) => {
		if (!timestamp) return 'recently';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString(undefined, {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
		});
	};

	// Calculate total services count safely
	const envs = (project as any).environments || (project as any).envs || [];
	const totalServices = Array.isArray(envs)
		? envs.reduce((acc: number, env: any) => {
				const apps = env.applications?.length || env.apps?.length || 0;
				const comp = env.compose?.length || 0;
				const dbs = env.databases?.length || 0;
				return acc + apps + comp + dbs;
			}, 0)
		: 0;

	return (
		<div className="w-full lg:max-w-md">
			<Card
				onClick={handleNavigate}
				className="group relative w-full h-full bg-transparent transition-colors hover:bg-border flex flex-col cursor-pointer border border-border/80"
			>
				<CardHeader>
					<CardTitle className="flex items-center justify-between gap-2 overflow-clip font-normal">
						<span className="flex flex-col gap-1.5">
							{/* Icon + Title */}
							<div className="flex items-center gap-2">
								<Book className="size-4 text-muted-foreground shrink-0" />
								<span className="text-base font-medium leading-none text-foreground">
									{project.name}
								</span>
							</div>

							{/* Description */}
							<span className="text-sm font-medium text-muted-foreground break-normal leading-relaxed">
								{project.description || 'No description provided.'}
							</span>

							{/* Tags */}
							{(project as any).tags && (project as any).tags.length > 0 && (
								<div className="flex flex-wrap gap-1.5 mt-2">
									{(project as any).tags.map((t: string) => (
										<Badge key={t} variant="secondary" className="text-[10px] font-medium px-2 py-0.5">
											{t}
										</Badge>
									))}
								</div>
							)}
						</span>

						{/* Actions Menu (Dokploy exact) */}
						<div className="flex self-start space-x-1" onClick={(e) => e.stopPropagation()}>
							<DropdownMenu>
								<DropdownMenuTrigger
									render={
										<Button
											variant="ghost"
											size="icon"
											onClick={(e) => e.stopPropagation()}
											className="px-2 size-8 text-muted-foreground hover:text-foreground"
										>
											<MoreHorizontal className="size-5" />
										</Button>
									}
								/>
								<DropdownMenuContent
									align="end"
									className="w-44 border border-border bg-popover/95 backdrop-blur-md shadow-lg"
									onClick={(e) => e.stopPropagation()}
								>
									<DropdownMenuItem
										onClick={handleNavigate}
										className="flex items-center gap-2 cursor-pointer text-xs font-semibold py-2 text-foreground"
									>
										<Eye className="size-3.5 text-muted-foreground" />
										View details
									</DropdownMenuItem>
									<DropdownMenuSeparator className="bg-border/60" />
									<DropdownMenuItem
										onClick={() => onDelete(project.id)}
										className="flex items-center gap-2 cursor-pointer text-xs font-semibold py-2 text-destructive focus:text-destructive focus:bg-destructive/10"
									>
										<Trash2 className="size-3.5 text-destructive" />
										Delete project
									</DropdownMenuItem>
								</DropdownMenuContent>
							</DropdownMenu>
						</div>
					</CardTitle>
				</CardHeader>

				{/* CardFooter (Dokploy exact) */}
				<CardFooter className="pt-4 mt-auto">
					<div className="space-y-1 text-xs flex flex-row justify-between max-sm:flex-wrap w-full gap-2 sm:gap-4 text-muted-foreground">
						<span>Created {formatDate(project.created_at)}</span>
						<span>
							{totalServices} {totalServices === 1 ? 'service' : 'services'}
						</span>
					</div>
				</CardFooter>
			</Card>
		</div>
	);
};
