import { useState, useMemo } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { $api } from '#/api/query';
import { formatApiError } from '#/api/utils';
import { toast } from 'sonner';
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
			{/* Simple Minimal Header */}
			<div className="flex items-center justify-between gap-4 border-b border-border/40 pb-6 flex-wrap">
				<div className="flex flex-col gap-1">
					<div className="flex items-center gap-2">
						<Users className="size-5 text-muted-foreground shrink-0" />
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
					className="h-9 text-xs font-semibold px-4 gap-2 bg-primary hover:bg-primary/90 text-primary-foreground cursor-pointer"
				>
					<UserPlus className="size-4" />
					<span>Add or Invite User</span>
				</Button>
			</div>

			{/* Simple & Minimal Tab & Filter Bar */}
			<div className="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3">
				{/* Clean Minimal Tab Switcher */}
				<div className="flex items-center border-b border-border/40 gap-6">
					<button
						onClick={() => setActiveTab('members')}
						className={`pb-2.5 text-xs font-semibold transition-colors relative cursor-pointer ${
							activeTab === 'members'
								? 'text-primary border-b-2 border-primary'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						Members ({members.length})
					</button>
					<button
						onClick={() => setActiveTab('invitations')}
						className={`pb-2.5 text-xs font-semibold transition-colors relative cursor-pointer ${
							activeTab === 'invitations'
								? 'text-primary border-b-2 border-primary'
								: 'text-muted-foreground hover:text-foreground'
						}`}
					>
						Pending Invites ({invites.length})
					</button>
				</div>

				{/* Search & Role Filter Inputs */}
				<div className="flex items-center gap-2">
					<div className="relative flex-1 sm:w-64">
						<Search className="size-3.5 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
						<Input
							placeholder={activeTab === 'members' ? 'Search members...' : 'Search invites...'}
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							className="h-8 pl-8 text-xs bg-background border-border/80"
						/>
					</div>

					{activeTab === 'members' && (
						<Select value={roleFilter} onValueChange={setRoleFilter}>
							<SelectTrigger className="h-8 text-xs w-[110px] bg-background border-border/80 font-mono">
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
				<div className="flex justify-center py-20">
					<div className="size-5 border-2 border-primary/30 border-t-primary rounded-full animate-spin" />
				</div>
			) : activeTab === 'members' ? (
				/* Members Table */
				filteredMembers.length === 0 ? (
					<div className="flex flex-col items-center justify-center py-16 border border-dashed border-border/60 rounded-xl bg-card/10">
						<Users className="size-8 mb-2 text-muted-foreground/40" />
						<h3 className="text-xs font-semibold text-foreground">No members found</h3>
						<p className="text-[11px] text-muted-foreground mt-0.5">
							{searchQuery || roleFilter !== 'ALL'
								? 'Try adjusting your search query or role filter.'
								: 'Add team members to collaborate on projects.'}
						</p>
					</div>
				) : (
					<div className="border border-border/80 rounded-xl overflow-hidden bg-card shadow-xs">
						<Table>
							<TableHeader className="bg-muted/40">
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
										<TableRow key={member.id} className="border-border/60 hover:bg-muted/30 transition-colors">
											<TableCell className="py-2.5">
												<div className="flex items-center gap-3">
													{displayAvatar ? (
														<img
															src={displayAvatar}
															alt={displayName}
															className="size-7 rounded-full object-cover border border-border/60 shrink-0"
														/>
													) : (
														<div className="size-7 rounded-full bg-muted border border-border/60 flex items-center justify-center text-foreground font-bold text-xs uppercase shrink-0">
															{(displayName || 'U')[0]}
														</div>
													)}
													<div className="flex flex-col min-w-0">
														<div className="flex items-center gap-1.5">
															<span className="text-xs font-semibold text-foreground truncate">
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
											<TableCell className="py-2.5">
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
											<TableCell className="py-2.5">
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
											<TableCell className="py-2.5 text-right pr-4">
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
						<Mail className="size-8 mb-2 text-muted-foreground/40" />
						<h3 className="text-xs font-semibold text-foreground">No pending invitations</h3>
						<p className="text-[11px] text-muted-foreground mt-0.5">
							All invited team members have accepted or no pending invites exist.
						</p>
					</div>
				) : (
					<div className="border border-border/80 rounded-xl overflow-hidden bg-card shadow-xs">
						<Table>
							<TableHeader className="bg-muted/40">
								<TableRow className="border-border/60 hover:bg-transparent">
									<TableHead className="text-xs font-semibold text-foreground h-9">Invited Email</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Assigned Role</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9">Sent Date</TableHead>
									<TableHead className="text-xs font-semibold text-foreground h-9 text-right pr-4">Action</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredInvites.map((invite) => (
									<TableRow key={invite.id} className="border-border/60 hover:bg-muted/30 transition-colors">
										<TableCell className="py-2.5">
											<div className="flex items-center gap-2">
												<Mail className="size-3.5 text-muted-foreground shrink-0" />
												<span className="text-xs font-mono font-medium text-foreground">{invite.email}</span>
											</div>
										</TableCell>
										<TableCell className="py-2.5">
											<span className="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-mono bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
												{invite.role.toUpperCase()}
											</span>
										</TableCell>
										<TableCell className="py-2.5 text-xs text-muted-foreground font-mono">
											{invite.created_at ? new Date(invite.created_at * 1000).toLocaleDateString() : 'Recently'}
										</TableCell>
										<TableCell className="py-2.5 text-right pr-4">
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

			{/* Simple & Minimal Create/Invite Modal (NO CANCEL BUTTON) */}
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md border-border/80">
					<DialogHeader>
						<DialogTitle className="text-base font-bold text-foreground">
							{actionType === 'direct' ? 'Add User Member' : 'Send Invitation Email'}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Add a user to this workspace or send an email invitation.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleCreateOrInvite} className="flex flex-col gap-4 mt-2">
						<div className="flex items-center gap-2">
							<Button
								type="button"
								variant={actionType === 'direct' ? 'default' : 'outline'}
								size="sm"
								onClick={() => setActionType('direct')}
								className="flex-1 text-xs h-8"
							>
								Direct Add
							</Button>
							<Button
								type="button"
								variant={actionType === 'invite' ? 'default' : 'outline'}
								size="sm"
								onClick={() => setActionType('invite')}
								className="flex-1 text-xs h-8"
							>
								Send Invitation
							</Button>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">Email Address</Label>
							<Input
								type="email"
								placeholder="user@company.com"
								value={inputEmail}
								onChange={(e) => setInputEmail(e.target.value)}
								required
								className="h-9 text-xs border-border/80"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">Role</Label>
							<Select value={selectedRole} onValueChange={setSelectedRole}>
								<SelectTrigger className="h-9 text-xs border-border/80 font-mono">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin" className="text-xs font-mono">Admin (Full Access)</SelectItem>
									<SelectItem value="developer" className="text-xs font-mono">Developer (Deploy & Manage)</SelectItem>
									<SelectItem value="viewer" className="text-xs font-mono">Viewer (Read Only)</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="mt-4">
							<Button type="submit" disabled={isSubmitting} className="w-full text-xs font-semibold h-9">
								{isSubmitting ? 'Saving...' : actionType === 'direct' ? 'Add User' : 'Send Invite'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Simple & Minimal Edit Role Dialog (NO CANCEL BUTTON) */}
			<Dialog open={Boolean(editingRoleUser)} onOpenChange={() => setEditingRoleUser(null)}>
				<DialogContent className="sm:max-w-xs border-border/80">
					<DialogHeader>
						<DialogTitle className="text-sm font-bold text-foreground">Change User Role</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground font-mono">
							{editingRoleUser?.email}
						</DialogDescription>
					</DialogHeader>

					<div className="flex flex-col gap-2 mt-2">
						{['admin', 'developer', 'viewer'].map((r) => (
							<Button
								key={r}
								variant={editingRoleUser?.role.toLowerCase() === r ? 'default' : 'outline'}
								onClick={() => handleUpdateRole(r)}
								className="text-xs justify-start h-8 font-mono capitalize"
							>
								<Shield className="size-3 mr-2" />
								{r}
							</Button>
						))}
					</div>
				</DialogContent>
			</Dialog>

			{/* Simple & Minimal Delete Member Confirmation Modal (NO CANCEL BUTTON) */}
			<AlertDialog open={Boolean(deleteTarget)} onOpenChange={() => setDeleteTarget(null)}>
				<AlertDialogContent className="sm:max-w-md border-border/80">
					<AlertDialogHeader>
						<AlertDialogTitle className="text-base font-bold text-foreground">
							Remove Workspace Member
						</AlertDialogTitle>
						<AlertDialogDescription className="text-xs text-muted-foreground mt-1">
							Are you sure you want to remove <span className="font-mono font-semibold text-foreground">{deleteTarget?.email}</span>?
						</AlertDialogDescription>
					</AlertDialogHeader>
					<div className="mt-4">
						<AlertDialogAction
							onClick={confirmDeleteUser}
							disabled={isDeleting}
							className="w-full text-xs h-9 bg-rose-600 hover:bg-rose-700 text-white font-bold"
						>
							{isDeleting ? 'Removing...' : 'Confirm Remove'}
						</AlertDialogAction>
					</div>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
