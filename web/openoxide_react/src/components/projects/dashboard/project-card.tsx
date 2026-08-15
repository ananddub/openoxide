import React, { useState, useEffect } from 'react';
import { useNavigate } from '@tanstack/react-router';
import type { components } from '#/types/api.d.ts';
import {
	FolderOpen,
	Calendar,
	MoreVertical,
	Trash2,
	Eye,
	ArrowUpRight,
	Terminal,
	Play,
	Square,
	Rocket,
} from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Card, CardTitle } from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import { toast } from 'sonner';

type Project = components['schemas']['ProjectResponseDto'];

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

export const ProjectCard: React.FC<ProjectCardProps> = ({ project, onDelete }) => {
	const navigate = useNavigate();
	const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);

	// Close context menu on outside click or scroll
	useEffect(() => {
		const handleClose = () => setContextMenu(null);
		if (contextMenu) {
			window.addEventListener('click', handleClose);
			window.addEventListener('scroll', handleClose, true);
		}
		return () => {
			window.removeEventListener('click', handleClose);
			window.removeEventListener('scroll', handleClose, true);
		};
	}, [contextMenu]);

	const handleNavigate = () => {
		navigate({ to: '/projects/$id', params: { id: String(project.id) } });
	};

	const handleContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		setContextMenu({ x: e.clientX, y: e.clientY });
	};

	const handleStartProject = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		toast.success(`Starting all services in "${project.name}"...`);
	};

	const handleStopProject = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		toast.success(`Stopping all services in "${project.name}"...`);
	};

	const handleDeployProject = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		setContextMenu(null);
		toast.success(`Deploying all services in "${project.name}"...`);
	};

	const formatDate = (timestamp: number) => {
		if (!timestamp) return 'N/A';
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString(undefined, {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
		});
	};

	return (
		<>
			<Card
				onClick={handleNavigate}
				onContextMenu={handleContextMenu}
				className="group relative overflow-hidden border border-border/70 bg-card/80 backdrop-blur-xs hover:bg-card shadow-xs hover:shadow-md hover:border-primary/40 transition-all duration-200 flex flex-col justify-between p-5 min-h-[170px] cursor-pointer rounded-xl hover:-translate-y-0.5"
			>
				{/* Subtle Gradient Hover Backdrop Accent */}
				<div className="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />

				{/* Top Header Section */}
				<div className="flex items-start justify-between gap-3 min-w-0 relative z-10">
					<div className="flex items-start gap-3 min-w-0">
						<div className="p-2.5 rounded-lg bg-primary/10 text-primary border border-primary/20 shrink-0 group-hover:scale-105 group-hover:bg-primary group-hover:text-primary-foreground transition-all duration-200">
							<FolderOpen className="size-4.5" />
						</div>
						<div className="min-w-0 flex-1">
							<div className="flex items-center gap-1.5">
								<CardTitle className="text-sm font-semibold tracking-tight text-foreground truncate group-hover:text-primary transition-colors">
									{project.name}
								</CardTitle>
								<ArrowUpRight className="size-3.5 text-muted-foreground/50 opacity-0 group-hover:opacity-100 group-hover:text-primary transition-all shrink-0 -translate-x-1 group-hover:translate-x-0" />
							</div>
							<p className="text-xs text-muted-foreground/90 line-clamp-2 mt-1 leading-relaxed">
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
									onClick={(e) => e.stopPropagation()}
									className="size-7 text-muted-foreground hover:text-foreground hover:bg-muted/80 rounded-md shrink-0 focus-visible:ring-0"
								>
									<MoreVertical className="size-4" />
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
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-2"
							>
								<Eye className="size-3.5 text-muted-foreground" />
								View details
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={handleStartProject}
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-2 text-emerald-600 dark:text-emerald-400"
							>
								<Play className="size-3.5" />
								Start all
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={handleStopProject}
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-2 text-amber-600 dark:text-amber-400"
							>
								<Square className="size-3.5" />
								Stop all
							</DropdownMenuItem>
							<DropdownMenuItem
								onClick={handleDeployProject}
								className="flex items-center gap-2 cursor-pointer text-xs font-medium py-2 text-blue-600 dark:text-blue-400"
							>
								<Rocket className="size-3.5" />
								Deploy all
							</DropdownMenuItem>
							<DropdownMenuSeparator className="bg-border/60" />
							<DropdownMenuItem
								onClick={() => onDelete(project.id)}
								className="flex items-center gap-2 cursor-pointer text-xs text-destructive font-medium py-2 focus:text-destructive focus:bg-destructive/10"
							>
								<Trash2 className="size-3.5" />
								Delete project
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>

				{/* Footer Info Metadata */}
				<div className="flex items-center justify-between mt-4 pt-3 border-t border-border/50 relative z-10">
					<span className="text-[11px] text-muted-foreground flex items-center gap-1.5 font-medium">
						<Calendar className="size-3 text-muted-foreground/70" />
						{formatDate(project.created_at)}
					</span>

					{project.env_var ? (
						<span className="inline-flex items-center gap-1 text-[10px] font-mono font-medium text-primary bg-primary/10 border border-primary/20 px-2 py-0.5 rounded-md">
							<Terminal className="size-2.5" />
							Configured
						</span>
					) : null}
				</div>
			</Card>

			{/* Custom Right Click Context Menu */}
			{contextMenu && (
				<div
					style={{ top: `${contextMenu.y}px`, left: `${contextMenu.x}px` }}
					className="fixed z-50 w-44 rounded-lg border border-border bg-popover/95 backdrop-blur-md shadow-xl p-1 animate-in fade-in duration-100"
					onClick={(e) => e.stopPropagation()}
				>
					<button
						onClick={handleNavigate}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-popover-foreground hover:bg-muted transition-colors text-left"
					>
						<Eye className="size-3.5 text-muted-foreground" />
						View details
					</button>
					<button
						onClick={handleStartProject}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-emerald-600 dark:text-emerald-400 hover:bg-emerald-500/10 transition-colors text-left"
					>
						<Play className="size-3.5" />
						Start all
					</button>
					<button
						onClick={handleStopProject}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-amber-600 dark:text-amber-400 hover:bg-amber-500/10 transition-colors text-left"
					>
						<Square className="size-3.5" />
						Stop all
					</button>
					<button
						onClick={handleDeployProject}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-500/10 transition-colors text-left"
					>
						<Rocket className="size-3.5" />
						Deploy all
					</button>
					<div className="my-1 h-px bg-border/60" />
					<button
						onClick={() => {
							setContextMenu(null);
							onDelete(project.id);
						}}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors text-left"
					>
						<Trash2 className="size-3.5" />
						Delete project
					</button>
				</div>
			)}
		</>
	);
};
