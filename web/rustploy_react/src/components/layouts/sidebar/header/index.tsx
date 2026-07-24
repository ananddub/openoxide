import * as React from 'react';
import {cn} from '#/api/utils';
import {Logo} from '#/components/shared/logo';
import {ChevronsUpDown, Star, Plus, PenBoxIcon} from 'lucide-react';
import {
	SidebarMenu,
	SidebarMenuItem,
	SidebarMenuButton,
} from '#/components/ui/sidebar';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuGroup,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {AddOrganization} from './add-dialog';
import {DeleteOrganization} from './del-dialog';
import {NotificationBell} from './notification';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {useOrganizationStore} from '#/stores/organization-store';

type Props = {
	isCollapsed: boolean;
	isMobile: boolean;
};

// Header dropdown showing the active org logo/name with a switcher menu.
// The add/edit dialog is rendered outside DropdownMenuContent to avoid
// Base UI's focus trap blocking keyboard input in dialog forms.
export function HeaderDropdown({isCollapsed, isMobile}: Props) {
	// Controlled state for the dropdown menu.
	const [menuOpen, setMenuOpen] = React.useState(false);
	// Controlled state for the organization dialog.
	const [dialog, setDialog] = React.useState<{
		open: boolean;
		orgId?: string;
	}>({
		open: false,
	});

	// Fetch Organizations from backend
	const {data: orgs, isLoading} = $api.useQuery('get', '/organizations');
	const {
		organizations: orgList,
		activeOrg,
		setOrganizations,
		setActiveOrg,
	} = useOrganizationStore();

	React.useEffect(() => {
		if (orgs) {
			setOrganizations(orgs);
		}
	}, [orgs, setOrganizations]);

	const handleSetDefault = (orgId: number, e: React.MouseEvent) => {
		e.stopPropagation();
		toast.success('Default organization updated');
	};

	// If the deleted org was active, fall back to the next default or first in list.
	const handleDelete = (orgId: number) => {
		if (orgList.length <= 1) {
			toast.error('Cannot delete the last organization');
			return;
		}
		const targetOrg = orgList.find(o => o.id === orgId);
		toast.success(`Organization ${targetOrg?.name} deleted`);
	};

	if (isLoading || !activeOrg) {
		return (
			<div className="flex h-10 w-full items-center gap-2 px-3 py-1.5 animate-pulse">
				<div className="size-6 shrink-0 rounded-sm bg-muted/60" />
				<div className="h-4 grow rounded bg-muted/60" />
			</div>
		);
	}

	return (
		<>
			<SidebarMenu
				className={cn(
					'flex w-full gap-2 p-0',
					isCollapsed
						? 'flex-col'
						: 'flex-row items-center justify-between',
				)}>
				<SidebarMenuItem className="min-w-0 grow">
					<DropdownMenu open={menuOpen} onOpenChange={setMenuOpen}>
						<DropdownMenuTrigger
							render={
								<SidebarMenuButton
									size={isCollapsed ? 'sm' : 'lg'}
									className={cn(
										'data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground',
										isCollapsed &&
											'mx-auto flex h-10 w-10 items-center justify-center rounded-md  p-0 group-data-[collapsible=icon]:size-10!',
									)}
								/>
							}>
							<div
								className={cn(
									'flex items-center gap-2',
									isCollapsed && 'w-full justify-center',
								)}>
								<Logo
									className={cn(
										'shrink-0 text-primary transition-all',
										isCollapsed ? 'size-5' : 'size-8',
									)}
									logoUrl={activeOrg.logo}
								/>
								<span
									className={cn(
										'max-w-27.5 truncate text-sm leading-tight font-semibold tracking-tight text-foreground',
										isCollapsed && 'hidden',
									)}>
									{activeOrg.name}
								</span>
							</div>
							<ChevronsUpDown
								className={cn(
									'ml-auto text-muted-foreground/60',
									isCollapsed && 'hidden',
								)}
							/>
						</DropdownMenuTrigger>
						<DropdownMenuContent
							className="flex max-h-[min(70vh,28rem)] w-64 flex-col rounded-lg"
							align="start"
							side={isMobile ? 'bottom' : 'right'}
							sideOffset={4}>
							<DropdownMenuGroup>
								<DropdownMenuLabel className="shrink-0 text-xs text-muted-foreground">
									Organizations
								</DropdownMenuLabel>
								<div className="-mx-1 flex max-h-64 min-h-0 flex-col gap-1 overflow-x-hidden overflow-y-auto px-1">
									{orgList.map(org => {
										const isDefault = org.id === activeOrg.id;
										return (
											<div
												className="flex flex-row items-center justify-between gap-1"
												key={org.id}>
												<DropdownMenuItem
													onClick={() => {
														setActiveOrg(org);
														toast.success(`Switched to ${org.name}`);
													}}
													className="flex min-w-0 flex-1 gap-2 p-2">
													<div className="flex size-6 shrink-0 items-center justify-center rounded-sm border">
														<Logo className="size-4" logoUrl={org.logo} />
													</div>
													<span className="truncate text-xs font-medium">
														{org.name}
													</span>
												</DropdownMenuItem>
												<div className="flex shrink-0 items-center gap-1">
													{/* Set Default */}
													<Button
														variant="ghost"
														size="icon"
														className={cn(
															'group',
															isDefault
																? 'hover:bg-yellow-500/10'
																: 'hover:bg-blue-500/10',
														)}
														onClick={e => handleSetDefault(org.id, e)}
														title={
															isDefault
																? 'Default organization'
																: 'Set as default'
														}>
														<Star
															className={cn(
																'size-4',
																isDefault &&
																	'fill-yellow-500 text-yellow-500',
															)}
														/>
													</Button>
													{/* Edit Dialog Trigger */}
													<button
														className="shrink-0 cursor-pointer rounded-md p-1.5 text-primary transition-colors hover:bg-blue-500/10 hover:text-blue-500"
														onClick={e => {
															e.stopPropagation();
															// Open edit dialog and close dropdown to release focus/keyboard trap
															setDialog({
																open: true,
																orgId: String(org.id),
															});
															setMenuOpen(false);
														}}
														title="Update organization">
														<PenBoxIcon className="size-4" />
													</button>
													{/* Delete Button */}
													<DeleteOrganization
														organizationId={String(org.id)}
														onDelete={() => handleDelete(org.id)}
														disabled={orgList.length <= 1}
													/>
												</div>
											</div>
										);
									})}
								</div>
							</DropdownMenuGroup>
							<DropdownMenuSeparator />
							<DropdownMenuItem
								onClick={() => {
									// Open add dialog and close dropdown to release focus/keyboard trap
									setDialog({open: true});
									setMenuOpen(false);
								}}
								className="cursor-pointer gap-2 p-2">
								<div className="flex size-6 items-center justify-center rounded-md border bg-background">
									<Plus className="size-4" />
								</div>
								<div className="font-medium text-muted-foreground">
									Add organization
								</div>
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</SidebarMenuItem>
				<SidebarMenuItem
					className={cn('shrink-0', isCollapsed ? 'mx-auto mt-1' : '')}>
					<NotificationBell
						isCollapsed={isCollapsed}
						isMobile={isMobile}
					/>
				</SidebarMenuItem>
			</SidebarMenu>
			{/* Dialog rendered outside dropdown tree to avoid focus trap/interception */}
			<AddOrganization
				open={dialog.open}
				onOpenChange={open => {
					setDialog(prev => ({...prev, open}));
				}}
				organizationId={dialog.orgId}
			/>
		</>
	);
}
