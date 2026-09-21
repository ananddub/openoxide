import {Check, ChevronsUpDown, Folder} from 'lucide-react';
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
	activeProject: {name?: string} | null;
	activeOrg: {name?: string} | null;
	projectId: number | null;
	organizations: Array<{id: any; name?: string}>;
	activeId: any;
	switchTo: (id: number) => void;
	setOpen: (open: boolean) => void;
	onSelectWorkspace: () => void;
}

/** Left-hand workspace/project scope dropdown of the top bar. */
export function TopBarScopeSwitcher({
	activeProject,
	activeOrg,
	projectId,
	organizations,
	activeId,
	switchTo,
	setOpen,
	onSelectWorkspace,
}: TopBarScopeSwitcherProps) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						className="h-8 max-w-[260px] gap-2 rounded-md px-2 text-[13px] font-medium text-foreground hover:bg-sidebar-accent">
						{activeProject ? (
							<Folder className="size-4 shrink-0 text-muted-foreground" />
						) : (
							<span className="flex size-4 shrink-0 items-center justify-center rounded-[4px] bg-emerald-600 text-[10px] font-semibold text-white uppercase">
								{activeOrg?.name?.[0] ?? 'W'}
							</span>
						)}
						<span className="truncate">
							{activeProject?.name ?? activeOrg?.name ?? 'All Projects'}
						</span>
						<ChevronsUpDown className="size-3.5 shrink-0 text-muted-foreground" />
					</Button>
				}
			/>
			<DropdownMenuContent
				align="start"
				sideOffset={8}
				className="w-64 rounded-lg">
				{/* Workspace-level scope: clears project deep views. */}
				<DropdownMenuItem
					className={cn(
						'gap-2.5 text-[13px]',
						!activeProject && 'bg-accent/50',
					)}
					onClick={onSelectWorkspace}>
					<span className="flex size-5 shrink-0 items-center justify-center rounded-[4px] bg-emerald-600 text-[10px] font-semibold text-white uppercase">
						{activeOrg?.name?.[0] ?? 'W'}
					</span>
					<span className="min-w-0 flex-1 truncate">
						{activeOrg?.name ?? 'All Projects'}
					</span>
					{!activeProject && <Check className="size-4 shrink-0" />}
				</DropdownMenuItem>
				{activeProject && (
					<DropdownMenuItem
						className="gap-2.5 bg-accent/50 text-[13px]"
						onClick={() => setOpen(false)}>
						<Folder className="size-5 shrink-0 text-muted-foreground" />
						<span className="min-w-0 flex-1 truncate">
							{activeProject.name}
						</span>
						<Check className="size-4 shrink-0" />
					</DropdownMenuItem>
				)}
				<DropdownMenuSeparator />
				<DropdownMenuLabel className="text-[11px] font-medium text-muted-foreground">
					Switch Workspace
				</DropdownMenuLabel>
				{organizations.map(org => {
					const isCurrent = String(org.id) === String(activeId);
					return (
						<DropdownMenuItem
							key={Number(org.id)}
							className="gap-2.5 text-[13px]"
							onClick={() => {
								if (projectId !== null) onSelectWorkspace();
								switchTo(Number(org.id));
							}}>
							<span className="flex size-5 shrink-0 items-center justify-center rounded-[4px] bg-emerald-600 text-[10px] font-semibold text-white uppercase">
								{org.name?.[0] ?? 'W'}
							</span>
							<span className="min-w-0 flex-1 truncate">{org.name}</span>
							{isCurrent && <Check className="size-4 shrink-0" />}
						</DropdownMenuItem>
					);
				})}
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
