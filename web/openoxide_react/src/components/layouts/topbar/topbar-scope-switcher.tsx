import {Check, ChevronsUpDown, Folder, FolderOpen} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Button} from '#/components/ui/button';

interface TopBarScopeSwitcherProps {
	projects: Array<{id: unknown; name?: string}>;
	activeProject: {name?: string} | null;
	projectId: number | null;
	onAllProjects: () => void;
	onSelectProject: (id: string) => void;
}

/** Left slot of the top bar — the project scope switcher. Lists the workspace's *projects* (the org switcher lives in the sidebar rail): "All Projects" clears the deep view, rows below jump into a project. */
export function TopBarScopeSwitcher({
	projects,
	activeProject,
	projectId,
	onAllProjects,
	onSelectProject,
}: TopBarScopeSwitcherProps) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						className="h-9 max-w-[260px] gap-2 rounded-md px-2.5 text-[13px] font-medium text-foreground hover:bg-sidebar-accent">
						{activeProject ? (
							<FolderOpen className="size-4 shrink-0 text-muted-foreground" />
						) : (
							<Folder className="size-4 shrink-0 text-muted-foreground" />
						)}
						<span className="truncate">
							{activeProject?.name ?? 'All Projects'}
						</span>
						<ChevronsUpDown className="size-3.5 shrink-0 text-muted-foreground" />
					</Button>
				}
			/>
			<DropdownMenuContent
				align="start"
				sideOffset={8}
				className="w-64 rounded-lg">
				<DropdownMenuItem
					className={cn(
						'gap-2.5 text-[13px]',
						!activeProject && 'bg-accent/50',
					)}
					onClick={onAllProjects}>
					<Folder className="size-4 shrink-0 text-muted-foreground" />
					<span className="min-w-0 flex-1 truncate">All Projects</span>
					{!activeProject && <Check className="size-4 shrink-0" />}
				</DropdownMenuItem>
				{projects.length > 0 && (
					<>
						<DropdownMenuSeparator />
						<DropdownMenuLabel className="text-[11px] font-medium text-muted-foreground">
							Projects
						</DropdownMenuLabel>
					</>
				)}
				<div className="max-h-64 overflow-y-auto">
					{projects.map(project => {
						const isCurrent =
							projectId !== null && Number(project.id) === projectId;
						return (
							<DropdownMenuItem
								key={String(project.id)}
								className="gap-2.5 text-[13px]"
								onClick={() => onSelectProject(String(project.id))}>
								<Folder className="size-4 shrink-0 text-muted-foreground" />
								<span className="min-w-0 flex-1 truncate">
									{project.name ?? `Project ${project.id}`}
								</span>
								{isCurrent && <Check className="size-4 shrink-0" />}
							</DropdownMenuItem>
						);
					})}
				</div>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
