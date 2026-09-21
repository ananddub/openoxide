import * as React from 'react';
import {
	Building2,
	Check,
	ChevronsUpDown,
	Loader2,
	LogOut,
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
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
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

type Props = {
	collapsed: boolean;
};

/**
 * Bottom-of-rail account + organization switcher (OpenShip layout): the
 * active workspace as an avatar tile, a popover listing every organization
 * with per-org role chips, workspace management (create / rename / leave /
 * delete), and a signed-in footer with logout.
 */
export function AccountSwitcher({collapsed}: Props) {
	const {data: organizations = []} = useOrganizationListByOwner();
	const {activeOrg, setOrganizations, setActiveOrg} = useOrganizations();
	const {user, logout} = useAuthSession();
	const queryClient = useQueryClient();

	const createMutation = useOrganizationCreate();
	const updateMutation = useOrganizationUpdate();
	const leaveMutation = useOrganizationLeave();
	const deleteMutation = useOrganizationDelete();

	const [open, setOpen] = React.useState(false);
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

	const displayName =
		[user?.firstName, user?.lastName].filter(Boolean).join(' ') ||
		user?.email?.split('@')[0] ||
		'User';
	const displayEmail = user?.email || '';
	const displayInitial = displayName[0] ?? '?';

	const activeId = activeOrg?.id ?? null;
	const activeEntry = organizations.find(org => String(org.id) === String(activeId));
	const activeRole = (activeEntry?.role ?? 'MEMBER').toUpperCase();

	const switchTo = (id: number) => {
		if (id === activeId) {
			setOpen(false);
			return;
		}
		persistActiveOrganizationId(id);
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
		setOpen(false);
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
			<div className="px-3 pb-4 pt-1">
				<div className="mx-2 mb-3 h-px bg-border/60" />
				{!collapsed && (
					<p className="mb-1.5 px-2 text-[10px] font-semibold tracking-widest text-muted-foreground/60 uppercase">
						Account
					</p>
				)}

				<DropdownMenu open={open} onOpenChange={setOpen}>
					<DropdownMenuTrigger
						render={
							<button
								type="button"
								className={cn(
									'group flex w-full items-center rounded-lg px-2 py-1.5 text-start transition-colors hover:bg-foreground/[0.06]',
									collapsed ? 'justify-center' : 'gap-2.5',
								)}
								aria-haspopup="dialog"
								aria-expanded={open}
								title={collapsed ? activeOrg?.name : undefined}
							/>
						}>
						{/* Org avatar / initial */}
						<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.08] text-[13px] font-semibold text-foreground uppercase">
							{activeOrg?.name?.[0] ?? <Building2 className="size-4" />}
						</div>
						{!collapsed && (
							<>
								<div className="min-w-0 flex-1 text-start">
									<p className="truncate text-[13px] leading-tight font-semibold text-foreground">
										{activeOrg?.name || 'Workspace'}
									</p>
									<p className="truncate text-[11px] leading-tight text-muted-foreground">
										{organizations.length > 1
											? `${organizations.length} workspaces`
											: displayEmail}
									</p>
								</div>
								<ChevronsUpDown className="size-4 shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
							</>
						)}
					</DropdownMenuTrigger>

					<DropdownMenuContent
						side={collapsed ? 'right' : 'top'}
						align="start"
						className={cn(
							'overflow-hidden rounded-2xl',
							collapsed ? 'ml-2 w-72' : 'mr-1 w-[calc(var(--sidebar-width)-2rem)]',
						)}>
						{/* Heading */}
						<div className="px-3 pt-3 pb-2">
							<p className="text-[11px] font-semibold tracking-widest text-muted-foreground/70 uppercase">
								Switch Organization
							</p>
						</div>

						{/* Org list */}
						<div className="max-h-64 overflow-y-auto pb-1">
							{organizations.map(org => {
								const id = Number(org.id);
								const role = (org.role ?? 'MEMBER').toUpperCase();
								const isCurrent = String(org.id) === String(activeId);
								return (
									<button
										key={id}
										type="button"
										onClick={() => switchTo(id)}
										className={cn(
											'flex w-full items-center gap-2.5 rounded-lg px-3 py-2 text-start transition-colors hover:bg-foreground/[0.05]',
											isCurrent && 'bg-foreground/[0.03]',
										)}>
										<div className="flex size-7 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.08] text-[12px] font-semibold text-foreground uppercase">
											{org.name?.[0] ?? <Building2 className="size-3.5" />}
										</div>
										<div className="min-w-0 flex-1">
											<p className="truncate text-[13px] leading-tight font-medium text-foreground">
												{org.name}
											</p>
											<p className="mt-0.5 flex items-center gap-1.5 truncate text-[11px] leading-tight text-muted-foreground">
												{isCurrent && (
													<span className="rounded-md bg-foreground/[0.06] px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
														Current
													</span>
												)}
												<span className="capitalize">{role.toLowerCase()}</span>
											</p>
										</div>
										{isCurrent && (
											<Check className="size-4 shrink-0 text-primary" />
										)}
									</button>
								);
							})}
						</div>

						{/* Workspace management */}
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
										openEdit(String(activeEntry.id), activeEntry.name)
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

						{/* Footer: signed-in-as + sign out */}
						<div className="border-t  px-2 py-2">
							<div className="flex items-center gap-2.5 rounded-xl px-2 py-1.5">
								<div className="flex size-7 shrink-0 items-center justify-center rounded-full bg-foreground/[0.08] text-[11px] font-semibold text-foreground uppercase">
									{displayInitial}
								</div>
								<div className="min-w-0 flex-1">
									<p className="truncate text-[12px] leading-tight font-medium text-foreground">
										{displayName}
									</p>
									<p className="truncate text-[11px] leading-tight text-muted-foreground">
										{displayEmail}
									</p>
								</div>
							</div>
							<button
								type="button"
								onClick={() => logout()}
								className="mt-1 flex w-full items-center gap-2 rounded-xl px-2 py-2 text-[13px] font-medium text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
								<LogOut className="size-4" />
								Log out
							</button>
						</div>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>

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
