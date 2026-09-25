import {
	Building2,
	Check,
	ChevronsUpDown,
	PencilLine,
	Plus,
	Trash2,
	UserMinus,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {SIDEBAR_COLLAPSE} from '#/components/ui/sidebar';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {useAccountSwitcher} from './use-account-switcher';
import {AccountOrgDialog} from './account-org-dialog';

type Props = {
	collapsed: boolean;
};

/** Vercel-style top-of-rail team switcher: one pill split in two — the left half (avatar + workspace name) switches workspace, the right half (plan chip + chevrons) opens the menu. The popover lists every organization with role chips, workspace management (create / rename / leave / delete), and a signed-in footer with logout. */
export function AccountSwitcher({collapsed}: Props) {
	const ctl = useAccountSwitcher();
	const {
		organizations,
		activeOrg,
		user,
		open,
		setOpen,
		activeId,
		activeEntry,
		activeRole,
		switchTo,
		openCreate,
		openEdit,
		leave,
		remove,
	} = ctl;

	return (
		<>
			<div className="h-full">
				<DropdownMenu open={open} onOpenChange={setOpen}>
					<div
						className={cn(
							'flex h-full w-full items-stretch overflow-hidden rounded-md transition-colors hover:bg-sidebar-accent',
						)}>
						{/* Left half: avatar + workspace name (the identity) */}
						<DropdownMenuTrigger
							render={
								<button
									type="button"
									className={cn(
										'flex h-full min-w-0 flex-1 items-center gap-2.5 px-1.5 text-start transition-[padding,gap,color]',
										SIDEBAR_COLLAPSE,
										collapsed && 'gap-0 px-0 pl-[calc((var(--sidebar-width-icon)_-_1.5rem)/2)]',
									)}
									aria-haspopup="dialog"
									title={collapsed ? activeOrg?.name : undefined}>
									<div className="flex size-6 shrink-0 items-center justify-center rounded-full bg-emerald-600 text-[10px] font-semibold text-white uppercase">
										{activeOrg?.name?.[0] ?? (
											<Building2 className="size-3.5" />
										)}
									</div>
									<span
										className={cn(
											'min-w-0 flex-1 truncate text-[13px] leading-tight font-medium text-foreground transition-opacity duration-200 ease-out motion-reduce:transition-none',
											collapsed && 'opacity-0',
										)}>
										{activeOrg?.name || 'Workspace'}
									</span>
								</button>
							}
						/>{' '}
						{/* Right half: plan chip + chevrons (the menu). Hidden
							when collapsed — the avatar alone opens the menu. */}
						{!collapsed && (
							<DropdownMenuTrigger
								render={
									<button
										type="button"
										className="flex h-full items-center gap-1 px-1.5 transition-colors"
										aria-haspopup="dialog"
										aria-expanded={open}
										title="Workspace options">
										<ChevronsUpDown className="size-4 shrink-0 text-muted-foreground" />
									</button>
								}
							/>
						)}
					</div>

					<DropdownMenuContent
						side={collapsed ? 'right' : 'bottom'}
						align="start"
						className={cn(
							'overflow-hidden rounded-lg',
							collapsed
								? 'ml-2 w-72'
								: 'w-[calc(var(--sidebar-width)-0.5rem)]',
						)}>
						{/* Org list — Vercel row: avatar + name + chip + check */}
						<div className="max-h-64 overflow-y-auto p-1">
							{organizations.map(org => {
								const id = Number(org.id);
								const isCurrent = String(org.id) === String(activeId);
								return (
									<button
										key={id}
										type="button"
										onClick={() => switchTo(id)}
										className={cn(
											'flex w-full items-center gap-2.5 rounded-md px-2 py-2 text-start transition-colors hover:bg-accent',
											isCurrent && 'bg-accent/50',
										)}>
										<div className="flex size-6 shrink-0 items-center justify-center rounded-full bg-emerald-600 text-[11px] font-semibold text-white uppercase">
											{org.name?.[0] ?? <Building2 className="size-3" />}
										</div>
										<span className="min-w-0 flex-1 truncate text-[13px] font-medium text-foreground">
											{org.name}
										</span>
										{isCurrent && (
											<Check className="size-4 shrink-0 text-foreground" />
										)}
									</button>
								);
							})}
						</div>

						{/* Workspace management */}
						<DropdownMenuSeparator />
						<DropdownMenuItem className="text-[13px]" onClick={openCreate}>
							<Plus className="size-4" /> Create Workspace
						</DropdownMenuItem>
						{activeEntry && (
							<>
								<DropdownMenuItem
									className="text-[13px]"
									onClick={() =>
										openEdit(String(activeEntry.id), activeEntry.name)
									}>
									<PencilLine className="size-4" /> Rename
								</DropdownMenuItem>
								{activeRole !== 'OWNER' && (
									<DropdownMenuItem
										className="text-[13px]"
										onClick={() => void leave(Number(activeEntry.id))}>
										<UserMinus className="size-4" /> Leave
									</DropdownMenuItem>
								)}
								{String(activeEntry.owner_id) === String(user?.id ?? '') &&
									organizations.length > 1 && (
										<DropdownMenuItem
											className="text-[13px] text-destructive focus:text-destructive"
											onClick={() => void remove(Number(activeEntry.id))}>
											<Trash2 className="size-4" /> Delete
										</DropdownMenuItem>
									)}
							</>
						)}
					</DropdownMenuContent>
				</DropdownMenu>
			</div>
			<AccountOrgDialog ctl={ctl} />
		</>
	);
}
