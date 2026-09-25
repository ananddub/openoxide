import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useInviteCancel,
	useMemberAdd,
	useMemberGroupAssign,
	useMemberRemove,
	useMemberUpdateRole,
	useInviteCreate,
	usePermissionGroupInvites,
	usePermissionGroupList,
	usePermissionGroupMembers,
	useProfile,
	type MemberItem,
	type InviteItem,
	type PermissionGroupPayload,
} from '#/api/live';
import {usePermissions} from '#/hooks/permissions/use-permissions';

export const ROLES = ['owner', 'admin', 'member'] as const;

/** What each role can do — shown wherever a role is being picked. Fine-grained rights come from the member's permission group. */
export const ROLE_DESC: Record<string, string> = {
	owner: 'Full access, including billing and removing members',
	admin: 'Manage projects, deployments and team members',
	member: 'Works on whatever its permission group grants',
};

/** Owner first, then alphabetically — the list never reshuffles on refresh. */
const ROLE_ORDER: Record<string, number> = {
	owner: 0,
	admin: 1,
	member: 2,
};

/** Controller for Settings → Team: live member/invite queries (invalidated by the table-watch ORGANIZATION_* stream), the role/delete/invite mutations, filter state, and the dialog open targets. Kept out of the page component so the render layer stays a thin composition of cards + dialogs. */
export function useUsersManagement() {
	const membersQuery = usePermissionGroupMembers();
	const invitesQuery = usePermissionGroupInvites();
	const groupsQuery = usePermissionGroupList();
	const members = (membersQuery.data ?? []) as unknown as MemberItem[];
	const groups = useMemo(
		() => (groupsQuery.data ?? []) as PermissionGroupPayload[],
		[groupsQuery.data],
	);
	const invites = useMemo(
		() =>
			((invitesQuery.data ?? []) as unknown as InviteItem[])
				.slice()
				.sort(
					(a, b) => Number(b.created_at ?? 0) - Number(a.created_at ?? 0),
				),
		[invitesQuery.data],
	);
	const userProfile = useProfile().data;
	const {can, canAny} = usePermissions();

	const addMemberMutation = useMemberAdd();
	const inviteMutation = useInviteCreate();
	const cancelInviteMutation = useInviteCancel();
	const updateRoleMutation = useMemberUpdateRole();
	const removeMemberMutation = useMemberRemove();
	const assignGroupMutation = useMemberGroupAssign();

	const isLoading = membersQuery.isLoading;

	const [searchQuery, setSearchQuery] = useState('');
	const [roleFilter, setRoleFilter] = useState('ALL');

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [actionType, setActionType] = useState<'direct' | 'invite'>(
		'direct',
	);
	const [inputEmail, setInputEmail] = useState('');
	const [selectedRole, setSelectedRole] = useState('member');
	const [selectedGroupId, setSelectedGroupId] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const [editingRoleUser, setEditingRoleUser] =
		useState<MemberItem | null>(null);
	const [overridesTarget, setOverridesTarget] =
		useState<MemberItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<MemberItem | null>(
		null,
	);
	const [isDeleting, setIsDeleting] = useState(false);

	const filteredMembers = useMemo(() => {
		const q = searchQuery.trim().toLowerCase();
		return members
			.filter(m => {
				const email = (m.email ?? '').toLowerCase();
				const name = (m.name ?? '').toLowerCase();
				const matchesSearch = !q || email.includes(q) || name.includes(q);
				const matchesRole =
					roleFilter === 'ALL' ||
					(m.role ?? '').toLowerCase() === roleFilter.toLowerCase();
				return matchesSearch && matchesRole;
			})
			.sort((a, b) => {
				const ra = ROLE_ORDER[(a.role ?? '').toLowerCase()] ?? 9;
				const rb = ROLE_ORDER[(b.role ?? '').toLowerCase()] ?? 9;
				if (ra !== rb) return ra - rb;
				return (a.name || a.email || '').localeCompare(
					b.name || b.email || '',
				);
			});
	}, [members, searchQuery, roleFilter]);

	const openCreate = (mode: 'direct' | 'invite') => {
		setActionType(mode);
		setInputEmail('');
		setSelectedRole('member');
		setSelectedGroupId('');
		setIsCreateOpen(true);
	};

	const handleCreateOrInvite = async (e: React.FormEvent) => {
		e.preventDefault();
		const email = inputEmail.trim().toLowerCase();
		if (!email) return;
		setIsSubmitting(true);
		const groupId = selectedGroupId ? Number(selectedGroupId) : undefined;
		try {
			if (actionType === 'direct') {
				await addMemberMutation.mutateAsync({
					email,
					role: selectedRole.toUpperCase(),
					group_id: groupId,
				});
				toast.success(`${email} added to the workspace`);
			} else {
				await inviteMutation.mutateAsync({
					email,
					role: selectedRole.toUpperCase(),
					group_id: groupId,
				});
				toast.success(`Invitation sent to ${email}`);
			}
			setIsCreateOpen(false);
			setInputEmail('');
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleUpdateRole = async (newRole: string) => {
		if (!editingRoleUser) return;
		try {
			// The backend addresses memberships by user id, not by the row id.
			await updateRoleMutation.mutateAsync({
				id: Number(editingRoleUser.user_id),
				role: newRole.toUpperCase(),
			});
			toast.success('User role updated');
			setEditingRoleUser(null);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const handleAssignGroup = async (groupId: string) => {
		if (!editingRoleUser) return;
		try {
			await assignGroupMutation.mutateAsync({
				user_id: Number(editingRoleUser.user_id),
				group_id: groupId === '' ? 0 : Number(groupId),
			});
			toast.success('Permission group updated');
			setEditingRoleUser(null);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const confirmDeleteUser = async () => {
		if (!deleteTarget) return;
		setIsDeleting(true);
		try {
			await removeMemberMutation.mutateAsync(Number(deleteTarget.user_id));
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
		groups,
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
		selectedGroupId,
		setSelectedGroupId,
		isSubmitting,
		editingRoleUser,
		setEditingRoleUser,
		overridesTarget,
		setOverridesTarget,
		deleteTarget,
		setDeleteTarget,
		isDeleting,
		filteredMembers,
		openCreate,
		handleCreateOrInvite,
		handleUpdateRole,
		handleAssignGroup,
		confirmDeleteUser,
		handleRevokeInvitation,
		isSelf,
		canManageMember,
	};
}

export type UsersManagementController = ReturnType<
	typeof useUsersManagement
>;
