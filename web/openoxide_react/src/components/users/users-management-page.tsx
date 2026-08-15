import { useState, useMemo } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Label } from '#/components/ui/label';
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
	DialogFooter,
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
	AlertCircle,
} from 'lucide-react';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import { useAppStore, type MemberItem } from '#/stores/app-store';

export function UsersManagementPage() {
	const queryClient = useQueryClient();
	const members = useAppStore((state) => state.members);
	const invites = useAppStore((state) => state.invites);
	const userProfile = useAppStore((state) => state.profile);
	const isMembersLoading = useAppStore((state) => state.isMembersLoading);
	const addMemberStore = useAppStore((state) => state.addMember);
	const updateMemberStore = useAppStore((state) => state.updateMember);
	const deleteMemberStore = useAppStore((state) => state.deleteMember);
	const addInviteStore = useAppStore((state) => state.addInvite);
	const deleteInviteStore = useAppStore((state) => state.deleteInvite);

	// Mutations for invites and members
	const addMemberMutation = $api.useMutation('post', '/permission-groups/members' as any);
	const inviteMutation = $api.useMutation('post', '/permission-groups/invites' as any);
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
	const [activeTab, setActiveTab] = useState<'members' | 'invitations'>('members');
	const [searchQuery, setSearchQuery] = useState('');
	const [roleFilter, setRoleFilter] = useState('ALL');

	// Create/Invite Dialog State
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [actionType, setActionType] = useState<'direct' | 'invite'>('direct');
	const [inputEmail, setInputEmail] = useState('');
	const [selectedRole, setSelectedRole] = useState('developer');
	const [isSubmitting, setIsSubmitting] = useState(false);

	// Edit Role Dialog State
	const [editingRoleUser, setEditingRoleUser] = useState<MemberItem | null>(null);

	// Delete Confirmation Modal State
	const [deleteTarget, setDeleteTarget] = useState<MemberItem | null>(null);
	const [isDeleting, setIsDeleting] = useState(false);

	// Filter Members
	const filteredMembers = useMemo(() => {
		return members.filter((m) => {
			const matchesSearch =
				m.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(m.name || '').toLowerCase().includes(searchQuery.toLowerCase());
			const matchesRole = roleFilter === 'ALL' || m.role.toLowerCase() === roleFilter.toLowerCase();
			return matchesSearch && matchesRole;
		});
	}, [members, searchQuery, roleFilter]);

	// Filter Invitations
	const filteredInvites = useMemo(() => {
		return invites.filter((inv) => inv.email.toLowerCase().includes(searchQuery.toLowerCase()));
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
		updateMemberStore(targetId, { role: newRole });
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
		<div className="flex flex-col gap-6 w-full max-w-5xl mx-auto pb-12 animate-in fade-in duration-150">
			{/* Top Header Row */}
			<div className="flex items-center justify-between gap-4 border-b border-border/40 pb-6 flex-wrap">
				<div className="flex flex-col gap-1">
					<div className="flex items-center gap-2.5">
						<Users className="size-6 text-primary shrink-0" />
						<h1 className="text-xl font-bold tracking-tight text-foreground">
							Users & Organization Access
						</h1>
					</div>
					<p className="text-xs text-muted-foreground">
						Manage workspace members, invite teammates, and assign role-based access control (RBAC).
					</p>
				</div>

				<Button
					onClick={() => setIsCreateOpen(true)}
					className="h-9 text-xs font-bold px-4 gap-2 bg-primary hover:bg-primary/90 text-primary-foreground shadow-xs transition-all active:scale-95 cursor-pointer"
				>
					<UserPlus className="size-4" />
					<span>Add or Invite User</span>
				</Button>
			</div>

			{/* Filter Toolbar & Tabs */}
			<div className="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-4 bg-card/40 p-3 border border-border/60 rounded-xl">
				{/* Tab Selector */}
				<div className="flex items-center p-1 bg-muted/40 rounded-lg border border-border/40 shrink-0">
					<button
						onClick={() => setActiveTab('members')}
						className={`px-3 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-2 ${
							activeTab === 'members'
								? 'bg-background text-foreground shadow-xs font-semibold'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						<UserCheck className="size-3.5" />
						<span>Members ({members.length})</span>
					</button>
					<button
						onClick={() => setActiveTab('invitations')}
						className={`px-3 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-2 ${
							activeTab === 'invitations'
								? 'bg-background text-foreground shadow-xs font-semibold'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						<Clock className="size-3.5" />
						<span>Pending Invites ({invites.length})</span>
					</button>
				</div>

				{/* Search & Role Filters */}
				<div className="flex items-center gap-2.5 flex-1 max-w-md">
					<div className="relative flex-1">
						<Search className="size-3.5 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
						<Input
							placeholder={activeTab === 'members' ? 'Search member email or name...' : 'Search pending invites...'}
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							className="h-8 pl-8 text-xs bg-background/50 border-border/80"
						/>
					</div>

					{activeTab === 'members' && (
						<Select value={roleFilter} onValueChange={setRoleFilter}>
							<SelectTrigger className="h-8 text-xs w-[120px] bg-background/50 border-border/80 font-mono">
								<SelectValue placeholder="Role" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="ALL" className="text-xs">All Roles</SelectItem>
								<SelectItem value="owner" className="text-xs">Owner</SelectItem>
								<SelectItem value="admin" className="text-xs">Admin</SelectItem>
								<SelectItem value="developer" className="text-xs">Developer</SelectItem>
								<SelectItem value="viewer" className="text-xs">Viewer</SelectItem>
							</SelectContent>
						</Select>
					)}
				</div>
			</div>

			{/* Main Content Area */}
			{isLoading ? (
				<div className="flex justify-center py-24">
					<div className="size-6 border-2 border-primary/30 border-t-primary rounded-full animate-spin" />
				</div>
			) : activeTab === 'members' ? (
				/* Members Table */
				filteredMembers.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 border border-dashed border-border/60 rounded-xl bg-card/10">
						<Users className="size-10 mb-3 text-muted-foreground/40" />
						<h3 className="text-sm font-semibold text-foreground">No members found</h3>
						<p className="text-xs text-muted-foreground mt-1">
							{searchQuery || roleFilter !== 'ALL'
								? 'Try adjusting your search query or role filter.'
								: 'Add team members to collaborate on projects.'}
						</p>
					</div>
				) : (
					<div className="border border-border/80 rounded-xl overflow-hidden bg-card/30 shadow-xs">
						<Table>
							<TableHeader className="bg-muted/30">
								<TableRow className="border-border/60 hover:bg-transparent">
									<TableHead className="text-xs font-semibold text-foreground h-9">User</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Role</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Status</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9 text-right pr-4">Actions</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredMembers.map((member) => {
									const isCurrentUser = userProfile && (
										(userProfile.email && userProfile.email.toLowerCase() === member.email.toLowerCase()) ||
										(userProfile.id && member.user_id && Number(userProfile.id) === Number(member.user_id))
									);

									const displayName = (isCurrentUser && userProfile?.name)
										? userProfile.name
										: member.name || (member.email ? member.email.split('@')[0] : 'User');

									const displayAvatar = (isCurrentUser && userProfile?.avatar)
										? userProfile.avatar
										: member.avatar;

									return (
										<TableRow key={member.id} className="border-border/60 hover:bg-muted/20 transition-colors">
											<TableCell className="py-3">
												<div className="flex items-center gap-3">
													{displayAvatar ? (
														<img
															src={displayAvatar}
															alt={displayName}
															className="size-8 rounded-full object-cover border border-primary/20 shrink-0"
														/>
													) : (
														<div className="size-8 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-primary font-bold text-xs uppercase shrink-0">
															{(displayName || 'U')[0]}
														</div>
													)}
													<div className="flex flex-col min-w-0">
														<div className="flex items-center gap-1.5">
															<span className="text-xs font-bold text-foreground truncate">
																{displayName}
															</span>
															{isCurrentUser && (
																<span className="text-[10px] bg-primary/10 text-primary font-semibold px-1.5 py-0.2 rounded border border-primary/20 font-mono">
																	You
																</span>
															)}
														</div>
														<span className="text-[11px] text-muted-foreground font-mono truncate">
															{member.email}
														</span>
													</div>
												</div>
											</TableCell>
											<TableCell className="py-3">
												<span className={`inline-flex items-center px-2 py-0.5 rounded text-[11px] font-mono font-medium border ${
													member.role.toLowerCase() === 'owner' ? 'bg-amber-500/10 text-amber-400 border-amber-500/20' :
													member.role.toLowerCase() === 'admin' ? 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20' :
													member.role.toLowerCase() === 'developer' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' :
													'bg-zinc-500/10 text-zinc-400 border-zinc-500/20'
												}`}>
													<Shield className="size-3 mr-1" />
													{member.role.toUpperCase()}
												</span>
											</TableCell>
											<TableCell className="py-3">
												{member.banned ? (
													<span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-mono bg-rose-500/10 text-rose-400 border border-rose-500/20">
														<XCircle className="size-3 mr-1" /> Banned
													</span>
												) : (
													<span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-mono bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
														Active
													</span>
												)}
											</TableCell>
											<TableCell className="py-3 text-right pr-4">
												<DropdownMenu>
													<DropdownMenuTrigger asChild>
														<Button variant="ghost" size="icon" className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
															<MoreHorizontal className="size-4" />
														</Button>
													</DropdownMenuTrigger>
													<DropdownMenuContent align="end" className="w-40">
														<DropdownMenuItem
															onClick={() => setEditingRoleUser(member)}
															className="text-xs gap-2 cursor-pointer"
														>
															<Shield className="size-3.5 text-primary" />
															<span>Change Role</span>
														</DropdownMenuItem>
														<DropdownMenuItem
															onClick={() => setDeleteTarget(member)}
															className="text-xs text-rose-400 focus:text-rose-400 gap-2 cursor-pointer"
														>
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
			) : (
				/* Invitations Table */
				filteredInvites.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 border border-dashed border-border/60 rounded-xl bg-card/10">
						<Mail className="size-10 mb-3 text-muted-foreground/40" />
						<h3 className="text-sm font-semibold text-foreground">No pending invitations</h3>
						<p className="text-xs text-muted-foreground mt-1">
							All invited team members have accepted or no pending invites exist.
						</p>
					</div>
				) : (
					<div className="border border-border/80 rounded-xl overflow-hidden bg-card/30 shadow-xs">
						<Table>
							<TableHeader className="bg-muted/30">
								<TableRow className="border-border/60 hover:bg-transparent">
									<TableHead className="text-xs font-semibold text-foreground h-9">Invited Email</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Assigned Role</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Sent Date</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9 text-right pr-4">Action</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredInvites.map((invite) => (
									<TableRow key={invite.id} className="border-border/60 hover:bg-muted/20 transition-colors">
										<TableCell className="py-3">
											<div className="flex items-center gap-2.5">
												<Mail className="size-4 text-muted-foreground shrink-0" />
												<span className="text-xs font-mono font-medium text-foreground">{invite.email}</span>
											</div>
										</TableCell>
										<TableCell className="py-3">
											<span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-mono bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
												{invite.role.toUpperCase()}
											</span>
										</TableCell>
										<TableCell className="py-3 text-xs text-muted-foreground font-mono">
											{invite.created_at ? new Date(invite.created_at * 1000).toLocaleDateString() : 'Recently'}
										</TableCell>
										<TableCell className="py-3 text-right pr-4">
											<Button
												variant="ghost"
												size="sm"
												onClick={() => handleRevokeInvitation(invite.id)}
												className="h-7 text-xs text-rose-400 hover:text-rose-300 hover:bg-rose-500/10 font-mono"
											>
												Revoke
											</Button>
										</TableCell>
									</TableRow>
								))}
							</TableBody>
						</Table>
					</div>
				)
			)}

			{/* Create/Invite Modal */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md bg-[#09090b] border-border/80">
					<DialogHeader>
						<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<UserPlus className="size-5 text-primary" />
							Add User or Send Invite
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Add an existing user directly to this workspace or send an email invitation.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleCreateOrInvite} className="flex flex-col gap-4 mt-2">
						<div className="flex items-center p-1 bg-muted/40 rounded-lg border border-border/40">
							<button
								type="button"
								onClick={() => setActionType('direct')}
								className={`flex-1 py-1.5 text-xs font-medium rounded-md transition-all ${
									actionType === 'direct'
										? 'bg-background text-foreground shadow-xs font-semibold'
										: 'text-muted-foreground hover:text-foreground'
								}`}
							>
								Direct Add
							</button>
							<button
								type="button"
								onClick={() => setActionType('invite')}
								className={`flex-1 py-1.5 text-xs font-medium rounded-md transition-all ${
									actionType === 'invite'
										? 'bg-background text-foreground shadow-xs font-semibold'
										: 'text-muted-foreground hover:text-foreground'
								}`}
							>
								Send Invitation Email
							</button>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-medium text-foreground">User Email Address</Label>
							<Input
								type="email"
								placeholder="teammate@company.com"
								value={inputEmail}
								onChange={(e) => setInputEmail(e.target.value)}
								required
								className="h-9 text-xs bg-background/50 border-border/80"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-medium text-foreground">Assigned Role</Label>
							<Select value={selectedRole} onValueChange={setSelectedRole}>
								<SelectTrigger className="h-9 text-xs bg-background/50 border-border/80">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin" className="text-xs">Admin (Full Access)</SelectItem>
									<SelectItem value="developer" className="text-xs">Developer (Deploy & Manage Services)</SelectItem>
									<SelectItem value="viewer" className="text-xs">Viewer (Read Only)</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<DialogFooter className="mt-4 gap-2 sm:gap-0">
							<Button type="button" variant="ghost" size="sm" onClick={() => setIsCreateOpen(false)} className="text-xs">
								Cancel
							</Button>
							<Button type="submit" size="sm" disabled={isSubmitting} className="text-xs font-bold px-4">
								{isSubmitting ? 'Processing...' : actionType === 'direct' ? 'Add User' : 'Send Invite'}
							</Button>
						</DialogFooter>
					</form>
				</DialogContent>
			</Dialog>

			{/* Edit Role Dialog */}
			<Dialog open={Boolean(editingRoleUser)} onOpenChange={() => setEditingRoleUser(null)}>
				<DialogContent className="sm:max-w-xs bg-[#09090b] border-border/80">
					<DialogHeader>
						<DialogTitle className="text-sm font-bold text-foreground">Change User Role</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Select new role for {editingRoleUser?.email}
						</DialogDescription>
					</DialogHeader>

					<div className="flex flex-col gap-3 mt-2">
						{['admin', 'developer', 'viewer'].map((r) => (
							<Button
								key={r}
								variant={editingRoleUser?.role.toLowerCase() === r ? 'default' : 'outline'}
								onClick={() => handleUpdateRole(r)}
								className="text-xs justify-start h-9 capitalize"
							>
								<Shield className="size-3.5 mr-2" />
								{r}
							</Button>
						))}
					</div>
				</DialogContent>
			</Dialog>

			{/* Delete Member Confirmation Modal */}
			<AlertDialog open={Boolean(deleteTarget)} onOpenChange={() => setDeleteTarget(null)}>
				<AlertDialogContent className="sm:max-w-md bg-[#09090b] border-border/80">
					<AlertDialogHeader>
						<AlertDialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
							<AlertCircle className="size-5 text-rose-500 shrink-0" />
							Remove Workspace Member
						</AlertDialogTitle>
						<AlertDialogDescription className="text-xs text-muted-foreground">
							Are you sure you want to remove <span className="font-semibold text-foreground">{deleteTarget?.email}</span> from this organization? They will lose access to all projects.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter className="mt-4 gap-2 sm:gap-0">
						<AlertDialogCancel className="text-xs h-8">Cancel</AlertDialogCancel>
						<AlertDialogAction
							onClick={confirmDeleteUser}
							disabled={isDeleting}
							className="text-xs h-8 bg-rose-600 hover:bg-rose-700 text-white font-bold"
						>
							{isDeleting ? 'Removing...' : 'Confirm Remove'}
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
