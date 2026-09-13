import * as React from 'react';
import {
	Building2,
	Check,
	ChevronsUpDown,
	Loader2,
	PencilLine,
	Plus,
	Trash2,
	UserMinus,
} from 'lucide-react';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {cn} from '#/api/utils';
import {useOrganizations, useAuthSession} from '#/stores/session-context';
import {persistActiveOrganizationId} from '#/stores/organization-context';
import {
	useOrganizationCreate,
	useOrganizationDelete,
	useOrganizationLeave,
	useOrganizationListByOwner,
	useOrganizationUpdate,
} from '#/api/live/organization';
import {formatApiError} from '#/api/utils';
import {Logo} from '#/components/shared/logo';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
} from '#/components/ui/sidebar';
import {NotificationBell} from './notification';

type Props = {
	isCollapsed: boolean;
	isMobile: boolean;
};

/**
 * Organization workspace switcher: lists every organization the user owns
 * or joined (with their role), switches the active one (which re-scopes
 * every query and the X-Organization-Id transport header), and manages
 * create / rename / leave / delete.
 */
export function HeaderDropdown({isCollapsed, isMobile}: Props) {
	const {data: organizations = []} = useOrganizationListByOwner();
	const {activeOrg, setOrganizations, setActiveOrg} = useOrganizations();
	const {user} = useAuthSession();
	const queryClient = useQueryClient();

	const createMutation = useOrganizationCreate();
	const updateMutation = useOrganizationUpdate();
	const leaveMutation = useOrganizationLeave();
	const deleteMutation = useOrganizationDelete();

	const [isFormOpen, setIsFormOpen] = React.useState(false);
	const [editingId, setEditingId] = React.useState<string | null>(null);
	const [formName, setFormName] = React.useState('');
	const [isSubmitting, setIsSubmitting] = React.useState(false);
	const [confirmDelete, setConfirmDelete] = React.useState(false);

	// Keep the session store's org list in sync with the server list.
	React.useEffect(() => {
		if (organizations.length) {
			setOrganizations(
				organizations.map(org => ({
					id: Number(org.id),
					name: org.name,
					logo: org.logo,
					slug: org.slug,
					owner_id: Number(org.owner_id),
					role: org.role,
				})),
			);
		}
	}, [organizations, setOrganizations]);

	const activeId = activeOrg?.id ?? null;
	const activeEntry = organizations.find(org => String(org.id) === String(activeId));
	const activeRole = (activeEntry?.role ?? 'MEMBER').toUpperCase();

	const switchTo = (id: number) => {
		if (id === activeId) return;
		persistActiveOrganizationId(id);
		// setActiveOrg is the state update the de-duping setOrganizations
		// would skip for an unchanged list; the transport reads the fresh
		// id from localStorage via getActiveOrganizationId().
		const target = organizations.find(org => Number(org.id) === id);
		if (target) {
			setActiveOrg({
				id: Number(target.id),
				name: target.name,
				slug: target.slug,
				owner_id: Number(target.owner_id),
				role: target.role,
			});
		}
		// Every query is organization-scoped server-side; drop the cache so
		// nothing from the previous workspace survives the switch.
		queryClient.invalidateQueries();
	};

	const openCreate = () => {
		setEditingId(null);
		setFormName('');
		setConfirmDelete(false);
		setIsFormOpen(true);
	};

	const openEdit = (id: string, name: string) => {
		setEditingId(id);
		setFormName(name);
		setConfirmDelete(false);
		setIsFormOpen(true);
	};

	const submitForm = async () => {
		const name = formName.trim();
		if (!name) {
			toast.error('Organization name is required');
			return;
		}
		setIsSubmitting(true);
		try {
			if (editingId) {
				await updateMutation.mutateAsync({id: Number(editingId), name});
				toast.success('Organization updated');
			} else {
				const created = await createMutation.mutateAsync({name});
				toast.success('Organization created');
				// Move straight into the new workspace.
				switchTo(Number(created.id));
			}
			setIsFormOpen(false);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const leave = async (id: number) => {
		try {
			await leaveMutation.mutateAsync(id);
			toast.success('You left the organization');
			if (activeId === id) {
				const next = organizations.find(org => Number(org.id) !== id);
				persistActiveOrganizationId(next ? Number(next.id) : null);
				queryClient.invalidateQueries();
			}
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const remove = async (id: number) => {
		if (!confirmDelete) {
			setConfirmDelete(true);
			return;
		}
		setIsSubmitting(true);
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Organization deleted');
			setIsFormOpen(false);
			if (activeId === id) {
				const next = organizations.find(org => Number(org.id) !== id);
				persistActiveOrganizationId(next ? Number(next.id) : null);
				queryClient.invalidateQueries();
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
			setConfirmDelete(false);
		}
	};

	return (
		<>
			<SidebarMenu
				className={cn(
					'flex w-full gap-2 p-0',
					isCollapsed ? 'flex-col' : 'flex-row items-center justify-between',
				)}>
				<SidebarMenuItem className="min-w-0 grow">
					<DropdownMenu>
						<DropdownMenuTrigger
							render={
								<SidebarMenuButton
									size={isCollapsed ? 'sm' : 'lg'}
									className={cn(
										'cursor-pointer',
										isCollapsed &&
											'mx-auto flex h-10 w-10 items-center justify-center rounded-md p-0 group-data-[collapsible=icon]:size-10!',
									)}
								/>
							}>
							<div
								className={cn(
									'flex min-w-0 items-center gap-2',
									isCollapsed && 'w-full justify-center',
								)}>
								<Logo
									className={cn(
										'shrink-0 text-primary',
										isCollapsed ? 'size-5' : 'size-8',
									)}
								/>
								<div
									className={cn(
										'min-w-0 flex-1 text-left',
										isCollapsed && 'hidden',
									)}>
									<div className="truncate text-sm leading-tight font-semibold text-foreground">
										{activeOrg?.name || 'OpenOxide'}
									</div>
									<div className="text-[10px] leading-tight text-muted-foreground">
										{activeRole.toLowerCase()} ·{' '}
										{organizations.length} workspace
										{organizations.length === 1 ? '' : 's'}
									</div>
								</div>
								<ChevronsUpDown
									className={cn('size-3.5 shrink-0 opacity-50', isCollapsed && 'hidden')}
								/>
							</div>
						</DropdownMenuTrigger>
						<DropdownMenuContent align="start" className="w-64">
							<DropdownMenuLabel className="text-[10px] font-bold tracking-wider uppercase">
								Organizations
							</DropdownMenuLabel>
							{organizations.map(org => {
								const id = Number(org.id);
								const role = (org.role ?? 'MEMBER').toUpperCase();
								return (
									<DropdownMenuItem
										key={id}
										className="flex items-center gap-2 text-xs"
										onClick={() => switchTo(id)}>
										<Building2 className="size-3.5 shrink-0 text-muted-foreground" />
										<span className="min-w-0 flex-1 truncate">{org.name}</span>
										<span className="shrink-0 text-[9px] text-muted-foreground">
											{role}
										</span>
										{id === activeId && (
											<Check className="size-3.5 shrink-0 text-primary" />
										)}
									</DropdownMenuItem>
								);
							})}
							<DropdownMenuSeparator />
							<DropdownMenuItem
								className="text-xs"
								onClick={openCreate}>
								<Plus className="size-3.5" /> New Organization
							</DropdownMenuItem>
							{activeEntry && (
								<>
									<DropdownMenuItem
										className="text-xs"
										onClick={() =>
											openEdit(
												String(activeEntry.id),
												activeEntry.name,
											)
										}>
										<PencilLine className="size-3.5" /> Rename Current
									</DropdownMenuItem>
									{activeRole !== 'OWNER' && (
										<DropdownMenuItem
											className="text-xs"
											onClick={() => void leave(Number(activeEntry.id))}>
											<UserMinus className="size-3.5" /> Leave Current
										</DropdownMenuItem>
									)}
									{String(activeEntry.owner_id) === String(user?.id ?? '') &&
										organizations.length > 1 && (
											<DropdownMenuItem
												className="text-xs text-destructive focus:text-destructive"
												onClick={() => void remove(Number(activeEntry.id))}>
												<Trash2 className="size-3.5" /> Delete Current
											</DropdownMenuItem>
										)}
								</>
							)}
						</DropdownMenuContent>
					</DropdownMenu>
				</SidebarMenuItem>
				<SidebarMenuItem
					className={cn('shrink-0', isCollapsed ? 'mx-auto mt-1' : '')}>
					<NotificationBell isCollapsed={isCollapsed} isMobile={isMobile} />
				</SidebarMenuItem>
			</SidebarMenu>

			<Dialog open={isFormOpen} onOpenChange={setIsFormOpen}>
				<DialogContent className="sm:max-w-sm">
					<DialogHeader>
						<DialogTitle className="text-base font-bold">
							{editingId ? 'Rename Organization' : 'New Organization'}
						</DialogTitle>
						<DialogDescription className="text-xs">
							{editingId
								? 'Update the workspace name.'
								: 'You will become the owner of the new workspace.'}
						</DialogDescription>
					</DialogHeader>
					<div className="space-y-1 py-2">
						<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
							Name
						</label>
						<Input
							value={formName}
							autoFocus
							onChange={event => setFormName(event.target.value)}
							onKeyDown={event => {
								if (event.key === 'Enter') void submitForm();
							}}
							placeholder="Acme Platform"
							className="h-10 bg-muted/20 text-xs"
						/>
					</div>
					<DialogFooter className="gap-2 sm:justify-between">
						{editingId && confirmDelete ? (
							<Button
								type="button"
								variant="ghost"
								size="sm"
								className="text-xs text-destructive"
								onClick={() => void remove(Number(editingId))}
								disabled={isSubmitting}>
								<Trash2 className="size-3.5" /> Click again to confirm delete
							</Button>
						) : (
							<span />
						)}
						<div className="flex gap-2">
							<Button
								type="button"
								variant="secondary"
								size="sm"
								className="h-9 text-xs"
								onClick={() => setIsFormOpen(false)}>
								Cancel
							</Button>
							<Button
								type="button"
								size="sm"
								className="h-9 text-xs"
								disabled={isSubmitting}
								onClick={() => void submitForm()}>
								{isSubmitting && (
									<Loader2 className="size-3.5 animate-spin" />
								)}
								{editingId ? 'Save' : 'Create'}
							</Button>
						</div>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</>
	);
}
