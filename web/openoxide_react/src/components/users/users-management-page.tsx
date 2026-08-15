import {useState, useMemo} from 'react';
import {
	Users,
	Plus,
	Search,
	Trash2,
	Pencil,
	Mail,
	MoreHorizontal,
	ShieldCheck,
} from 'lucide-react';
import {$api} from '#/api/query';
import {client} from '#/api/client';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Badge} from '#/components/ui/badge';
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
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

export interface UserMember {
	id: string;
	email: string;
	role: 'owner' | 'admin' | 'member';
	avatar?: string;
	banned: boolean;
	twoFactorEnabled: boolean;
	createdAt: string;
	isSelf?: boolean;
}

export interface InvitationItem {
	id: string;
	email: string;
	role: string;
	expiresAt: string;
}

import {
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useAuthWhoAmI,
} from 'virtual:openoxide-live';

import { useEffect } from 'react';
import { useAppStore } from '#/stores/app-store';

export function UsersManagementPage() {
	// Live Reactive WhoAmI / Profile Hook from openoxide-live
	const { data: whoamiData } = useAuthWhoAmI();

	const [activeTab, setActiveTab] = useState<'users' | 'invitations'>('users');
	const [searchQuery, setSearchQuery] = useState('');

	// 1. Live Reactive Hooks from openoxide-live (Auto-updates via Socket stream)
	const { data: membersData } = usePermissionGroupMembers();
	const { data: invitesData } = usePermissionGroupInvites();

	// 2. Zustand Store Subscriptions & Actions for Instant 0ms Optimistic Updates
	const storeMembers = useAppStore((state) => state.members);
	const storeInvites = useAppStore((state) => state.invites);
	const setMembersStore = useAppStore((state) => state.setMembers);
	const setInvitesStore = useAppStore((state) => state.setInvites);

	const addMemberStore = useAppStore((state) => state.addMember);
	const updateMemberStore = useAppStore((state) => state.updateMember);
	const deleteMemberStore = useAppStore((state) => state.deleteMember);
	const addInviteStore = useAppStore((state) => state.addInvite);
	const deleteInviteStore = useAppStore((state) => state.deleteInvite);

	const { refetch: refetchMembers } = $api.useQuery('get', '/permission-groups/members' as any, {} as any);
	const { refetch: refetchInvites } = $api.useQuery('get', '/permission-groups/invites' as any, {} as any);

	// Sync Live Backend Stream into Zustand Store
	useEffect(() => {
		if (membersData && Array.isArray(membersData)) {
			setMembersStore(membersData as any);
		}
	}, [membersData, setMembersStore]);

	useEffect(() => {
		if (invitesData && Array.isArray(invitesData)) {
			setInvitesStore(invitesData as any);
		}
	}, [invitesData, setInvitesStore]);

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

	const users: UserMember[] = useMemo(() => {
		const source = (membersData && Array.isArray(membersData) && membersData.length > 0)
			? membersData
			: (storeMembers || []);
		if (!Array.isArray(source)) return [];
		return source.map((m: any) => {
			const isSelf = whoamiData?.user_id === (m.user_id || m.id);
			return {
				id: String(m.user_id || m.id),
				email: isSelf ? (whoamiData?.email || m.email) : (m.email || `User #${m.user_id}`),
				role: (m.role || 'member').toLowerCase() as any,
				avatar: isSelf ? (whoamiData?.avatar || m.avatar) : m.avatar,
				banned: false,
				twoFactorEnabled: false,
				createdAt: m.created_at
					? new Date(m.created_at * 1000).toISOString()
					: new Date().toISOString(),
				isSelf,
			};
		});
	}, [membersData, storeMembers, whoamiData]);

	const invitations: InvitationItem[] = useMemo(() => {
		const source = (invitesData && Array.isArray(invitesData) && invitesData.length > 0)
			? invitesData
			: (storeInvites || []);
		if (!Array.isArray(source)) return [];
		return source.map((inv: any) => ({
			id: String(inv.id),
			email: inv.email,
			role: (inv.role || 'member').toLowerCase(),
			expiresAt: inv.expired_at
				? new Date(inv.expired_at * 1000).toISOString()
				: new Date().toISOString(),
		}));
	}, [invitesData, storeInvites]);

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [deleteTarget, setDeleteTarget] = useState<UserMember | null>(null);
	const [editingRoleUser, setEditingRoleUser] = useState<UserMember | null>(null);

	// Form State
	const [createMode, setCreateMode] = useState<'credentials' | 'invitation'>('invitation');
	const [formEmail, setFormEmail] = useState('');
	const [formRole, setFormRole] = useState('member');
	const [formPassword, setFormPassword] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [isDeleting, setIsDeleting] = useState(false);

	const filteredUsers = useMemo(() => {
		if (!searchQuery.trim()) return users;
		const q = searchQuery.toLowerCase();
		return users.filter(
			u => u.email.toLowerCase().includes(q) || u.role.toLowerCase().includes(q),
		);
	}, [users, searchQuery]);

	const filteredInvitations = useMemo(() => {
		if (!searchQuery.trim()) return invitations;
		const q = searchQuery.toLowerCase();
		return invitations.filter(i => i.email.toLowerCase().includes(q));
	}, [invitations, searchQuery]);

	const handleOpenCreate = () => {
		setFormEmail('');
		setFormRole('member');
		setFormPassword('');
		setIsCreateOpen(true);
	};

	const handleSaveUser = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!formEmail.trim()) return;

		setIsSubmitting(true);
		try {
			if (createMode === 'credentials') {
				if (!formPassword || formPassword.length < 8) {
					toast.error('Password must be at least 8 characters long');
					setIsSubmitting(false);
					return;
				}
				const res: any = await addMemberMutation.mutateAsync({
					body: {
						email: formEmail.trim(),
						password: formPassword,
						role: formRole.toUpperCase(),
					},
				} as any);

				if (res?.data || res) {
					addMemberStore(res?.data || res);
				}

				toast.success('User account created successfully');
				setActiveTab('users');
			} else {
				const res: any = await inviteMutation.mutateAsync({
					body: {
						email: formEmail.trim(),
						role: formRole.toUpperCase(),
						group_id: 1,
					},
				} as any);

				if (res?.data || res) {
					addInviteStore(res?.data || res);
				}

				toast.success('Invitation sent successfully');
				setActiveTab('invitations');
			}

			setIsCreateOpen(false);
			await refetchMembers();
			await refetchInvites();
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
			await refetchMembers();
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
			await refetchMembers();
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
			await refetchInvites();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full max-w-5xl mx-auto pb-12 animate-in fade-in duration-150">
			{/* OpenOxide Standard Header */}
			<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 border-b border-border/40 pb-4">
				<div className="flex items-center gap-3">
					<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
						<Users className="w-4 h-4 text-primary" />
					</div>
					<div>
						<h1 className="text-base font-semibold text-foreground leading-none">Users &amp; Access</h1>
						<p className="text-xs text-muted-foreground mt-1">
							Manage team members, roles, and pending organization invitations
						</p>
					</div>
				</div>

				<div className="flex items-center gap-2 sm:ml-auto">
					<Button size="sm" onClick={handleOpenCreate} className="h-8 text-xs gap-1.5 cursor-pointer">
						<Plus className="w-3.5 h-3.5" />
						Create User / Invite
					</Button>
				</div>
			</div>

			{/* Search + Filter Bar */}
			<div className="flex items-center justify-between gap-3 flex-wrap">
				<div className="relative flex-1 max-w-sm">
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search users or invitations..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-8 h-8 text-xs bg-muted/20 border-border/60"
					/>
				</div>

				<div className="flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/60">
					<button
						type="button"
						onClick={() => setActiveTab('users')}
						className={`px-3 py-1 text-xs font-semibold rounded-md transition-all cursor-pointer ${
							activeTab === 'users' ? 'bg-card text-foreground shadow-2xs border border-border/60' : 'text-muted-foreground hover:text-foreground'
						}`}
					>
						Active Users ({users.length})
					</button>
					<button
						type="button"
						onClick={() => setActiveTab('invitations')}
						className={`px-3 py-1 text-xs font-semibold rounded-md transition-all cursor-pointer ${
							activeTab === 'invitations' ? 'bg-card text-foreground shadow-2xs border border-border/60' : 'text-muted-foreground hover:text-foreground'
						}`}
					>
						Invitations ({invitations.length})
					</button>
				</div>
			</div>

			{/* Dark Mode Table Format for Users */}
			{activeTab === 'users' && (
				<div className="border border-border/70 shadow-xs rounded-xl overflow-hidden bg-card">
					{filteredUsers.length === 0 ? (
						<div className="p-12 text-center flex flex-col items-center justify-center gap-2">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-xs text-muted-foreground font-medium">
								{searchQuery ? `No users match "${searchQuery}"` : 'No users created yet'}
							</span>
							{!searchQuery && (
								<Button size="sm" variant="outline" onClick={handleOpenCreate} className="h-8 text-xs mt-1 font-semibold">
									<Plus className="size-3.5 mr-1.5" /> Add First User
								</Button>
							)}
						</div>
					) : (
						<Table>
							<TableHeader className="bg-muted/40 border-b border-border/60">
								<TableRow className="hover:bg-transparent border-border/60">
									<TableHead className="px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">User Email</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Role</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Status</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">2FA Security</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Joined Date</TableHead>
									<TableHead className="text-right px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Actions</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredUsers.map(member => {
									const isOwner = member.role === 'owner';
									const canManage = !isOwner && !member.isSelf;

									return (
										<TableRow key={member.id} className="border-b border-border/40 hover:bg-muted/20 transition-colors">
											<TableCell className="px-4 py-3 font-medium text-xs">
												<div className="flex items-center gap-3">
													<div className="size-7 rounded-full bg-primary/10 text-primary font-bold text-[11px] flex items-center justify-center border border-primary/20 shrink-0 overflow-hidden">
														{member.avatar ? (
															<img src={member.avatar} alt={member.email} className="size-full object-cover rounded-full" />
														) : (
															member.email.substring(0, 2).toUpperCase()
														)}
													</div>
													<span className="font-semibold text-xs text-foreground flex items-center gap-1.5">
														{member.email}
														{member.isSelf && (
															<span className="text-[10px] bg-primary/10 text-primary px-1.5 py-0.5 rounded font-mono font-bold">
																You
															</span>
														)}
													</span>
												</div>
											</TableCell>
											<TableCell className="text-center px-4 py-3">
												<Badge
													variant="outline"
													className={`text-[11px] font-semibold capitalize px-2.5 py-0.5 ${
														member.role === 'owner'
															? 'bg-purple-500/10 text-purple-500 border-purple-500/30'
															: member.role === 'admin'
															? 'bg-blue-500/10 text-blue-500 border-blue-500/30'
															: 'bg-emerald-500/10 text-emerald-500 border-emerald-500/30'
													}`}
												>
													{member.role}
												</Badge>
											</TableCell>
											<TableCell className="text-center px-4 py-3">
												<span className="inline-flex items-center gap-1.5 text-xs font-semibold text-emerald-500">
													<span className="size-1.5 rounded-full bg-emerald-500 animate-pulse" />
													{member.banned ? 'Deactivated' : 'Active'}
												</span>
											</TableCell>
											<TableCell className="text-center px-4 py-3">
												{member.twoFactorEnabled ? (
													<span className="inline-flex items-center gap-1 text-xs text-emerald-500 font-semibold">
														<ShieldCheck className="w-3.5 h-3.5" /> Enabled
													</span>
												) : (
													<span className="text-xs text-muted-foreground font-mono">
														Disabled
													</span>
												)}
											</TableCell>
											<TableCell className="text-center px-4 py-3 text-xs text-muted-foreground font-mono">
												{new Date(member.createdAt).toLocaleDateString()}
											</TableCell>
											<TableCell className="text-right px-4 py-3">
												{canManage ? (
													<DropdownMenu>
														<DropdownMenuTrigger
															render={
																<Button
																	variant="ghost"
																	size="icon"
																	className="size-8 text-muted-foreground hover:text-foreground"
																>
																	<MoreHorizontal className="size-4" />
																</Button>
															}
														/>
														<DropdownMenuContent align="end" className="w-40">
															<DropdownMenuLabel className="text-xs">
																Actions
															</DropdownMenuLabel>
															<DropdownMenuItem
																onClick={() => setEditingRoleUser(member)}
																className="cursor-pointer text-xs"
															>
																<Pencil className="size-3.5 mr-2" />
																Change Role
															</DropdownMenuItem>
															<DropdownMenuItem
																onClick={() => setDeleteTarget(member)}
																className="cursor-pointer text-xs text-rose-500 hover:text-rose-600"
															>
																<Trash2 className="size-3.5 mr-2" />
																Delete User
															</DropdownMenuItem>
														</DropdownMenuContent>
													</DropdownMenu>
												) : (
													<span className="text-xs text-muted-foreground/40 font-mono w-8 text-center">—</span>
												)}
											</TableCell>
										</TableRow>
									);
								})}
							</TableBody>
						</Table>
					)}
				</div>
			)}

			{/* Dark Mode Table Format for Invitations */}
			{activeTab === 'invitations' && (
				<div className="border border-border/70 shadow-xs rounded-xl overflow-hidden bg-card">
					{filteredInvitations.length === 0 ? (
						<div className="p-12 text-center flex flex-col items-center justify-center gap-2">
							<Mail className="size-8 text-muted-foreground/40" />
							<span className="text-xs text-muted-foreground font-medium">No pending invitations</span>
							<Button size="sm" variant="outline" onClick={handleOpenCreate} className="h-8 text-xs mt-1 font-semibold">
								<Plus className="size-3.5 mr-1.5" /> Send Invitation
							</Button>
						</div>
					) : (
						<Table>
							<TableHeader className="bg-muted/40 border-b border-border/60">
								<TableRow className="hover:bg-transparent border-border/60">
									<TableHead className="px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Invited Email</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Assigned Role</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Expires At</TableHead>
									<TableHead className="text-right px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Actions</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{filteredInvitations.map(inv => (
									<TableRow key={inv.id} className="border-b border-border/40 hover:bg-muted/20 transition-colors">
										<TableCell className="px-4 py-3 font-medium text-xs">
											<div className="flex items-center gap-2.5">
												<Mail className="size-4 text-primary shrink-0" />
												<span className="font-semibold text-foreground">{inv.email}</span>
											</div>
										</TableCell>
										<TableCell className="text-center px-4 py-3">
											<Badge variant="outline" className="text-xs capitalize font-semibold">
												{inv.role}
											</Badge>
										</TableCell>
										<TableCell className="text-center px-4 py-3 text-xs text-muted-foreground font-mono">
											{new Date(inv.expiresAt).toLocaleDateString()}
										</TableCell>
										<TableCell className="text-right px-4 py-3">
											<Button
												variant="ghost"
												size="sm"
												onClick={() => handleRevokeInvitation(inv.id)}
												className="h-8 text-xs text-rose-500 hover:bg-rose-500/10 font-semibold"
											>
												Revoke
											</Button>
										</TableCell>
									</TableRow>
								))}
							</TableBody>
						</Table>
					)}
				</div>
			)}

			{/* Create User / Invite Modal */}
			<Dialog open={isCreateOpen} onOpenChange={open => !open && setIsCreateOpen(false)}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
							<Users className="size-5 text-primary" />
							Create User / Invite Member
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Create direct account credentials or send an email invitation.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleSaveUser} className="flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Account Mode
							</label>
							<Select value={createMode} onValueChange={v => v && setCreateMode(v as any)}>
								<SelectTrigger className="h-10 text-xs bg-muted/30 border-border/80">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="credentials">Create Direct Credentials</SelectItem>
									<SelectItem value="invitation">Send Email Invitation</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Email Address <span className="text-destructive">*</span>
							</label>
							<Input
								type="email"
								placeholder="user@example.com"
								value={formEmail}
								onChange={e => setFormEmail(e.target.value)}
								className="h-10 text-xs bg-muted/30 border-border/80"
								autoFocus
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Assign Role
							</label>
							<Select value={formRole} onValueChange={v => v && setFormRole(v)}>
								<SelectTrigger className="h-10 text-xs bg-muted/30 border-border/80">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin">Admin</SelectItem>
									<SelectItem value="member">Member</SelectItem>
								</SelectContent>
							</Select>
						</div>

						{createMode === 'credentials' && (
							<div className="flex flex-col gap-1.5">
								<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
									Password <span className="text-destructive">*</span>
								</label>
								<Input
									type="password"
									placeholder="••••••••"
									value={formPassword}
									onChange={e => setFormPassword(e.target.value)}
									className="h-10 text-xs bg-muted/30 border-border/80"
								/>
							</div>
						)}

						<div className="flex items-center justify-end gap-3 pt-4 border-t border-border/60">
							<Button
								type="button"
								variant="ghost"
								onClick={() => setIsCreateOpen(false)}
								className="h-9 text-xs font-semibold"
							>
								Cancel
							</Button>
							<Button
								type="submit"
								disabled={isSubmitting}
								className="h-9 text-xs font-bold px-4 bg-primary text-primary-foreground hover:bg-primary/90"
							>
								{isSubmitting ? 'Saving...' : createMode === 'credentials' ? 'Create User' : 'Send Invite'}
							</Button>
						</div>
					</form>
				</DialogContent>
			</Dialog>

			{/* Edit Role Dialog */}
			<Dialog open={editingRoleUser !== null} onOpenChange={open => !open && setEditingRoleUser(null)}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 flex flex-col gap-4 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground">
							Change Role for {editingRoleUser?.email}
						</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Select the permission role level for this user
						</DialogDescription>
					</DialogHeader>

					<div className="flex items-center gap-3 py-2">
						<Button
							variant={editingRoleUser?.role === 'admin' ? 'default' : 'outline'}
							onClick={() => handleUpdateRole('admin')}
							className="flex-1 h-9 text-xs font-semibold"
						>
							Admin
						</Button>
						<Button
							variant={editingRoleUser?.role === 'member' ? 'default' : 'outline'}
							onClick={() => handleUpdateRole('member')}
							className="flex-1 h-9 text-xs font-semibold"
						>
							Member
						</Button>
					</div>
				</DialogContent>
			</Dialog>

			{/* Delete User Alert Dialog */}
			<AlertDialog open={deleteTarget !== null} onOpenChange={open => !open && setDeleteTarget(null)}>
				<AlertDialogContent className="rounded-2xl bg-card border border-border shadow-2xl">
					<AlertDialogHeader>
						<AlertDialogTitle>Delete User</AlertDialogTitle>
						<AlertDialogDescription className="text-xs">
							Are you sure you want to delete user "{deleteTarget?.email}"? This action cannot be undone.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter className="pt-2">
						<AlertDialogCancel disabled={isDeleting} className="h-9 text-xs font-semibold">Cancel</AlertDialogCancel>
						<AlertDialogAction
							onClick={confirmDeleteUser}
							disabled={isDeleting}
							className="h-9 text-xs font-bold bg-destructive text-destructive-foreground hover:bg-destructive/90"
						>
							{isDeleting ? 'Deleting...' : 'Delete'}
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
