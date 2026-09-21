import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
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

export const ROLES = ['owner', 'admin', 'developer', 'viewer'] as const;

/**
 * Controller for Settings → Team: live member/invite queries (invalidated by
 * the table-watch ORGANIZATION_* stream), the role/delete/invite mutations,
 * filter state, and the dialog open targets. Kept out of the page component so
 * the render layer stays a thin composition of cards + dialogs.
 */
export function useUsersManagement() {
	const membersQuery = usePermissionGroupMembers();
	const invitesQuery = usePermissionGroupInvites();
	const members = (membersQuery.data ?? []) as unknown as MemberItem[];
	const invites = (invitesQuery.data ?? []) as unknown as InviteItem[];
	const userProfile = useProfile().data;
	const {can, canAny} = usePermissions();

	const addMemberMutation = useMemberAdd();
	const inviteMutation = useInviteCreate();
	const cancelInviteMutation = useInviteCancel();
	const updateRoleMutation = useMemberUpdateRole();
	const removeMemberMutation = useMemberRemove();

	const isLoading = membersQuery.isLoading;

	const [searchQuery, setSearchQuery] = useState('');
	const [roleFilter, setRoleFilter] = useState('ALL');

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [actionType, setActionType] = useState<'direct' | 'invite'>(
		'direct',
	);
	const [inputEmail, setInputEmail] = useState('');
	const [selectedRole, setSelectedRole] = useState('developer');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const [editingRoleUser, setEditingRoleUser] =
		useState<MemberItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<MemberItem | null>(
		null,
	);
	const [isDeleting, setIsDeleting] = useState(false);

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
		try {
			await updateRoleMutation.mutateAsync({
				id: Number(editingRoleUser.id),
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
		setIsDeleting(true);
		try {
			await removeMemberMutation.mutateAsync(Number(deleteTarget.id));
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

	const changeRole = (member: MemberItem, role: string) =>
		updateRoleMutation
			.mutateAsync({id: Number(member.id), role: role.toUpperCase()})
			.then(() => toast.success('Role updated'))
			.catch(err => toast.error(formatApiError(err)));

	const isSelf = (m: MemberItem) =>
		Boolean(
			userProfile &&
			((userProfile.email &&
				userProfile.email.toLowerCase() === m.email.toLowerCase()) ||
				(userProfile.id &&
					m.user_id &&
					Number(userProfile.id) === Number(m.user_id))),
		);

	const canManageMember = (m: MemberItem) =>
		can('member:update') && !isSelf(m) && m.role.toLowerCase() !== 'owner';

	return {
		members,
		invites,
		userProfile,
		can,
		canAny,
		isLoading,
		searchQuery,
		setSearchQuery,
		roleFilter,
		setRoleFilter,
		isCreateOpen,
		setIsCreateOpen,
		actionType,
		setActionType,
		inputEmail,
		setInputEmail,
		selectedRole,
		setSelectedRole,
		isSubmitting,
		editingRoleUser,
		setEditingRoleUser,
		deleteTarget,
		setDeleteTarget,
		isDeleting,
		filteredMembers,
		handleCreateOrInvite,
		handleUpdateRole,
		confirmDeleteUser,
		handleRevokeInvitation,
		changeRole,
		isSelf,
		canManageMember,
	};
}

export type UsersManagementController = ReturnType<
	typeof useUsersManagement
>;
