import {useState, useMemo} from 'react';
import {formatApiError} from '#/api/utils';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {
	Users,
	UserPlus,
	Mail,
	Search,
	MoreHorizontal,
	Trash2,
	UserCog,
} from 'lucide-react';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	useInviteCancel,
	useMemberAdd,
	useMemberRemove,
	useMemberUpdateRole,
	useInviteCreate,
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useProfile,
	type MemberItem,
	type InviteItem,
} from '#/api/live';
import {usePermissions} from '#/hooks/use-permissions';
import {getAvatarInitials} from '#/lib/avatar-utils';
import {ListCard, ListRow, ListEmpty, RowTile} from '#/components/settings/list-card';

const ROLES = ['owner', 'admin', 'developer', 'viewer'] as const;

/**
 * Settings → Team, OpenShip's team-page shape: one quiet list card per
 * subject (Members, Pending Invitations) — slim header with the action,
 * divide-y rows of avatar tile + name/email + inline role control +
 * kebab. No table chrome, no frames.
 */
export function UsersManagementPage() {
	// Live members/invites queries; the table-watch stream (ORGANIZATION_*
	// tables) invalidates them while mounted, so no zustand slices involved.
	const membersQuery = usePermissionGroupMembers();
	const invitesQuery = usePermissionGroupInvites();
	const members = (membersQuery.data ?? []) as unknown as MemberItem[];
	const invites = (invitesQuery.data ?? []) as unknown as InviteItem[];
	const userProfile = useProfile().data;
	const {can, canAny} = usePermissions();

	// Mutations for invites and members
	const addMemberMutation = useMemberAdd();
	const inviteMutation = useInviteCreate();
	const cancelInviteMutation = useInviteCancel();
	const updateRoleMutation = useMemberUpdateRole();
	const removeMemberMutation = useMemberRemove();

	const isLoading = membersQuery.isLoading;

	const [searchQuery, setSearchQuery] = useState('');
	const [roleFilter, setRoleFilter] = useState('ALL');

	// Create/Invite Dialog State
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [actionType, setActionType] = useState<'direct' | 'invite'>(
		'direct',
	);
	const [inputEmail, setInputEmail] = useState('');
	const [selectedRole, setSelectedRole] = useState('developer');
	const [isSubmitting, setIsSubmitting] = useState(false);

	// Edit Role Dialog State
	const [editingRoleUser, setEditingRoleUser] =
		useState<MemberItem | null>(null);

	// Delete Confirmation Modal State
	const [deleteTarget, setDeleteTarget] = useState<MemberItem | null>(
		null,
	);
	const [isDeleting, setIsDeleting] = useState(false);

	// Filter Members
	const filteredMembers = useMemo(() => {
		return members.filter(m => {
			const matchesSearch =
				m.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(m.name || '').toLowerCase().includes(searchQuery.toLowerCase());
			const matchesRole =
				roleFilter === 'ALL' ||
				m.role.toLowerCase() === roleFilter.toLowerCase();
			return matchesSearch && matchesRole;
		});
	}, [members, searchQuery, roleFilter]);

	const handleCreateOrInvite = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!inputEmail) return;

		setIsSubmitting(true);
		try {
			if (actionType === 'direct') {
				await addMemberMutation.mutateAsync({
					email: inputEmail,
					role: selectedRole.toUpperCase(),
				});

				toast.success(`User ${inputEmail} added successfully`);
				setInputEmail('');
			} else {
				await inviteMutation.mutateAsync({
					email: inputEmail,
					role: selectedRole.toUpperCase(),
				});

				toast.success('Invitation sent successfully');
			}

			setIsCreateOpen(false);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleUpdateRole = async (newRole: string) => {
		if (!editingRoleUser) return;
		const targetId = editingRoleUser.id;
		try {
			await updateRoleMutation.mutateAsync({
				id: Number(targetId),
				role: newRole.toUpperCase(),
			});
			toast.success('User role updated');
			setEditingRoleUser(null);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const confirmDeleteUser = async () => {
		if (!deleteTarget) return;
		const targetId = deleteTarget.id;
		setIsDeleting(true);
		try {
			await removeMemberMutation.mutateAsync(Number(targetId));
			toast.success('User removed from organization');
			setDeleteTarget(null);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsDeleting(false);
		}
	};

	const handleRevokeInvitation = async (id: string) => {
		try {
			await cancelInviteMutation.mutateAsync(Number(id));
			toast.success('Invitation revoked successfully');
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const isSelf = (m: MemberItem) =>
		userProfile &&
		((userProfile.email &&
			userProfile.email.toLowerCase() === m.email.toLowerCase()) ||
			(userProfile.id &&
				m.user_id &&
				Number(userProfile.id) === Number(m.user_id)));

	const canManageMember = (m: MemberItem) =>
		can('member:update') && !isSelf(m) && m.role.toLowerCase() !== 'owner';

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-150 fade-in">
			{/* Header */}
			<div className="flex flex-wrap items-start justify-between gap-4 sm:items-center">
				<div className="flex flex-col gap-0.5">
					<h2 className="text-sm font-medium text-foreground">Team</h2>
					<p className="text-xs text-muted-foreground">
						Members, roles, and pending invitations in this workspace.
					</p>
				</div>
				{canAny('member:create', 'invitation:create') && (
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="gap-2 px-4 text-sm">
						<UserPlus className="size-4" />
						Add or invite
					</Button>
				)}
			</div>

			{/* Members */}
			<ListCard
				title={`Members (${members.length})`}
				description="Everyone with access to this workspace."
				action={
					<div className="flex items-center gap-2">
						<div className="relative">
							<Search className="absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search members..."
								value={searchQuery}
								onChange={e => setSearchQuery(e.target.value)}
								className="h-8 w-48 bg-muted/40 pl-8 text-xs"
							/>
						</div>
						<Select value={roleFilter} onValueChange={v => setRoleFilter(v ?? 'ALL')}>
							<SelectTrigger className="h-8 w-28 bg-muted/40 text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="ALL" className="text-xs">
									All roles
								</SelectItem>
								{ROLES.map(r => (
									<SelectItem key={r} value={r} className="text-xs capitalize">
										{r}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>
				}>
				{isLoading ? (
					<div className="space-y-2 p-5">
						{[...Array(3)].map((_, i) => (
							<div
								key={i}
								className="h-12 animate-pulse rounded-xl bg-muted/30"
							/>
						))}
					</div>
				) : filteredMembers.length === 0 ? (
					<ListEmpty
						icon={Users}
						title="No members found"
						description={
							searchQuery || roleFilter !== 'ALL'
								? 'Try adjusting your search query or role filter.'
								: 'Add team members to collaborate on projects.'
						}
					/>
				) : (
					<div className="divide-y divide-border/40">
						{filteredMembers.map(member => {
							const self = isSelf(member);
							const displayName =
								self && userProfile?.name
									? userProfile.name
									: member.name || member.email.split('@')[0];
							const displayAvatar =
								self && userProfile?.avatar
									? userProfile.avatar
									: member.avatar;
							return (
								<ListRow
									key={String(member.id)}
									icon={
										displayAvatar ? (
											<img
												src={displayAvatar}
												alt={displayName}
												className="size-9 shrink-0 rounded-full bg-muted object-cover"
											/>
										) : (
											<RowTile>
												{getAvatarInitials(displayName || member.email)}
											</RowTile>
										)
									}
									title={displayName}
									badges={
										self ? (
											<span className="rounded-md bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium text-primary">
												you
											</span>
										) : member.banned ? (
											<span className="rounded-md bg-rose-500/10 px-1.5 py-0.5 text-[10px] font-medium text-rose-400">
												Banned
											</span>
										) : undefined
									}
									subtitle={member.email}>
									{canManageMember(member) ? (
										<Select
											value={member.role.toLowerCase()}
											onValueChange={v =>
												void updateRoleMutation
													.mutateAsync({
														id: Number(member.id),
														role: (v ?? '').toUpperCase(),
													})
													.then(() => toast.success('Role updated'))
													.catch(err =>
														toast.error(formatApiError(err)),
													)
											}>
											<SelectTrigger className="h-7 w-28 bg-muted/40 text-xs capitalize">
												<SelectValue />
											</SelectTrigger>
											<SelectContent>
												{ROLES.filter(r => r !== 'owner').map(r => (
													<SelectItem
														key={r}
														value={r}
														className="text-xs capitalize">
														{r}
													</SelectItem>
												))}
											</SelectContent>
										</Select>
									) : (
										<span className="rounded-md bg-muted/50 px-2 py-1 text-xs font-medium capitalize text-muted-foreground">
											{member.role.toLowerCase()}
										</span>
									)}
									{can('member:delete') && !self && (
										<DropdownMenu>
											<DropdownMenuTrigger
								render={
												<Button
													variant="ghost"
													size="icon"
													className="size-7 rounded-lg text-muted-foreground hover:text-foreground"
												/>
								}>
												<MoreHorizontal className="size-4" />
											</DropdownMenuTrigger>
											<DropdownMenuContent align="end" className="w-44">
												<DropdownMenuItem
													onClick={() => setEditingRoleUser(member)}
													className="gap-2 text-xs">
													<UserCog className="size-3.5" /> Change role
												</DropdownMenuItem>
												<DropdownMenuItem
													onClick={() => setDeleteTarget(member)}
													className="gap-2 text-xs text-destructive focus:text-destructive">
													<Trash2 className="size-3.5" /> Remove from
													workspace
												</DropdownMenuItem>
											</DropdownMenuContent>
										</DropdownMenu>
									)}
								</ListRow>
							);
						})}
					</div>
				)}
			</ListCard>

			{/* Pending invitations */}
			<ListCard
				title={`Pending invitations (${invites.length})`}
				description="Invited emails that have not accepted yet — valid for 7 days.">
				{invites.length === 0 ? (
					<ListEmpty
						icon={Mail}
						title="No pending invitations"
						description="All invited team members have accepted, or no invites exist yet."
					/>
				) : (
					<div className="divide-y divide-border/40">
						{invites.map(invite => (
							<ListRow
								key={String(invite.id)}
								icon={
									<RowTile className="bg-muted/50">
										<Mail className="size-4 text-muted-foreground" />
									</RowTile>
								}
								title={invite.email}
								subtitle={`Invited ${
									invite.created_at
										? new Date(
												Number(invite.created_at) * 1000,
											).toLocaleDateString()
										: 'recently'
								} · role ${invite.role.toLowerCase()}`}
								badges={
									<span className="rounded-md bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-400">
										Pending
									</span>
								}>
								<Button
									variant="ghost"
									size="sm"
									onClick={() => void handleRevokeInvitation(invite.id)}
									className="h-7 text-xs text-muted-foreground hover:text-destructive">
									Revoke
								</Button>
							</ListRow>
						))}
					</div>
				)}
			</ListCard>

			{/* Create/Invite dialog */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Add or invite a user</DialogTitle>
						<DialogDescription>
							Add a member directly to your workspace or send an email
							invitation.
						</DialogDescription>
					</DialogHeader>

					<form
						onSubmit={handleCreateOrInvite}
						className="mt-2 flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Action
							</Label>
							<Select
								value={actionType}
								onValueChange={val =>
									setActionType(val as 'direct' | 'invite')
								}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="direct" className="text-xs">
										Direct add member
									</SelectItem>
									<SelectItem value="invite" className="text-xs">
										Send email invitation
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Email address
							</Label>
							<Input
								type="email"
								placeholder="user@company.com"
								value={inputEmail}
								onChange={e => setInputEmail(e.target.value)}
								required
								className="h-9 text-sm"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Role
							</Label>
							<Select value={selectedRole} onValueChange={v => setSelectedRole(v ?? 'developer')}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin" className="text-xs">
										Admin
									</SelectItem>
									<SelectItem value="developer" className="text-xs">
										Developer
									</SelectItem>
									<SelectItem value="viewer" className="text-xs">
										Viewer
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<Button
							type="submit"
							disabled={isSubmitting}
							className="mt-2 h-9 w-full text-sm">
							{isSubmitting
								? 'Saving...'
								: actionType === 'direct'
									? 'Add user'
									: 'Send invite'}
						</Button>
					</form>
				</DialogContent>
			</Dialog>

			{/* Change role dialog */}
			<Dialog
				open={Boolean(editingRoleUser)}
				onOpenChange={() => setEditingRoleUser(null)}>
				<DialogContent className="sm:max-w-xs">
					<DialogHeader>
						<DialogTitle>Change role</DialogTitle>
						<DialogDescription className="font-mono text-xs">
							{editingRoleUser?.email}
						</DialogDescription>
					</DialogHeader>

					<div className="mt-2 flex flex-col gap-2">
						{ROLES.filter(r => r !== 'owner').map(r => (
							<Button
								key={r}
								variant={
									editingRoleUser?.role.toLowerCase() === r
										? 'default'
										: 'outline'
								}
								onClick={() => void handleUpdateRole(r)}
								className="h-9 justify-start capitalize">
								{r}
							</Button>
						))}
					</div>
				</DialogContent>
			</Dialog>

			{/* Remove member confirmation */}
			<AlertDialog
				open={Boolean(deleteTarget)}
				onOpenChange={() => setDeleteTarget(null)}>
				<AlertDialogContent className="sm:max-w-md">
					<AlertDialogHeader>
						<AlertDialogTitle>Remove workspace member</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to remove{' '}
							<span className="font-mono font-semibold text-foreground">
								{deleteTarget?.email}
							</span>
							?
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogAction
						onClick={confirmDeleteUser}
						disabled={isDeleting}
						className="h-9 w-full bg-destructive text-white hover:bg-destructive/90">
						{isDeleting ? 'Removing...' : 'Confirm remove'}
					</AlertDialogAction>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
