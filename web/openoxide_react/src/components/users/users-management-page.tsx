import {useState, useMemo} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
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
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';

import {
	Users,
	UserPlus,
	Shield,
	Mail,
	Search,
	MoreHorizontal,
	Trash2,
	UserCheck,
	Clock,
	XCircle,
} from 'lucide-react';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {useAppStore, type MemberItem} from '#/stores/app-store';

export function UsersManagementPage() {
	const queryClient = useQueryClient();
	const members = useAppStore(state => state.members);
	const invites = useAppStore(state => state.invites);
	const userProfile = useAppStore(state => state.profile);
	const isMembersLoading = useAppStore(state => state.isMembersLoading);
	const addMemberStore = useAppStore(state => state.addMember);
	const updateMemberStore = useAppStore(state => state.updateMember);
	const deleteMemberStore = useAppStore(state => state.deleteMember);
	const addInviteStore = useAppStore(state => state.addInvite);
	const deleteInviteStore = useAppStore(state => state.deleteInvite);

	// Mutations for invites and members
	const addMemberMutation = $api.useMutation(
		'post',
		'/permission-groups/members' as any,
	);
	const inviteMutation = $api.useMutation(
		'post',
		'/permission-groups/invites' as any,
	);
	const cancelInviteMutation = $api.useMutation(
		'delete',
		'/permission-groups/invites/{invite_id}' as any,
	);
	const updateRoleMutation = $api.useMutation(
		'put',
		'/permission-groups/members/{user_id}/role' as any,
	);
	const removeMemberMutation = $api.useMutation(
		'delete',
		'/permission-groups/members/{user_id}' as any,
	);

	const isLoading = isMembersLoading && members.length === 0;

	// Tabs: "members" | "invitations"
	const [activeTab, setActiveTab] = useState<'members' | 'invitations'>(
		'members',
	);
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

	// Filter Invitations
	const filteredInvites = useMemo(() => {
		return invites.filter(inv =>
			inv.email.toLowerCase().includes(searchQuery.toLowerCase()),
		);
	}, [invites, searchQuery]);

	const handleCreateOrInvite = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!inputEmail) return;

		setIsSubmitting(true);
		try {
			if (actionType === 'direct') {
				const tempId = `mem_${Date.now()}`;
				addMemberStore({
					id: tempId,
					email: inputEmail,
					name: inputEmail.split('@')[0],
					role: selectedRole.toLowerCase(),
				});
				await addMemberMutation.mutateAsync({
					body: {
						email: inputEmail,
						role: selectedRole.toUpperCase(),
					},
				} as any);

				toast.success(`User ${inputEmail} added successfully`);
				setInputEmail('');
			} else {
				const tempId = String(Date.now());
				addInviteStore({
					id: tempId,
					email: inputEmail,
					role: selectedRole.toLowerCase(),
					created_at: Math.floor(Date.now() / 1000),
				});
				await inviteMutation.mutateAsync({
					body: {
						email: inputEmail,
						role: selectedRole.toUpperCase(),
					},
				} as any);

				toast.success('Invitation sent successfully');
				setActiveTab('invitations');
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
		updateMemberStore(targetId, {role: newRole});
		try {
			await updateRoleMutation.mutateAsync({
				params: {
					path: {
						user_id: Number(targetId),
					},
				},
				body: {
					role: newRole.toUpperCase(),
				},
			} as any);
			toast.success('User role updated');
			setEditingRoleUser(null);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const confirmDeleteUser = async () => {
		if (!deleteTarget) return;
		const targetId = deleteTarget.id;
		deleteMemberStore(targetId);
		setIsDeleting(true);
		try {
			await removeMemberMutation.mutateAsync({
				params: {
					path: {
						user_id: Number(targetId),
					},
				},
			} as any);
			toast.success('User removed from organization');
			setDeleteTarget(null);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsDeleting(false);
		}
	};

	const handleRevokeInvitation = async (id: string) => {
		deleteInviteStore(id);
		try {
			await cancelInviteMutation.mutateAsync({
				params: {
					path: {
						invite_id: Number(id),
					},
				},
			} as any);
			toast.success('Invitation revoked successfully');
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-5xl animate-in flex-col gap-6 pb-12 duration-150 fade-in">
			{/* Simple Minimal Header */}
			<div className="flex flex-wrap items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div className="flex flex-col gap-1">
					<div className="flex items-center gap-2">
						<Users className="size-5 shrink-0 text-muted-foreground" />
						<h1 className="text-xl font-bold tracking-tight text-foreground">
							Users & Organization Access
						</h1>
					</div>
					<p className="text-xs text-muted-foreground">
						Manage workspace members, pending invitations, and roles
					</p>
				</div>

				<Button
					onClick={() => setIsCreateOpen(true)}
					className="h-9 cursor-pointer gap-2 bg-primary px-4 text-xs font-semibold text-primary-foreground hover:bg-primary/90">
					<UserPlus className="size-4" />
					<span>Add or Invite User</span>
				</Button>
			</div>

			{/* Simple & Minimal Tab & Filter Bar */}
			<div className="flex flex-col items-stretch justify-between gap-3 sm:flex-row sm:items-center">
				{/* Clean Minimal Tab Switcher */}
				<div className="flex items-center gap-6 border-b border-border/40">
					<button
						onClick={() => setActiveTab('members')}
						className={`relative cursor-pointer pb-2.5 text-xs font-semibold transition-colors ${
							activeTab === 'members'
								? 'border-b-2 border-primary text-primary'
								: 'text-muted-foreground hover:text-foreground'
						}`}>
						Members ({members.length})
					</button>
					<button
						onClick={() => setActiveTab('invitations')}
						className={`relative cursor-pointer pb-2.5 text-xs font-semibold transition-colors ${
							activeTab === 'invitations'
								? 'border-b-2 border-primary text-primary'
								: 'text-muted-foreground hover:text-foreground'
						}`}>
						Pending Invites ({invites.length})
					</button>
				</div>

				{/* Search & Role Filter Inputs */}
				<div className="flex items-center gap-2">
					<div className="relative flex-1 sm:w-64">
						<Search className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
						<Input
							placeholder={
								activeTab === 'members'
									? 'Search members...'
									: 'Search invites...'
							}
							value={searchQuery}
							onChange={e => setSearchQuery(e.target.value)}
							className="h-8 border-border/80 bg-background pl-8 text-xs"
						/>
					</div>

					{activeTab === 'members' && (
						<Select value={roleFilter} onValueChange={setRoleFilter}>
							<SelectTrigger className="h-8 w-[110px] border-border/80 bg-background font-mono text-xs">
								<SelectValue placeholder="Role" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="ALL" className="text-xs">
									All Roles
								</SelectItem>
								<SelectItem value="owner" className="text-xs">
									Owner
								</SelectItem>
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
					)}
				</div>
			</div>

			{/* Main Content Area */}
			{isLoading ? (
				<div className="flex justify-center py-20">
					<div className="size-5 animate-spin rounded-full border-2 border-primary/30 border-t-primary" />
				</div>
			) : activeTab === 'members' ? (
				/* Members Table */
				filteredMembers.length === 0 ? (
					<div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-border/60 bg-card/10 py-16">
						<Users className="mb-2 size-8 text-muted-foreground/40" />
						<h3 className="text-xs font-semibold text-foreground">
							No members found
						</h3>
						<p className="mt-0.5 text-[11px] text-muted-foreground">
							{searchQuery || roleFilter !== 'ALL'
								? 'Try adjusting your search query or role filter.'
								: 'Add team members to collaborate on projects.'}
						</p>
					</div>
				) : (
					<div className="overflow-hidden rounded-xl border border-border/80 bg-card shadow-xs">
						<Table>
							<TableHeader className="bg-muted/40">
								<TableRow className="border-border/60 hover:bg-transparent">
									<TableHead className="h-9 text-xs font-semibold text-foreground">
										User
									</TableHead>
									<TableHead className="h-9 text-xs font-semibold text-foreground">
										Role
									</TableHead>
									<TableHead className="h-9 text-xs font-semibold text-foreground">
										Status
									</TableHead>
									<TableHead className="h-9 pr-4 text-right text-xs font-semibold text-foreground">
										Actions
									</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredMembers.map(member => {
									const isCurrentUser =
										userProfile &&
										((userProfile.email &&
											userProfile.email.toLowerCase() ===
												member.email.toLowerCase()) ||
											(userProfile.id &&
												member.user_id &&
												Number(userProfile.id) ===
													Number(member.user_id)));

									const displayName =
										isCurrentUser && userProfile?.name
											? userProfile.name
											: member.name ||
												(member.email
													? member.email.split('@')[0]
													: 'User');

									const displayAvatar =
										isCurrentUser && userProfile?.avatar
											? userProfile.avatar
											: member.avatar;

									return (
										<TableRow
											key={member.id}
											className="border-border/60 transition-colors hover:bg-muted/30">
											<TableCell className="py-2.5">
												<div className="flex items-center gap-3">
													{displayAvatar ? (
														<img
															src={displayAvatar}
															alt={displayName}
															className="size-7 shrink-0 rounded-full border border-border/60 object-cover"
														/>
													) : (
														<div className="flex size-7 shrink-0 items-center justify-center rounded-full border border-border/60 bg-muted text-xs font-bold text-foreground uppercase">
															{(displayName || 'U')[0]}
														</div>
													)}
													<div className="flex min-w-0 flex-col">
														<div className="flex items-center gap-1.5">
															<span className="truncate text-xs font-semibold text-foreground">
																{displayName}
															</span>
															{isCurrentUser && (
																<span className="py-0.2 rounded border border-primary/20 bg-primary/10 px-1.5 font-mono text-[10px] font-semibold text-primary">
																	You
																</span>
															)}
														</div>
														<span className="truncate font-mono text-[11px] text-muted-foreground">
															{member.email}
														</span>
													</div>
												</div>
											</TableCell>
											<TableCell className="py-2.5">
												<span
													className={`inline-flex items-center rounded border px-2 py-0.5 font-mono text-[11px] font-medium ${
														member.role.toLowerCase() === 'owner'
															? 'border-amber-500/20 bg-amber-500/10 text-amber-400'
															: member.role.toLowerCase() === 'admin'
																? 'border-indigo-500/20 bg-indigo-500/10 text-indigo-400'
																: member.role.toLowerCase() === 'developer'
																	? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400'
																	: 'border-zinc-500/20 bg-zinc-500/10 text-zinc-400'
													}`}>
													<Shield className="mr-1 size-3" />
													{member.role.toUpperCase()}
												</span>
											</TableCell>
											<TableCell className="py-2.5">
												{member.banned ? (
													<span className="inline-flex items-center rounded border border-rose-500/20 bg-rose-500/10 px-2 py-0.5 font-mono text-[11px] text-rose-400">
														<XCircle className="mr-1 size-3" /> Banned
													</span>
												) : (
													<span className="inline-flex items-center rounded border border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 font-mono text-[11px] text-emerald-400">
														Active
													</span>
												)}
											</TableCell>
											<TableCell className="py-2.5 pr-4 text-right">
												<DropdownMenu>
													<DropdownMenuTrigger asChild>
														<Button
															variant="ghost"
															size="icon"
															className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
															<MoreHorizontal className="size-4" />
														</Button>
													</DropdownMenuTrigger>
													<DropdownMenuContent
														align="end"
														className="w-40">
														<DropdownMenuItem
															onClick={() => setEditingRoleUser(member)}
															className="cursor-pointer gap-2 text-xs">
															<Shield className="size-3.5 text-primary" />
															<span>Change Role</span>
														</DropdownMenuItem>
														<DropdownMenuItem
															onClick={() => setDeleteTarget(member)}
															className="cursor-pointer gap-2 text-xs text-rose-400 focus:text-rose-400">
															<Trash2 className="size-3.5" />
															<span>Remove User</span>
														</DropdownMenuItem>
													</DropdownMenuContent>
												</DropdownMenu>
											</TableCell>
										</TableRow>
									);
								})}
							</TableBody>
						</Table>
					</div>
				)
			) : /* Invitations Table */
			filteredInvites.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-border/60 bg-card/10 py-16">
					<Mail className="mb-2 size-8 text-muted-foreground/40" />
					<h3 className="text-xs font-semibold text-foreground">
						No pending invitations
					</h3>
					<p className="mt-0.5 text-[11px] text-muted-foreground">
						All invited team members have accepted or no pending invites
						exist.
					</p>
				</div>
			) : (
				<div className="overflow-hidden rounded-xl border border-border/80 bg-card shadow-xs">
					<Table>
						<TableHeader className="bg-muted/40">
							<TableRow className="border-border/60 hover:bg-transparent">
								<TableHead className="h-9 text-xs font-semibold text-foreground">
									Invited Email
								</TableHead>
								<TableHead className="h-9 text-xs font-semibold text-foreground">
									Assigned Role
								</TableHead>
								<TableHead className="h-9 text-xs font-semibold text-foreground">
									Sent Date
								</TableHead>
								<TableHead className="h-9 pr-4 text-right text-xs font-semibold text-foreground">
									Action
								</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{filteredInvites.map(invite => (
								<TableRow
									key={invite.id}
									className="border-border/60 transition-colors hover:bg-muted/30">
									<TableCell className="py-2.5">
										<div className="flex items-center gap-2">
											<Mail className="size-3.5 shrink-0 text-muted-foreground" />
											<span className="font-mono text-xs font-medium text-foreground">
												{invite.email}
											</span>
										</div>
									</TableCell>
									<TableCell className="py-2.5">
										<span className="inline-flex items-center rounded border border-indigo-500/20 bg-indigo-500/10 px-2 py-0.5 font-mono text-[11px] text-indigo-400">
											{invite.role.toUpperCase()}
										</span>
									</TableCell>
									<TableCell className="py-2.5 font-mono text-xs text-muted-foreground">
										{invite.created_at
											? new Date(
													invite.created_at * 1000,
												).toLocaleDateString()
											: 'Recently'}
									</TableCell>
									<TableCell className="py-2.5 pr-4 text-right">
										<Button
											variant="ghost"
											size="sm"
											onClick={() => handleRevokeInvitation(invite.id)}
											className="h-7 font-mono text-xs text-rose-400 hover:bg-rose-500/10 hover:text-rose-300">
											Revoke
										</Button>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</div>
			)}

			{/* Simple & Minimal Create/Invite Modal (NO CANCEL BUTTON) */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle className="text-base font-bold text-foreground">
							Add or Invite User
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
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
										Direct Add Member
									</SelectItem>
									<SelectItem value="invite" className="text-xs">
										Send Email Invitation
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Email Address
							</Label>
							<Input
								type="email"
								placeholder="user@company.com"
								value={inputEmail}
								onChange={e => setInputEmail(e.target.value)}
								required
								className="h-9 text-xs"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Role
							</Label>
							<Select value={selectedRole} onValueChange={setSelectedRole}>
								<SelectTrigger className="h-9 font-mono text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin" className="font-mono text-xs">
										Admin
									</SelectItem>
									<SelectItem
										value="developer"
										className="font-mono text-xs">
										Developer
									</SelectItem>
									<SelectItem value="viewer" className="font-mono text-xs">
										Viewer
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="mt-2">
							<Button
								type="submit"
								disabled={isSubmitting}
								className="h-9 w-full text-xs font-semibold">
								{isSubmitting
									? 'Saving...'
									: actionType === 'direct'
										? 'Add User'
										: 'Send Invite'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Simple & Minimal Edit Role Dialog (NO CANCEL BUTTON) */}
			<Dialog
				open={Boolean(editingRoleUser)}
				onOpenChange={() => setEditingRoleUser(null)}>
				<DialogContent className="border-border/80 sm:max-w-xs">
					<DialogHeader>
						<DialogTitle className="text-sm font-bold text-foreground">
							Change User Role
						</DialogTitle>
						<DialogDescription className="font-mono text-xs text-muted-foreground">
							{editingRoleUser?.email}
						</DialogDescription>
					</DialogHeader>

					<div className="mt-2 flex flex-col gap-2">
						{['admin', 'developer', 'viewer'].map(r => (
							<Button
								key={r}
								variant={
									editingRoleUser?.role.toLowerCase() === r
										? 'default'
										: 'outline'
								}
								onClick={() => handleUpdateRole(r)}
								className="h-8 justify-start font-mono text-xs capitalize">
								<Shield className="mr-2 size-3" />
								{r}
							</Button>
						))}
					</div>
				</DialogContent>
			</Dialog>

			{/* Simple & Minimal Delete Member Confirmation Modal (NO CANCEL BUTTON) */}
			<AlertDialog
				open={Boolean(deleteTarget)}
				onOpenChange={() => setDeleteTarget(null)}>
				<AlertDialogContent className="border-border/80 sm:max-w-md">
					<AlertDialogHeader>
						<AlertDialogTitle className="text-base font-bold text-foreground">
							Remove Workspace Member
						</AlertDialogTitle>
						<AlertDialogDescription className="mt-1 text-xs text-muted-foreground">
							Are you sure you want to remove{' '}
							<span className="font-mono font-semibold text-foreground">
								{deleteTarget?.email}
							</span>
							?
						</AlertDialogDescription>
					</AlertDialogHeader>
					<div className="mt-4">
						<AlertDialogAction
							onClick={confirmDeleteUser}
							disabled={isDeleting}
							className="h-9 w-full bg-rose-600 text-xs font-bold text-white hover:bg-rose-700">
							{isDeleting ? 'Removing...' : 'Confirm Remove'}
						</AlertDialogAction>
					</div>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
